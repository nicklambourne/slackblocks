from __future__ import annotations

import pytest

from slackblocks.errors import InvalidUsageError
from slackblocks.objects import (
    Confirm,
    ConfirmationDialogue,
    ConversationFilter,
    DispatchActionConfiguration,
    InputParameter,
    Markdown,
    Option,
    OptionGroup,
    PlainText,
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
    assert fetch_sample(path="test/samples/objects/text_markdown_verbatim.json") == repr(
        Text(text="hi", type_=TextType.MARKDOWN, verbatim=True)
    )


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
    assert fetch_sample(path="test/samples/objects/confirmation_dialogue_basic.json") == repr(
        confirmation_dialogue
    )


def test_confirm_alias_equivalent_to_confirmation_dialogue() -> None:
    """Regression test for #126: the ``Confirm`` backwards-compat alias must
    forward all arguments to ``ConfirmationDialogue.__init__`` and produce
    the same JSON output."""
    confirm = Confirm(
        title=Text("Maybe?", type_=TextType.PLAINTEXT),
        text=Text("Would you like to play checkers?", type_=TextType.PLAINTEXT),
        confirm=Text("Yes", type_=TextType.PLAINTEXT),
        deny=Text("Nope!", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(path="test/samples/objects/confirmation_dialogue_basic.json") == repr(
        confirm
    )


def test_conversation_filter_basic() -> None:
    conversation_filter = ConversationFilter(
        include=[
            "public",
            "mpim",
        ],
        exclude_bot_users=True,
    )
    assert fetch_sample(path="test/samples/objects/conversation_filter_basic.json") == repr(
        conversation_filter
    )


def test_dispatch_action_config_basic() -> None:
    dispatch_action_config = DispatchActionConfiguration(
        trigger_actions_on=["on_character_entered"]
    )
    assert fetch_sample(
        path="test/samples/objects/dispatch_action_configuration_basic.json"
    ) == repr(dispatch_action_config)


def test_dispatch_action_config_has_type_attribute() -> None:
    """Regression test for #128: DispatchActionConfiguration must call
    super().__init__ so that the inherited ``type`` attribute is set."""
    dispatch_action_config = DispatchActionConfiguration(
        trigger_actions_on=["on_character_entered"]
    )
    assert hasattr(dispatch_action_config, "type")
    assert dispatch_action_config.type.value == "dispatch"


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
    assert fetch_sample(path="test/samples/objects/option_group_basic.json") == repr(option_group)


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
    assert fetch_sample(path="test/samples/objects/workflow_basic.json") == repr(workflow)


# -- PlainText / Markdown convenience aliases -----------------------------


def test_plaintext_alias_basic() -> None:
    """``PlainText('Hi')`` renders to the same JSON as the equivalent
    ``Text`` call with ``TextType.PLAINTEXT``."""
    assert repr(PlainText("hi")) == repr(Text("hi", type_=TextType.PLAINTEXT))


def test_plaintext_alias_with_emoji() -> None:
    assert repr(PlainText("hi", emoji=True)) == repr(
        Text("hi", type_=TextType.PLAINTEXT, emoji=True)
    )


def test_plaintext_is_a_text() -> None:
    """``PlainText`` is a ``Text`` subclass so it works anywhere a ``Text``
    is expected."""
    pt = PlainText("hi")
    assert isinstance(pt, Text)
    assert pt.text_type == TextType.PLAINTEXT


def test_markdown_alias_basic() -> None:
    """``Markdown('hi')`` renders to the same JSON as the equivalent
    ``Text`` call with ``TextType.MARKDOWN``."""
    assert repr(Markdown("hi")) == repr(Text("hi", type_=TextType.MARKDOWN))


def test_markdown_alias_with_verbatim() -> None:
    assert repr(Markdown("hi", verbatim=True)) == repr(
        Text("hi", type_=TextType.MARKDOWN, verbatim=True)
    )


def test_markdown_is_a_text() -> None:
    md = Markdown("hi")
    assert isinstance(md, Text)
    assert md.text_type == TextType.MARKDOWN


def test_aliases_accept_text_like_callers() -> None:
    """Anywhere a ``TextLike`` (or ``Text``) is expected, the aliases work."""
    from slackblocks import SectionBlock

    sb = SectionBlock(text=PlainText("Hi"))
    assert sb.text == PlainText("Hi")
    sb2 = SectionBlock(text=Markdown("_hi_"))
    assert sb2.text == Markdown("_hi_")


def test_plaintext_empty_raises() -> None:
    """Length validation is inherited from ``Text`` and continues to raise
    via the existing ``LengthError`` path."""
    from slackblocks import LengthError

    with pytest.raises(LengthError):
        PlainText("")


def test_markdown_empty_raises() -> None:
    from slackblocks import LengthError

    with pytest.raises(LengthError):
        Markdown("")


def test_plaintext_exported_at_top_level() -> None:
    """The alias must be importable from the top-level ``slackblocks`` package."""
    from slackblocks import PlainText as TopLevel

    assert TopLevel is PlainText


def test_markdown_exported_at_top_level() -> None:
    from slackblocks import Markdown as TopLevel

    assert TopLevel is Markdown


# -- Workflow.from_url convenience factory --------------------------------


def test_workflow_from_url_matches_verbose_form() -> None:
    """``Workflow.from_url(url, **params)`` must produce identical JSON to
    the equivalent verbose construction."""
    import json

    verbose = Workflow(
        trigger=Trigger(
            url="https://slack.com/x",
            customizable_input_parameters=[
                InputParameter(name="a", value="1"),
                InputParameter(name="b", value="2"),
            ],
        )
    )
    concise = Workflow.from_url("https://slack.com/x", a="1", b="2")
    assert json.loads(repr(verbose)) == json.loads(repr(concise))


def test_workflow_from_url_no_params_omits_input_parameters() -> None:
    """No keyword arguments -> no ``customizable_input_parameters`` in JSON."""
    workflow = Workflow.from_url("https://slack.com/x")
    resolved = workflow._resolve()
    assert resolved == {"trigger": {"url": "https://slack.com/x"}}


def test_workflow_from_url_preserves_param_order_and_values() -> None:
    workflow = Workflow.from_url("https://slack.com/x", first="one", second="two")
    resolved = workflow._resolve()
    params = resolved["trigger"]["customizable_input_parameters"]
    assert params == [
        {"name": "first", "value": "one"},
        {"name": "second", "value": "two"},
    ]


def test_workflow_from_url_returns_workflow_instance() -> None:
    workflow = Workflow.from_url("https://slack.com/x", a="1")
    assert isinstance(workflow, Workflow)
