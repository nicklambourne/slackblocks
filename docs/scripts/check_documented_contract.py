#!/usr/bin/env python3
"""Check copyable configuration and contract facts against their authoritative inputs.

Uses Python 3.11+ (the documentation CI runtime); it does not change package floors.
"""

import json
import re
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "docs/docs"


def read(path):
    return (DOCS / path).read_text()


def main():
    compatibility = read("usage/compatibility.mdx").split("</Python>")[0]
    configs = [
        tomllib.loads(code)
        for code in re.findall(r"```toml\n(.*?)```", compatibility, re.S)
    ]
    assert len(configs) == 2
    assert configs[0]["tool"]["poetry"]["dependencies"]["slackblocks"] == "^3"
    assert configs[1]["project"]["dependencies"] == ["slackblocks>=3,<4"]
    assert re.findall(r"^slackblocks[^\n]*", compatibility, re.M) == [
        "slackblocks>=3,<4",
        'slackblocks = "^3"',
    ]

    for page in ["reference/index.mdx", "usage/migration.mdx"]:
        assert not re.search(
            r"(?:Elixir|HexDocs).*?(?:coming soon|not yet published|not published)",
            read(page),
            re.I,
        )

    engines = json.loads((ROOT / "typescript/package.json").read_text())["engines"][
        "node"
    ]
    for page in [
        "quick-start.mdx",
        "usage/installation.mdx",
        "usage/compatibility.mdx",
    ]:
        assert engines in read(page), (
            f"{page} must state the package's exact Node range"
        )

    model = {
        t["name"]: t
        for t in json.loads((ROOT / "spec/model.json").read_text())["types"]
    }
    guide = read("usage/using_blocks.mdx")
    for title, name in [
        ("Task Card Block", "TaskCardBlock"),
        ("Plan Block", "PlanBlock"),
    ]:
        section = guide.split("## " + title, 1)[1].split("\n## ", 1)[0]
        summary = re.search(r"Required: (.*?)\.", section)[1]
        documented = set(re.findall(r"`(\w+)`", summary))
        required = {f["wire"] for f in model[name]["fields"] if f.get("required")}
        assert documented == required, (name, documented, required)

    for page in [
        "usage/compatibility.mdx",
        "quick-start.mdx",
        "usage/sending_messages.mdx",
    ]:
        source = read(page)
        for language in [
            "Python",
            "TypeScript",
            "Go",
            "Java",
            "CSharp",
            "Ruby",
            "Rust",
            "PHP",
            "Elixir",
            "Haskell",
        ]:
            panel = re.search(rf"<{language}>(.*?)</{language}>", source, re.S)[1]
            assert len(panel.split()) >= 45, (
                f"{page}: {language} lacks substantive guidance"
            )
    assert "Plain tables allow rows of different lengths" in guide
    assert "column_settings` may describe only the leading columns" in guide
    for filename in ["messages.py", "objects.py"]:
        assert (
            "InvalidUsageException"
            not in (ROOT / "python/slackblocks" / filename).read_text()
        )
    print(
        "Copyable TOML, exact Node engines, required fields and ten-language guide coverage passed."
    )


if __name__ == "__main__":
    main()
