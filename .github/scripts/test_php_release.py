"""Exercise PHP release gates, exact artifacts, local pushes and registry failures."""

import io
import contextlib
import json
import os
import textwrap
import subprocess
from pathlib import Path
import unittest
import urllib.error
import zipfile
from unittest.mock import patch

import test_release as fixtures

php = fixtures.load("php_release", fixtures.ROOT / "php/bin/release.py")


class PHPReleaseTests(unittest.TestCase):
    def setUp(self):
        self.fixture = fixtures.ReleaseTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.doCleanups)
        self.root = self.fixture.root
        for name in php.FILES - {"composer.json", "php/CHANGELOG.md"}:
            (self.root / name).write_bytes((fixtures.ROOT / name).read_bytes())
        (self.root / "php/src/MessagePayload.php").write_text(
            "<?php // distribution fixture\n"
        )
        (self.root / "php/examples").mkdir()
        (self.root / "php/examples/consumer.php").write_text(
            "<?php // consumer fixture\n"
        )
        (self.root / "php/tests").mkdir()
        (self.root / "php/tests/private.php").write_text("development-only fixture")
        self.fixture.commit()
        self.directory = self.root / "artifact"
        self.record = php.prepare(self.root, self.directory)

    def test_disabled_php_blocks_coordinator_and_every_publisher_without_tags(self):
        settings = self.root / "php/release.json"
        data = json.loads(settings.read_text())
        data["enabled"] = False
        settings.write_text(json.dumps(data))
        for language in fixtures.release.LANGUAGES:
            with (
                self.subTest(language=language),
                self.assertRaisesRegex(
                    php.preflight.ReleaseError, "PHP publication is disabled"
                ),
            ):
                php.preflight.publisher(
                    self.root,
                    language,
                    f"refs/tags/{fixtures.release.LANGUAGES[language]}/v{fixtures.VERSION}",
                )
        with self.assertRaisesRegex(
            php.preflight.ReleaseError, "PHP publication is disabled"
        ):
            php.preflight.coordinator(
                self.root, fixtures.VERSION, "workflow_dispatch", "refs/heads/master"
            )
        self.assertEqual(self.fixture.git("tag", "--list"), "")

    def test_php_constraint_path_version_and_version_constant_are_guarded(self):
        for name in ["php/src/Version.php", "php/integrations/composer.json"]:
            path = self.root / name
            original = path.read_text()
            path.write_text(original.replace(fixtures.VERSION, "9.9.9"))
            with self.assertRaises(php.preflight.ReleaseError):
                php.preflight.validate(self.root)
            path.write_text(original)
        settings = self.root / "php/release.json"
        data = json.loads(settings.read_text())
        data["enabled"] = 1
        settings.write_text(json.dumps(data))
        with self.assertRaisesRegex(php.preflight.ReleaseError, "boolean"):
            php.preflight.validate(self.root)

    def test_artifact_is_allowlisted_reproducible_and_excludes_development_files(self):
        self.assertEqual(php.verify(self.root, self.directory), self.record)
        other = self.root / "second-artifact"
        php.prepare(self.root, other)
        archive = php.ARCHIVE
        self.assertEqual(
            (self.directory / archive).read_bytes(), (other / archive).read_bytes()
        )
        self.assertNotIn("php/tests/private.php", self.record["files"])
        self.assertNotIn("php/release.json", self.record["files"])
        self.assertIn("php/LICENSE.BSD-3-Clause", self.record["files"])
        self.assertNotIn(
            "version", json.loads((self.root / "php/composer.json").read_text())
        )

    def test_corrupt_metadata_archive_and_extra_paths_fail(self):
        manifest = self.directory / "release.json"
        for key in [
            "version",
            "source_commit",
            "source_timestamp",
            "package",
            "repository",
        ]:
            manifest.write_text(json.dumps({**self.record, key: "wrong"}))
            with (
                self.subTest(key=key),
                self.assertRaisesRegex(php.preflight.ReleaseError, "metadata"),
            ):
                php.verify(self.root, self.directory)
        manifest.write_text(json.dumps(self.record))
        archive = self.directory / php.ARCHIVE
        with zipfile.ZipFile(archive, "a") as package:
            package.writestr("../escape.php", "bad")
        with self.assertRaisesRegex(php.preflight.ReleaseError, "checksum"):
            php.verify(self.root, self.directory)
        manifest.write_text(
            json.dumps({**self.record, "sha256": php.digest(archive.read_bytes())})
        )
        with self.assertRaisesRegex(php.preflight.ReleaseError, "inventory"):
            php.verify(self.root, self.directory)

    def test_dirty_checkout_cannot_prepare(self):
        (self.root / "php/src/MessagePayload.php").write_text("dirty")
        with self.assertRaisesRegex(php.preflight.ReleaseError, "Commit changes"):
            php.prepare(self.root, self.root / "dirty-artifact")

    def test_monorepo_tag_is_deterministic_and_retry_is_immutable(self):
        remote = self.root / "remote.git"
        subprocess.run(["git", "init", "--quiet", "--bare", str(remote)], check=True)
        tag = "v" + fixtures.VERSION
        self.assertEqual(php.push_tag(self.root, self.record, str(remote)), "dry-run")
        self.assertEqual(php.run("git", "ls-remote", str(remote), cwd=self.root), "")
        self.assertEqual(
            php.push_tag(self.root, self.record, str(remote), apply=True), "published"
        )
        clone = self.root / "second-clone"
        php.run("git", "clone", "--quiet", str(self.root), str(clone), cwd=self.root)
        self.assertEqual(
            php.push_tag(clone, self.record, str(remote), apply=True), "unchanged"
        )
        self.assertEqual(
            php.run(
                "git",
                "--git-dir=" + str(remote),
                "rev-parse",
                tag + "^{commit}",
                cwd=self.root,
            ),
            self.record["source_commit"],
        )
        # Neither the monorepo's default branch nor local tag refs are mutated.
        self.assertEqual(
            php.run("git", "ls-remote", str(remote), "refs/heads/*", cwd=self.root), ""
        )
        self.assertEqual(self.fixture.git("tag", "--list"), "")
        for wrong in (
            self.record["source_commit"],
            self.fixture.git("rev-parse", "HEAD~1"),
        ):
            php.run(
                "git",
                "--git-dir=" + str(remote),
                "update-ref",
                "refs/tags/" + tag,
                wrong,
                cwd=self.root,
            )
            with self.assertRaisesRegex(php.preflight.ReleaseError, "never move"):
                php.push_tag(self.root, self.record, str(remote), apply=True)

    def test_root_descriptor_drift_and_premature_composer_tags_block_release(self):
        path = self.root / "composer.json"
        original = path.read_text()
        for field in ("autoload", "dist", "require", "archive", "name"):
            changed = json.loads(original)
            changed[field] = "wrong"
            path.write_text(json.dumps(changed))
            with (
                self.subTest(field=field),
                self.assertRaisesRegex(php.preflight.ReleaseError, "manifest is stale"),
            ):
                php.preflight.validate(self.root)
        path.write_text(original)
        tag = "v" + fixtures.VERSION
        self.fixture.git("tag", "-a", tag, "-m", "premature version")
        with self.assertRaisesRegex(php.preflight.ReleaseError, "must follow"):
            php.preflight.tag_state(
                self.root, fixtures.VERSION, self.record["source_commit"]
            )
        self.fixture.tags()
        self.assertEqual(
            php.preflight.tag_state(
                self.root, fixtures.VERSION, self.record["source_commit"]
            ),
            len(fixtures.release.LANGUAGES),
        )
        self.fixture.git("tag", "-f", tag, "HEAD")
        with self.assertRaisesRegex(php.preflight.ReleaseError, "annotated"):
            php.preflight.tag_state(
                self.root, fixtures.VERSION, self.record["source_commit"]
            )
        self.fixture.git("tag", "-a", "-f", tag, "HEAD~1", "-m", "wrong commit")
        with self.assertRaisesRegex(php.preflight.ReleaseError, "different commit"):
            php.preflight.tag_state(
                self.root, fixtures.VERSION, self.record["source_commit"]
            )

    def test_registration_requires_exact_identity_and_repository(self):
        settings = php.policy(self.root)
        package = {
            "name": settings["package"],
            "repository": "https://github.com/" + settings["repository"],
        }

        def response(*args, **kwargs):
            return io.StringIO(json.dumps({"package": package}))

        php.check_registration(settings, response)
        for field in ["name", "repository"]:
            original = package[field]
            package[field] = "wrong"
            with self.assertRaises(php.preflight.ReleaseError):
                php.check_registration(settings, response)
            package[field] = original
        for status in [404, 403, 429, 500]:

            def failure(*args, **kwargs):
                raise urllib.error.HTTPError(
                    "https://packagist.org", status, "not ready", {}, None
                )

            with self.subTest(status=status), self.assertRaises(urllib.error.HTTPError):
                php.check_registration(settings, failure)

    def test_registry_consumer_rejects_wrong_refs_repository_and_installed_files(self):
        commit = self.record["source_commit"]
        original_run = php.run
        failure = None

        def install(*args, cwd, env=None, input=None):
            if args[0] == "composer":
                installed = cwd / "vendor" / self.record["package"]
                installed.mkdir(parents=True)
                with zipfile.ZipFile(self.directory / php.ARCHIVE) as archive:
                    archive.extractall(installed)
                package = {
                    "name": self.record["package"],
                    "source": {
                        "reference": commit,
                        "url": "https://github.com/"
                        + self.record["repository"]
                        + ".git",
                    },
                    "dist": {
                        "reference": commit,
                        "type": "zip",
                        "url": php.preflight.php_manifest.DIST_URL,
                    },
                }
                if failure in ("source", "dist"):
                    package[failure]["reference"] = "wrong"
                elif failure == "repository":
                    package["source"]["url"] = "https://github.com/unexpected/package"
                elif failure == "url":
                    package["dist"]["url"] = "https://example.invalid/unrelated.zip"
                elif failure == "file":
                    (installed / "php/src/MessagePayload.php").write_text("tampered")
                elif failure == "inventory":
                    (installed / "unexpected.php").write_text("unexpected")
                (cwd / "composer.lock").write_text(json.dumps({"packages": [package]}))
                info = cwd / "vendor/composer/installed.json"
                info.parent.mkdir()
                info.write_text(
                    json.dumps(
                        {
                            "packages": [
                                {
                                    "name": self.record["package"],
                                    "installation-source": "source"
                                    if failure == "transport"
                                    else "dist",
                                }
                            ]
                        }
                    )
                )
                return ""
            if args[0] == "php":
                return ""  # Actual PHP execution is exercised by artifact CI.
            return original_run(*args, cwd=cwd, env=env, input=input)

        with patch.object(php, "run", side_effect=install):
            self.assertEqual(
                php.consumer(self.root, self.directory, registry=True), self.record
            )
            for failure in (
                "source",
                "dist",
                "repository",
                "url",
                "file",
                "inventory",
                "transport",
            ):
                with (
                    self.subTest(failure=failure),
                    self.assertRaises(php.preflight.ReleaseError),
                ):
                    php.consumer(self.root, self.directory, registry=True)

    def test_registry_indexing_retry_is_bounded_and_content_mismatches_fail_immediately(
        self,
    ):
        unavailable = subprocess.CalledProcessError(2, "composer")
        cases = [
            ([unavailable, unavailable, self.record, self.record], None, 4, 2),
            ([unavailable] * 12, subprocess.CalledProcessError, 12, 11),
            (
                [php.preflight.ReleaseError("content differs")],
                php.preflight.ReleaseError,
                1,
                0,
            ),
        ]
        for outcomes, error, calls, sleeps in cases:
            with (
                self.subTest(error=error),
                patch("sys.argv", ["release.py", "verify-registry"]),
                patch.object(php.preflight, "publisher"),
                patch.object(php, "consumer", side_effect=outcomes) as consumer,
                patch.object(php.time, "sleep") as sleep,
                contextlib.redirect_stdout(io.StringIO()),
            ):
                if error is None:
                    php.main()
                else:
                    with self.assertRaises(error):
                        php.main()
                self.assertEqual(consumer.call_count, calls)
                self.assertEqual(sleep.call_count, sleeps)
                for call in sleep.call_args_list:
                    self.assertEqual(call.args, (15,))

    def test_github_lookup_finds_drafts_across_pages_and_fails_closed(self):
        tag = "php/v" + fixtures.VERSION
        release = {"tag_name": tag, "draft": True}
        pages = [[{"tag_name": "python/v" + fixtures.VERSION}], [release]]
        with patch.object(php, "run", return_value=json.dumps(pages)) as run:
            self.assertEqual(php.release_info(self.root, "owner/repo", tag), release)
            self.assertIn("--paginate", run.call_args.args)
            self.assertIn("--slurp", run.call_args.args)
            self.assertIsNone(php.release_info(self.root, "owner/repo", "absent"))
        with patch.object(php, "run", return_value=json.dumps([[release], [release]])):
            with self.assertRaisesRegex(
                php.preflight.ReleaseError, "Multiple PHP releases"
            ):
                php.release_info(self.root, "owner/repo", tag)
        for status in (404, 401, 403, 429, 500, "network failure"):
            error = subprocess.CalledProcessError(1, "gh", output=str(status))
            with (
                self.subTest(status=status),
                patch.object(php, "run", side_effect=error),
            ):
                with self.assertRaises(subprocess.CalledProcessError):
                    php.release_info(self.root, "owner/repo", tag)

    def test_release_listing_waits_for_creation_and_publication_visibility(self):
        draft = {"tag_name": "php/v" + fixtures.VERSION, "draft": True}
        public = {**draft, "draft": False}
        for published, responses, expected in (
            (False, [None, None, draft], draft),
            (True, [draft, None, public], public),
        ):
            with (
                self.subTest(published=published),
                patch.object(php, "release_info", side_effect=responses),
                patch.object(php.time, "sleep") as sleep,
            ):
                self.assertEqual(
                    php.wait_for_release(
                        self.root, "owner/repo", draft["tag_name"], published=published
                    ),
                    expected,
                )
                self.assertEqual(sleep.call_count, 2)

    def test_release_listing_timeout_and_api_failures_remain_closed(self):
        with (
            patch.object(php, "release_info", return_value=None) as lookup,
            patch.object(php.time, "sleep") as sleep,
            self.assertRaisesRegex(php.preflight.ReleaseError, "not visible yet"),
        ):
            php.wait_for_release(self.root, "owner/repo", "php/v" + fixtures.VERSION)
        self.assertEqual(lookup.call_count, 12)
        self.assertEqual(sleep.call_count, 11)
        with (
            patch.object(php, "release_info", side_effect=subprocess.CalledProcessError(1, "gh")),
            patch.object(php.time, "sleep") as sleep,
            self.assertRaises(subprocess.CalledProcessError),
        ):
            php.wait_for_release(self.root, "owner/repo", "php/v" + fixtures.VERSION)
        sleep.assert_not_called()

    def asset_service(self, draft=True, initial=None):
        assets = {} if initial is None else dict(initial)
        state = {"exists": initial is not None, "draft": draft, "events": []}
        tag = "php/v" + fixtures.VERSION

        def gh(*args, cwd, env=None, input=None):
            self.assertEqual(args[0], "gh")
            if args[1] == "api":
                if not state["exists"]:
                    return "[[]]"
                return json.dumps(
                    [
                        [
                            {
                                "tag_name": tag,
                                "draft": state["draft"],
                                "prerelease": False,
                                "assets": [{"name": name} for name in assets],
                            }
                        ]
                    ]
                )
            command = args[2]
            state["events"].append(command)
            if command == "create":
                self.assertIn("--draft", args)
                self.assertIn("--verify-tag", args)
                state["exists"] = True
                state["draft"] = True
            elif command == "upload":
                self.assertTrue(state["draft"], "Never edit an already public release")
                self.assertNotIn("--clobber", args)
                path = Path(args[4])
                self.assertNotIn(path.name, assets)
                assets[path.name] = path.read_bytes()
            elif command == "download":
                name = args[args.index("--pattern") + 1]
                directory = Path(args[args.index("--dir") + 1])
                (directory / name).write_bytes(assets[name])
            elif command == "edit":
                self.assertEqual(set(assets), {php.ARCHIVE, "release.json"})
                self.assertIn("--draft=false", args)
                state["draft"] = False
            else:
                self.fail("Unexpected release operation: " + repr(args))
            return ""

        return assets, state, gh

    def test_release_assets_are_verified_before_publication_and_retries_do_not_write(
        self,
    ):
        original_run = php.run
        assets, state, gh = self.asset_service()
        with patch.object(php, "run", side_effect=gh):
            php.release_assets(self.root, self.directory, self.record)
            self.assertFalse(state["draft"])
            self.assertEqual(
                state["events"],
                ["create", "upload", "download", "upload", "download", "edit"],
            )
            state["events"].clear()
            php.release_assets(self.root, self.directory, self.record)
            self.assertEqual(state["events"], ["download", "download"])
        self.assertIs(php.run, original_run)
        self.assertEqual(
            assets[php.ARCHIVE], (self.directory / php.ARCHIVE).read_bytes()
        )

    def test_release_assets_tolerate_delayed_github_listing_updates(self):
        assets, state, gh = self.asset_service()
        original_lookup = php.release_info
        delayed = {True: True, False: True}

        def lookup(*args):
            info = original_lookup(*args)
            if info is not None and delayed[info["draft"]]:
                delayed[info["draft"]] = False
                return None if info["draft"] else {**info, "draft": True}
            return info

        with (
            patch.object(php, "run", side_effect=gh),
            patch.object(php, "release_info", side_effect=lookup),
            patch.object(php.time, "sleep") as sleep,
        ):
            php.release_assets(self.root, self.directory, self.record)
        self.assertFalse(state["draft"])
        self.assertEqual(sleep.call_count, 2)
        self.assertEqual(
            state["events"],
            ["create", "upload", "download", "upload", "download", "edit"],
        )
        self.assertEqual(set(assets), {php.ARCHIVE, "release.json"})

    def test_partial_draft_is_resumable_but_changed_or_missing_public_assets_fail(self):
        for draft, changed in (
            (True, False),
            (True, True),
            (False, False),
            (False, True),
        ):
            data = b"wrong" if changed else (self.directory / php.ARCHIVE).read_bytes()
            assets, state, gh = self.asset_service(
                draft=draft, initial={php.ARCHIVE: data}
            )
            with (
                self.subTest(draft=draft, changed=changed),
                patch.object(php, "run", side_effect=gh),
            ):
                if draft and not changed:
                    php.release_assets(self.root, self.directory, self.record)
                    self.assertFalse(state["draft"])
                    self.assertEqual(
                        state["events"], ["download", "upload", "download", "edit"]
                    )
                else:
                    with self.assertRaises(php.preflight.ReleaseError):
                        php.release_assets(self.root, self.directory, self.record)
                    self.assertFalse(
                        any(
                            action in state["events"]
                            for action in ("create", "upload", "edit")
                        )
                    )

    def test_publish_exposes_composer_tag_only_after_verified_public_assets(self):
        events = []

        def tag(*args, apply=False):
            events.append("tag" if apply else "dry-run")
            return "published" if apply else "dry-run"

        def assets(*args):
            events.append("assets")

        with (
            patch.object(php, "push_tag", side_effect=tag),
            patch.object(php, "release_assets", side_effect=assets),
            patch.object(
                php,
                "check_registration",
                side_effect=lambda *_: events.append("registration"),
            ),
        ):
            self.assertEqual(php.publish(self.root, self.directory), "dry-run")
            self.assertEqual(events, ["dry-run"])
            events.clear()
            self.assertEqual(
                php.publish(self.root, self.directory, apply=True), "published"
            )
            self.assertEqual(events, ["dry-run", "registration", "assets", "tag"])
        with (
            patch.object(php, "push_tag", side_effect=tag),
            patch.object(
                php,
                "release_assets",
                side_effect=php.preflight.ReleaseError("asset mismatch"),
            ),
            patch.object(php, "check_registration"),
        ):
            events.clear()
            with self.assertRaisesRegex(php.preflight.ReleaseError, "asset mismatch"):
                php.publish(self.root, self.directory, apply=True)
            self.assertEqual(events, ["dry-run"])

    def test_required_release_check_handles_relevant_unrelated_and_failed_jobs(self):
        workflow = (
            fixtures.ROOT / ".github/workflows/publish-packagist.yml"
        ).read_text()
        job = workflow.split("  checks:\n", 1)[1].split("  publish:\n", 1)[0]
        script = textwrap.dedent(job.split("        run: |\n", 1)[1])
        cases = [
            ("success", "true", "success", "success", True),
            ("success", "false", "skipped", "skipped", True),
            ("failure", "false", "skipped", "skipped", False),
            ("success", "true", "failure", "skipped", False),
            ("success", "true", "success", "cancelled", False),
            ("success", "true", "success", "skipped", False),
            ("success", "false", "success", "success", False),
            ("success", "", "skipped", "skipped", False),
        ]
        for changes, relevant, build, artifact, success in cases:
            with self.subTest(
                changes=changes, relevant=relevant, build=build, artifact=artifact
            ):
                result = subprocess.run(
                    ["bash", "-c", script],
                    env={
                        **os.environ,
                        "CHANGES_RESULT": changes,
                        "RELEVANT": relevant,
                        "BUILD_RESULT": build,
                        "ARTIFACT_RESULT": artifact,
                    },
                    capture_output=True,
                )
                self.assertEqual(result.returncode == 0, success, result.stderr)

    def test_write_credentials_follow_artifact_checks_and_readiness_precedes_tags(self):
        workflow = (
            fixtures.ROOT / ".github/workflows/publish-packagist.yml"
        ).read_text()
        self.assertNotIn("secrets.", workflow)
        self.assertNotIn("contents: write", workflow.split("  publish:")[0])
        self.assertLess(
            workflow.index("release.py verify"), workflow.index("gh auth setup-git")
        )
        self.assertIn("needs: [build, artifact]", workflow)
        self.assertIn("environment: packagist", workflow)
        self.assertIn("release.py consumer --source", workflow)
        coordinator = (
            fixtures.ROOT / ".github/workflows/coordinated-release.yml"
        ).read_text()
        self.assertIn("needs: [validate, php-readiness, hex-readiness, hackage-readiness]", coordinator)
        self.assertIn("release.py readiness", coordinator)
        self.assertIn("workflow: publish-packagist.yml", coordinator)
        self.assertNotIn("secrets.PHP_DIST", coordinator)


if __name__ == "__main__":
    unittest.main()
