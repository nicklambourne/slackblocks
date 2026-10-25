#!/usr/bin/env python3
"""Complete the exact tag's GitHub Release without overwriting an existing asset."""

import json
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path

from release import ROOT, digest, preflight, require, scratch, verify


def gh(*args):
    return subprocess.check_output(["gh", *args], text=True)


def lookup(tag):
    # Exact-tag lookup includes drafts; list indexing must not decide identity.
    result = subprocess.run(
        ["gh", "release", "view", tag, "--json", "tagName,isDraft,assets"],
        capture_output=True,
        text=True,
    )
    if result.returncode == 0:
        return json.loads(result.stdout)
    if "release not found" in result.stderr.lower():
        return None
    raise subprocess.CalledProcessError(
        result.returncode, result.args, stderr=result.stderr
    )


def wait_release(tag, attempts=6, sleep=time.sleep):
    for attempt in range(attempts):
        release = lookup(tag)
        if release is not None:
            return release
        if attempt + 1 < attempts:
            sleep(2)
    raise preflight.ReleaseError(
        "Exact-tag GitHub release did not become visible within the bounded wait"
    )


def release_assets(root, directory, ref):
    version = preflight.publisher(root, "haskell", ref)
    verify(root, directory)
    tag = ref.removeprefix("refs/tags/")
    release = lookup(tag)
    if release is None:
        notes = re.search(
            r"^## \[" + re.escape(version) + r"\] [^\n]*\n([\s\S]*?)(?=^## \[|\Z)",
            (root / "haskell/CHANGELOG.md").read_text(),
            re.M,
        )[1].strip()
        with tempfile.TemporaryDirectory(
            prefix="slackblocks-hackage-notes-", dir=scratch()
        ) as tmp:
            file = Path(tmp) / "notes.md"
            file.write_text(notes + "\n")
            gh(
                "release",
                "create",
                tag,
                "--draft",
                "--verify-tag",
                "--title",
                f"Haskell v{version}",
                "--notes-file",
                str(file),
            )
        release = wait_release(tag)
    require(
        release["tagName"] == tag, "Existing GitHub Release must name the exact tag"
    )
    names = {asset["name"] for asset in release["assets"]}
    for name in [*names(version), "release.json"]:
        if name in names:
            with tempfile.TemporaryDirectory(
                prefix="slackblocks-hackage-asset-", dir=scratch()
            ) as tmp:
                gh("release", "download", tag, "--pattern", name, "--dir", tmp)
                require(
                    digest(Path(tmp) / name) == digest(directory / name),
                    "Existing GitHub asset checksum differs",
                )
        else:
            gh("release", "upload", tag, str(directory / name))
    # Publish only after all immutable assets are present and verified.
    if release["isDraft"]:
        gh("release", "edit", tag, "--draft=false")


def main():
    release_assets(
        ROOT, ROOT / "haskell/dist-release", os.environ.get("GITHUB_REF", "")
    )


if __name__ == "__main__":
    main()
