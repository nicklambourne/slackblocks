"""Credential-free Hackage identity, immutable recovery and train guards."""

import importlib.util
import io
import json
import os
import re
from pathlib import Path
import subprocess
import sys
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
        spec_version = json.loads((self.root / "spec/manifest.json").read_text())["spec_version"]
        for file, old, new, error in [
            (
                "haskell/slackblocks.cabal",
                f"version: {VERSION}",
                "version: 9.9.9",
                "versions differ",
            ),
            (
                "haskell/src/Slackblocks/Models.hs",
                f'specVersion = "{spec_version}"',
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
            "/packages/uploaders/": b'{"members":[{"username":"maintainer"}]}',
            "/package/slackblocks.json": None,
            "/package/slackblocks/maintainers/": b'{"members":[{"username":"maintainer"}]}',
        }[path]

    def test_read_only_owner_and_first_publish_checks(self):
        with (
            patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
            patch.object(hackage, "fetch", side_effect=self.response) as fetch,
        ):
            self.assertEqual(hackage.account(self.root), "fake123")
            self.assertEqual(fetch.call_args_list[0].kwargs["token"], "fake123")
            self.policy["first_publication"] = False
            self.save()
            with self.assertRaisesRegex(ValueError, "first-publication"):
                hackage.account(self.root)

    def test_account_checks_use_non_admin_canonical_http_resources(self):
        # Hackage's /api lists slash-terminated group resources. The similarly
        # named /packages/uploaders.json route returns 404 on the live service.
        responses = {
            "/packages/uploaders/": b'{"members":[{"username":"maintainer"}]}',
            "/package/slackblocks.json": b'{"3.0.0":"normal"}',
            "/package/slackblocks/maintainers/": b'{"members":[{"username":"maintainer"}]}',
        }
        observed = []

        def respond(request, timeout):
            self.assertEqual(request.get_method(), "GET")
            self.assertEqual(request.get_header("Accept"), "application/json")
            path = request.selector
            observed.append(path)
            if path == "/user/maintainer/enabled.json":
                # Hackage restricts this endpoint to administrators, including
                # when an enabled uploader requests their own account status.
                raise urllib.error.HTTPError(
                    request.full_url, 403, "Forbidden", {}, None
                )
            if path == "/users/account-management":
                self.assertEqual(
                    request.get_header("Authorization"), "X-ApiKey fake123"
                )
                raise urllib.error.HTTPError(
                    request.full_url,
                    303,
                    "See Other",
                    {"Location": "/user/maintainer/manage"},
                    None,
                )
            if path not in responses:
                raise urllib.error.HTTPError(
                    request.full_url, 404, "Not Found", {}, None
                )
            return io.BytesIO(responses[path])

        with (
            patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
            patch.object(urllib.request.OpenerDirector, "open", side_effect=respond),
        ):
            self.assertEqual(hackage.account(self.root), "fake123")
        self.assertEqual(set(observed), {"/users/account-management", *responses})

    def test_disabled_account_invalid_token_and_auth_errors_reject(self):
        # Token authentication rejects disabled/deleted users before returning
        # the account-management redirect (Hackage Framework.Auth.getUserAuth).
        for status in (401, 403, 429, 500):
            observed = []

            def fail(request, timeout):
                observed.append(request.selector)
                raise urllib.error.HTTPError(
                    request.full_url, status, "Authentication failed", {}, None
                )

            with (
                self.subTest(status=status),
                patch.dict(os.environ, {"HACKAGE_TOKEN": "fake123"}),
                patch.object(urllib.request.OpenerDirector, "open", side_effect=fail),
                self.assertRaises(urllib.error.HTTPError) as failure,
            ):
                hackage.account(self.root)
            self.assertEqual(failure.exception.code, status)
            self.assertEqual(observed, ["/users/account-management"])

    def test_foreign_owner_permissions_and_errors_reject(self):
        for path, response, message in [
            ("/users/account-management", "/user/foreign/manage", "Token owner"),
            (
                "/users/account-management",
                "https://evil.invalid/user/maintainer/manage",
                "Token owner",
            ),
            ("/packages/uploaders/", b'{"members":[]}', "uploader"),
            (
                "/package/slackblocks/maintainers/",
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

    def test_required_release_check_is_present_on_unrelated_prs(self):
        source = (ROOT / ".github/workflows/publish-hackage.yml").read_text()
        self.assertIn("  pull_request:\n  push:", source)
        build = source.split("  build:\n", 1)[1].split("  publish:\n", 1)[0]
        self.assertNotIn("    if:", build.split("    steps:", 1)[0])
        steps = re.split(r"^      - ", build, flags=re.M)[1:]
        self.assertIn("id: scope", steps[0])
        for step in steps[1:]:
            self.assertTrue(
                "if: steps.scope.outputs.relevant == 'true'" in step
                or "if: github.event_name != 'pull_request'" in step,
                step,
            )

    def test_release_scope_runs_on_shared_inputs_renames_and_lookup_failure(self):
        source = (ROOT / ".github/workflows/publish-hackage.yml").read_text()
        script = source.split("        run: |\n", 1)[1].split("      - uses:", 1)[0]
        script = "\n".join(line[10:] for line in script.splitlines())
        gh = self.root / "gh"
        gh.write_text('#!/bin/sh\nprintf "%s" "$CHANGED_FILES"\nexit "$GH_EXIT"\n')
        gh.chmod(0o755)
        for event, files, code, expected in [
            ("pull_request", "docs/docs/index.mdx", 0, "false"),
            ("pull_request", "haskell/slackblocks.cabal", 0, "true"),
            ("pull_request", "spec/model.json", 0, "true"),
            ("pull_request", ".github/scripts/verify-release.py", 0, "true"),
            ("pull_request", ".github/workflows/publish-hackage.yml", 0, "true"),
            ("pull_request", ".github/workflows/coordinated-release.yml", 0, "true"),
            ("pull_request", "retired/Model.hs\nhaskell/src/Model.hs", 0, "true"),
            ("pull_request", "", 0, "true"),
            ("pull_request", "docs/docs/index.mdx", 1, "true"),
            ("push", "", 1, "true"),
            ("workflow_dispatch", "", 1, "true"),
        ]:
            with self.subTest(event=event, files=files, code=code):
                output = self.root / "output"
                output.write_text("")
                env = dict(
                    os.environ,
                    PATH=str(self.root) + os.pathsep + os.environ["PATH"],
                    EVENT=event,
                    REPOSITORY="owner/repo",
                    PR="1",
                    GITHUB_OUTPUT=str(output),
                    CHANGED_FILES=files,
                    GH_EXIT=str(code),
                )
                subprocess.run(
                    ["bash", "-e", "-o", "pipefail", "-c", script],
                    env=env,
                    check=True,
                )
                self.assertEqual(output.read_text(), f"relevant={expected}\n")


class HaskellGitHubRecoveryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir=hackage.scratch())
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.directory = self.root / "assets"
        self.directory.mkdir()
        self.version = "3.1.0"
        self.tag = "haskell/v" + self.version
        self.names = [*hackage.names(self.version), "release.json"]
        for name in self.names:
            (self.directory / name).write_bytes(name.encode())
        (self.root / "haskell").mkdir()
        (self.root / "haskell/CHANGELOG.md").write_text(
            "## [3.1.0] - 2026-10-25\n\nFirst Haskell release.\n"
        )
        with patch.dict(sys.modules, {"release": hackage}):
            self.module = common.load(
                "haskell_github", ROOT / "haskell/bin/github_release.py"
            )

    def test_draft_visibility_wait_and_timeout_are_bounded(self):
        release = {"tagName": self.tag, "isDraft": True, "assets": []}
        with patch.object(self.module, "lookup", side_effect=[None, release]):
            self.assertEqual(
                self.module.wait_release(self.tag, sleep=lambda _: None), release
            )
        with (
            patch.object(self.module, "lookup", return_value=None),
            self.assertRaisesRegex(ValueError, "bounded wait"),
        ):
            self.module.wait_release(self.tag, attempts=2, sleep=lambda _: None)

    def test_create_resume_and_public_rerun_never_overwrite(self):
        for draft, initial, missing in [
            (True, set(), True),
            (True, {self.names[0]}, False),
            (False, set(self.names), False),
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
                self.assertEqual(args[2], self.tag)
                if args[1] == "download":
                    name = args[args.index("--pattern") + 1]
                    (Path(args[args.index("--dir") + 1]) / name).write_bytes(
                        (self.directory / name).read_bytes()
                    )
                if args[1] == "create":
                    self.assertIn("--verify-tag", args)
                    self.assertIn("First Haskell release.", Path(args[-1]).read_text())
                return ""

            with (
                self.subTest(draft=draft, initial=initial),
                patch.object(
                    self.module.preflight, "publisher", return_value=self.version
                ),
                patch.object(self.module, "verify"),
                patch.object(
                    self.module, "lookup", return_value=None if missing else release
                ),
                patch.object(self.module, "wait_release", return_value=release),
                patch.object(self.module, "gh", side_effect=gh),
            ):
                self.module.release_assets(
                    self.root, self.directory, "refs/tags/" + self.tag
                )
            expected = ["create"] if missing else []
            expected += ["download" if n in initial else "upload" for n in self.names]
            expected += ["edit"] if draft else []
            self.assertEqual(events, expected)

    def test_foreign_tag_or_existing_asset_mismatch_stops_before_publish(self):
        for foreign in [True, False]:
            release = {
                "tagName": "haskell/v9.9.9" if foreign else self.tag,
                "isDraft": True,
                "assets": [{"name": self.names[0]}],
            }
            events = []

            def gh(*args):
                events.append(args[1])
                name = args[args.index("--pattern") + 1]
                (Path(args[args.index("--dir") + 1]) / name).write_bytes(b"wrong")

            with (
                patch.object(
                    self.module.preflight, "publisher", return_value=self.version
                ),
                patch.object(self.module, "verify"),
                patch.object(self.module, "lookup", return_value=release),
                patch.object(self.module, "gh", side_effect=gh),
                self.assertRaisesRegex(
                    ValueError, "exact tag" if foreign else "checksum"
                ),
            ):
                self.module.release_assets(
                    self.root, self.directory, "refs/tags/" + self.tag
                )
            self.assertEqual(events, [] if foreign else ["download"])


if __name__ == "__main__":
    unittest.main()
