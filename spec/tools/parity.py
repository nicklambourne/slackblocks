"""Generate the documentation parity page from conformance skip lists."""

from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SPEC_ROOT = REPO_ROOT / "spec"
IMPLEMENTATIONS = ("python", "typescript")
LABELS = {"python": "Python", "typescript": "TypeScript"}


def entries(path: Path) -> list[str]:
    return [
        line.split(maxsplit=1)[0]
        for line in path.read_text().splitlines()
        if line and not line.startswith("#")
    ]


def render() -> str:
    manifest = json.loads((SPEC_ROOT / "manifest.json").read_text())
    invalid = json.loads((SPEC_ROOT / "fixtures/invalid/manifest.json").read_text())
    total = len(manifest["fixtures"]) + len(invalid["cases"])
    lines = [
        "---",
        "title: Language parity",
        "---",
        "",
        "# Language parity",
        "",
        f"Generated from spec {manifest['spec_version']} manifests and checked-in skip lists.",
        "",
        "| Implementation | Passing | Skipped | Conformance |",
        "|---|---:|---:|---:|",
    ]
    for implementation in IMPLEMENTATIONS:
        skipped = entries(REPO_ROOT / implementation / "conformance/skiplist.txt")
        passing = total - len(skipped)
        lines.append(
            f"| {LABELS[implementation]} | {passing} | {len(skipped)} | {passing / total:.0%} |"
        )
    lines.extend(
        [
            "",
            f"The contract currently contains {len(manifest['fixtures'])} valid fixtures and "
            f"{len(invalid['cases'])} invalid cases.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    output = (
        Path(sys.argv[1]) if len(sys.argv) > 1 else REPO_ROOT / "docs/docs/reference/parity.mdx"
    )
    if not output.is_absolute():
        output = REPO_ROOT / "docs" / output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(render())


if __name__ == "__main__":
    main()
