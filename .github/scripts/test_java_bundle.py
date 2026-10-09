"""Regression checks for the archive Maven uploads to Central."""

import hashlib
import importlib.util
import tempfile
import unittest
import zipfile
from pathlib import Path

spec = importlib.util.spec_from_file_location(
    "java_bundle", Path(__file__).with_name("check-java-bundle.py")
)
bundle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bundle)


class BundleTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.archive = Path(self.temp.name) / "bundle.zip"
        self.files = {}
        prefix = "io/github/nicklambourne/slackblocks/2.6.0/slackblocks-2.6.0"
        for suffix in (".pom", ".jar", "-sources.jar", "-javadoc.jar"):
            name = prefix + suffix
            self.files[name] = b"artifact"
            self.files[name + ".asc"] = b"detached signature checked by gpg in CI"
            for algorithm in ("md5", "sha1", "sha256", "sha512"):
                self.files[name + "." + algorithm] = (
                    hashlib.new(algorithm, b"artifact").hexdigest().encode()
                )

    def verify(self):
        with zipfile.ZipFile(self.archive, "w") as archive:
            for name, data in self.files.items():
                archive.writestr(name, data)
        bundle.verify_bundle(self.archive, "2.6.0")

    def test_complete_version_directory(self):
        self.verify()

    def test_rejects_maven_310_local_metadata(self):
        self.files["io/github/nicklambourne/slackblocks/maven-metadata-local.xml"] = b"metadata"
        with self.assertRaisesRegex(ValueError, "unexpected=.*maven-metadata-local"):
            self.verify()

    def test_rejects_missing_signature(self):
        del self.files[next(name for name in self.files if name.endswith(".pom.asc"))]
        with self.assertRaisesRegex(ValueError, "missing=.*pom.asc"):
            self.verify()

    def test_rejects_corrupt_checksum(self):
        self.files[next(name for name in self.files if name.endswith(".jar.sha256"))] = b"bad"
        with self.assertRaisesRegex(ValueError, "Invalid sha256 checksum"):
            self.verify()


if __name__ == "__main__":
    unittest.main()
