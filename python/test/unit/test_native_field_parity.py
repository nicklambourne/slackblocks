import json

import pytest

from slackblocks import (
    ConfirmationDialogue,
    Message,
    RichTextEmoji,
    TypeMismatchError,
    Workflow,
    WorkflowButton,
)


def test_confirmation_style_is_preserved_on_round_trip() -> None:
    confirm = ConfirmationDialogue("Proceed?", "Confirm", "Yes", "No", style="danger")
    assert confirm._resolve()["style"] == "danger"
    assert ConfirmationDialogue.from_dict(confirm._resolve())._resolve() == confirm._resolve()
    assert "style" not in ConfirmationDialogue("Proceed?", "Confirm", "Yes", "No")._resolve()
    with pytest.raises(TypeMismatchError, match="style"):
        ConfirmationDialogue("Proceed?", "Confirm", "Yes", "No", style="invalid")  # type: ignore[arg-type]


def test_workflow_confirmation_is_a_native_child() -> None:
    confirm = ConfirmationDialogue("Proceed?", "Confirm", "Yes", "No", style="primary")
    workflow = Workflow.from_url("https://slack.com/shortcuts/example")
    button = WorkflowButton("Run", workflow=workflow, action_id="run", confirm=confirm)
    assert button._resolve()["confirm"] == confirm._resolve()
    assert "confirm" not in WorkflowButton("Run", workflow=workflow, action_id="run")._resolve()
    with pytest.raises(TypeMismatchError, match="confirm"):
        WorkflowButton("Run", workflow=workflow, action_id="run", confirm="invalid")  # type: ignore[arg-type]


def test_emoji_skin_tone_and_omission() -> None:
    assert RichTextEmoji("wave", skin_tone=6)._resolve() == {
        "type": "emoji",
        "name": "wave",
        "skin_tone": 6,
    }
    assert RichTextEmoji("wave")._resolve() == {"type": "emoji", "name": "wave"}


def test_message_metadata_preserves_application_keys_and_false_values() -> None:
    metadata = {
        "event_type": "test_completed",
        "event_payload": {"recordId": "9007199254740993", "ok": False},
    }
    assert Message("C123", metadata=metadata).to_dict()["metadata"] == metadata
    assert "metadata" not in Message("C123").to_dict()


@pytest.mark.parametrize("serialization", ["dict", "json", "repr", "unpack"])
def test_message_metadata_preserves_nested_json_nulls(serialization: str) -> None:
    metadata = {
        "event_type": "test_completed",
        "event_payload": {
            "presentNull": None,
            "nested": [None, {"presentNull": None, "ok": False, "count": 0}],
        },
    }
    message = Message("C123", metadata=metadata)
    payload = {
        "dict": message.to_dict,
        "json": lambda: json.loads(message.json()),
        "repr": lambda: json.loads(repr(message)),
        "unpack": lambda: {**message},
    }[serialization]()
    assert payload["metadata"] == metadata
    payload["metadata"]["event_payload"]["nested"][1]["count"] = 1
    assert message.to_dict()["metadata"] == metadata
    assert metadata["event_payload"]["nested"][1]["count"] == 0
