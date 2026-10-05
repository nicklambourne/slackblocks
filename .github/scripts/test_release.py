"""Exercise release decisions with real isolated Git repositories and fake registry responses."""

import importlib.util
import io
import json
import os
import subprocess
import tempfile
import unittest
import urllib.error
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


release = load("release", ROOT / ".github/scripts/verify-release.py")
cargo = load("cargo_release", ROOT / "rust/bin/release.py")
SOURCE_VERSION = release.rust_package(ROOT)["version"]
VERSION = "2.6.0"
FILES = [
    "python/pyproject.toml",
    "python/uv.lock",
    "typescript/package.json",
    "go/go.mod",
    "java/pom.xml",
    "csharp/src/Slackblocks/Slackblocks.csproj",
    "ruby/lib/slackblocks/version.rb",
    "ruby/Gemfile.lock",
    "rust/Cargo.toml",
    "rust/Cargo.lock",
    "rust/integrations/Cargo.lock",
]


class ReleaseTests(unittest.TestCase):
    def setUp(self):
        scratch = None
        if Path("/Volumes/Repos").exists():
            for mount in ("Repos", "Cache", "Scratch"):
                self.assertTrue(os.path.ismount("/Volumes/" + mount))
            scratch = "/Volumes/Scratch"
        self.temp = tempfile.TemporaryDirectory(prefix="slackblocks-release-test-", dir=scratch)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for name in FILES + [language + "/CHANGELOG.md" for language in release.LANGUAGES]:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text((ROOT / name).read_text().replace(SOURCE_VERSION, VERSION))
        import re

        manifest = self.root / "rust/Cargo.toml"
        text = re.sub(
            r"^publish = .+$", 'publish = ["crates-io"]', manifest.read_text(), flags=re.M
        )
        text = re.sub(r"^authentication = .+$", 'authentication = "bootstrap"', text, flags=re.M)
        text = re.sub(r"^bootstrap-version = .+\n", "", text, flags=re.M)
        text = text.replace(
            'authentication = "bootstrap"',
            'authentication = "bootstrap"\nbootstrap-version = "2.6.0"',
        )
        manifest.write_text(text)
        for language in release.LANGUAGES:
            (self.root / language / "CHANGELOG.md").write_text(
                f"## [{VERSION}] — 2026-10-04\n\nRelease notes.\n"
            )
        pom = self.root / "java/pom.xml"
        import re

        pom.write_text(
            re.sub(
                r"<project.build.outputTimestamp>[^<]+<",
                "<project.build.outputTimestamp>2026-10-04T00:00:00Z<",
                pom.read_text(),
            )
        )
        self.git("init", "--quiet")
        self.git("config", "user.name", "Release guard tests")
        self.git("config", "user.email", "release-tests@example.invalid")
        self.commit()
        self.git("update-ref", "refs/remotes/origin/master", "HEAD")

    def git(self, *args):
        return subprocess.check_output(
            ["git", "-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false", *args],
            cwd=self.root,
            text=True,
            stderr=subprocess.PIPE,
        ).strip()

    def commit(self):
        self.git("add", ".")
        self.git("commit", "--quiet", "--allow-empty", "-m", "test state")

    def edit(self, name, old, new):
        path = self.root / name
        self.assertIn(old, path.read_text())
        path.write_text(path.read_text().replace(old, new))

    def tags(self):
        for prefix in release.LANGUAGES.values():
            self.git("tag", "-a", f"{prefix}/v{VERSION}", "-m", "fixture release")

    def test_valid_coordinator_before_and_after_all_tags(self):
        self.assertEqual(
            release.coordinator(self.root, VERSION, "workflow_dispatch", "refs/heads/master"),
            VERSION,
        )
        self.tags()
        self.assertEqual(
            release.coordinator(self.root, VERSION, "workflow_dispatch", "refs/heads/master"),
            VERSION,
        )

    def test_all_seven_publishers_require_their_own_tag(self):
        self.tags()
        for language, prefix in release.LANGUAGES.items():
            with self.subTest(language=language):
                self.assertEqual(
                    release.publisher(self.root, language, f"refs/tags/{prefix}/v{VERSION}"),
                    VERSION,
                )
                with self.assertRaisesRegex(release.ReleaseError, "exact"):
                    release.publisher(self.root, language, "refs/heads/master")

    def test_disabled_activation_blocks_every_publisher(self):
        self.edit("rust/Cargo.toml", 'publish = ["crates-io"]', "publish = false")
        self.commit()
        self.git("update-ref", "refs/remotes/origin/master", "HEAD")
        self.tags()
        before = self.git("show-ref")
        for language, prefix in release.LANGUAGES.items():
            with (
                self.subTest(language=language),
                self.assertRaisesRegex(release.ReleaseError, "disabled"),
            ):
                release.publisher(self.root, language, f"refs/tags/{prefix}/v{VERSION}")
        with self.assertRaisesRegex(release.ReleaseError, "disabled"):
            release.coordinator(self.root, VERSION, "workflow_dispatch", "refs/heads/master")
        self.assertEqual(before, self.git("show-ref"), "Preflight must never create or move tags")

    def test_pull_request_allows_disabled_unreleased_source(self):
        self.edit("rust/Cargo.toml", 'publish = ["crates-io"]', "publish = false")
        for language in release.LANGUAGES:
            self.edit(language + "/CHANGELOG.md", "2026-10-04", "Unreleased")
        self.assertEqual(
            release.coordinator(self.root, VERSION, "pull_request", "refs/pull/1/merge"),
            VERSION,
        )

    def test_requested_version_mismatch(self):
        with self.assertRaisesRegex(release.ReleaseError, "versions differ"):
            release.validate(self.root, "9.9.9")

    def test_rust_manifest_mismatch(self):
        self.edit("rust/Cargo.toml", f'version = "{VERSION}"', 'version = "9.9.9"')
        with self.assertRaisesRegex(release.ReleaseError, "versions differ"):
            release.validate(self.root, VERSION)

    def test_own_lockfiles_must_match(self):
        for name in (
            "python/uv.lock",
            "ruby/Gemfile.lock",
            "rust/Cargo.lock",
            "rust/integrations/Cargo.lock",
        ):
            path = self.root / name
            original = path.read_text()
            path.write_text(original.replace(VERSION, "9.9.9"))
            with (
                self.subTest(name=name),
                self.assertRaisesRegex(release.ReleaseError, "lockfile"),
            ):
                release.validate(self.root)
            path.write_text(original)

    def test_wrong_go_major(self):
        self.edit("go/go.mod", "/go/v2", "/go/v3")
        with self.assertRaisesRegex(release.ReleaseError, "Go module"):
            release.validate(self.root)

    def test_missing_or_duplicate_release_notes(self):
        path = self.root / "rust/CHANGELOG.md"
        original = path.read_text()
        for text in (
            original.replace(f"## [{VERSION}]", "## [9.9.9]"),
            original + f"\n## [{VERSION}] — 2026-10-04\n",
        ):
            path.write_text(text)
            with self.assertRaisesRegex(release.ReleaseError, "exactly one"):
                release.validate(self.root)

    def test_empty_release_notes(self):
        (self.root / "rust/CHANGELOG.md").write_text(f"## [{VERSION}] — 2026-10-04\n\n")
        with self.assertRaisesRegex(release.ReleaseError, "must not be empty"):
            release.validate(self.root)

    def test_missing_invalid_and_different_dates(self):
        path = self.root / "rust/CHANGELOG.md"
        original = path.read_text()
        for date in ("Unreleased", "2026-02-30", "2026-10-03"):
            path.write_text(original.replace("2026-10-04", date))
            with self.subTest(date=date), self.assertRaises(release.ReleaseError):
                release.validate(self.root, strict=True)

    def test_java_timestamp_mismatch(self):
        self.edit("java/pom.xml", "2026-10-04T00:00:00Z", "2026-10-03T00:00:00Z")
        with self.assertRaisesRegex(release.ReleaseError, "timestamp"):
            release.validate(self.root, strict=True)

    def test_dispatch_requires_master(self):
        with self.assertRaisesRegex(release.ReleaseError, "from master"):
            release.coordinator(self.root, VERSION, "workflow_dispatch", "refs/heads/feature")

    def test_partial_tags_fail(self):
        self.git("tag", "-a", f"rust/v{VERSION}", "-m", "partial")
        with self.assertRaisesRegex(release.ReleaseError, "Partial"):
            release.tag_state(self.root, VERSION, self.git("rev-parse", "HEAD"))

    def test_tags_must_name_the_same_commit(self):
        parent = self.git("rev-parse", "HEAD")
        self.commit()
        self.git("tag", "-a", f"rust/v{VERSION}", parent, "-m", "wrong commit")
        with self.assertRaisesRegex(release.ReleaseError, "different commit"):
            release.tag_state(self.root, VERSION, self.git("rev-parse", "HEAD"))

    def test_lightweight_tag_rejected(self):
        self.git("tag", f"rust/v{VERSION}")
        with self.assertRaisesRegex(release.ReleaseError, "annotated"):
            release.tag_state(self.root, VERSION, self.git("rev-parse", "HEAD"))

    def test_publisher_requires_all_tags(self):
        with self.assertRaises(subprocess.CalledProcessError):
            release.publisher(self.root, "rust", f"refs/tags/rust/v{VERSION}")

    def test_dirty_or_unmerged_commit_rejected(self):
        self.edit("typescript/package.json", '"sideEffects": false', '"sideEffects": true')
        with self.assertRaisesRegex(release.ReleaseError, "modifications"):
            release.release_commit(self.root)
        self.commit()
        with self.assertRaisesRegex(release.ReleaseError, "origin/master"):
            release.release_commit(self.root)

    def test_authentication_is_explicit_and_first_version_only(self):
        self.assertEqual(cargo.authentication(self.root), "bootstrap")
        self.edit(
            "rust/Cargo.toml",
            'bootstrap-version = "2.6.0"',
            'bootstrap-version = "9.9.9"',
        )
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "first Rust release"):
            cargo.authentication(self.root)
        self.edit(
            "rust/Cargo.toml",
            'authentication = "bootstrap"',
            'authentication = "trusted"',
        )
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "Remove bootstrap"):
            cargo.authentication(self.root)
        self.edit("rust/Cargo.toml", 'bootstrap-version = "9.9.9"\n', "")
        self.assertEqual(cargo.authentication(self.root), "trusted")
        self.edit(
            "rust/Cargo.toml",
            'authentication = "trusted"',
            'authentication = "fallback"',
        )
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "explicit"):
            cargo.authentication(self.root)

    def test_artifact_checks_metadata_contents_presence_and_reproduction(self):
        directory = self.root / "artifact"
        directory.mkdir()
        archive = directory / f"slackblocks-{VERSION}.crate"
        archive.write_bytes(b"tested archive bytes")
        record = {
            "version": VERSION,
            "commit": self.git("rev-parse", "HEAD"),
            "toolchain": cargo.TOOLCHAIN,
            "authentication": "bootstrap",
            "sha256": cargo.sha256(archive),
        }
        manifest = directory / "release.json"
        manifest.write_text(json.dumps(record))
        self.assertEqual(cargo.verify_artifact(self.root, directory), record)
        for key in ("version", "commit", "toolchain", "authentication"):
            manifest.write_text(json.dumps({**record, key: "wrong"}))
            with (
                self.subTest(key=key),
                self.assertRaisesRegex(cargo.preflight.ReleaseError, "metadata"),
            ):
                cargo.verify_artifact(self.root, directory)
        manifest.write_text(json.dumps(record))
        with patch.object(cargo, "package", return_value=archive):
            self.assertEqual(cargo.verify_artifact(self.root, directory, rebuild=True), record)
        other = self.root / "other.crate"
        other.write_bytes(b"different package")
        with (
            patch.object(cargo, "package", return_value=other),
            self.assertRaisesRegex(cargo.preflight.ReleaseError, "reproduction"),
        ):
            cargo.verify_artifact(self.root, directory, rebuild=True)
        archive.write_bytes(b"tampered")
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "checksum"):
            cargo.verify_artifact(self.root, directory)
        archive.unlink()
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "exactly"):
            cargo.verify_artifact(self.root, directory)


class RegistryTests(unittest.TestCase):
    def absent(self, *args, **kwargs):
        raise urllib.error.HTTPError("https://crates.io", 404, "absent", {}, None)

    def exists(self, *args, **kwargs):
        return io.StringIO(json.dumps({"versions": [{"num": VERSION}]}))

    def test_bootstrap_requires_absent_crate(self):
        cargo.check_registry(VERSION, "bootstrap", self.absent)
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "already exists"):
            cargo.check_registry(VERSION, "bootstrap", self.exists)

    def test_trusted_requires_existing_crate_and_new_version(self):
        cargo.check_registry("9.9.9", "trusted", self.exists)
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "already published"):
            cargo.check_registry(VERSION, "trusted", self.exists)
        with self.assertRaisesRegex(cargo.preflight.ReleaseError, "existing crate"):
            cargo.check_registry(VERSION, "trusted", self.absent)

    def test_registry_errors_fail_closed(self):
        for status in (403, 429, 500):

            def failed(*args, response_status=status, **kwargs):
                raise urllib.error.HTTPError(
                    "https://crates.io", response_status, "unavailable", {}, None
                )

            with self.subTest(status=status), self.assertRaises(urllib.error.HTTPError):
                cargo.check_registry(VERSION, "bootstrap", failed)


class WorkflowTests(unittest.TestCase):
    def test_every_publisher_uses_shared_guard(self):
        names = {
            "python": "publish",
            "typescript": "publish-npm",
            "go": "publish-go",
            "java": "publish-java",
            "csharp": "publish-nuget",
            "ruby": "publish-rubygems",
        }
        for language, name in names.items():
            text = (ROOT / f".github/workflows/{name}.yml").read_text()
            self.assertIn(f"verify-release.py publisher --language {language}", text)
            self.assertIn("fetch-depth: 0", text)
        rust = (ROOT / ".github/workflows/publish-crates.yml").read_text()
        self.assertIn("release.py registry", rust)
        self.assertIn("release.py verify", rust)
        self.assertLess(rust.index("release.py verify"), rust.index("crates-io-auth-action@"))
        self.assertIn("--no-verify", rust)
        self.assertIn("if: github.event_name != 'pull_request'\n    needs: [build, artifact]", rust)
        self.assertNotIn("secrets.", rust.split("  publish:")[0])
        self.assertNotIn("id-token: write", rust.split("  publish:")[0])
        coordinator = (ROOT / ".github/workflows/coordinated-release.yml").read_text()
        for prefix in release.LANGUAGES.values():
            self.assertIn(f'"{prefix}/v$VERSION"', coordinator)
            self.assertIn(f"tag_prefix: {prefix}/v", coordinator)
        self.assertIn("verify-release.py coordinator", coordinator)
        self.assertIn("git push --atomic", coordinator)


if __name__ == "__main__":
    unittest.main()
