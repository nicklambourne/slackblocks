import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import subprocess

spec = importlib.util.spec_from_file_location("consumer", Path(__file__).with_name("verify-nuget-consumer.py"))
consumer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(consumer)


class ExactConsumer(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR"))
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "consumer"

    def test_exact_version_and_public_only_source(self):
        consumer.prepare(self.root, "2.7.0")
        self.assertIn('Version="[2.7.0]"', (self.root / "Consumer.csproj").read_text())
        self.assertIn('<clear/>', (self.root / "NuGet.Config").read_text())
        with self.assertRaises(FileExistsError):
            consumer.prepare(self.root, "2.7.0")

    def test_untrusted_or_floating_version_rejected(self):
        for version in ["*", "2.7.*", '2.7.0"/>', "[2.6,)"]:
            with self.assertRaises(ValueError):
                consumer.prepare(self.root, version)

    def test_fallback_version_and_local_feed_rejected(self):
        consumer.prepare(self.root, "2.7.0")
        (self.root / "obj").mkdir()
        assets = self.root / "obj/project.assets.json"
        assets.write_text(json.dumps({"libraries": {"Slackblocks/2.6.0": {}}}))
        with self.assertRaisesRegex(ValueError, "Expected exactly"):
            consumer.verify_assets(self.root, "2.7.0")
        assets.write_text(json.dumps({"libraries": {"Slackblocks/2.7.0": {}}}))
        package = self.root / "packages/slackblocks/2.7.0"
        package.mkdir(parents=True)
        (package / ".nupkg.metadata").write_text(json.dumps({"source": "local-feed"}))
        with self.assertRaisesRegex(ValueError, "public NuGet"):
            consumer.verify_assets(self.root, "2.7.0")
        (package / ".nupkg.metadata").write_text(json.dumps({"source": consumer.SOURCE}))
        (package / "slackblocks.2.7.0.nupkg").write_bytes(b"test archive")
        self.assertEqual(len(consumer.verify_assets(self.root, "2.7.0")), 64)


    def run_consumer(self, results, attempts=1):
        args = ["verify-nuget-consumer.py", "--version", "2.7.0", "--workdir", str(self.root),
                "--attempts", str(attempts)]
        with patch("sys.argv", args), patch.dict(os.environ, {"NUGET_API_KEY": "not-for-consumer"}), \
             patch.object(consumer.subprocess, "run", side_effect=results) as run, \
             patch.object(consumer.time, "sleep") as sleep, \
             patch.object(consumer, "verify_assets", return_value="a" * 64):
            try:
                consumer.main()
            finally:
                self.calls = run.call_args_list
                self.sleeps = sleep.call_args_list

    def test_clean_retry_then_native_success(self):
        def restore_missing(*args, **kwargs):
            self.assertNotIn("NUGET_API_KEY", kwargs["env"])
            self.assertEqual(kwargs["env"]["NUGET_PACKAGES"], str(self.root / "attempt-1/packages"))
            self.assertIn("--configfile", args[0])
            return subprocess.CompletedProcess(args[0], 1, "error NU1102: Unable to find package Slackblocks")

        def restore_success(*args, **kwargs):
            self.assertEqual(kwargs["env"]["NUGET_PACKAGES"], str(self.root / "attempt-2/packages"))
            return subprocess.CompletedProcess(args[0], 0, "restored")

        def native_success(*args, **kwargs):
            self.assertIn("--no-restore", args[0])
            return subprocess.CompletedProcess(args[0], 0, "native-consumer-ok")

        outcomes = iter([restore_missing, restore_success, native_success])
        self.run_consumer(lambda *a, **kw: next(outcomes)(*a, **kw), attempts=2)
        receipt = json.loads((self.root / "receipt.json").read_text())
        self.assertTrue(receipt["passed"])
        self.assertEqual([a["restore_exit_code"] for a in receipt["attempts"]], [1, 0])
        self.assertEqual(receipt["sha256"], "a" * 64)
        self.assertEqual(len(self.sleeps), 1)
        self.assertEqual(self.sleeps[0].args, (10,))

    def test_other_restore_error_does_not_retry_or_pass(self):
        failure = subprocess.CompletedProcess([], 1, "error NU1102: Missing version\nerror NU1301: Feed unavailable")
        with self.assertRaisesRegex(RuntimeError, "Restore failed"):
            self.run_consumer([failure], attempts=6)
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.sleeps, [])
        self.assertFalse(json.loads((self.root / "receipt.json").read_text())["passed"])

    def test_missing_version_stops_at_attempt_bound(self):
        failure = subprocess.CompletedProcess([], 1, "error NU1102: Unable to find package Slackblocks")
        with self.assertRaisesRegex(RuntimeError, "Restore failed"):
            self.run_consumer([failure] * 6, attempts=6)
        receipt = json.loads((self.root / "receipt.json").read_text())
        self.assertEqual(len(receipt["attempts"]), 6)
        self.assertEqual(len(self.sleeps), 5)
        self.assertFalse(receipt["passed"])

    def test_native_failure_retains_failed_receipt(self):
        results = [subprocess.CompletedProcess([], 0, "restored"),
                   subprocess.CompletedProcess([], 1, "native-consumer-ok")]
        with self.assertRaisesRegex(RuntimeError, "Native consumer failed"):
            self.run_consumer(results)
        self.assertFalse(json.loads((self.root / "receipt.json").read_text())["passed"])
        self.assertTrue((self.root / "attempt-1/consumer.log").exists())

    def test_checkout_workdir_rejected_before_creation(self):
        root = Path(__file__).resolve().parents[2] / "never-create-nuget-consumer"
        with patch("sys.argv", ["consumer", "--version", "2.7.0", "--workdir", str(root)]), \
             patch.object(consumer.subprocess, "run") as run:
            with self.assertRaises(SystemExit):
                consumer.main()
            run.assert_not_called()
        self.assertFalse(root.exists())

    def test_publisher_verifies_before_github_release(self):
        workflow = (Path(__file__).resolve().parents[1] / "workflows/publish-nuget.yml").read_text()
        self.assertLess(workflow.index('dotnet nuget push'), workflow.index('verify-nuget-consumer.py'))
        self.assertLess(workflow.index('verify-nuget-consumer.py'), workflow.index('  github-release:'))
        self.assertIn('    needs: publish', workflow)
        self.assertIn('--attempts 6', workflow)


if __name__ == "__main__":
    unittest.main()
