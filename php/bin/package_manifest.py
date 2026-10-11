#!/usr/bin/env python3
"""Generate the monorepo's public Composer descriptor from the PHP development package."""

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PACKAGE_FILES = {
    "README.md",
    "IMPLEMENTATION.md",
    "CHANGELOG.md",
    "LICENSE",
    "LICENSE.BSD-3-Clause",
}
DIST_URL = (
    "https://github.com/nicklambourne/slackblocks/releases/download/"
    "php/%prettyVersion%/slackblocks-php.zip"
)


def manifest(root):
    package = json.loads((root / "php/composer.json").read_text())
    for field in ("require-dev", "autoload-dev", "scripts", "config", "archive"):
        package.pop(field, None)
    package["autoload"] = {"psr-4": {"Slackblocks\\": "php/src/"}}
    package["dist"] = {"type": "zip", "url": DIST_URL}
    package["archive"] = {
        "exclude": [
            "/**",
            "/.*",
            "!/composer.json",
            "!/php/src/**",
            "!/php/examples/consumer.php",
            *["!/php/" + name for name in sorted(PACKAGE_FILES)],
        ]
    }
    return package


def check(root):
    if json.loads((root / "composer.json").read_text()) != manifest(root):
        raise ValueError("Root Composer manifest is stale; run python php/bin/package_manifest.py")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        check(ROOT)
    else:
        (ROOT / "composer.json").write_text(json.dumps(manifest(ROOT), indent=4) + "\n")


if __name__ == "__main__":
    main()
