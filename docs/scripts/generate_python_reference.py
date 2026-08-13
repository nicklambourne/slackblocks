"""Generate Docusaurus Markdown reference pages from Griffe-discovered exports."""

from __future__ import annotations

import importlib
import inspect
import json
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

import griffe

DOCS_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = DOCS_ROOT.parent
PYTHON_ROOT = REPO_ROOT / "python"
OUTPUT_ROOT = DOCS_ROOT / "docs" / "reference" / "python"

CATEGORIES = {
    "attachments": "Attachments",
    "blocks": "Blocks",
    "builder": "Utilities",
    "elements": "Elements",
    "errors": "Errors",
    "messages": "Messages",
    "modals": "Modals",
    "objects": "Composition objects",
    "rich_text": "Rich text",
    "utils": "Utilities",
    "views": "Views",
}


def category_for(value: Any) -> str:
    module = getattr(value, "__module__", "slackblocks")
    tail = module.removeprefix("slackblocks.").split(".", 1)[0]
    return tail if tail in CATEGORIES else "objects"


def signature(value: Any) -> str:
    try:
        return str(inspect.signature(value))
    except (TypeError, ValueError):
        return ""


def render_symbol(name: str, value: Any) -> str:
    kind = "Class" if inspect.isclass(value) else "Function" if callable(value) else "Value"
    definition = f"{name}{signature(value)}" if callable(value) else name
    doc = inspect.getdoc(value) or "No public documentation is available."
    return "\n".join(
        [
            f"## `{name}`",
            "",
            f"**{kind}**",
            "",
            "```python",
            definition,
            "```",
            "",
            f'<div className="api-docstring">{{{json.dumps(doc)}}}</div>',
            "",
        ]
    )


def main() -> None:
    analysis = griffe.load(PYTHON_ROOT / "slackblocks", submodules=True)
    sys.path.insert(0, str(PYTHON_ROOT))
    package = importlib.import_module("slackblocks")
    discovered = {
        name for name in analysis.members if not name.startswith("_") and hasattr(package, name)
    }
    runtime_exports = {
        name for name in vars(package) if not name.startswith("_") and name != "name"
    }
    exports = sorted(discovered | runtime_exports)

    grouped: dict[str, list[tuple[str, Any]]] = defaultdict(list)
    for name in exports:
        grouped[category_for(getattr(package, name))].append((name, getattr(package, name)))

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    index = ["# Python API reference", "", f"{len(exports)} public symbols are documented.", ""]
    for category in sorted(grouped):
        title = CATEGORIES[category]
        index.append(f"- [{title}](/reference/python/{category})")
        page = [f"# {title}", ""]
        page.extend(render_symbol(name, value) for name, value in grouped[category])
        (OUTPUT_ROOT / f"{category}.mdx").write_text("\n".join(page))
    (OUTPUT_ROOT / "index.mdx").write_text("\n".join(index) + "\n")

    if len(exports) < 86:
        raise RuntimeError(f"Expected at least 86 public symbols, generated {len(exports)}")
    print(f"Generated Python reference for {len(exports)} public symbols")


if __name__ == "__main__":
    main()
