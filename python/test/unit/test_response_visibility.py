import pytest

from slackblocks import MessageResponse


@pytest.mark.parametrize("ephemeral, expected", [(True, "ephemeral"), (False, "in_channel")])
def test_explicit_response_visibility(ephemeral: bool, expected: str) -> None:
    payload = MessageResponse(text="Requested visibility", ephemeral=ephemeral).to_dict()
    assert payload["response_type"] == expected
    assert payload["text"] == "Requested visibility"


def test_default_visibility_remains_unspecified() -> None:
    assert "response_type" not in MessageResponse(text="Transport default").to_dict()
    assert "response_type" not in MessageResponse(ephemeral=None).to_dict()
