#!/usr/bin/env python3
"""Build HexDocs in credential-free dry-run mode; never publish a package."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile

PACKAGE = Path(__file__).resolve().parents[1]
scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
if scratch and not Path(scratch).is_dir():
    raise SystemExit("Required /Volumes/Scratch mount is unavailable")
with tempfile.TemporaryDirectory(
    prefix="slackblocks-hex-dry-run-", dir=scratch
) as root:
    env = {**os.environ, "HEX_HOME": root}
    env.pop("HEX_API_KEY", None)
    subprocess.run(
        [shutil.which("mix") or "mix", "hex.publish", "docs", "--dry-run"],
        cwd=PACKAGE,
        env=env,
        check=True,
    )
