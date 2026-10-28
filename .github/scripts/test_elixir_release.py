"""Exercise Hex gates and partial-publication recovery without credentials or writes."""

import io
import json
import os
import subprocess
import sys
import unittest
import urllib.error
from unittest.mock import patch

import test_release as fixtures

sys.path.insert(0, str(fixtures.ROOT / "elixir/tools"))
hex_release = fixtures.load("hex_release", fixtures.ROOT / "elixir/tools/release.py")


class ElixirReleaseTests(unittest.TestCase):
    def setUp(self):
        self.fixture = fixtures.ReleaseTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.doCleanups)
        self.root = self.fixture.root
        self.directory = self.root / "artifact"
        self.metadata = {"version": fixtures.VERSION}

    def artifact(self):
        def build(_root, destination):
            destination.write_bytes(b"deterministic tested Hex archive")

        with (
            patch.object(hex_release, "metadata", return_value=self.metadata),
            patch.object(hex_release, "package", side_effect=build),
        ):
            return hex_release.prepare(self.root, self.directory)

    def policy(self, **changes):
        path = self.root / "elixir/release.json"
        policy = json.loads(path.read_text())
        path.write_text(json.dumps({**policy, **changes}))

    def test_disabled_elixir_blocks_every_publisher_and_tag_creation(self):
        self.policy(enabled=False)
        for language, prefix in fixtures.release.LANGUAGES.items():
            with (
                self.subTest(language=language),
                self.assertRaisesRegex(
                    fixtures.release.ReleaseError, "Elixir publication is disabled"
                ),
            ):
                fixtures.release.publisher(
                    self.root, language, f"refs/tags/{prefix}/v{fixtures.VERSION}"
                )
        with self.assertRaisesRegex(
            fixtures.release.ReleaseError, "Elixir publication is disabled"
        ):
            fixtures.release.coordinator(
                self.root, fixtures.VERSION, "workflow_dispatch", "refs/heads/master"
            )
        self.assertEqual(self.fixture.git("tag", "--list"), "")
        self.assertEqual(
            fixtures.release.coordinator(
                self.root, None, "pull_request", "refs/pull/1/merge"
            ),
            fixtures.VERSION,
        )

    def test_native_versions_spec_skiplist_and_auth_policy_are_guarded(self):
        spec_version = json.loads((self.root / "spec/manifest.json").read_text())["spec_version"]
        for name, old, new in [
            ("elixir/mix.exs", fixtures.VERSION, "9.9.9"),
            ("elixir/integrations/mix.exs", fixtures.VERSION, "9.9.9"),
            ("elixir/erlang/rebar.config", fixtures.VERSION, "9.9.9"),
            (
                "elixir/lib/slackblocks.ex",
                f'def spec_version, do: "{spec_version}"',
                'def spec_version, do: "0.0.0"',
            ),
        ]:
            path = self.root / name
            original = path.read_text()
            self.assertIn(old, original, f"mutation must change {name}")
            path.write_text(original.replace(old, new))
            with (
                self.subTest(name=name),
                self.assertRaises(fixtures.release.ReleaseError),
            ):
                fixtures.release.validate(self.root)
            path.write_text(original)
        (self.root / "elixir/conformance/skiplist.txt").write_text("blocks/missing\n")
        with self.assertRaisesRegex(fixtures.release.ReleaseError, "skip list"):
            fixtures.release.validate(self.root)
        (self.root / "elixir/conformance/skiplist.txt").write_text("")
        for changes in [
            dict(enabled=1),
            dict(enabled=True, auth_mode="fallback"),
            dict(auth_mode="api-key", owner=""),
        ]:
            self.policy(**changes)
            with (
                self.subTest(changes=changes),
                self.assertRaises(fixtures.release.ReleaseError),
            ):
                fixtures.release.validate(self.root, strict=True)

    def test_artifact_identity_checksum_lock_and_reproduction(self):
        record = self.artifact()
        self.assertEqual(hex_release.verify(self.root, self.directory), record)
        with (
            patch.object(hex_release, "metadata", return_value=self.metadata),
            patch.object(
                hex_release,
                "package",
                side_effect=lambda _, p: p.write_bytes(b"different"),
            ),
            self.assertRaisesRegex(hex_release.preflight.ReleaseError, "reproduction"),
        ):
            hex_release.verify(self.root, self.directory, rebuild=True)
        path = self.directory / "release.json"
        for key in ["commit", "version", "toolchain", "lock_sha256"]:
            path.write_text(json.dumps({**record, key: "wrong"}))
            with (
                self.subTest(key=key),
                self.assertRaisesRegex(hex_release.preflight.ReleaseError, "metadata"),
            ):
                hex_release.verify(self.root, self.directory)
        path.write_text(json.dumps(record))
        (self.directory / hex_release.ARCHIVE).write_bytes(b"corrupt")
        with self.assertRaisesRegex(hex_release.preflight.ReleaseError, "checksum"):
            hex_release.verify(self.root, self.directory)

    def test_artifacts_reject_untracked_and_ignored_package_files(self):
        # Hex includes lib contents independently of Git's ignore rules.
        (self.root / ".git/info/exclude").write_text("*.ez\n")
        for name in ["lib/uncommitted.ex", "lib/ignored.ez", "README.md"]:
            path = self.root / "elixir" / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("uncommitted package content")
            try:
                with (
                    self.subTest(name=name),
                    self.assertRaisesRegex(
                        hex_release.preflight.ReleaseError, "Untracked package"
                    ),
                ):
                    self.artifact()
            finally:
                path.unlink()
        # Unrelated scratch and build outputs do not enter the Hex archive.
        (self.root / "elixir/dist").mkdir()
        (self.root / "elixir/dist/slackblocks.tar").write_bytes(b"previous build")
        record = self.artifact()
        self.assertEqual(hex_release.verify(self.root, self.directory), record)

    def test_readiness_rejects_missing_credentials_before_dry_run(self):
        with (
            patch.dict(os.environ, {}, clear=True),
            self.assertRaisesRegex(hex_release.preflight.ReleaseError, "HEX_API_KEY"),
        ):
            hex_release.account(self.root)

    def account_api(self, existing=True, permissions=None, owner="maintainer", active=True,
                    package_owner="maintainer", level="full"):
        if permissions is None:
            permissions = [{"domain": "api", "resource": "write"}]

        def response(path, key=None, absent=False):
            self.assertNotIn("domain=package", path)
            if path in ("/auth?domain=api&resource=read", "/auth?domain=api&resource=write"):
                self.assertEqual(key, "test-only")
                return {"key": {"name": "release key", "owner": {"type": "user", "name": owner}}}
            if path == "/keys/release%20key":
                self.assertEqual(key, "test-only")
                return {"name": "release key", "authing_key": active, "permissions": permissions}
            if path == "/packages/slackblocks":
                self.assertTrue(absent)
                return {"name": "slackblocks"} if existing else None
            if path == "/packages/slackblocks/owners/maintainer":
                self.assertEqual(key, "test-only")
                return {"username": package_owner, "level": level}
            self.fail(f"Unexpected authenticated endpoint: {path}")

        return response

    def test_readiness_accepts_write_key_for_first_and_existing_package(self):
        for existing in [False, True]:
            with (
                self.subTest(existing=existing),
                patch.dict(os.environ, {"HEX_API_KEY": "test-only"}),
                patch.object(hex_release, "api", side_effect=self.account_api(existing=existing)),
            ):
                hex_release.account(self.root)

    def test_readiness_accepts_exact_package_key_for_existing_package(self):
        permissions = [{"domain": "api", "resource": "read"},
                       {"domain": "package", "resource": "hexpm/slackblocks"}]
        with (
            patch.dict(os.environ, {"HEX_API_KEY": "test-only"}),
            patch.object(hex_release, "api", side_effect=self.account_api(
                permissions=permissions, level="maintainer")),
        ):
            hex_release.account(self.root)

    def test_readiness_rejects_read_only_wrong_package_and_first_publish_package_keys(self):
        for existing, permissions in [
            (True, [{"domain": "api", "resource": "read"}]),
            (True, [{"domain": "package", "resource": "hexpm/other"}]),
            (True, [{"domain": "package", "resource": "private/slackblocks"}]),
            (True, [{"domain": "package", "resource": "slackblocks"}]),
            (False, [{"domain": "package", "resource": "hexpm/slackblocks"}]),
        ]:
            with (
                self.subTest(existing=existing, permissions=permissions),
                patch.dict(os.environ, {"HEX_API_KEY": "test-only"}),
                patch.object(hex_release, "api", side_effect=self.account_api(
                    existing=existing, permissions=permissions)),
                self.assertRaisesRegex(hex_release.preflight.ReleaseError, "publish permission"),
            ):
                hex_release.account(self.root)

    def test_readiness_rejects_wrong_identity_inactive_key_and_missing_ownership(self):
        for options, message in [
            ({"owner": "someone-else"}, "owner"),
            ({"active": False}, "active key"),
            ({"package_owner": "someone-else"}, "package owner"),
            ({"level": "read"}, "package owner"),
        ]:
            with (
                self.subTest(options=options),
                patch.dict(os.environ, {"HEX_API_KEY": "test-only"}),
                patch.object(hex_release, "api", side_effect=self.account_api(**options)),
                self.assertRaisesRegex(hex_release.preflight.ReleaseError, message),
            ):
                hex_release.account(self.root)

    def test_readiness_authentication_failure_stops_without_fallback(self):
        with (
            patch.dict(os.environ, {"HEX_API_KEY": "test-only"}),
            patch.object(hex_release, "api", side_effect=hex_release.preflight.ReleaseError(
                "Hex request failed with HTTP 401")) as api,
            self.assertRaisesRegex(hex_release.preflight.ReleaseError, "401"),
        ):
            hex_release.account(self.root)
        api.assert_called_once_with("/auth?domain=api&resource=read", "test-only")

    def test_hex_absence_is_only_404_and_errors_fail_closed(self):
        for code in [401, 403, 429, 500]:
            error = urllib.error.HTTPError(
                "https://hex.pm/api/packages/slackblocks", code, "failed", {}, None
            )
            with (
                self.subTest(code=code),
                self.assertRaises(hex_release.preflight.ReleaseError),
            ):
                hex_release.api(
                    "/packages/slackblocks",
                    absent=True,
                    opener=lambda *a, **k: (_ for _ in ()).throw(error),
                )
        error = urllib.error.HTTPError(
            "https://hex.pm/api/packages/slackblocks", 404, "absent", {}, None
        )
        self.assertIsNone(
            hex_release.api(
                "/packages/slackblocks",
                absent=True,
                opener=lambda *a, **k: (_ for _ in ()).throw(error),
            )
        )
        self.assertEqual(
            hex_release.api(
                "/packages/slackblocks",
                opener=lambda *a, **k: io.StringIO('{"name":"slackblocks"}'),
            ),
            {"name": "slackblocks"},
        )

    def test_existing_version_requires_exact_checksum_and_no_retirement(self):
        record = self.artifact()
        existing = {
            "version": record["version"],
            "checksum": record["sha256"],
            "has_docs": True,
        }
        with patch.object(hex_release, "api", return_value=existing):
            self.assertEqual(hex_release.registry(record), existing)
        for changes in [
            dict(checksum="wrong"),
            dict(version="9.9.9"),
            dict(retirement={"reason": "invalid"}),
        ]:
            with (
                self.subTest(changes=changes),
                patch.object(hex_release, "api", return_value={**existing, **changes}),
                self.assertRaises(hex_release.preflight.ReleaseError),
            ):
                hex_release.registry(record)

    def test_indexing_wait_is_bounded_and_identity_is_checked_each_time(self):
        record = self.artifact()
        existing = {"version": record["version"], "checksum": record["sha256"]}
        with (
            patch.object(hex_release, "api", side_effect=[None, None, existing]),
            patch.object(hex_release.time, "sleep") as sleep,
        ):
            self.assertEqual(hex_release.wait_registry(record, sleep=sleep), existing)
            self.assertEqual(sleep.call_count, 2)
        with (
            patch.object(hex_release, "api", return_value=None),
            self.assertRaisesRegex(hex_release.preflight.ReleaseError, "bounded wait"),
        ):
            hex_release.wait_registry(record, attempts=2, sleep=lambda _: None)
        with patch.object(hex_release, "docs_available", side_effect=[False, True]):
            hex_release.wait_docs(record["version"], attempts=2, sleep=lambda _: None)
        with (
            patch.object(hex_release, "docs_available", return_value=False),
            self.assertRaisesRegex(hex_release.preflight.ReleaseError, "bounded wait"),
        ):
            hex_release.wait_docs(record["version"], attempts=2, sleep=lambda _: None)

    def test_partial_success_recovers_docs_without_republishing_package(self):
        record = self.artifact()
        for docs in [False, True]:
            existing = {
                "version": record["version"],
                "checksum": record["sha256"],
                "has_docs": docs,
            }
            with (
                patch.object(hex_release.preflight, "publisher"),
                patch.object(hex_release, "verify", return_value=record),
                patch.object(hex_release, "account"),
                patch.object(hex_release, "registry", return_value=existing),
                patch.object(hex_release, "wait_docs"),
                patch.object(hex_release, "run") as run,
            ):
                hex_release.publish(self.root, self.directory)
                self.assertEqual(run.call_count, 0 if docs else 1)
                if not docs:
                    self.assertEqual(
                        run.call_args.args[1:], ("mix", "hex.publish", "docs", "--yes")
                    )

    def test_uncertain_upload_queries_identity_without_retrying_write(self):
        record = self.artifact()
        existing = {
            "version": record["version"],
            "checksum": record["sha256"],
            "has_docs": True,
        }
        with (
            patch.object(hex_release.preflight, "publisher"),
            patch.object(hex_release, "verify", return_value=record),
            patch.object(hex_release, "account"),
            patch.object(hex_release, "registry", return_value=None),
            patch.object(hex_release, "wait_registry", return_value=existing),
            patch.object(hex_release, "wait_docs"),
            patch.object(
                hex_release, "run", side_effect=subprocess.CalledProcessError(1, "mix")
            ) as run,
        ):
            hex_release.publish(self.root, self.directory)
            self.assertEqual(run.call_count, 1)


class ElixirDocsTests(unittest.TestCase):
    def test_rendered_source_links_require_exact_ref_and_package_directory(self):
        import tempfile
        from pathlib import Path

        docs = fixtures.load(
            "elixir_docs", fixtures.ROOT / "elixir/tools/docs_check.py"
        )
        with tempfile.TemporaryDirectory(dir=hex_release.scratch()) as tmp:
            directory = Path(tmp)
            page = directory / "Slackblocks.html"
            prefix = "https://github.com/nicklambourne/slackblocks/blob/"
            for link in [
                "%{ref}/elixir/lib/slackblocks.ex#L1",
                "master/elixir/lib/slackblocks.ex#L1",
                "abc123/lib/slackblocks.ex#L1",
                "",
            ]:
                page.write_text(
                    f'<a href="{prefix}{link}" title="View Source">Source</a>'
                )
                with self.subTest(link=link), self.assertRaises(SystemExit):
                    docs.verify_source_links(directory, "abc123")
            page.write_text("<p>No source links</p>")
            with self.assertRaises(SystemExit):
                docs.verify_source_links(directory, "abc123")
            page.write_text(
                f'<a href="{prefix}abc123/elixir/lib/slackblocks.ex#L1" title="View Source">Source</a>'
                f'<a href="{prefix}abc123/elixir/lib/slackblocks.ex#L20" aria-label="View Source">Source</a>'
                f'<a href="{prefix}master/README.md">An ordinary prose link</a>'
                '<a href="https://github.com/nicklambourne/slackblocks" title="View Source">API index</a>'
            )
            docs.verify_source_links(directory, "abc123")


class ElixirWorkflowTests(unittest.TestCase):
    def test_secrets_are_only_in_protected_job_and_preflight_precedes_them(self):
        workflow = (fixtures.ROOT / ".github/workflows/publish-hex.yml").read_text()
        build, rest = workflow.split("  publish:\n", 1)
        self.assertNotIn("secrets.", build)
        self.assertNotIn("id-token: write", workflow)
        self.assertIn("name: hex", rest)
        self.assertLess(
            rest.index("verify-release.py publisher --language elixir"),
            rest.index("secrets.HEX_API_KEY"),
        )
        self.assertNotIn("--replace", workflow)
        coordinator = (
            fixtures.ROOT / ".github/workflows/coordinated-release.yml"
        ).read_text()
        self.assertIn("needs: [validate, php-readiness, hex-readiness, hackage-readiness]", coordinator)
        self.assertIn('"elixir/v$VERSION"', coordinator)
        native = (fixtures.ROOT / ".github/workflows/elixir.yml").read_text()
        self.assertIn("  pull_request:\n", native)
        self.assertIn("name: Elixir release checks", native)
        self.assertIn("verify-release.py validate", native)
        self.assertNotIn("GITHUB_EVENT_NAME:", native)


class ElixirGitHubRecoveryTests(unittest.TestCase):
    def setUp(self):
        self.fixture = fixtures.ReleaseTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.doCleanups)
        self.root = self.fixture.root
        self.directory = self.root / "assets"
        self.directory.mkdir()
        for name in ["slackblocks.tar", "release.json"]:
            (self.directory / name).write_bytes(name.encode())
        self.module = fixtures.load(
            "elixir_github", fixtures.ROOT / "elixir/tools/github_release.py"
        )
        self.tag = "elixir/v" + fixtures.VERSION

    def test_draft_visibility_wait_and_timeout_are_bounded(self):
        release = {"tagName": self.tag, "isDraft": True, "assets": []}
        with patch.object(self.module, "lookup", side_effect=[None, None, release]):
            self.assertEqual(
                self.module.wait_release(self.tag, sleep=lambda _: None), release
            )
        with (
            patch.object(self.module, "lookup", return_value=None),
            self.assertRaisesRegex(self.module.preflight.ReleaseError, "bounded wait"),
        ):
            self.module.wait_release(self.tag, attempts=2, sleep=lambda _: None)

    def test_resume_partial_draft_then_public_rerun_never_overwrites(self):
        for draft, initial in [
            (True, {"slackblocks.tar"}),
            (False, {"slackblocks.tar", "release.json"}),
        ]:
            release = {
                "tagName": self.tag,
                "isDraft": draft,
                "assets": [{"name": name} for name in initial],
            }
            events = []

            def gh(*args):
                events.append(args[1])
                self.assertNotIn("--clobber", args)
                if args[1] == "download":
                    from pathlib import Path

                    name = args[args.index("--pattern") + 1]
                    (Path(args[args.index("--dir") + 1]) / name).write_bytes(
                        (self.directory / name).read_bytes()
                    )
                return ""

            with (
                patch.object(
                    self.module.preflight, "publisher", return_value=fixtures.VERSION
                ),
                patch.object(self.module, "verify"),
                patch.object(self.module, "lookup", return_value=release),
                patch.object(self.module, "gh", side_effect=gh),
            ):
                self.module.release_assets(
                    self.root, self.directory, "refs/tags/" + self.tag
                )
            self.assertEqual(
                events,
                ["download", "upload", "edit"] if draft else ["download", "download"],
            )
