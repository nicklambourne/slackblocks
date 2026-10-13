import pytest

from slackblocks import (
    AlertBlock,
    ContextBlock,
    DividerBlock,
    FileBlock,
    HeaderBlock,
    ImageBlock,
    MarkdownBlock,
    MissingRequiredError,
    SectionBlock,
    TypeMismatchError,
    VideoBlock,
    Workflow,
    WorkflowButton,
)
from slackblocks.elements import ButtonStyle


@pytest.mark.parametrize(
    "parser",
    [
        AlertBlock,
        ContextBlock,
        DividerBlock,
        FileBlock,
        HeaderBlock,
        ImageBlock,
        MarkdownBlock,
        SectionBlock,
        VideoBlock,
    ],
)
def test_typed_parsers_reject_missing_and_wrong_discriminators(parser) -> None:
    with pytest.raises(MissingRequiredError, match="type"):
        parser.from_dict({})
    for wrong_type in ["not_this_block", None, 42]:
        with pytest.raises(TypeMismatchError, match="type"):
            parser.from_dict({"type": wrong_type})


@pytest.mark.parametrize(
    "value,expected",
    [
        (None, None),
        ("DEFAULT", None),
        (ButtonStyle.DEFAULT, None),
        ("primary", "primary"),
        ("PRIMARY", "primary"),
        (ButtonStyle.PRIMARY, "primary"),
        ("danger", "danger"),
        ("DANGER", "danger"),
        (ButtonStyle.DANGER, "danger"),
    ],
)
def test_workflow_button_accepts_consistent_styles(value, expected) -> None:
    button = WorkflowButton(
        "Run",
        Workflow.from_url("https://slack.com/shortcuts/example"),
        style=value,
        action_id="run",
    )
    assert button._resolve().get("style") == expected


@pytest.mark.parametrize("value", ["", "invalid", 42, False])
def test_invalid_styles_raise_library_error(value) -> None:
    with pytest.raises(TypeMismatchError, match="style"):
        ButtonStyle.to_button_style(value)
