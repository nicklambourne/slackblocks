"""Exercise PHP release gates, exact artifacts, local pushes and registry failures."""

import io
import contextlib
import json
import subprocess
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
        for name in php.FILES - {"composer.json", "CHANGELOG.md"}:
            (self.root / "php" / name).write_bytes(
                (fixtures.ROOT / "php" / name).read_bytes()
            )
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
        archive = f"slackblocks-{fixtures.VERSION}.zip"
        self.assertEqual(
            (self.directory / archive).read_bytes(), (other / archive).read_bytes()
        )
        self.assertNotIn("tests/private.php", self.record["files"])
        self.assertNotIn("release.json", self.record["files"])
        self.assertIn("LICENSE.BSD-3-Clause", self.record["files"])
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
        archive = self.directory / f"slackblocks-{fixtures.VERSION}.zip"
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

    def test_distribution_commit_and_tag_are_deterministic_and_retry_is_immutable(self):
        first = self.root / "distribution-one"
        second = self.root / "distribution-two"
        _, tag = php.distribution(self.root, self.directory, first)
        php.distribution(self.root, self.directory, second)
        self.assertEqual(
            php.run("git", "rev-parse", "HEAD", cwd=first),
            php.run("git", "rev-parse", "HEAD", cwd=second),
        )
        remote = self.root / "remote.git"
        subprocess.run(["git", "init", "--quiet", "--bare", str(remote)], check=True)
        self.assertEqual(php.push_tag(first, str(remote), tag), "dry-run")
        self.assertEqual(php.run("git", "ls-remote", str(remote), cwd=first), "")
        self.assertEqual(php.push_tag(first, str(remote), tag, apply=True), "published")
        self.assertEqual(
            php.push_tag(second, str(remote), tag, apply=True), "unchanged"
        )
        # Even a lightweight tag pointing to the same commit is not interchangeable.
        php.run("git", "tag", "-d", tag, cwd=second)
        php.run("git", "-c", "tag.gpgsign=false", "tag", tag, cwd=second)
        with self.assertRaisesRegex(php.preflight.ReleaseError, "never move"):
            php.push_tag(second, str(remote), tag, apply=True)
        self.assertEqual(
            php.run("git", "ls-remote", str(remote), "refs/heads/*", cwd=first), ""
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
        checkout = self.root / "expected-registry"
        php.distribution(self.root, self.directory, checkout)
        commit = php.run("git", "rev-parse", "HEAD", cwd=checkout)
        original_run = php.run
        failure = None

        def install(*args, cwd, env=None):
            if args[0] == "composer":
                installed = cwd / "vendor" / self.record["package"]
                installed.mkdir(parents=True)
                with zipfile.ZipFile(
                    self.directory / f"slackblocks-{fixtures.VERSION}.zip"
                ) as archive:
                    archive.extractall(installed)
                package = {
                    "name": self.record["package"],
                    "source": {
                        "reference": commit,
                        "url": "https://github.com/"
                        + self.record["repository"]
                        + ".git",
                    },
                    "dist": {"reference": commit},
                }
                if failure in ("source", "dist"):
                    package[failure]["reference"] = "wrong"
                elif failure == "repository":
                    package["source"]["url"] = "https://github.com/unexpected/package"
                elif failure == "file":
                    (installed / "src/MessagePayload.php").write_text("tampered")
                elif failure == "inventory":
                    (installed / "unexpected.php").write_text("unexpected")
                (cwd / "composer.lock").write_text(json.dumps({"packages": [package]}))
                return ""
            if args[0] == "php":
                return ""  # Actual PHP execution is exercised by artifact CI.
            return original_run(*args, cwd=cwd, env=env)

        with patch.object(php, "run", side_effect=install):
            self.assertEqual(
                php.consumer(self.root, self.directory, registry=True), self.record
            )
            for failure in ("source", "dist", "repository", "file", "inventory"):
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
            ([unavailable, unavailable, self.record], None, 3, 2),
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

    def test_credentials_follow_artifact_checks_and_readiness_precedes_tags(self):
        workflow = (
            fixtures.ROOT / ".github/workflows/publish-packagist.yml"
        ).read_text()
        self.assertNotIn("secrets.", workflow.split("  publish:")[0])
        self.assertLess(
            workflow.index("release.py verify"),
            workflow.index("secrets.PHP_DIST_DEPLOY_KEY"),
        )
        self.assertIn("needs: [build, artifact]", workflow)
        self.assertIn("environment: packagist", workflow)
        coordinator = (
            fixtures.ROOT / ".github/workflows/coordinated-release.yml"
        ).read_text()
        self.assertIn("needs: [validate, php-readiness]", coordinator)
        self.assertIn("php-publish-ssh.sh readiness", coordinator)
        self.assertIn("workflow: publish-packagist.yml", coordinator)


if __name__ == "__main__":
    unittest.main()
