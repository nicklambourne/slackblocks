#!/usr/bin/env python3
"""Exercise real pinned Hex archives across independent filesystem layouts."""

import os
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
FILES = {
    "mix.exs": """defmodule ArchiveProbe.MixProject do
  use Mix.Project
  def project do
    [app: :archive_probe, version: "1.0.0", description: "Hex reproduction regression fixture",
     package: [licenses: ["MIT"], links: %{"Source" => "https://example.com"}, files: ~w(lib mix.exs README.md LICENSE)]]
  end
end
""",
    "lib/archive_probe.ex": "defmodule ArchiveProbe do\n  def value, do: 42\nend\n",
    "lib/nested/other.ex": "defmodule ArchiveProbe.Other do\n  def value, do: :ok\nend\n",
    "README.md": "A real archive reproduction fixture.\n",
    "LICENSE": "MIT\n",
}


class ArchiveTests(unittest.TestCase):
    def setUp(self):
        volumes = Path("/Volumes/Repos").is_dir()
        if volumes and not all(Path("/Volumes/" + p).is_dir() for p in ("Cache", "Scratch")):
            self.fail("Required external storage is unavailable")
        self.temp = tempfile.TemporaryDirectory(dir="/Volumes/Scratch" if volumes else None)
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.env = dict(os.environ)
        for key in ("MIX_BUILD_PATH", "MIX_BUILD_ROOT", "MIX_DEPS_PATH", "MIX_ENV", "HEX_API_KEY"):
            self.env.pop(key, None)

    def fixture(self, name, reverse=False):
        root = self.base / name
        root.mkdir()
        items = list(FILES.items())
        for name, content in reversed(items) if reverse else items:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        return root

    def run_mix(self, root, *args):
        result = subprocess.run(
            ["mix", *args], cwd=root, env=self.env, text=True, capture_output=True
        )
        if result.returncode:
            raise subprocess.CalledProcessError(
                result.returncode, result.args, result.stdout, result.stderr
            )
        return result

    def build(self, root, name, reference=None):
        destination = self.base / (name + ".tar")
        args = [str(destination)]
        if reference:
            with tarfile.open(reference) as archive:
                metadata = self.base / (name + ".config")
                metadata.write_bytes(archive.extractfile("metadata.config").read())
            args.append(str(metadata))
        self.run_mix(
            root,
            "do",
            "compile",
            "--no-prune-code-paths",
            "+",
            "run",
            "--no-start",
            str(TOOLS / "release_archive.exs"),
            *args,
        )
        return destination

    def test_new_archives_ignore_file_creation_order(self):
        first = self.build(self.fixture("first"), "first")
        second = self.build(self.fixture("second", reverse=True), "second")
        self.assertEqual(first.read_bytes(), second.read_bytes())

    def test_existing_native_archive_reproduces_on_an_independent_tree(self):
        original = self.base / "native.tar"
        self.run_mix(self.fixture("native", reverse=True), "hex.build", "--output", str(original))
        rebuilt = self.build(self.fixture("fresh"), "reproduced", reference=original)
        self.assertEqual(original.read_bytes(), rebuilt.read_bytes())

    def test_changed_source_or_inventory_cannot_reproduce(self):
        original = self.build(self.fixture("original"), "original")
        changed = self.fixture("changed")
        (changed / "README.md").write_text("Changed package content\n")
        rebuilt = self.build(changed, "changed", reference=original)
        self.assertNotEqual(original.read_bytes(), rebuilt.read_bytes())
        (changed / "lib/extra.ex").write_text("defmodule Extra do\nend\n")
        with self.assertRaises(subprocess.CalledProcessError) as error:
            self.build(changed, "extra", reference=original)
        self.assertIn("file inventory differs", error.exception.stderr)
        (changed / "lib/extra.ex").unlink()
        (changed / "lib/nested/other.ex").unlink()
        with self.assertRaises(subprocess.CalledProcessError) as error:
            self.build(changed, "missing", reference=original)
        self.assertIn("file inventory differs", error.exception.stderr)

    def test_upload_checksum_guard_precedes_authentication(self):
        root = self.fixture("upload")
        archive = self.build(root, "upload")
        with self.assertRaises(subprocess.CalledProcessError) as error:
            self.run_mix(
                root,
                "do",
                "compile",
                "--no-prune-code-paths",
                "+",
                "run",
                "--no-start",
                str(TOOLS / "publish_archive.exs"),
                str(archive),
                "0" * 64,
            )
        self.assertIn("archive checksum differs", error.exception.stderr)


if __name__ == "__main__":
    unittest.main()
