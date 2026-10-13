import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest

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


if __name__ == "__main__":
    unittest.main()
