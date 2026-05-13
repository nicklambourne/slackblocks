"""Tests for the ``from_dict`` parsers added in Phase 7.4.

These verify the round-trip contract: for every supported class,
``Foo.from_dict(Foo(...)._resolve())`` produces an instance whose
``_resolve()`` equals the original.
"""

from __future__ import annotations

import pytest

from slackblocks import (
    ActionsBlock,
    Block,
    Button,
    ConfirmationDialogue,
    ContextBlock,
    ConversationFilter,
    DispatchActionConfiguration,
    DividerBlock,
    FileBlock,
    HeaderBlock,
    ImageBlock,
    InputBlock,
    InputParameter,
    MarkdownBlock,
    MissingRequiredError,
    Option,
    OptionGroup,
    PlainTextInput,
    RichTextBlock,
    SectionBlock,
    SlackFile,
    TableBlock,
    Text,
    TextType,
    Trigger,
    TypeMismatchError,
    VideoBlock,
    Workflow,
)

# -- Composition objects ---------------------------------------------------


def test_text_round_trip_markdown() -> None:
    t = Text("hi", type_=TextType.MARKDOWN, verbatim=True)
    assert Text.from_dict(t._resolve())._resolve() == t._resolve()


def test_text_round_trip_plaintext_with_emoji() -> None:
    t = Text("hi", type_=TextType.PLAINTEXT, emoji=True)
    assert Text.from_dict(t._resolve())._resolve() == t._resolve()


def test_text_from_dict_missing_text_raises() -> None:
    with pytest.raises(MissingRequiredError):
        Text.from_dict({"type": "mrkdwn"})


def test_text_from_dict_unknown_type_raises() -> None:
    with pytest.raises(TypeMismatchError):
        Text.from_dict({"type": "weird", "text": "hi"})


def test_text_from_dict_ignores_unknown_fields() -> None:
    """Forward-compat: unknown fields in the payload do not break parsing."""
    t = Text.from_dict({"type": "mrkdwn", "text": "hi", "future_field": "x"})
    assert t.text == "hi"


def test_confirmation_dialogue_round_trip() -> None:
    c = ConfirmationDialogue(
        title="T",
        text="Body",
        confirm="OK",
        deny="Cancel",
    )
    assert ConfirmationDialogue.from_dict(c._resolve())._resolve() == c._resolve()


def test_confirmation_dialogue_missing_field_raises() -> None:
    with pytest.raises(MissingRequiredError):
        ConfirmationDialogue.from_dict(
            {
                "title": {"type": "plain_text", "text": "T"},
                "text": {"type": "plain_text", "text": "x"},
            }
        )


def test_option_round_trip_minimal() -> None:
    o = Option(text="A", value="a")
    assert Option.from_dict(o._resolve())._resolve() == o._resolve()


def test_option_round_trip_full() -> None:
    o = Option(text="A", value="a", description="desc", url="https://x.com")
    assert Option.from_dict(o._resolve())._resolve() == o._resolve()


def test_option_missing_value_raises() -> None:
    with pytest.raises(MissingRequiredError):
        Option.from_dict({"text": {"type": "plain_text", "text": "A"}})


def test_option_group_round_trip() -> None:
    og = OptionGroup(label="G", options=[Option("A", "a"), Option("B", "b")])
    assert OptionGroup.from_dict(og._resolve())._resolve() == og._resolve()


def test_dispatch_action_configuration_round_trip() -> None:
    dac = DispatchActionConfiguration(trigger_actions_on=["on_enter_pressed"])
    assert DispatchActionConfiguration.from_dict(dac._resolve())._resolve() == dac._resolve()


def test_conversation_filter_round_trip() -> None:
    cf = ConversationFilter(include=["public"], exclude_bot_users=True)
    assert ConversationFilter.from_dict(cf._resolve())._resolve() == cf._resolve()


def test_input_parameter_round_trip() -> None:
    ip = InputParameter(name="n", value="v")
    assert InputParameter.from_dict(ip._resolve())._resolve() == ip._resolve()


def test_input_parameter_missing_field_raises() -> None:
    with pytest.raises(MissingRequiredError):
        InputParameter.from_dict({"name": "x"})


def test_slack_file_round_trip_url() -> None:
    sf = SlackFile(url="https://x.png", id=None)
    assert SlackFile.from_dict(sf._resolve())._resolve() == sf._resolve()


def test_slack_file_round_trip_id() -> None:
    sf = SlackFile(url=None, id="F123")
    assert SlackFile.from_dict(sf._resolve())._resolve() == sf._resolve()


def test_trigger_round_trip_with_params() -> None:
    tr = Trigger(
        url="https://slack.com/x",
        customizable_input_parameters=[InputParameter("a", "1")],
    )
    assert Trigger.from_dict(tr._resolve())._resolve() == tr._resolve()


def test_trigger_round_trip_no_params() -> None:
    tr = Trigger(url="https://slack.com/x", customizable_input_parameters=None)
    assert Trigger.from_dict(tr._resolve())._resolve() == tr._resolve()


def test_trigger_missing_url_raises() -> None:
    with pytest.raises(MissingRequiredError):
        Trigger.from_dict({})


def test_workflow_round_trip() -> None:
    w = Workflow.from_url("https://slack.com/x", a="1", b="2")
    assert Workflow.from_dict(w._resolve())._resolve() == w._resolve()


def test_workflow_missing_trigger_raises() -> None:
    with pytest.raises(MissingRequiredError):
        Workflow.from_dict({})


# -- Block dispatcher and supported blocks --------------------------------


def test_block_from_dict_missing_type_raises() -> None:
    with pytest.raises(MissingRequiredError):
        Block.from_dict({})


def test_block_from_dict_unknown_type_raises() -> None:
    with pytest.raises(TypeMismatchError):
        Block.from_dict({"type": "no_such_block"})


def test_divider_block_round_trip() -> None:
    d = DividerBlock(block_id="b1")
    parsed = Block.from_dict(d._resolve())
    assert isinstance(parsed, DividerBlock)
    assert parsed._resolve() == d._resolve()


def test_file_block_round_trip() -> None:
    fb = FileBlock(external_id="F1", block_id="b1")
    parsed = Block.from_dict(fb._resolve())
    assert isinstance(parsed, FileBlock)
    assert parsed._resolve() == fb._resolve()


def test_file_block_missing_external_id_raises() -> None:
    with pytest.raises(MissingRequiredError):
        FileBlock.from_dict({"type": "file", "block_id": "b1"})


def test_header_block_round_trip() -> None:
    hb = HeaderBlock(text="Hi", block_id="b1")
    parsed = Block.from_dict(hb._resolve())
    assert isinstance(parsed, HeaderBlock)
    assert parsed._resolve() == hb._resolve()


def test_markdown_block_round_trip() -> None:
    mb = MarkdownBlock(text="**hi**", block_id="b1")
    parsed = Block.from_dict(mb._resolve())
    assert isinstance(parsed, MarkdownBlock)
    assert parsed._resolve() == mb._resolve()


def test_image_block_round_trip_with_title() -> None:
    ib = ImageBlock(image_url="https://x.png", alt_text="alt", title="T", block_id="b1")
    parsed = Block.from_dict(ib._resolve())
    assert isinstance(parsed, ImageBlock)
    assert parsed._resolve() == ib._resolve()


def test_image_block_round_trip_no_title() -> None:
    ib = ImageBlock(image_url="https://x.png", alt_text="alt", block_id="b1")
    parsed = Block.from_dict(ib._resolve())
    assert isinstance(parsed, ImageBlock)
    assert parsed._resolve() == ib._resolve()


def test_section_block_round_trip_text_only() -> None:
    sb = SectionBlock("Hi", block_id="b1")
    parsed = Block.from_dict(sb._resolve())
    assert isinstance(parsed, SectionBlock)
    assert parsed._resolve() == sb._resolve()


def test_section_block_round_trip_with_fields() -> None:
    sb = SectionBlock("Hi", block_id="b1", fields=["a", "b"])
    parsed = Block.from_dict(sb._resolve())
    assert isinstance(parsed, SectionBlock)
    assert parsed._resolve() == sb._resolve()


def test_section_block_with_accessory_raises_not_implemented() -> None:
    """SectionBlock with an accessory element is deferred to Phase 7.4b."""
    sb = SectionBlock("Hi", block_id="b1", accessory=Button("X", action_id="a"))
    with pytest.raises(NotImplementedError):
        Block.from_dict(sb._resolve())


def test_context_block_round_trip_text_only() -> None:
    cb = ContextBlock(elements=[Text("hi", type_=TextType.PLAINTEXT)], block_id="b1")
    parsed = Block.from_dict(cb._resolve())
    assert isinstance(parsed, ContextBlock)
    assert parsed._resolve() == cb._resolve()


def test_context_block_with_image_element_raises_not_implemented() -> None:
    """ContextBlock containing an Image element is deferred to Phase 7.4b."""
    from slackblocks import Image

    cb = ContextBlock(elements=[Image(image_url="https://x.png", alt_text="a")], block_id="b1")
    with pytest.raises(NotImplementedError):
        Block.from_dict(cb._resolve())


def test_video_block_round_trip_minimal() -> None:
    vb = VideoBlock(
        alt_text="alt",
        thumbnail_url="https://x.png",
        title="Title",
        video_url="https://v.mp4",
        block_id="b1",
    )
    parsed = Block.from_dict(vb._resolve())
    assert isinstance(parsed, VideoBlock)
    assert parsed._resolve() == vb._resolve()


def test_video_block_round_trip_full() -> None:
    vb = VideoBlock(
        alt_text="alt",
        thumbnail_url="https://x.png",
        title="Title",
        video_url="https://v.mp4",
        block_id="b1",
        author_name="A",
        description="D",
        provider_icon_url="https://i.png",
        provider_name="P",
        title_url="https://t.com",
    )
    parsed = Block.from_dict(vb._resolve())
    assert parsed._resolve() == vb._resolve()


def test_video_block_missing_required_raises() -> None:
    for missing in ("alt_text", "thumbnail_url", "title", "video_url"):
        base = {
            "type": "video",
            "alt_text": "a",
            "thumbnail_url": "u",
            "title": {"type": "plain_text", "text": "T"},
            "video_url": "u",
        }
        del base[missing]
        with pytest.raises(MissingRequiredError):
            VideoBlock.from_dict(base)


# -- Deferred blocks ------------------------------------------------------


def test_actions_block_round_trip_raises_not_implemented() -> None:
    ab = ActionsBlock(elements=[Button("Go", action_id="a")], block_id="b1")
    with pytest.raises(NotImplementedError):
        Block.from_dict(ab._resolve())


def test_input_block_round_trip_raises_not_implemented() -> None:
    ib = InputBlock(label="L", element=PlainTextInput(action_id="t"), block_id="b1")
    with pytest.raises(NotImplementedError):
        Block.from_dict(ib._resolve())


def test_rich_text_block_round_trip_raises_not_implemented() -> None:
    from slackblocks import RichText, RichTextSection

    rtb = RichTextBlock(elements=RichTextSection(elements=RichText("hi")), block_id="b1")
    with pytest.raises(NotImplementedError):
        Block.from_dict(rtb._resolve())


def test_table_block_round_trip_raises_not_implemented() -> None:
    from slackblocks import RawText

    tb = TableBlock(rows=[[RawText("cell")]], block_id="b1")
    with pytest.raises(NotImplementedError):
        Block.from_dict(tb._resolve())
