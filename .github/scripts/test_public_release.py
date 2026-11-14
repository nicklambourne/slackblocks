"""Completion must prove every exact-version consumer; omissions never pass."""

import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import public_release as public
import release_evidence as evidence
import test_release as common


class PublicReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR"))
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.version, self.source = "4.0.0", "a" * 40

    def receipt(self, language):
        return dict(
            language=language,
            version=self.version,
            source=self.source,
            passed=True,
            state="verified",
            runtime="minimum runtime",
            commands=[{"exit_code": 0}],
            registry_evidence="registry.json",
            payload_sha256="b" * 64,
            run_id="123",
        )

    def consumers(self, directory):
        for language in public.LANGUAGES:
            folder = directory / language
            folder.mkdir(parents=True)
            (folder / "receipt.json").write_text(json.dumps(self.receipt(language)))

    def test_inventory_matches_shared_contract_and_literal_recipes(self):
        self.assertEqual(set(public.LANGUAGES), set(common.release.LANGUAGES))
        outputs = []
        for language in public.LANGUAGES:
            code, payload = public.recipe(common.ROOT, language)
            self.assertTrue(code.strip())
            outputs.append(payload)
        self.assertTrue(all(output == outputs[0] for output in outputs))

    def test_missing_duplicate_stale_pending_and_divergent_consumers_block_completion(self):
        self.consumers(self.root)
        self.assertTrue(public.aggregate(self.root, self.version, self.source)["passed"])
        path = self.root / "php/receipt.json"
        original = path.read_text()
        for changes in (
            {"state": "awaiting_availability", "passed": False},
            {"source": "c" * 40},
            {"version": "3.2.0"},
            {"payload_sha256": "d" * 64},
            {"runtime": ""},
            {"commands": []},
        ):
            path.write_text(json.dumps({**json.loads(original), **changes}))
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                public.aggregate(self.root, self.version, self.source)
        path.unlink()
        with self.assertRaises(ValueError):
            public.aggregate(self.root, self.version, self.source)
        path.write_text(original)
        duplicate = self.root / "duplicate"
        duplicate.mkdir()
        (duplicate / "receipt.json").write_text(original)
        with self.assertRaises(ValueError):
            public.aggregate(self.root, self.version, self.source)

    def test_registry_rejects_wrong_version_and_nuget_absence_is_pending(self):
        with (
            patch.object(public.availability, "json_response", return_value={"version": "3.2.0"}),
            self.assertRaises(ValueError),
        ):
            public.registry("typescript", self.version, self.source)
        with (
            patch.object(
                public.availability, "json_response", return_value={"versions": ["3.2.0"]}
            ),
            self.assertRaises(public.availability.Pending),
        ):
            public.registry("csharp", self.version, self.source)

    def test_consumer_cannot_use_publishing_credentials_or_local_package_sources(self):
        with patch.dict(
            os.environ,
            {
                "GH_TOKEN": "secret",
                "HEX_API_KEY": "secret",
                "COMPOSER_AUTH": "secret",
                "PYTHONPATH": "checkout",
            },
        ):
            env = public.clean_environment(self.root)
        for name in ("GH_TOKEN", "HEX_API_KEY", "COMPOSER_AUTH", "PYTHONPATH"):
            self.assertNotIn(name, env)
        self.assertEqual(env["GOTOOLCHAIN"], "local")
        self.assertEqual(env["GOPROXY"], "https://proxy.golang.org")
        self.assertTrue(env["COMPOSER_HOME"].startswith(str(self.root)))

    def test_aggregate_requires_this_coordinators_successful_publishers(self):
        self.consumers(self.root / "consumers")
        (self.root / "identity").mkdir()
        identity = dict(
            version=self.version,
            source=self.source,
            passed=True,
            state="verified",
            releases=[{"language": language} for language in public.LANGUAGES],
        )
        (self.root / "identity/identity.json").write_text(json.dumps(identity))
        publishers = self.root / "publishers"
        for language in public.LANGUAGES:
            directory = publishers / language
            directory.mkdir(parents=True)
            (directory / "publisher.json").write_text(
                json.dumps(
                    {
                        **self.receipt(language),
                        "state": "publisher_verified",
                        "coordinator_run": "123",
                        "run_id": 42,
                    }
                )
            )
        self.assertTrue(evidence.complete(self.root, self.version, self.source, "123")["complete"])
        with self.assertRaises(ValueError):
            evidence.complete(self.root, self.version, self.source, "124")
        path = publishers / "php/publisher.json"
        record = json.loads(path.read_text())
        path.write_text(json.dumps({**record, "state": "awaiting_availability", "passed": False}))
        with self.assertRaises(ValueError):
            evidence.complete(self.root, self.version, self.source, "123")

    def test_workflow_is_read_only_and_all_minimum_runtimes_are_present(self):
        workflow = (common.ROOT / ".github/workflows/verify-public-release.yml").read_text()
        self.assertNotIn("secrets.", workflow)
        self.assertNotIn("contents: write", workflow)
        self.assertNotIn("id-token: write", workflow)
        self.assertNotIn("workflow run", workflow)
        for version in (
            "3.10",
            "20.19.0",
            "1.22.0",
            "17",
            "8.0.x",
            "3.3",
            "1.85.0",
            "8.2",
            "1.16.3",
            "26.2",
            "9.6.7",
        ):
            self.assertIn(version, workflow)
