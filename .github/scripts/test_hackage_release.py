"""Credential-free Hackage identity, immutable recovery and train guards."""

import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
import urllib.error
from unittest.mock import patch

import test_release as common

ROOT, VERSION = common.ROOT, common.VERSION

spec = importlib.util.spec_from_file_location(
    "hackage_release", ROOT / "haskell/bin/release.py"
)
hackage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hackage)


class HaskellTrainTests(unittest.TestCase):
    setUp = common.ReleaseTests.setUp
    git = common.ReleaseTests.git
    commit = common.ReleaseTests.commit
    tags = common.ReleaseTests.tags

    def test_haskell_activation_blocks_all_ten_publishers(self):
        policy = self.root / "haskell/release.json"
        value = json.loads(policy.read_text())
        value["enabled"] = False
        policy.write_text(json.dumps(value))
        self.commit()
        self.git("update-ref", "refs/remotes/origin/master", "HEAD")
        self.tags()
        before = self.git("show-ref")
        for language, prefix in hackage.preflight.LANGUAGES.items():
            with (
                self.subTest(language=language),
                self.assertRaisesRegex(ValueError, "Haskell publication is disabled"),
            ):
                hackage.preflight.publisher(
                    self.root, language, f"refs/tags/{prefix}/v{VERSION}"
                )
        self.assertEqual(before, self.git("show-ref"))

    def test_haskell_skip_spec_version_and_owner_fail_closed(self):
        for file, old, new, error in [
            (
                "haskell/slackblocks.cabal",
                f"version: {VERSION}",
                "version: 9.9.9",
                "versions differ",
            ),
            (
                "haskell/src/Slackblocks/Models.hs",
                'specVersion = "1.4.0"',
                'specVersion = "0.0.0"',
                "spec version",
            ),
            (
                "docs/examples/multiselect-compat/rust/Cargo.lock",
                f'version = "{VERSION}"',
                'version = "9.9.9"',
                "documentation consumer",
            ),
            ("haskell/release.json", '"maintainer"', "null", "owner"),
        ]:
            path = self.root / file
            original = path.read_text()
            path.write_text(original.replace(old, new))
            with self.subTest(file=file), self.assertRaisesRegex(ValueError, error):
                hackage.preflight.validate(self.root, strict=True)
            path.write_text(original)
        (self.root / "haskell/conformance/skiplist.txt").write_text(
            "unsupported.json\n"
        )
        with self.assertRaisesRegex(ValueError, "skip list"):
            hackage.preflight.validate(self.root)


class HackageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(
            prefix="hackage-tests-", dir=hackage.scratch()
        )
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "haskell").mkdir()
        self.policy = {
            "enabled": True,
            "owner": "maintainer",
            "first_publication": True,
        }
        self.save()

    def save(self):
        (self.root / "haskell/release.json").write_text(json.dumps(self.policy))

    def response(self, path, **kwargs):
        return {
            "/users/account-management": "/user/maintainer/manage",
            "/user/maintainer/enabled.json": b'{"enabled":true}',
            "/packages/uploaders.json": b'{"members":[{"username":"maintainer"}]}',
            "/package/slackblocks.json": None,
            "/package/slackblocks/maintainers.json": b'{"members":[{"username":"maintainer"}]}',
        }[path]

    def test_read_only_owner_and_first_publish_checks(self):
        with (
            patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
            patch.object(hackage, "fetch", side_effect=self.response) as fetch,
        ):
            self.assertEqual(hackage.account(self.root), "fake123")
            self.assertEqual(fetch.call_args_list[1].kwargs["token"], "fake123")
            self.policy["first_publication"] = False
            self.save()
            with self.assertRaisesRegex(ValueError, "first-publication"):
                hackage.account(self.root)

    def test_foreign_owner_disabled_account_permissions_and_errors_reject(self):
        for path, response, message in [
            ("/users/account-management", "/user/foreign/manage", "Token owner"),
            (
                "/users/account-management",
                "https://evil.invalid/user/maintainer/manage",
                "Token owner",
            ),
            ("/user/maintainer/enabled.json", b'{"enabled":false}', "disabled"),
            ("/packages/uploaders.json", b'{"members":[]}', "uploader"),
            (
                "/package/slackblocks/maintainers.json",
                b'{"members":[]}',
                "does not maintain",
            ),
        ]:

            def fake(url, **kw):
                if url == path:
                    return response
                if url == "/package/slackblocks.json":
                    return b'{"3.0.0":"normal"}'
                return self.response(url, **kw)

            with (
                self.subTest(path=path),
                patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
                patch.object(hackage, "fetch", side_effect=fake),
            ):
                with self.assertRaisesRegex(ValueError, message):
                    hackage.account(self.root)
        with (
            patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
            patch.object(hackage, "fetch", side_effect=OSError("offline")),
        ):
            with self.assertRaises(OSError):
                hackage.account(self.root)

    def test_registry_404_is_only_absence_and_redirect_is_not_followed(self):
        for code in [401, 403, 429, 500, 404, 302]:

            def fail(request, **kw):
                raise urllib.error.HTTPError(
                    request.full_url,
                    code,
                    "failure",
                    {"Location": "/user/maintainer/manage"},
                    None,
                )

            if code == 404:
                self.assertIsNone(
                    hackage.fetch("/package/slackblocks.json", absent=True, opener=fail)
                )
            elif code == 302:
                self.assertEqual(
                    hackage.fetch(
                        "/users/account-management", redirect=True, opener=fail
                    ),
                    "/user/maintainer/manage",
                )
            else:
                with self.assertRaises(urllib.error.HTTPError):
                    hackage.fetch("/package/slackblocks.json", absent=True, opener=fail)
        self.assertIsNone(
            hackage.NoRedirect().redirect_request(
                None, None, 302, "", {}, "https://evil.invalid"
            )
        )

    def tar(self, path="slackblocks-3.1.0-docs/index.html", content=b"API", mode=0o644):
        result = io.BytesIO()
        with tarfile.open(fileobj=result, mode="w:gz") as archive:
            member = tarfile.TarInfo(path)
            member.size, member.mode = len(content), mode
            archive.addfile(member, io.BytesIO(content))
        return result.getvalue()

    def test_existing_source_and_docs_must_match_and_unsafe_archive_rejects(self):
        file = self.root / "slackblocks-3.1.0-docs.tar.gz"
        file.write_bytes(self.tar())
        self.assertTrue(hackage.matches(self.tar(mode=0o600), file, True))
        self.assertFalse(hackage.matches(None, file, True))
        with self.assertRaisesRegex(ValueError, "content differs"):
            hackage.matches(self.tar(content=b"wrong"), file, True)
        with self.assertRaisesRegex(ValueError, "checksum"):
            hackage.matches(b"wrong", file)
        with self.assertRaisesRegex(ValueError, "Unsafe"):
            hackage.archive_members(self.tar("../escape"), "slackblocks-3.1.0-docs/")

    def test_token_never_enters_command_arguments_and_config_is_removed(self):
        observed = []

        def fake(command, **kwargs):
            self.assertNotIn("fake123", " ".join(command))
            config = Path(command[1].split("=", 1)[1])
            self.assertEqual(config.stat().st_mode & 0o777, 0o600)
            self.assertIn("hackage-token: fake123", config.read_text())
            self.assertIn("--documentation", command)
            self.assertIn("--publish", command)
            observed.append(config)
            return subprocess.CompletedProcess(command, 1)

        with patch.object(hackage.subprocess, "run", side_effect=fake):
            with self.assertRaisesRegex(ValueError, "uncertain"):
                hackage.upload(self.root / "docs.tar.gz", "fake123", True)
        self.assertFalse(observed[0].exists())

    def test_wrong_tag_stops_before_credentials_or_writes(self):
        with (
            patch.object(
                hackage.preflight, "publisher", side_effect=ValueError("wrong tag")
            ),
            patch.object(hackage, "account") as account,
            patch.object(hackage, "upload") as upload,
        ):
            with self.assertRaisesRegex(ValueError, "wrong tag"):
                hackage.publish(self.root, self.root)
            account.assert_not_called()
            upload.assert_not_called()

    def test_recovery_uploads_only_missing_docs_and_uncertain_write_stops(self):
        version = "3.1.0"
        source, docs = [self.root / name for name in hackage.names(version)]
        source.write_bytes(
            self.tar("slackblocks-3.1.0/slackblocks.cabal", b"version: 3.1.0")
        )
        docs.write_bytes(self.tar())
        replies = [source.read_bytes(), None, docs.read_bytes(), b"version: 3.1.0"]
        with (
            patch.object(hackage.preflight, "publisher", return_value=version),
            patch.object(hackage, "verify", return_value={"version": version}),
            patch.object(hackage, "account", return_value="fake123"),
            patch.object(hackage, "fetch", side_effect=replies),
            patch.object(hackage, "upload") as upload,
        ):
            hackage.publish(self.root, self.root)
            upload.assert_called_once_with(docs, "fake123", True)
        with (
            patch.object(hackage.preflight, "publisher", return_value=version),
            patch.object(hackage, "verify", return_value={"version": version}),
            patch.object(hackage, "account", return_value="fake123"),
            patch.object(hackage, "fetch", return_value=None),
            patch.object(
                hackage, "upload", side_effect=ValueError("uncertain")
            ) as upload,
        ):
            with self.assertRaisesRegex(ValueError, "uncertain"):
                hackage.publish(self.root, self.root)
            self.assertEqual(upload.call_count, 1)

    def test_artifact_identity_checksum_and_reproduction_are_checked(self):
        version = "3.1.0"
        files = [self.root / name for name in hackage.names(version)]
        files[0].write_bytes(self.tar("slackblocks-3.1.0/slackblocks.cabal"))
        files[1].write_bytes(self.tar())
        identity = {
            "version": version,
            "commit": "a" * 40,
            "toolchain": hackage.TOOLCHAIN,
            "dependencies_sha256": "b" * 64,
        }
        record = {**identity, "assets": {p.name: hackage.digest(p) for p in files}}
        receipt = self.root / "release.json"
        receipt.write_text(json.dumps(record))
        with patch.object(hackage, "identity", return_value=identity):
            self.assertEqual(hackage.verify(self.root, self.root), record)
            with patch.object(
                hackage,
                "source",
                side_effect=lambda root, out: (out / files[0].name).write_bytes(
                    b"changed"
                ),
            ):
                with self.assertRaisesRegex(ValueError, "reproduction"):
                    hackage.verify(self.root, self.root, rebuild=True)
            files[1].write_bytes(b"tampered")
            with self.assertRaisesRegex(ValueError, "checksum"):
                hackage.verify(self.root, self.root)
        with patch.object(
            hackage, "identity", return_value={**identity, "commit": "c" * 40}
        ):
            with self.assertRaisesRegex(ValueError, "identity"):
                hackage.verify(self.root, self.root)

    def test_workflow_pr_cannot_publish_and_every_job_is_hosted(self):
        source = (ROOT / ".github/workflows/publish-hackage.yml").read_text()
        self.assertEqual(source.count("runs-on: ubuntu-24.04"), 4)
        self.assertEqual(source.count("if: github.event_name != 'pull_request'"), 4)
        self.assertNotIn("secrets.", source.split("  publish:")[0])
        self.assertIn("environment:\n      name: hackage", source)
        coordinator = (ROOT / ".github/workflows/coordinated-release.yml").read_text()
        self.assertIn(
            "needs: [validate, php-readiness, hex-readiness, hackage-readiness]",
            coordinator,
        )


if __name__ == "__main__":
    unittest.main()
