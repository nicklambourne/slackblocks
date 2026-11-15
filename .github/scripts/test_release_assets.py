"""Public identity must reject absent assets and self-consistent wrong archives."""

import copy
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
import zipfile

import release_evidence as evidence


def sha(data):
    return hashlib.sha256(data).hexdigest()


def tar(files):
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode="w:gz") as archive:
        for name, data in files.items():
            member = tarfile.TarInfo(name)
            member.size = len(data)
            archive.addfile(member, io.BytesIO(data))
    return stream.getvalue()


class ReleaseAssetTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR"))
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.version, self.repo = "4.0.0", "nicklambourne/slackblocks"
        paths = [
            "composer.json",
            "php/src/MessagePayload.php",
            "php/examples/consumer.php",
        ]
        paths += [
            "php/" + name
            for name in (
                "README.md",
                "IMPLEMENTATION.md",
                "CHANGELOG.md",
                "LICENSE",
                "LICENSE.BSD-3-Clause",
            )
        ]
        paths += [
            "elixir/" + name
            for name in (
                "lib/slackblocks.ex",
                "mix.exs",
                "mix.lock",
                "README.md",
                "CHANGELOG.md",
                "LICENSE",
                "LICENSE.BSD-3-Clause",
            )
        ]
        paths += [
            "haskell/" + name
            for name in (
                "src/Slackblocks.hs",
                "slackblocks.cabal",
                "README.md",
                "LICENSE",
                "dependencies/locked-9.6.7.json",
            )
        ]
        self.files = {name: (name + "\n").encode() for name in paths}
        for name, data in self.files.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        self.git("init", "--quiet")
        self.git("config", "user.name", "Release fixture")
        self.git("config", "user.email", "fixture@example.invalid")
        self.git("add", ".")
        self.git("commit", "--quiet", "-m", "Immutable fixture")
        self.source = self.git("rev-parse", "HEAD").strip()

    def git(self, *args):
        return subprocess.check_output(
            ["git", "-c", "commit.gpgsign=false", *args],
            cwd=self.root,
            text=True,
            stderr=subprocess.PIPE,
        )

    def assets(self, language):
        record = dict(version=self.version)
        if language == "php":
            files = {
                name: data
                for name, data in self.files.items()
                if name.startswith("php/") or name == "composer.json"
            }
            stream = io.BytesIO()
            with zipfile.ZipFile(stream, "w") as archive:
                for name, data in files.items():
                    archive.writestr(name, data)
            assets = {"slackblocks-php.zip": stream.getvalue()}
            record.update(
                source_commit=self.source,
                repository=self.repo,
                package="nicklambourne/slackblocks",
                files={name: sha(data) for name, data in files.items()},
                sha256=sha(stream.getvalue()),
            )
        elif language == "elixir":
            files = {
                name.removeprefix("elixir/"): data
                for name, data in self.files.items()
                if name.startswith("elixir/") and name != "elixir/mix.lock"
            }
            assets = {
                "slackblocks.tar": tar(
                    {"contents.tar.gz": tar(files), "metadata.config": b"fixture"}
                )
            }
            record.update(
                commit=self.source,
                lock_sha256=sha(self.files["elixir/mix.lock"]),
                sha256=sha(assets["slackblocks.tar"]),
            )
        else:
            files = {
                f"slackblocks-{self.version}/" + name.removeprefix("haskell/"): data
                for name, data in self.files.items()
                if name.startswith("haskell/")
            }
            assets = {
                f"slackblocks-{self.version}.tar.gz": tar(files),
                f"slackblocks-{self.version}-docs.tar.gz": tar(
                    {
                        f"slackblocks-{self.version}-docs/Slackblocks-Models.html": b"<html>API</html>"
                    }
                ),
            }
            record.update(
                commit=self.source,
                dependencies_sha256=sha(
                    self.files["haskell/dependencies/locked-9.6.7.json"]
                ),
                source_members=len(files),
                assets={name: sha(data) for name, data in assets.items()},
            )
        return {**assets, "release.json": json.dumps(record).encode()}

    def verify(self, language, assets):
        evidence.verify_asset_contents(
            language, self.version, self.source, self.repo, assets, self.root
        )

    def test_each_asset_contract_accepts_complete_source_bound_archives(self):
        for language in ("php", "elixir", "haskell"):
            with self.subTest(language=language):
                self.verify(language, self.assets(language))
        for language in (
            "python",
            "typescript",
            "go",
            "java",
            "csharp",
            "ruby",
            "rust",
        ):
            self.verify(language, {})

    def test_missing_and_unexpected_assets_fail_for_each_published_archive(self):
        for language in ("php", "elixir", "haskell"):
            assets = self.assets(language)
            for missing in assets:
                with (
                    self.subTest(language=language, missing=missing),
                    self.assertRaises(ValueError),
                ):
                    self.verify(
                        language,
                        {
                            name: data
                            for name, data in assets.items()
                            if name != missing
                        },
                    )
            for invalid in ({}, {**assets, "unexpected.zip": b"extra"}):
                with self.assertRaises(ValueError):
                    self.verify(language, invalid)

    def test_wrong_source_version_and_archive_hashes_are_fatal(self):
        for language in ("php", "elixir", "haskell"):
            assets = self.assets(language)
            original = json.loads(assets["release.json"])
            for field, value in (
                ("version", "3.2.0"),
                ("source_commit" if language == "php" else "commit", "b" * 40),
            ):
                with (
                    self.subTest(language=language, field=field),
                    self.assertRaises(ValueError),
                ):
                    self.verify(
                        language,
                        {
                            **assets,
                            "release.json": json.dumps(
                                {**original, field: value}
                            ).encode(),
                        },
                    )
            archive = next(name for name in assets if name != "release.json")
            with self.assertRaises(ValueError):
                self.verify(language, {**assets, archive: assets[archive] + b"changed"})

    def test_even_matching_archive_and_manifest_checksums_must_match_tagged_source(
        self,
    ):
        for language in ("php", "elixir", "haskell"):
            original = copy.deepcopy(self.files)
            name = {
                "php": "php/src/MessagePayload.php",
                "elixir": "elixir/lib/slackblocks.ex",
                "haskell": "haskell/src/Slackblocks.hs",
            }[language]
            self.files[name] = (
                b"different source with matching forged manifest checksum"
            )
            assets = self.assets(language)
            self.files = original
            with (
                self.subTest(language=language),
                self.assertRaisesRegex(ValueError, "tagged source"),
            ):
                self.verify(language, assets)

    def test_modified_lock_and_dependency_inputs_are_rejected(self):
        for language, field in (
            ("elixir", "lock_sha256"),
            ("haskell", "dependencies_sha256"),
        ):
            assets = self.assets(language)
            record = json.loads(assets["release.json"])
            record[field] = "b" * 64
            assets["release.json"] = json.dumps(record).encode()
            with self.assertRaises(ValueError):
                self.verify(language, assets)
