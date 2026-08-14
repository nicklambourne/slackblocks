#!/usr/bin/env python3
"""Inventory and deterministically port the published MkDocs documentation."""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import re
import shutil
import subprocess
import tarfile
import tempfile
import warnings
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING, Any

import griffe

if TYPE_CHECKING:
    from collections.abc import Iterable, Sequence

TOOL_VERSION = "2"
SITE_BASE_URL = "/slackblocks"
DOCS_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = DOCS_ROOT.parent
LEGACY_ROOT = DOCS_ROOT / "legacy"
MANIFEST_PATH = LEGACY_ROOT / "manifest.json"
INVENTORY_ROOT = LEGACY_ROOT / "inventories"
FIXTURE_ARCHIVE = LEGACY_ROOT / "fixtures" / "v1.0.0.tar.gz"
VERSIONS_PATH = DOCS_ROOT / "versions.json"

REFERENCE_PAGES = (
    "attachments",
    "blocks",
    "elements",
    "messages",
    "modals",
    "objects",
    "rich_text",
    "views",
    "utils",
)

PAGE_TITLES = {
    "index": "Welcome",
    "contributing": "Contributing",
    "usage/installation": "Installation",
    "usage/using_blocks": "Using Blocks",
    "usage/sending_messages": "Sending Messages",
    "usage/cookbook": "Recipe Book",
    "usage/troubleshooting": "Troubleshooting & FAQ",
    "usage/compatibility": "Compatibility",
    "usage/migration": "Migrating from 1.x to 2.x",
    "reference/attachments": "Attachments",
    "reference/blocks": "Blocks",
    "reference/elements": "Elements",
    "reference/messages": "Messages",
    "reference/modals": "Modals",
    "reference/objects": "Objects",
    "reference/rich_text": "Rich Text",
    "reference/views": "Views",
    "reference/utils": "Utilities",
}

HEADING_RE = re.compile(
    r"<h(?P<level>[1-6])(?P<attrs>[^>]*)>(?P<body>.*?)</h(?P=level)>",
    re.IGNORECASE | re.DOTALL,
)
ATTRIBUTE_RE = re.compile(r'(?P<name>[\w:-]+)="(?P<value>[^"]*)"')
TAG_RE = re.compile(r"<[^>]+>")
ARTICLE_RE = re.compile(r"<article\b[^>]*>(?P<body>.*?)</article>", re.IGNORECASE | re.DOTALL)
TAB_HEADER_RE = re.compile(r'^===\s+"(?P<label>.*)"\s*$')
CODE_FENCE_RE = re.compile(r"^\s*```")
CODE_SPAN_RE = re.compile(r"(?<!`)(`+)(?!`)(.+?)(?<!`)\1(?!`)")
CODE_MASK_RE = re.compile(r"\x00(\d+)\x00")
RAW_IMG_SRC_RE = re.compile(r'(?P<attr><img\b[^>]*\bsrc=")(?P<path>/[^"]*)"')
ADMONITION_RE = re.compile(r'^(?P<marker>!!!|\?\?\?)\s+(?P<kind>[\w-]+)(?:\s+"(?P<title>.*)")?\s*$')
DIRECTIVE_RE = re.compile(r"^:::\s*(?P<target>[A-Za-z_][A-Za-z0-9_.]*)\s*$")


def run_git(*args: str, text: bool = True) -> str | bytes:
    result = subprocess.run(
        ["git", *args],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=text,
    )
    return result.stdout


def git_text(spec: str) -> str:
    return str(run_git("show", spec))


def git_object_exists(spec: str) -> bool:
    return (
        subprocess.run(
            ["git", "cat-file", "-e", spec],
            cwd=REPO_ROOT,
            capture_output=True,
            check=False,
        ).returncode
        == 0
    )


def clean_html(value: str) -> str:
    value = re.sub(r"<a\b[^>]*class=\"headerlink\"[^>]*>.*?</a>", "", value, flags=re.DOTALL)
    return " ".join(html.unescape(TAG_RE.sub("", value)).split())


def page_html(gh_pages_ref: str, prefix: str, route: str) -> str:
    suffix = "index.html" if route == "/" else f"{route.lstrip('/')}/index.html"
    return git_text(f"{gh_pages_ref}:{prefix}/{suffix}")


def inventory_page(rendered: str, route: str) -> dict[str, Any]:
    article_match = ARTICLE_RE.search(rendered)
    article = article_match.group("body") if article_match else rendered
    headings: list[dict[str, Any]] = []
    for match in HEADING_RE.finditer(article):
        attributes = {
            item.group("name"): item.group("value")
            for item in ATTRIBUTE_RE.finditer(match.group("attrs"))
        }
        headings.append(
            {
                "level": int(match.group("level")),
                "id": html.unescape(attributes.get("id", "")),
                "text": clean_html(match.group("body")),
            }
        )

    images = [
        html.unescape(match.group("value"))
        for match in re.finditer(r'<img\b[^>]*\bsrc="(?P<value>[^"]+)"', article, re.IGNORECASE)
    ]
    links = []
    for match in re.finditer(r'<a\b[^>]*\bhref="(?P<value>[^"]+)"', article, re.IGNORECASE):
        target = html.unescape(match.group("value"))
        if target.startswith(("http://", "https://", "mailto:", "#")):
            continue
        links.append(target.removeprefix("/slackblocks"))

    title_match = re.search(r"<title>(.*?)</title>", rendered, re.IGNORECASE | re.DOTALL)
    return {
        "route": route,
        "published_title": clean_html(title_match.group(1)) if title_match else "",
        "headings": headings,
        "api_anchors": [
            heading["id"]
            for heading in headings
            if route.startswith("/reference/") and heading["id"]
        ],
        "code_block_count": len(re.findall(r'class="highlight(?:\s|\")', article)),
        "images": images,
        "internal_links": sorted(set(links)),
    }


def published_routes(gh_pages_ref: str, prefix: str) -> list[str]:
    files = str(run_git("ls-tree", "-r", "--name-only", gh_pages_ref, prefix)).splitlines()
    routes = []
    for filename in files:
        if not filename.endswith("/index.html"):
            continue
        relative = filename[len(prefix) : -len("/index.html")]
        routes.append(relative or "/")
    return sorted(routes, key=lambda route: (route != "/", route))


def refresh_manifest(gh_pages_ref: str) -> dict[str, Any]:
    gh_pages_commit = str(run_git("rev-parse", gh_pages_ref)).strip()
    published = json.loads(git_text(f"{gh_pages_ref}:versions.json"))
    previous: dict[str, Any] = {}
    if MANIFEST_PATH.exists():
        previous = json.loads(MANIFEST_PATH.read_text())
    previous_entries = {entry["version"]: entry for entry in previous.get("versions", [])}

    versions: list[dict[str, Any]] = []
    inventories: dict[str, dict[str, Any]] = {}
    for published_version in published:
        tag = published_version["version"]
        if not tag.startswith("v"):
            continue
        version = tag.removeprefix("v")
        tag_commit = str(run_git("rev-parse", f"{tag}^{{commit}}")).strip()
        previous_entry = previous_entries.get(version, {})
        docs_source_ref = previous_entry.get("documentation_source_ref", tag)
        docs_tree = str(run_git("rev-parse", f"{docs_source_ref}:docs_src")).strip()
        python_tree = str(run_git("rev-parse", f"{tag}:slackblocks")).strip()
        routes = published_routes(gh_pages_ref, tag)
        alias_prefixes = [version] if git_object_exists(f"{gh_pages_ref}:{version}") else []
        entry = {
            "version": version,
            "tag": tag,
            "tag_commit": tag_commit,
            "canonical_prefix": tag,
            "old_alias_prefixes": alias_prefixes,
            "expected_routes": routes,
            "documentation_source_tree_hash": docs_tree,
            "python_source_tree_hash": python_tree,
            "generated_snapshot_tree_hash": previous_entries.get(version, {}).get(
                "generated_snapshot_tree_hash"
            ),
            "language_availability": ["python"],
            "inventory": f"inventories/{tag}.json",
        }
        if docs_source_ref != tag:
            entry["documentation_source_ref"] = docs_source_ref
        versions.append(entry)
        inventories[tag] = {
            "version": version,
            "canonical_prefix": tag,
            "gh_pages_commit": gh_pages_commit,
            "pages": [
                inventory_page(page_html(gh_pages_ref, tag, route), route) for route in routes
            ],
        }

    generated_at = previous.get("migration", {}).get("generated_at")
    if not generated_at:
        generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat()
    manifest = {
        "schema_version": 1,
        "migration": {
            "tool_version": TOOL_VERSION,
            "generated_at": generated_at,
            "gh_pages_ref": gh_pages_ref,
            "gh_pages_commit": gh_pages_commit,
            "aliases_enabled": previous.get("migration", {}).get("aliases_enabled", False),
        },
        "versions": versions,
    }

    LEGACY_ROOT.mkdir(parents=True, exist_ok=True)
    INVENTORY_ROOT.mkdir(parents=True, exist_ok=True)
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n")
    for tag, inventory in inventories.items():
        (INVENTORY_ROOT / f"{tag}.json").write_text(json.dumps(inventory, indent=2) + "\n")
    return manifest


def load_manifest() -> dict[str, Any]:
    return json.loads(MANIFEST_PATH.read_text())


def version_entry(manifest: dict[str, Any], value: str) -> dict[str, Any]:
    normalized = value.removeprefix("v")
    for entry in manifest["versions"]:
        if entry["version"] == normalized:
            return entry
    raise KeyError(f"Unknown published documentation version: {value}")


def selected_entries(
    manifest: dict[str, Any], versions: Sequence[str] | None
) -> list[dict[str, Any]]:
    if not versions:
        return list(manifest["versions"])
    return [version_entry(manifest, version) for version in versions]


def extract_release(entry: dict[str, Any], destination: Path, archive: Path | None = None) -> None:
    if archive is None:
        actual_commit = str(run_git("rev-parse", f"{entry['tag']}^{{commit}}")).strip()
        if actual_commit != entry["tag_commit"]:
            raise RuntimeError(
                f"{entry['tag']} resolves to {actual_commit}, expected {entry['tag_commit']}"
            )
        docs_source_ref = entry.get("documentation_source_ref", entry["tag"])
        docs_tree = str(run_git("rev-parse", f"{docs_source_ref}:docs_src")).strip()
        if docs_tree != entry["documentation_source_tree_hash"]:
            raise RuntimeError(
                f"{entry['tag']} docs source is {docs_tree}, expected "
                f"{entry['documentation_source_tree_hash']}"
            )
        python_tree = str(run_git("rev-parse", f"{entry['tag']}:slackblocks")).strip()
        if python_tree != entry["python_source_tree_hash"]:
            raise RuntimeError(
                f"{entry['tag']} Python source is {python_tree}, expected "
                f"{entry['python_source_tree_hash']}"
            )
        archives = [
            (destination / "docs-source.tar", docs_source_ref, "docs_src"),
            (destination / "python-source.tar", entry["tag"], "slackblocks"),
        ]
        for generated_archive, source_ref, source_path in archives:
            subprocess.run(
                [
                    "git",
                    "archive",
                    "--format=tar",
                    f"--output={generated_archive}",
                    source_ref,
                    source_path,
                ],
                cwd=REPO_ROOT,
                check=True,
            )
            with tarfile.open(generated_archive) as tar:
                tar.extractall(destination, filter="data")
    else:
        with tarfile.open(archive) as tar:
            tar.extractall(destination, filter="data")

    docs_root = destination / "docs_src"
    python_root = destination / "slackblocks"
    if not docs_root.is_dir() or not python_root.is_dir():
        raise RuntimeError(f"{entry['tag']} does not contain docs_src/ and slackblocks/")


def markdown_cell(value: Any) -> str:
    lines = (str(value) if value is not None else "").strip().splitlines()
    return "<br />".join(line.strip() for line in lines).replace("|", r"\|") or "—"


def markdown_table(headers: tuple[str, ...], rows: list[tuple[Any, ...]]) -> str:
    header = "| " + " | ".join(headers) + " |"
    separator = "| " + " | ".join("---" for _ in headers) + " |"
    body = ["| " + " | ".join(markdown_cell(cell) for cell in row) + " |" for row in rows]
    return "\n".join([header, separator, *body])


def version_doc_link(version: str, match: re.Match[str]) -> str:
    prefix = match.group("prefix") or ""
    category = match.group("category")
    fragment = match.group("fragment") or ""
    return f"{prefix}/v{version}/reference/{category}{fragment}"


def rewrite_absolute_links(value: str, version: str) -> str:
    value = value.replace("/slackblocks/latest/", f"/v{version}/")
    value = value.replace("/slackblocks/master/", f"/v{version}/")
    value = re.sub(
        r"https://nicklambourne\.github\.io/slackblocks/reference/"
        r"(?P<category>attachments|blocks|elements|messages|modals|objects|rich_text|views|utils)"
        r"/?(?P<anchor>#[A-Za-z0-9_.-]+)?",
        lambda match: (
            f"/v{version}/reference/{match.group('category')}{match.group('anchor') or ''}"
        ),
        value,
    )
    return re.sub(
        r"(?<![A-Za-z0-9._/-])/reference/"
        r"(?P<category>attachments|blocks|elements|messages|modals|objects|rich_text|views|utils)"
        r"/?(?P<anchor>#[A-Za-z0-9_.-]+)?",
        lambda match: version_doc_link(
            version,
            _LinkMatch(None, match.group("category"), match.group("anchor")),
        ),
        value,
    )


class _LinkMatch:
    def __init__(self, prefix: str | None, category: str, fragment: str | None) -> None:
        self.values = {"prefix": prefix, "category": category, "fragment": fragment}

    def group(self, name: str) -> str | None:
        return self.values[name]


def render_docstring(docstring: griffe.Docstring | None, version: str) -> str:
    if not docstring or not docstring.value.strip():
        return "No public documentation was provided in this release."
    raw = rewrite_absolute_links(docstring.value.strip(), version)
    raw = re.sub(r"<(https?://[^>]+)>", r"[\1](\1)", raw)
    raw = re.sub(
        r'<table style="width:(?P<width>\d+)%">',
        lambda match: f'<table style={{{{ width: "{match.group("width")}%" }}}}>',
        raw,
    )

    html_tags = {
        "a",
        "br",
        "code",
        "div",
        "em",
        "img",
        "li",
        "ol",
        "p",
        "strong",
        "table",
        "tbody",
        "td",
        "th",
        "thead",
        "tr",
        "ul",
    }

    def escape_angle_token(match: re.Match[str]) -> str:
        value = match.group("value")
        tag_name = value.removeprefix("/").lower()
        return match.group(0) if tag_name in html_tags else f"&lt;{value}&gt;"

    raw = re.sub(
        r"<(?P<value>/?[A-Za-z][A-Za-z0-9_.\[\], |:-]*)>",
        escape_angle_token,
        raw,
    )
    raw = re.sub(r"^Throws:$", "Raises:", raw, flags=re.MULTILINE)
    if not re.search(
        r"^(?:Args|Arguments|Parameters|Raises|Returns|Yields|Examples):$",
        raw,
        re.MULTILINE,
    ):
        return raw

    sections = griffe.Docstring(raw, parent=docstring.parent).parse(
        griffe.Parser.google, warnings=False
    )
    rendered: list[str] = []
    for section in sections:
        if isinstance(section, griffe.DocstringSectionText):
            rendered.append(section.value)
        elif isinstance(section, griffe.DocstringSectionParameters):
            rows = [
                (
                    f"`{parameter.name}`",
                    f"`{parameter.annotation}`",
                    parameter.description,
                )
                for parameter in section.value
            ]
            rendered.extend(
                [
                    "<h4>Arguments</h4>",
                    markdown_table(("Argument", "Type", "Description"), rows),
                ]
            )
        elif isinstance(section, griffe.DocstringSectionRaises):
            rows = [(f"`{error.annotation}`", error.description) for error in section.value]
            rendered.extend(["<h4>Errors</h4>", markdown_table(("Error", "When"), rows)])
        elif isinstance(section, griffe.DocstringSectionReturns):
            rows = [(f"`{item.annotation}`", item.description) for item in section.value]
            rendered.extend(["#### Returns", markdown_table(("Type", "Description"), rows)])
        elif isinstance(section, griffe.DocstringSectionExamples):
            rendered.extend(["#### Examples", *(value for _, value in section.value)])
    return "\n\n".join(item for item in rendered if item)


def member_signature(name: str, member: Any) -> str:
    if isinstance(member, (griffe.Class, griffe.Function)):
        try:
            return str(member.signature())
        except (AttributeError, TypeError, ValueError):
            return name
    annotation = getattr(member, "annotation", None)
    value = getattr(member, "value", None)
    if annotation and value is not None:
        return f"{name}: {annotation} = {value}"
    if annotation:
        return f"{name}: {annotation}"
    return f"{name} = {value}" if value is not None else name


def member_kind(member: Any) -> str:
    if isinstance(member, griffe.Class):
        return "Class"
    if isinstance(member, griffe.Function):
        return "Function"
    return "Value"


def render_member(
    package: griffe.Module,
    anchor: str,
    nested_anchors: Sequence[str],
    version: str,
    heading_by_anchor: dict[str, dict[str, Any]],
    *,
    include_heading: bool,
) -> str:
    try:
        member = package.get_member(anchor)
    except (KeyError, griffe.AliasResolutionError) as error:
        raise RuntimeError(f"Could not resolve historical API anchor {anchor}") from error
    if isinstance(member, griffe.Alias):
        member = member.target
    name = anchor.rsplit(".", 1)[-1]
    content: list[str] = []
    if include_heading:
        heading = heading_by_anchor[anchor]
        content.extend(
            [
                f"{'#' * heading['level']} {heading['text']} {{#{anchor}}}",
                "",
            ]
        )
    else:
        content.append(f'<a id="{anchor}"></a>')
    content.extend(
        [
            f"**{member_kind(member)}**",
            "",
            "```python",
            member_signature(name, member),
            "```",
            "",
            '<div className="api-docstring">',
            "",
            render_docstring(getattr(member, "docstring", None), version),
            "",
            "</div>",
        ]
    )
    for nested_anchor in nested_anchors:
        try:
            nested = package.get_member(nested_anchor)
        except (KeyError, griffe.AliasResolutionError) as error:
            raise RuntimeError(
                f"Could not resolve historical API member {nested_anchor}"
            ) from error
        if isinstance(nested, griffe.Alias):
            nested = nested.target
        nested_name = nested_anchor.rsplit(".", 1)[-1]
        heading = heading_by_anchor[nested_anchor]
        content.extend(
            [
                "",
                f"{'#' * heading['level']} {heading['text']} {{#{nested_anchor}}}",
                "",
                "```python",
                member_signature(nested_name, nested),
                "```",
                "",
                render_docstring(getattr(nested, "docstring", None), version),
            ]
        )
    return "\n".join(content)


def directive_content(
    package: griffe.Module,
    target: str,
    anchors: Sequence[str],
    heading_by_anchor: dict[str, dict[str, Any]],
    version: str,
) -> str:
    try:
        target_member = package.get_member(target)
    except (KeyError, griffe.AliasResolutionError) as error:
        raise RuntimeError(f"Could not resolve mkdocstrings target {target}") from error
    if isinstance(target_member, griffe.Alias):
        target_member = target_member.target
    if not isinstance(target_member, griffe.Module):
        nested = [anchor for anchor in anchors if anchor.startswith(f"{target}.")]
        return render_member(
            package,
            target,
            nested,
            version,
            heading_by_anchor,
            include_heading=False,
        )

    prefix = f"{target}."
    top_level: list[str] = []
    for anchor in anchors:
        if not anchor.startswith(prefix):
            continue
        remainder = anchor[len(prefix) :]
        if "." not in remainder:
            top_level.append(anchor)
    rendered = []
    for anchor in top_level:
        nested = [candidate for candidate in anchors if candidate.startswith(f"{anchor}.")]
        rendered.append(
            render_member(
                package,
                anchor,
                nested,
                version,
                heading_by_anchor,
                include_heading=True,
            )
        )
    return "\n\n".join(rendered)


def replace_directives(
    source: str,
    package: griffe.Module,
    page_inventory: dict[str, Any],
    version: str,
) -> str:
    anchors = list(page_inventory.get("api_anchors", []))
    anchors.extend(
        heading["id"]
        for heading in page_inventory.get("headings", [])
        if "." in heading.get("id", "") and heading["id"] not in anchors
    )
    heading_by_anchor = {
        heading["id"]: heading for heading in page_inventory.get("headings", []) if heading["id"]
    }
    lines = source.splitlines()
    output: list[str] = []
    index = 0
    while index < len(lines):
        match = DIRECTIVE_RE.match(lines[index])
        if not match:
            output.append(lines[index])
            index += 1
            continue
        target = match.group("target")
        output.append(directive_content(package, target, anchors, heading_by_anchor, version))
        index += 1
        while index < len(lines) and (not lines[index].strip() or lines[index].startswith("    ")):
            index += 1
    return "\n".join(output)


def convert_tabs(source: str) -> tuple[str, bool]:
    lines = source.splitlines()
    output: list[str] = []
    index = 0
    found = False
    while index < len(lines):
        header = TAB_HEADER_RE.match(lines[index])
        if not header:
            output.append(lines[index])
            index += 1
            continue
        found = True
        tabs: list[tuple[str, list[str]]] = []
        while index < len(lines):
            header = TAB_HEADER_RE.match(lines[index])
            if not header:
                break
            label = header.group("label").replace("`", "")
            index += 1
            body: list[str] = []
            while index < len(lines):
                if TAB_HEADER_RE.match(lines[index]):
                    break
                if not lines[index].strip():
                    body.append("")
                    index += 1
                    continue
                if lines[index].startswith("    "):
                    body.append(lines[index][4:])
                    index += 1
                    continue
                break
            while body and not body[-1].strip():
                body.pop()
            tabs.append((label, body))
            while index < len(lines) and not lines[index].strip():
                index += 1
            if index >= len(lines) or not TAB_HEADER_RE.match(lines[index]):
                break
        output.append("<Tabs>")
        for tab_index, (label, body) in enumerate(tabs):
            value = re.sub(r"[^a-z0-9]+", "-", label.lower()).strip("-") or f"tab-{tab_index + 1}"
            output.extend(
                [
                    f"<TabItem value={json.dumps(value)} label={json.dumps(label)}>",
                    "",
                    *body,
                    "",
                    "</TabItem>",
                ]
            )
        output.append("</Tabs>")
    return "\n".join(output), found


def convert_admonitions(source: str) -> str:
    lines = source.splitlines()
    output: list[str] = []
    index = 0
    while index < len(lines):
        match = ADMONITION_RE.match(lines[index])
        if not match:
            output.append(lines[index])
            index += 1
            continue
        kind = "details" if match.group("marker") == "???" else match.group("kind")
        title = match.group("title")
        opening = f":::{kind}"
        if title:
            opening += f"[{title}]"
        output.append(opening)
        index += 1
        while index < len(lines) and (not lines[index].strip() or lines[index].startswith("    ")):
            output.append(lines[index][4:] if lines[index].startswith("    ") else "")
            index += 1
        output.append(":::")
    return "\n".join(output)


def mask_code_regions(source: str) -> tuple[str, list[str]]:
    """Replace fenced code blocks and inline code spans with opaque placeholders."""
    masked: list[str] = []

    def store(value: str) -> str:
        masked.append(value)
        return f"\x00{len(masked) - 1}\x00"

    output: list[str] = []
    fence: list[str] | None = None
    for line in source.splitlines():
        if fence is not None:
            fence.append(line)
            if CODE_FENCE_RE.match(line):
                output.append(store("\n".join(fence)))
                fence = None
            continue
        if CODE_FENCE_RE.match(line):
            fence = [line]
            continue
        output.append(CODE_SPAN_RE.sub(lambda match: store(match.group(0)), line))
    if fence is not None:
        output.append(store("\n".join(fence)))
    return "\n".join(output), masked


def unmask_code_regions(source: str, masked: Sequence[str]) -> str:
    return CODE_MASK_RE.sub(lambda match: masked[int(match.group(1))], source)


def prefix_site_root_images(value: str) -> str:
    """Give raw <img> tags the site baseUrl; Docusaurus only rewrites markdown images."""

    def replace(match: re.Match[str]) -> str:
        path = match.group("path")
        if path.startswith(f"{SITE_BASE_URL}/"):
            return match.group(0)
        return f'{match.group("attr")}{SITE_BASE_URL}{path}"'

    return RAW_IMG_SRC_RE.sub(replace, value)


def rewrite_markdown(source: str, version: str, docs_hash: str) -> str:
    source, masked = mask_code_regions(source)
    source = re.sub(r"<(https?://[^>]+)>", r"[\1](\1)", source)
    source = re.sub(r"<(mailto:[^>]+)>", r"[\1](\1)", source)
    source = re.sub(r"\[([^\]]+)\]\(\)", r"\1", source)
    source = re.sub(
        r"(?P<path>[A-Za-z0-9_./-]+)\.md(?P<fragment>#[A-Za-z0-9_.-]+)?",
        r"\g<path>\g<fragment>",
        source,
    )
    source = source.replace("./docs_src/img/", f"/img/legacy/{docs_hash}/")
    source = source.replace("../img/", f"/img/legacy/{docs_hash}/")
    source = source.replace("./img/", f"/img/legacy/{docs_hash}/")
    source = rewrite_absolute_links(source, version)
    source = source.replace("#rich_text.RichText", "#rich_text.elements.RichText")

    def rewrite_internal_link(match: re.Match[str]) -> str:
        label = match.group("label")
        destination = match.group("destination")
        if destination.startswith(("http://", "https://", "mailto:", "#", "/v", "/img/")):
            return match.group(0)
        normalized = destination.removeprefix("/slackblocks/").lstrip("/")
        while normalized.startswith(("./", "../")):
            normalized = normalized.partition("/")[2]
        if normalized.startswith("elements/"):
            normalized = f"reference/{normalized}"
        path, separator, fragment = normalized.partition("#")
        if path.rstrip("/") in REFERENCE_PAGES:
            normalized = f"reference/{path.rstrip('/')}"
            if separator:
                normalized += f"#{fragment}"
        if normalized.startswith("usage/basic_usage"):
            normalized = normalized.replace("usage/basic_usage", "usage/using_blocks", 1)
        if normalized.startswith(("reference/", "usage/", "contributing")):
            destination = f"/v{version}/{normalized}"
        return f"[{label}]({destination})"

    source = re.sub(
        r"(?<!!)\[(?P<label>[^\]]+)\]\((?P<destination>[^)]+)\)",
        rewrite_internal_link,
        source,
    )
    source = prefix_site_root_images(source)
    source = re.sub(r"^\s*\{[.:#][^}]+\}\s*$", "", source, flags=re.MULTILINE)
    return unmask_code_regions(source, masked)


def heading_text(value: str) -> str:
    value = re.sub(r"\s+\{#[^}]+\}\s*$", "", value)
    value = re.sub(r"!?(?:\[([^\]]+)\])\([^)]+\)", r"\1", value)
    value = re.sub(r"[`*_~]", "", value)
    return " ".join(html.unescape(TAG_RE.sub("", value)).split())


def preserve_heading_ids(source: str, page_inventory: dict[str, Any]) -> str:
    headings = [heading for heading in page_inventory.get("headings", []) if heading.get("id")]
    used: set[int] = set()
    output: list[str] = []
    in_fence = False
    for line in source.splitlines():
        if re.match(r"^\s*```", line):
            in_fence = not in_fence
            output.append(line)
            continue
        match = None if in_fence else re.match(r"^(#{1,6})\s+(.+?)\s*$", line)
        if not match:
            output.append(line)
            continue
        level = len(match.group(1))
        text = heading_text(match.group(2))
        selected = next(
            (
                index
                for index, heading in enumerate(headings)
                if index not in used and heading["level"] == level and heading["text"] == text
            ),
            None,
        )
        if selected is None:
            output.append(line)
            continue
        used.add(selected)
        heading_id = headings[selected]["id"]
        without_id = re.sub(r"\s+\{#[^}]+\}\s*$", "", match.group(2))
        if level == 1:
            output.extend([f'<a id="{heading_id}"></a>', f"# {without_id}"])
        else:
            output.append(f"{match.group(1)} {without_id} {{#{heading_id}}}")
    return "\n".join(output)


def generated_notice(entry: dict[str, Any]) -> str:
    return (
        f"<!-- Generated by port_legacy_docs.py v{TOOL_VERSION} from {entry['tag']} "
        f"({entry['tag_commit']}). Historical snapshot—do not edit directly. -->"
    )


def convert_page(
    source: str,
    entry: dict[str, Any],
    package: griffe.Module,
    page_inventory: dict[str, Any],
) -> str:
    has_level_one_heading = bool(re.search(r"^#\s+", source, re.MULTILINE))
    source = replace_directives(
        source,
        package,
        page_inventory,
        entry["version"],
    )
    source, has_tabs = convert_tabs(source)
    source = convert_admonitions(source)
    source = rewrite_markdown(
        source,
        entry["version"],
        entry["documentation_source_tree_hash"],
    )
    source = preserve_heading_ids(source, page_inventory)
    header: list[str] = []
    if not has_level_one_heading:
        page_title = next(
            (
                heading["text"]
                for heading in page_inventory.get("headings", [])
                if heading["level"] == 1
            ),
            page_inventory["published_title"].split(" - ", 1)[0],
        )
        header.extend(["---", f"title: {json.dumps(page_title)}", "---"])
    header.append(generated_notice(entry))
    if has_tabs:
        header.extend(
            [
                "import Tabs from '@theme/Tabs';",
                "import TabItem from '@theme/TabItem';",
            ]
        )
    return "\n".join([*header, "", source.strip(), ""])


def source_file_for_route(route: str) -> str:
    return "index.md" if route == "/" else f"{route.lstrip('/')}.md"


def output_file_for_route(output_root: Path, version: str, route: str) -> Path:
    relative = "index.mdx" if route == "/" else f"{route.lstrip('/')}.mdx"
    return output_root / "versioned_docs" / f"version-{version}" / relative


def sidebar_for(entry: dict[str, Any]) -> dict[str, Any]:
    route_set = set(entry["expected_routes"])
    usage_order = [
        "/usage/installation",
        "/usage/compatibility",
        "/usage/using_blocks",
        "/usage/sending_messages",
        "/usage/cookbook",
        "/usage/migration",
        "/usage/troubleshooting",
    ]
    reference_order = [f"/reference/{page}" for page in REFERENCE_PAGES]
    items: list[dict[str, Any]] = [{"type": "doc", "id": "index", "label": "Welcome"}]
    items.append(
        {
            "type": "category",
            "label": "Usage",
            "collapsed": False,
            "items": [
                {
                    "type": "doc",
                    "id": route.lstrip("/"),
                    "label": PAGE_TITLES[route.lstrip("/")],
                }
                for route in usage_order
                if route in route_set
            ],
        }
    )
    items.append(
        {
            "type": "category",
            "label": "Reference",
            "collapsed": False,
            "items": [
                {
                    "type": "doc",
                    "id": route.lstrip("/"),
                    "label": PAGE_TITLES[route.lstrip("/")],
                }
                for route in reference_order
                if route in route_set
            ],
        }
    )
    if "/contributing" in route_set:
        items.append({"type": "doc", "id": "contributing", "label": "Contributing"})
    return {"docs": items}


def generate_version(
    entry: dict[str, Any],
    output_root: Path,
    *,
    source_archive: Path | None = None,
) -> None:
    inventory = json.loads((LEGACY_ROOT / entry["inventory"]).read_text())
    pages_by_route = {page["route"]: page for page in inventory["pages"]}
    version_docs = output_root / "versioned_docs" / f"version-{entry['version']}"
    if version_docs.exists():
        shutil.rmtree(version_docs)
    version_docs.mkdir(parents=True)

    with tempfile.TemporaryDirectory(prefix=f"slackblocks-{entry['tag']}-") as temporary:
        extracted = Path(temporary)
        extract_release(entry, extracted, source_archive)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore", SyntaxWarning)
            package = griffe.load(extracted / "slackblocks", submodules=True)
        docs_source = extracted / "docs_src"
        for route in entry["expected_routes"]:
            source_path = docs_source / source_file_for_route(route)
            if not source_path.is_file():
                raise RuntimeError(
                    f"{entry['tag']} is missing {source_path.relative_to(extracted)}"
                )
            output_path = output_file_for_route(output_root, entry["version"], route)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(
                convert_page(source_path.read_text(), entry, package, pages_by_route[route])
            )

        source_assets = docs_source / "img"
        if source_assets.is_dir():
            asset_root = (
                output_root / "static" / "img" / "legacy" / entry["documentation_source_tree_hash"]
            )
            if asset_root.exists():
                shutil.rmtree(asset_root)
            shutil.copytree(source_assets, asset_root)

    sidebar_path = output_root / "versioned_sidebars" / f"version-{entry['version']}-sidebars.json"
    sidebar_path.parent.mkdir(parents=True, exist_ok=True)
    sidebar_path.write_text(json.dumps(sidebar_for(entry), indent=2) + "\n")


def hash_files(paths: Iterable[Path], relative_to: Path) -> str:
    digest = hashlib.sha256()
    files: list[Path] = []
    for path in paths:
        if path.is_dir():
            files.extend(item for item in path.rglob("*") if item.is_file())
        elif path.is_file():
            files.append(path)
    for path in sorted(set(files), key=lambda item: item.relative_to(relative_to).as_posix()):
        relative = path.relative_to(relative_to).as_posix().encode()
        digest.update(len(relative).to_bytes(4, "big"))
        digest.update(relative)
        content = path.read_bytes()
        digest.update(len(content).to_bytes(8, "big"))
        digest.update(content)
    return digest.hexdigest()


def snapshot_hash(output_root: Path, entry: dict[str, Any]) -> str:
    return hash_files(
        [
            output_root / "versioned_docs" / f"version-{entry['version']}",
            output_root / "versioned_sidebars" / f"version-{entry['version']}-sidebars.json",
            output_root / "static" / "img" / "legacy" / entry["documentation_source_tree_hash"],
        ],
        output_root,
    )


def write_registered_versions(manifest: dict[str, Any]) -> None:
    registered = [
        entry["version"]
        for entry in manifest["versions"]
        if entry.get("generated_snapshot_tree_hash")
    ]
    existing: list[str] = []
    if VERSIONS_PATH.exists():
        existing = json.loads(VERSIONS_PATH.read_text())
    # Newer, non-legacy versions may precede the frozen legacy suffix.
    preserved = [version for version in existing if version not in set(registered)]
    VERSIONS_PATH.write_text(json.dumps(preserved + registered, indent=2) + "\n")


def update_snapshots(entries: Sequence[dict[str, Any]], manifest: dict[str, Any]) -> None:
    for entry in entries:
        generate_version(entry, DOCS_ROOT)
        entry["generated_snapshot_tree_hash"] = snapshot_hash(DOCS_ROOT, entry)
    manifest["migration"]["tool_version"] = TOOL_VERSION
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2) + "\n")
    write_registered_versions(manifest)


def compare_trees(expected: Path, actual: Path) -> None:
    expected_files = {
        path.relative_to(expected): path.read_bytes()
        for path in expected.rglob("*")
        if path.is_file()
    }
    actual_files = {
        path.relative_to(actual): path.read_bytes() for path in actual.rglob("*") if path.is_file()
    }
    if expected_files != actual_files:
        missing = sorted(str(path) for path in expected_files.keys() - actual_files.keys())
        extra = sorted(str(path) for path in actual_files.keys() - expected_files.keys())
        changed = sorted(
            str(path)
            for path in expected_files.keys() & actual_files.keys()
            if expected_files[path] != actual_files[path]
        )
        raise RuntimeError(
            f"Generated legacy tree differs; missing={missing}, extra={extra}, changed={changed}"
        )


def check_snapshots(entries: Sequence[dict[str, Any]]) -> None:
    for entry in entries:
        expected_hash = entry.get("generated_snapshot_tree_hash")
        if not expected_hash:
            raise RuntimeError(f"{entry['tag']} has not been generated and registered")
        actual_hash = snapshot_hash(DOCS_ROOT, entry)
        if actual_hash != expected_hash:
            raise RuntimeError(
                f"{entry['tag']} snapshot hash is {actual_hash}, expected {expected_hash}"
            )
        with tempfile.TemporaryDirectory(prefix=f"check-{entry['tag']}-") as temporary:
            generated = Path(temporary)
            generate_version(entry, generated)
            compare_trees(
                generated / "versioned_docs" / f"version-{entry['version']}",
                DOCS_ROOT / "versioned_docs" / f"version-{entry['version']}",
            )
            generated_sidebar = (
                generated / "versioned_sidebars" / f"version-{entry['version']}-sidebars.json"
            )
            actual_sidebar = (
                DOCS_ROOT / "versioned_sidebars" / f"version-{entry['version']}-sidebars.json"
            )
            if generated_sidebar.read_bytes() != actual_sidebar.read_bytes():
                raise RuntimeError(f"{entry['tag']} sidebar is not reproducible")
            generated_assets = (
                generated / "static" / "img" / "legacy" / entry["documentation_source_tree_hash"]
            )
            actual_assets = (
                DOCS_ROOT / "static" / "img" / "legacy" / entry["documentation_source_tree_hash"]
            )
            if generated_assets.exists():
                compare_trees(generated_assets, actual_assets)


def check_fixture(manifest: dict[str, Any]) -> None:
    entry = version_entry(manifest, "1.0.0")
    if not FIXTURE_ARCHIVE.is_file():
        raise RuntimeError(f"Missing fixture archive: {FIXTURE_ARCHIVE}")
    with (
        tempfile.TemporaryDirectory(prefix="legacy-fixture-a-") as first_temp,
        tempfile.TemporaryDirectory(prefix="legacy-fixture-b-") as second_temp,
    ):
        first = Path(first_temp)
        second = Path(second_temp)
        generate_version(entry, first, source_archive=FIXTURE_ARCHIVE)
        generate_version(entry, second, source_archive=FIXTURE_ARCHIVE)
        compare_trees(first, second)
        print(f"v1.0.0 deterministic fixture hash: {snapshot_hash(first, entry)}")


def validate_manifest(manifest: dict[str, Any]) -> None:
    versions = [entry["version"] for entry in manifest["versions"]]
    if not versions:
        raise RuntimeError("The manifest does not record any published versions")
    if "1.0.4" in versions or "1.2.1" in versions:
        raise RuntimeError("Unpublished v1.0.4 or v1.2.1 leaked into the manifest")
    for entry in manifest["versions"]:
        if entry["language_availability"] != ["python"]:
            raise RuntimeError(f"{entry['tag']} must remain Python-only")
        inventory = json.loads((LEGACY_ROOT / entry["inventory"]).read_text())
        if inventory["gh_pages_commit"] != manifest["migration"]["gh_pages_commit"]:
            raise RuntimeError(f"{entry['tag']} inventory uses a different gh-pages commit")
        inventory_routes = [page["route"] for page in inventory["pages"]]
        if inventory_routes != entry["expected_routes"]:
            raise RuntimeError(f"{entry['tag']} route inventory does not match the manifest")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    inventory = subparsers.add_parser("inventory", help="Refresh manifest and frozen inventories")
    inventory.add_argument("--gh-pages-ref", default="origin/gh-pages")
    subparsers.add_parser("fixture", help="Verify deterministic v1.0.0 fixture generation")
    generate = subparsers.add_parser("generate", help="Write immutable historical snapshots")
    generate.add_argument("--versions", nargs="*")
    check = subparsers.add_parser("check", help="Verify manifests and checked-in snapshots")
    check.add_argument("--versions", nargs="*")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.command == "inventory":
        manifest = refresh_manifest(args.gh_pages_ref)
        validate_manifest(manifest)
        print(
            f"Recorded {len(manifest['versions'])} versions and "
            f"{sum(len(entry['expected_routes']) for entry in manifest['versions'])} routes"
        )
        return

    manifest = load_manifest()
    validate_manifest(manifest)
    if args.command == "fixture":
        check_fixture(manifest)
        return
    entries = selected_entries(manifest, args.versions)
    if args.command == "generate":
        update_snapshots(entries, manifest)
        print(f"Generated {len(entries)} immutable historical snapshots")
        return
    check_snapshots(entries)
    print(f"Verified {len(entries)} immutable historical snapshots")


if __name__ == "__main__":
    main()
