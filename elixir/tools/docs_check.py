#!/usr/bin/env python3
"""Build HexDocs without credentials and verify rendered monorepo source links."""

import os
import shutil
import subprocess
import tempfile
from html.parser import HTMLParser
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]


class SourceLinks(HTMLParser):
    def __init__(self):
        super().__init__()
        self.links = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "a" and "View Source" in (attrs.get("title"), attrs.get("aria-label")):
            self.links.append(attrs.get("href", ""))


def verify_source_links(directory, reference):
    parser = SourceLinks()
    for page in directory.glob("*.html"):
        parser.feed(page.read_text())
    repository = "https://github.com/nicklambourne/slackblocks"
    # ExDoc's generated API index links to the repository itself.
    links = [link for link in parser.links if link != repository]
    prefix = f"{repository}/blob/{reference}/elixir/"
    if not links or any(not link.startswith(prefix) or "%{" in link for link in links):
        raise SystemExit("HexDocs source links must resolve to the exact commit's elixir/ files")
    print(f"Verified {len(links)} rendered HexDocs source links at {reference}")


def main():
    scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
    if scratch and not Path(scratch).is_dir():
        raise SystemExit("Required /Volumes/Scratch mount is unavailable")
    reference = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=PACKAGE, text=True
    ).strip()
    with tempfile.TemporaryDirectory(prefix="slackblocks-hex-dry-run-", dir=scratch) as root:
        env = {**os.environ, "HEX_HOME": root, "SLACKBLOCKS_SOURCE_REF": reference}
        env.pop("HEX_API_KEY", None)
        subprocess.run(
            [shutil.which("mix") or "mix", "hex.publish", "docs", "--dry-run"],
            cwd=PACKAGE,
            env=env,
            check=True,
        )
        verify_source_links(PACKAGE / "doc", reference)


if __name__ == "__main__":
    main()
