"""Generate Docusaurus Markdown reference pages from Griffe-discovered exports."""

from __future__ import annotations

import importlib
import inspect
import re
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

LEGACY_REFERENCE_LINK = re.compile(
    r"(?<=\()/slackblocks/(?:latest/)?reference/"
    r"(?P<category>[a-z_]+)/?(?:#(?:[a-z_]+\.)?(?P<symbol>[A-Za-z0-9_]+))?"
)
GOOGLE_SECTION = re.compile(
    r"^(?:Args|Arguments|Parameters|Raises|Throws|Returns|Yields|Examples):$",
    re.MULTILINE,
)


def current_reference_link(match: re.Match[str]) -> str:
    path = f"/reference/python/{match.group('category')}"
    symbol = match.group("symbol")
    return f"{path}#{symbol.lower()}" if symbol else path


def category_for(value: Any) -> str:
    module = getattr(value, "__module__", "slackblocks")
    tail = module.removeprefix("slackblocks.").split(".", 1)[0]
    return tail if tail in CATEGORIES else "objects"


def signature(value: Any) -> str:
    try:
        return str(inspect.signature(value))
    except (TypeError, ValueError):
        return ""


def markdown_cell(value: Any) -> str:
    lines = (str(value) if value is not None else "").strip().splitlines()
    return "<br />".join(line.strip() for line in lines).replace("|", r"\|") or "—"


def markdown_table(headers: tuple[str, ...], rows: list[tuple[Any, ...]]) -> str:
    header = "| " + " | ".join(headers) + " |"
    separator = "| " + " | ".join("---" for _ in headers) + " |"
    body = ["| " + " | ".join(markdown_cell(cell) for cell in row) + " |" for row in rows]
    return "\n".join([header, separator, *body])


def render_docstring(doc: str, parent: Any = None) -> str:
    doc = re.sub(r"<(https?://[^>]+)>", r"[\1](\1)", doc)
    doc = LEGACY_REFERENCE_LINK.sub(current_reference_link, doc)
    if not GOOGLE_SECTION.search(doc):
        return doc

    # Griffe follows the Google convention of calling exception sections "Raises".
    # Keep supporting the project's historical "Throws" heading as an alias.
    doc = re.sub(r"^Throws:$", "Raises:", doc, flags=re.MULTILINE)
    sections = griffe.Docstring(doc, parent=parent).parse(griffe.Parser.google, warnings=False)
    rendered: list[str] = []

    for section in sections:
        if isinstance(section, griffe.DocstringSectionText):
            rendered.append(section.value)
        elif isinstance(section, griffe.DocstringSectionParameters):
            rows = [
                (f"`{parameter.name}`", f"`{parameter.annotation}`", parameter.description)
                for parameter in section.value
            ]
            rendered.extend(
                ["<h3>Arguments</h3>", markdown_table(("Argument", "Type", "Description"), rows)]
            )
        elif isinstance(section, griffe.DocstringSectionRaises):
            rows = [(f"`{error.annotation}`", error.description) for error in section.value]
            rendered.extend(["<h3>Errors</h3>", markdown_table(("Error", "When"), rows)])
        elif isinstance(section, griffe.DocstringSectionReturns):
            rows = [(f"`{item.annotation}`", item.description) for item in section.value]
            rendered.extend(["### Returns", markdown_table(("Type", "Description"), rows)])
        elif isinstance(section, griffe.DocstringSectionExamples):
            rendered.extend(["### Examples", *(value for _, value in section.value)])
        elif isinstance(section, griffe.DocstringSectionAdmonition):
            rendered.append(f"**{section.value.annotation.title()}:** {section.value.description}")

    return "\n\n".join(rendered)


def render_symbol(name: str, value: Any, parent: Any = None) -> str:
    kind = "Class" if inspect.isclass(value) else "Function" if callable(value) else "Value"
    definition = f"{name}{signature(value)}" if callable(value) else name
    doc = inspect.getdoc(value) or "No public documentation is available."
    doc = render_docstring(doc, parent)
    return "\n".join(
        [
            f"## {name}",
            "",
            f"**{kind}**",
            "",
            "```python",
            definition,
            "```",
            "",
            '<div className="api-docstring">',
            "",
            doc,
            "",
            "</div>",
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

    grouped: dict[str, list[tuple[str, Any, Any]]] = defaultdict(list)
    for name in exports:
        value = getattr(package, name)
        member = analysis.members.get(name)
        try:
            parent = member.target if isinstance(member, griffe.Alias) else member
        except griffe.AliasResolutionError:
            parent = None
        grouped[category_for(value)].append((name, value, parent))

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    index = [
        "# Python API reference",
        "",
        (
            "This is the complete guide to slackblocks' public Python API. "
            "Use it to check constructor signatures, accepted values, and class behavior "
            "while you build a message, modal, or home tab."
        ),
        "",
        (
            f"The {len(exports)} documented symbols are grouped by the part of Block Kit "
            "they help you create."
        ),
        "",
    ]
    for category in sorted(grouped):
        title = CATEGORIES[category]
        index.append(f"- [{title}](/reference/python/{category})")
        page = [f"# {title}", ""]
        page.extend(render_symbol(name, value, parent) for name, value, parent in grouped[category])
        (OUTPUT_ROOT / f"{category}.mdx").write_text("\n".join(page))
    (OUTPUT_ROOT / "index.mdx").write_text("\n".join(index) + "\n")

    if len(exports) < 86:
        raise RuntimeError(f"Expected at least 86 public symbols, generated {len(exports)}")
    print(f"Generated Python reference for {len(exports)} public symbols")


if __name__ == "__main__":
    main()
