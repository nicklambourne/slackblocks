import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("availability", Path(__file__).with_name("release_availability.py"))
a = importlib.util.module_from_spec(spec)
spec.loader.exec_module(a)


class Availability(unittest.TestCase):
    def setUp(self):
        import os
        self.tmp = tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR"))
        self.addCleanup(self.tmp.cleanup)
        self.path = Path(self.tmp.name) / "receipt.json"
        self.now = 0

    def sleep(self, seconds):
        self.now += seconds

    def wait(self, probe, timeout=60):
        return a.wait(probe, self.path, {"version": "4.0.0"}, timeout=timeout, interval=30,
                      clock=lambda: self.now, sleep=self.sleep)

    def test_delayed_version_becomes_available_but_not_verified(self):
        with patch.object(a, "packagist", side_effect=[a.Pending("old index"), {"version": "4.0.0"}]) as probe:
            self.assertEqual(self.wait(probe), {"version": "4.0.0"})
        receipt = json.loads(self.path.read_text())
        self.assertEqual(receipt["state"], "available")
        self.assertFalse(receipt["passed"])
        self.assertEqual(len(receipt["observations"]), 1)

    def test_pending_deadline_is_not_success(self):
        with self.assertRaises(a.Pending):
            self.wait(lambda: (_ for _ in ()).throw(a.Pending("absent")))
        self.assertEqual(self.now, 60)
        self.assertEqual(json.loads(self.path.read_text())["state"], "awaiting_availability")

    def test_wrong_identity_fails_without_retry(self):
        with self.assertRaises(ValueError):
            self.wait(lambda: (_ for _ in ()).throw(ValueError("wrong commit")))
        self.assertEqual(self.now, 0)
        self.assertEqual(json.loads(self.path.read_text())["state"], "failed")

    def test_nuget_only_retries_missing_target_version(self):
        target = "error NU1102: Unable to find package Slackblocks with version (= 4.0.0)"
        self.assertTrue(a.missing_nuget_version(target))
        for extra in ["\nerror NU1301: denied", "\nerror NU1102: Unable to find package Other with version (= 1.0.0)"]:
            self.assertFalse(a.missing_nuget_version(target + extra))

    def test_pending_response_retains_cache_diagnostics_without_cookie_headers(self):
        from types import SimpleNamespace
        response = SimpleNamespace(code=404, headers={"Age": "7200", "ETag": "old", "Set-Cookie": "private"})
        metadata = a.response_metadata(response, "https://registry.example/package")
        self.assertEqual(metadata["headers"], {"Age": "7200", "ETag": "old"})
        with self.assertRaises(a.Pending):
            self.wait(lambda: (_ for _ in ()).throw(a.Pending("stale", metadata)), timeout=0)
        receipt = json.loads(self.path.read_text())
        self.assertEqual(receipt["observations"][0]["response"]["headers"]["Age"], "7200")
        self.assertFalse(receipt["passed"])

    def test_normal_composer_feed_and_commit_guard(self):
        package = {"version": "v4.1.0", "description": "removed", "source": {"reference": "sha", "url": "https://github.com/nicklambourne/slackblocks.git"}, "dist": {"reference": "sha"}}
        data = {"packages": {"nicklambourne/slackblocks": [package, {"version": "v4.0.0", "description": "__unset"}]}}
        with patch.object(a, "json_response", return_value=data) as fetch:
            self.assertEqual(a.packagist("4.0.0", "sha")["version"], "v4.0.0")
            self.assertNotIn("?", fetch.call_args.args[0])
            self.assertNotIn("description", a.packagist("4.0.0", "sha"))
            with self.assertRaises(ValueError):
                a.packagist("4.0.0", "wrong")
            with self.assertRaises(a.Pending):
                a.packagist("4.2.0", "sha")


if __name__ == "__main__":
    unittest.main()
