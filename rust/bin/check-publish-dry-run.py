#!/usr/bin/env python3
"""Exercise native cargo publish --dry-run on a disposable activated archive copy."""

import json
import os
import re
import subprocess
import tarfile
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
cargo = ["cargo", "+1.85.0"]
subprocess.run(
    cargo + ["package", "--locked", "--manifest-path", str(ROOT / "rust/Cargo.toml")],
    check=True,
)
metadata = json.loads(
    subprocess.check_output(
        cargo
        + [
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
            str(ROOT / "rust/Cargo.toml"),
        ]
    )
)
version = next(p["version"] for p in metadata["packages"] if p["name"] == "slackblocks")
archive = Path(metadata["target_directory"]) / "package" / f"slackblocks-{version}.crate"
scratch = None
if Path("/Volumes/Repos").exists():
    for mount in ("Repos", "Cache", "Scratch"):
        assert os.path.ismount("/Volumes/" + mount), f"Missing /Volumes/{mount}"
    scratch = "/Volumes/Scratch"
with tempfile.TemporaryDirectory(prefix="slackblocks-publish-dry-run-", dir=scratch) as directory:
    directory = Path(directory)
    with tarfile.open(archive) as tar:
        tar.extractall(directory, filter="data")
    source = directory / f"slackblocks-{version}"
    manifest = source / "Cargo.toml"
    # Cargo correctly rejects publish=false even for dry runs. Only this extracted
    # temporary manifest is activated when necessary; the source manifest is unchanged.
    manifest.write_text(
        re.sub(
            r"^publish = false$",
            'publish = ["crates-io"]',
            manifest.read_text(),
            flags=re.M,
        )
    )
    subprocess.run(
        cargo + ["publish", "--dry-run", "--locked", "--registry", "crates-io"],
        cwd=source,
        check=True,
    )
print("Native Cargo publish dry run passed from the standalone archive; nothing was uploaded")
