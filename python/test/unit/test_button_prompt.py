import json

import pytest

from slackblocks import Button
from slackblocks.errors import TypeMismatchError


def test_prompt_fields_preserve_values_and_omission():
    assert "agent_prompt" not in json.loads(repr(Button("Ask")))
    assert "agent_prompt_display" not in json.loads(
        repr(Button("Ask", agent_prompt="Ask Slackbot"))
    )
    button = Button("Ask", agent_prompt="Ask Slackbot", agent_prompt_display="x" * 80)
    assert button.agent_prompt == "Ask Slackbot"
    assert json.loads(repr(button))["agent_prompt_display"] == "x" * 80
    # Existing positional arguments still bind to the same fields.
    old = Button("Open", "open", "https://example.com", "v", None, None, "Open link")
    assert json.loads(repr(old))["accessibility_label"] == "Open link"


@pytest.mark.parametrize("field", ["agent_prompt", "agent_prompt_display"])
@pytest.mark.parametrize("value", [1, True, [], {}])
def test_prompt_fields_reject_non_strings(field, value):
    with pytest.raises(TypeMismatchError):
        Button("Ask", **{field: value})
