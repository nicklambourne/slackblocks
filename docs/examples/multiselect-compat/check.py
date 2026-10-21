"""Assert the exact 15 native outputs, supplied as a JSONL file; no Slack sends."""

import json
import sys
from itertools import product
from pathlib import Path

rows = [json.loads(line) for line in Path(sys.argv[1]).read_text().splitlines()]
expected = set(
    product(
        ("static", "external", "users", "conversations", "channels"),
        ("actions", "section", "input"),
    )
)
assert len(rows) == 15
assert {(r["family"], r["parent"]) for r in rows} == expected
for row in rows:
    p = row["payload"]
    assert row["surface"] == ("modal" if row["parent"] == "input" else "message")
    if row["parent"] == "input":
        assert p["type"] == "modal"
        for field, text in (("title", "Multi-select probe"), ("submit", "Save")):
            assert p[field]["type"] == "plain_text" and p[field]["text"] == text
        assert "channel" not in p
    else:
        assert p["channel"] == "C0123456" and p["text"] == "Choose values"
        assert "type" not in p
    assert len(p["blocks"]) == 1
    block = p["blocks"][0]
    assert block["type"] == row["parent"] and block["block_id"] == "choices"
    if row["parent"] == "actions":
        assert len(block["elements"]) == 1
    else:
        text = block["label" if row["parent"] == "input" else "text"]
        assert text["text"] == "Choose values"
        assert text["type"] == ("plain_text" if row["parent"] == "input" else "mrkdwn")
    element = (
        block["elements"][0]
        if row["parent"] == "actions"
        else block["accessory" if row["parent"] == "section" else "element"]
    )
    assert (
        element["type"]
        == {
            "static": "multi_static_select",
            "external": "multi_external_select",
            "users": "multi_users_select",
            "conversations": "multi_conversations_select",
            "channels": "multi_channels_select",
        }[row["family"]]
    )
    assert element["action_id"] == "choose"
    if row["family"] == "static":
        assert element["options"] == [
            {"text": {"type": "plain_text", "text": "One"}, "value": "one"}
        ] or element["options"] == [
            {
                "text": {"type": "plain_text", "text": "One", "emoji": True},
                "value": "one",
            }
        ]
    if row["family"] == "external":
        assert element["min_query_length"] == 0 and "focus_on_load" not in element
    assert "submit_disabled" not in p
print("PASS: 15 native cases; local construction only, Slack acceptance not inferred")
