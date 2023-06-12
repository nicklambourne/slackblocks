import pytest

from slackblocks.errors import InvalidUsageError
from slackblocks.objects import (
    ConfirmationDialogue,
    ConversationFilter,
    DispatchActionConfiguration,
    InputParameter,
    Option,
    OptionGroup,
    Text,
    TextType,
    Trigger,
    Workflow,
)

from .utils import THREE_OPTIONS, fetch_sample

INPUT_PARAMETERS = [
    InputParameter(
        name="A",
        value="A",
    ),
    InputParameter(
        name="B",
        value="B",
    ),
]


def test_text_basic() -> None:
    assert fetch_sample(path="test/samples/objects/text_plaintext_basic.json") == repr(
        Text(text="hi", type_=TextType.PLAINTEXT)
    )
    assert fetch_sample(path="test/samples/objects/text_markdown_basic.json") == repr(
        Text(text="hi", type_=TextType.MARKDOWN)
    )


def test_text_plaintext_emoji() -> None:
    assert fetch_sample(path="test/samples/objects/text_plaintext_emoji.json") == repr(
        Text(text="hi", type_=TextType.PLAINTEXT, emoji=True)
    )


def test_text_markdown_verbatim() -> None:
    assert fetch_sample(
        path="test/samples/objects/text_markdown_verbatim.json"
    ) == repr(Text(text="hi", type_=TextType.MARKDOWN, verbatim=True))


def test_text_coerce_from_string() -> None:
    assert fetch_sample(path="test/samples/objects/text_markdown_basic.json") == repr(
        Text.to_text("hi")
    )


def test_text_coerce_from_text() -> None:
    assert fetch_sample(path="test/samples/objects/text_markdown_basic.json") == repr(
        Text.to_text(Text("hi"))
    )


def test_text_allow_none() -> None:
    assert Text.to_text(None, allow_none=True) is None


def test_text_disallow_none() -> None:
    with pytest.raises(InvalidUsageError):
        assert Text.to_text(None)


def test_text_coerce_from_invalid() -> None:
    with pytest.raises(InvalidUsageError):
        assert Text.to_text(123)


def test_text_coerce_max_length_exceeded() -> None:
    with pytest.raises(InvalidUsageError):
        assert Text.to_text("abcdef", max_length=5)


def test_confirmation_dialogue_basic() -> None:
    confirmation_dialogue = ConfirmationDialogue(
        title=Text("Maybe?", type_=TextType.PLAINTEXT),
        text=Text("Would you like to play checkers?", type_=TextType.PLAINTEXT),
        confirm=Text("Yes", type_=TextType.PLAINTEXT),
        deny=Text("Nope!", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(
        path="test/samples/objects/confirmation_dialogue_basic.json"
    ) == repr(confirmation_dialogue)


def test_conversation_filter_basic() -> None:
    conversation_filter = ConversationFilter(
        include=[
            "public",
            "mpim",
        ],
        exclude_bot_users=True,
    )
    assert fetch_sample(
        path="test/samples/objects/conversation_filter_basic.json"
    ) == repr(conversation_filter)


def test_dispatch_action_config_basic() -> None:
    dispatch_action_config = DispatchActionConfiguration(
        trigger_actions_on=["on_character_entered"]
    )
    assert fetch_sample(
        path="test/samples/objects/dispatch_action_configuration_basic.json"
    ) == repr(dispatch_action_config)


def test_input_parameter_basic() -> None:
    input_parameter = InputParameter(
        name="name",
        value="value",
    )
    assert fetch_sample(path="test/samples/objects/input_parameter_basic.json") == repr(
        input_parameter
    )


def test_option_basic() -> None:
    option = Option(
        text=Text(text="Canberra", type_=TextType.PLAINTEXT),
        value="canberra",
    )
    assert fetch_sample(path="test/samples/objects/option_basic.json") == repr(option)


def test_option_group_basic() -> None:
    option_group = OptionGroup(
        label="Group A",
        options=THREE_OPTIONS,
    )
    assert fetch_sample(path="test/samples/objects/option_group_basic.json") == repr(
        option_group
    )


def test_trigger_basic() -> None:
    trigger = Trigger(
        url="https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
        customizable_input_parameters=INPUT_PARAMETERS,
    )
    assert fetch_sample(path="test/samples/objects/trigger_basic.json") == repr(trigger)


def test_workflow_basic() -> None:
    workflow = Workflow(
        trigger=Trigger(
            url="https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
            customizable_input_parameters=INPUT_PARAMETERS,
        )
    )
    assert fetch_sample(path="test/samples/objects/workflow_basic.json") == repr(
        workflow
    )
