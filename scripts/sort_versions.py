#!/usr/bin/env python3
"""Sort ``versions.json`` on the docs ``gh-pages`` branch by PEP 440 semver,
newest first.

``mike deploy`` appends each new deploy to the end of ``versions.json``
rather than inserting it in sorted position. Run this after every deploy
(or one-off, to repair the order) so the docs version selector shows the
most-recent release at the top.

The ``master`` development entry (if present) is pinned at the very top so
``mike``'s selector continues to show it as the most-current option.

Usage::

    python scripts/sort_versions.py [path/to/versions.json]

The default path is ``./versions.json``. The script exits non-zero with a
descriptive message if the file is missing or unparseable. It does NOT
commit, push, or modify any other files -- the caller is responsible for
git plumbing.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

try:
    from packaging.version import InvalidVersion, Version
except ImportError:  # pragma: no cover - documented prerequisite
    print(
        "error: this script requires the 'packaging' library "
        "(install via 'pip install packaging' or 'uv add packaging').",
        file=sys.stderr,
    )
    sys.exit(2)


def sort_key(entry: dict) -> tuple:
    """Sort key for one ``versions.json`` entry.

    Returns a tuple usable as a sort key. ``master`` (and any other
    non-version dev preview) sorts above release versions; release
    versions sort by parsed PEP 440 ``Version``; anything unparseable
    sorts below the release versions but is otherwise preserved.
    """
    v = entry["version"]
    if v == "master":
        # 'master' wins above everything else.
        return (0, "")
    raw = v.removeprefix("v")
    try:
        parsed = Version(raw)
    except InvalidVersion:
        return (2, v)
    return (1, parsed)


def sort_versions(path: Path) -> bool:
    """Sort ``path`` in place. Returns True if the file content changed."""
    original_text = path.read_text()
    data = json.loads(original_text)

    master = [e for e in data if e.get("version") == "master"]
    others = sorted(
        (e for e in data if e.get("version") != "master"),
        key=sort_key,
        reverse=True,  # newest version first
    )
    sorted_data = master + others

    new_text = json.dumps(sorted_data, indent=2) + "\n"
    if new_text == original_text:
        return False
    path.write_text(new_text)
    return True


def main(argv: list[str]) -> int:
    path = Path(argv[1]) if len(argv) > 1 else Path("versions.json")
    if not path.exists():
        print(f"error: {path} does not exist", file=sys.stderr)
        return 1
    changed = sort_versions(path)
    if changed:
        print(f"sorted {path}")
    else:
        print(f"{path} already sorted")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
