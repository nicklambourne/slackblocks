#!/usr/bin/env python3
"""Check copyable configuration and contract facts against their authoritative inputs.

Uses Python 3.11+ (the documentation CI runtime); it does not change package floors.
"""

import json
import os
import re
import sys
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

import tomllib

ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "docs/docs"


def read(path):
    return (DOCS / path).read_text()


def panel(page, language):
    return re.search(rf"<{language}>(.*?)</{language}>", read(page), re.S)[1]


def check_journeys():
    # These are public native operations, not a prose-length proxy. Literal recipe
    # execution separately checks outputs, validation categories and ownership.
    operations = {
        "Python": (
            "3.10",
            "Message(",
            ".json()",
            "LengthError",
            "deepcopy",
            "Block.from_dict",
        ),
        "TypeScript": (
            "^20.19.0 || >=22.12.0",
            "Message()",
            "JSON.stringify",
            "LengthError",
            "S.Message()",
            "S.validate",
        ),
        "Go": (
            "1.22",
            "NewMessage()",
            "json.NewEncoder",
            "ValidationError",
            "NewMessage()",
            "json.Unmarshal",
        ),
        "Java": (
            "17",
            "MessagePayload.builder()",
            "SlackblocksJson.write",
            "ValidationException",
            "MessagePayload.builder()",
            "JsonParser",
        ),
        "CSharp": (
            ".NET 8",
            "new MessagePayload(",
            ".ToJson()",
            "ValidationException",
            "new MessagePayload(",
            "JsonDocument.Parse",
        ),
        "Ruby": (
            "3.3",
            "MessagePayload.new",
            ".to_json",
            "ValidationError",
            ".with(",
            "JSON.parse",
        ),
        "Rust": (
            "1.85.0",
            "MessagePayload::builder",
            "serde_json::to_string",
            "ErrorCategory",
            ".into_builder()",
            "try_from",
        ),
        "PHP": (
            "8.2",
            "new S\\MessagePayload(",
            "->toJson()",
            "ValidationError",
            "->with(",
            "fromJson",
        ),
        "Elixir": (
            "1.16.3",
            "Message.new!",
            "Slackblocks.to_map!",
            "ValidationError",
            "Message.change!",
            "Slackblocks.decode!",
        ),
        "Haskell": (
            "9.6.7",
            "buildMessagePayload",
            "encode",
            "LengthExceeded",
            "inspectMessagePayload",
            "decode",
        ),
    }
    for language, (
        runtime,
        construct,
        serialize,
        error,
        edit,
        parse,
    ) in operations.items():
        assert runtime in panel("usage/compatibility.mdx", language), (
            language,
            "runtime",
        )
        for page, tokens in [
            ("quick-start.mdx", [construct, serialize]),
            ("recipes/validation-errors.mdx", [error]),
            ("recipes/metadata.mdx", [edit]),
            ("recipes/parse-json.mdx", [parse]),
            ("recipes/accordion-pagination.mdx", ["accordion", "paginat"]),
        ]:
            contents = panel(page, language)
            for token in tokens:
                assert token.lower() in contents.lower(), (language, page, token)
        sending = panel("usage/sending_messages.mdx", language).lower()
        for surface in ["webhook", "response", "modal", "home"]:
            assert surface in sending, (language, surface)


def check_requests_example():
    source = read("usage/troubleshooting.mdx").split(
        "## How do I send a `slackblocks.Message` with `requests`?", 1
    )[1]
    code = re.search(r"```python\n(.*?)```", source, re.S)[1]
    sys.path.insert(0, str(ROOT / "python"))
    for case in ["success", "http", "slack", "invalid-json", "timeout"]:

        def post(url, **kwargs):
            assert url == "https://slack.com/api/chat.postMessage"
            assert kwargs["timeout"] == 30
            assert kwargs["json"]["text"] and kwargs["json"]["blocks"]
            if case == "timeout":
                raise TimeoutError("mock timeout")

            def status():
                if case == "http":
                    raise RuntimeError("mock HTTP 500")

            def body():
                if case == "invalid-json":
                    raise ValueError("mock invalid JSON")
                return {"ok": case != "slack", "error": "invalid_auth"}

            return SimpleNamespace(raise_for_status=status, json=body)

        with (
            patch.dict(sys.modules, requests=SimpleNamespace(post=post)),
            patch.dict(os.environ, SLACK_API_TOKEN="offline-test"),
        ):
            try:
                exec(compile(code, "troubleshooting requests example", "exec"), {})
            except (RuntimeError, ValueError, TimeoutError):
                assert case != "success", case
            else:
                assert case == "success", f"Example silently accepted {case} failure"


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

    limits = json.loads((ROOT / "spec/limits.json").read_text())
    image = guide.split("## Image Block", 1)[1].split("\n## ", 1)[0]
    image_summary = re.search(r"Required: ([^\n]+)", image)[1].split(". Use", 1)[0]
    assert "exactly one image source" in image_summary
    assert set(re.findall(r"`(\w+)`", image_summary)) == {
        "alt_text",
        "image_url",
        "slack_file",
    }
    assert {f["wire"] for f in model["ImageBlock"]["fields"] if f.get("required")} == {
        "alt_text"
    }
    assert model["ImageBlock"]["rules"] == [
        "Provide exactly one of an image URL or a Slack file."
    ]
    for field in ["alt_text", "image_url"]:
        assert (
            f"`{field}` (up to {limits['image'][field]['max_length']:,} characters)"
            in image_summary
        )
    markdown = guide.split("## Markdown Block", 1)[1].split("\n## ", 1)[0]
    assert "accepts an empty string locally" in markdown
    assert (
        f"maximum is {limits['markdown']['text']['max_length']:,} Unicode code points"
        in markdown
    )
    assert "aggregate Markdown text in a complete message" in markdown
    assert limits["markdown"]["text"] == limits["markdown"]["total_text"]
    assert "min_length" not in limits["markdown"]["text"]
    check_journeys()
    check_requests_example()
    assert "Plain tables allow rows of different lengths" in guide
    assert "column_settings` may describe only the leading columns" in guide
    for filename in ["messages.py", "objects.py"]:
        assert (
            "InvalidUsageException"
            not in (ROOT / "python/slackblocks" / filename).read_text()
        )
    print(
        "Copyable configuration, required fields/limits, ten-language journeys and mocked HTTP failures passed."
    )


if __name__ == "__main__":
    main()
