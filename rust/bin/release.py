#!/usr/bin/env python3
"""Prepare and verify the Cargo artifact; no registry writes are implemented here."""

import argparse
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOLCHAIN = "1.85.0"
_spec = importlib.util.spec_from_file_location(
    "release_preflight", ROOT / ".github/scripts/verify-release.py"
)
preflight = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(preflight)


def authentication(root):
    package = preflight.rust_package(root)
    policy = package["metadata"]["slackblocks-release"]
    mode = policy["authentication"]
    preflight.require(
        mode in ("token", "trusted"),
        "Choose explicit token or trusted authentication",
    )
    return mode


def check_registry(version, mode, opener=urllib.request.urlopen):
    request = urllib.request.Request(
        "https://crates.io/api/v1/crates/slackblocks",
        headers={"User-Agent": "slackblocks-release (github.com/nicklambourne/slackblocks)"},
    )
    try:
        with opener(request, timeout=30) as response:
            data = json.load(response)
    except urllib.error.HTTPError as error:
        with error:
            if error.code != 404:
                raise
        data = None
    if mode == "trusted":
        preflight.require(
            data is not None,
            "Trusted publishing requires the existing crate and configured publisher",
        )
    if data is not None:
        preflight.require(
            not any(v["num"] == version for v in data["versions"]),
            "This crate version is already published; recover a missing GitHub Release separately",
        )


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def package(root):
    command = ["cargo", "+" + TOOLCHAIN]
    subprocess.run(
        command + ["package", "--manifest-path", str(root / "rust/Cargo.toml"), "--locked"],
        cwd=root,
        check=True,
    )
    metadata = json.loads(
        subprocess.check_output(
            command
            + [
                "metadata",
                "--manifest-path",
                str(root / "rust/Cargo.toml"),
                "--no-deps",
                "--format-version",
                "1",
            ],
            cwd=root,
        )
    )
    version = preflight.rust_package(root)["version"]
    return Path(metadata["target_directory"]) / "package" / f"slackblocks-{version}.crate"


def prepare(root, directory):
    version = preflight.validate(root)
    archive = package(root)
    directory.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(archive, directory / archive.name)
    record = {
        "version": version,
        "commit": preflight.git(root, "rev-parse", "HEAD"),
        "toolchain": TOOLCHAIN,
        "sha256": sha256(archive),
        "authentication": authentication(root),
    }
    (directory / "release.json").write_text(json.dumps(record, indent=2) + "\n")
    return record


def verify_artifact(root, directory, rebuild=False):
    version = preflight.validate(root)
    record = json.loads((directory / "release.json").read_text())
    expected = {
        "version": version,
        "commit": preflight.git(root, "rev-parse", "HEAD"),
        "toolchain": TOOLCHAIN,
        "authentication": authentication(root),
    }
    preflight.require(
        all(record.get(k) == v for k, v in expected.items()),
        "Artifact metadata does not match the release checkout",
    )
    archive = directory / f"slackblocks-{version}.crate"
    preflight.require(
        sorted(p.name for p in directory.glob("*.crate")) == [archive.name],
        "Require exactly the expected crate artifact",
    )
    preflight.require(
        sha256(archive) == record["sha256"],
        "Downloaded crate checksum differs from the tested artifact",
    )
    if rebuild:
        preflight.require(
            sha256(package(root)) == record["sha256"],
            "Cargo reproduction differs from the tested artifact; do not publish",
        )
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("prepare", "verify", "registry"))
    parser.add_argument("--directory", type=Path, default=ROOT / "rust/dist")
    args = parser.parse_args()
    if args.command == "prepare":
        record = prepare(ROOT, args.directory)
        if output := os.environ.get("GITHUB_OUTPUT"):
            with open(output, "a") as stream:
                stream.write(
                    f"version={record['version']}\nauthentication={record['authentication']}\n"
                )
        print(f"Prepared {record['version']}: {record['sha256']}")
    elif args.command == "verify":
        record = verify_artifact(ROOT, args.directory, rebuild=True)
        print(f"Verified exact Cargo reproduction: {record['sha256']}")
    else:
        version = preflight.publisher(ROOT, "rust", os.environ.get("GITHUB_REF", ""))
        check_registry(version, authentication(ROOT))
        print(f"Registry preflight passed for {version}; no registry write performed")


if __name__ == "__main__":
    main()
