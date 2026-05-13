"""Serializability + structural-shape suite for every public class.

Constructs every concrete class exported from ``slackblocks`` with minimal
but valid inputs and asserts:

1. ``_resolve()`` succeeds without raising.
2. ``json.dumps(_resolve())`` succeeds (catches the Phase 1 bug class
   where a nested object was inserted raw instead of via ``._resolve()``).
3. Where applicable, the rendered JSON has the documented Slack ``type``
   value.

The pattern is parametrized so adding new public classes is one line.
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING, Any

import pytest

if TYPE_CHECKING:
    from collections.abc import Callable

from slackblocks import (
    ActionsBlock,
    Attachment,
    Button,
    ChannelMultiSelectMenu,
    ChannelSelectMenu,
    CheckboxGroup,
    Color,
    Confirm,
    ConfirmationDialogue,
    ContextBlock,
    ConversationFilter,
    ConversationMultiSelectMenu,
    ConversationSelectMenu,
    DatePicker,
    DateTimePicker,
    DispatchActionConfiguration,
    DividerBlock,
    EmailInput,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    FileBlock,
    FileInput,
    HeaderBlock,
    HomeTabView,
    Image,
    ImageBlock,
    InputBlock,
    InputParameter,
    Markdown,
    MarkdownBlock,
    Message,
    MessageResponse,
    Modal,
    ModalView,
    NumberInput,
    Option,
    OptionGroup,
    OverflowMenu,
    PlainText,
    PlainTextInput,
    RadioButtonGroup,
    RawText,
    RichText,
    RichTextBlock,
    RichTextInput,
    RichTextSection,
    SectionBlock,
    SlackFile,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    TableBlock,
    Text,
    TextType,
    TimePicker,
    Trigger,
    URLInput,
    UserMultiSelectMenu,
    UserSelectMenu,
    VideoBlock,
    WebhookMessage,
    Workflow,
    WorkflowButton,
)
from slackblocks.objects import ColumnSettings
from slackblocks.rich_text.elements import (
    RichTextChannel,
    RichTextEmoji,
    RichTextLink,
    RichTextUser,
    RichTextUserGroup,
)
from slackblocks.rich_text.objects import (
    RichTextCodeBlock,
    RichTextList,
    RichTextQuote,
)


def _serializability_check(
    obj: Any,
    *,
    expected_type: str | None = None,
    has_type_field: bool = True,
) -> None:
    """Shared assertion: _resolve() runs, the output is json.dumps-clean,
    and (if applicable) the type field matches the documented Slack value.
    """
    resolved = obj._resolve()
    # Fails loudly with a TypeError if any nested Resolvable was inserted
    # raw instead of via ._resolve() -- exactly the Phase 1 bug class.
    payload = json.dumps(resolved)
    assert isinstance(payload, str) and payload.startswith("{")
    if has_type_field and expected_type is not None:
        assert resolved.get("type") == expected_type


# Each entry: (display name, constructor lambda, expected_type, has_type_field)
# - has_type_field=False means "_resolve does not include a 'type' key"
#   (e.g. Option, OptionGroup, ConversationFilter,
#   DispatchActionConfiguration, InputParameter, SlackFile, Trigger,
#   Workflow, ColumnSettings, FileInput, the rich-text section/list etc.).

_BLOCK_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    (
        "ActionsBlock",
        lambda: ActionsBlock(elements=[Button(text="Go", action_id="g")]),
        "actions",
        True,
    ),
    (
        "ContextBlock",
        lambda: ContextBlock(elements=[Text("hi", type_=TextType.PLAINTEXT)]),
        "context",
        True,
    ),
    ("DividerBlock", lambda: DividerBlock(), "divider", True),
    ("FileBlock", lambda: FileBlock(external_id="F1"), "file", True),
    ("HeaderBlock", lambda: HeaderBlock(text="Heading"), "header", True),
    (
        "ImageBlock",
        lambda: ImageBlock(image_url="https://x.png", alt_text="alt"),
        "image",
        True,
    ),
    (
        "InputBlock",
        lambda: InputBlock(label="L", element=PlainTextInput(action_id="t")),
        "input",
        True,
    ),
    ("MarkdownBlock", lambda: MarkdownBlock(text="**hi**"), "markdown", True),
    (
        "RichTextBlock",
        lambda: RichTextBlock(elements=[RichTextSection(elements=RichText("hi"))]),
        "rich_text",
        True,
    ),
    ("SectionBlock", lambda: SectionBlock(text="Hi"), "section", True),
    (
        "SectionBlock_with_accessory",
        lambda: SectionBlock(text="Hi", accessory=Button("X", action_id="a")),
        "section",
        True,
    ),
    (
        "SectionBlock_with_fields",
        lambda: SectionBlock(text="Hi", fields=["a", "b"]),
        "section",
        True,
    ),
    (
        "TableBlock",
        lambda: TableBlock(rows=[[RawText("c1"), RawText("c2")]]),
        "table",
        True,
    ),
    (
        "VideoBlock",
        lambda: VideoBlock(
            alt_text="alt",
            thumbnail_url="https://x.png",
            title="Title",
            video_url="https://v.mp4",
        ),
        "video",
        True,
    ),
]


_ELEMENT_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    ("Button", lambda: Button(text="Go", action_id="g"), "button", True),
    (
        "Button_with_confirm",
        lambda: Button(
            text="Go",
            action_id="g",
            confirm=ConfirmationDialogue(title="T", text="x", confirm="y", deny="n"),
        ),
        "button",
        True,
    ),
    (
        "ChannelMultiSelectMenu",
        lambda: ChannelMultiSelectMenu(action_id="a", placeholder="p"),
        "multi_channels_select",
        True,
    ),
    (
        "ChannelSelectMenu",
        lambda: ChannelSelectMenu(action_id="a", placeholder="p"),
        "channels_select",
        True,
    ),
    (
        "CheckboxGroup",
        lambda: CheckboxGroup(action_id="a", options=[Option("A", "a")]),
        "checkboxes",
        True,
    ),
    (
        "ConversationMultiSelectMenu",
        lambda: ConversationMultiSelectMenu(
            action_id="a",
            initial_conversations=["C1"],
            filter=ConversationFilter(include=["public"]),
        ),
        "multi_conversations_select",
        True,
    ),
    (
        "ConversationSelectMenu",
        lambda: ConversationSelectMenu(
            action_id="a",
            placeholder="p",
            filter=ConversationFilter(include=["public"]),
        ),
        "conversations_select",
        True,
    ),
    (
        "DatePicker_basic",
        lambda: DatePicker(action_id="d"),
        "datepicker",
        True,
    ),
    (
        "DatePicker_with_confirm_and_initial",
        lambda: DatePicker(
            action_id="d",
            initial_date="2024-01-01",
            confirm=ConfirmationDialogue(title="T", text="x", confirm="y", deny="n"),
            placeholder="Pick a date",
        ),
        "datepicker",
        True,
    ),
    (
        "DateTimePicker_basic",
        lambda: DateTimePicker(action_id="d"),
        "datetimepicker",
        True,
    ),
    (
        "DateTimePicker_with_confirm",
        lambda: DateTimePicker(
            action_id="d",
            initial_datetime=1700000000,
            confirm=ConfirmationDialogue(title="T", text="x", confirm="y", deny="n"),
        ),
        "datetimepicker",
        True,
    ),
    (
        "EmailInput",
        lambda: EmailInput(action_id="e", placeholder="email"),
        "email_text_input",
        True,
    ),
    (
        "ExternalMultiSelectMenu",
        lambda: ExternalMultiSelectMenu(action_id="a", placeholder="p"),
        "multi_external_select",
        True,
    ),
    (
        "ExternalSelectMenu",
        lambda: ExternalSelectMenu(action_id="a", placeholder="p"),
        "external_select",
        True,
    ),
    # FileInput historically does not include "type" in its rendered JSON.
    ("FileInput", lambda: FileInput(action_id="fi"), "", False),
    (
        "Image_with_url",
        lambda: Image(image_url="https://x.png", alt_text="alt"),
        "image",
        True,
    ),
    (
        "Image_with_slack_file",
        lambda: Image(
            alt_text="alt",
            slack_file=SlackFile(url="https://y.png", id=None),
        ),
        "image",
        True,
    ),
    (
        "NumberInput",
        lambda: NumberInput(
            is_decimal_allowed=False,
            action_id="n",
            min_value=0,
            max_value=100,
            placeholder="number",
        ),
        "number_input",
        True,
    ),
    (
        "OverflowMenu",
        lambda: OverflowMenu(action_id="o", options=[Option("A", "a")]),
        "overflow",
        True,
    ),
    (
        "PlainTextInput",
        lambda: PlainTextInput(action_id="p", placeholder="enter text"),
        "plain_text_input",
        True,
    ),
    (
        "RadioButtonGroup",
        lambda: RadioButtonGroup(action_id="r", options=[Option("A", "a")]),
        "radio_buttons",
        True,
    ),
    (
        "RichTextInput",
        lambda: RichTextInput(
            action_id="rt",
            placeholder="type",
            dispatch_action_config=DispatchActionConfiguration(
                trigger_actions_on=["on_enter_pressed"]
            ),
        ),
        "rich_text_input",
        True,
    ),
    (
        "StaticMultiSelectMenu",
        lambda: StaticMultiSelectMenu(
            action_id="a",
            placeholder="p",
            options=[Option(text=Text("A", type_=TextType.PLAINTEXT), value="a")],
        ),
        "multi_static_select",
        True,
    ),
    (
        "StaticSelectMenu",
        lambda: StaticSelectMenu(
            action_id="a",
            placeholder="p",
            options=[Option(text=Text("A", type_=TextType.PLAINTEXT), value="a")],
        ),
        "static_select",
        True,
    ),
    (
        "TimePicker",
        lambda: TimePicker(
            action_id="t",
            initial_time="12:00",
            confirm=ConfirmationDialogue(title="T", text="x", confirm="y", deny="n"),
            placeholder="time",
            timezone="Australia/Sydney",
        ),
        "timepicker",
        True,
    ),
    (
        "URLInput",
        lambda: URLInput(action_id="u", placeholder="url"),
        "url_text_input",
        True,
    ),
    (
        "UserMultiSelectMenu",
        lambda: UserMultiSelectMenu(action_id="a", placeholder="p"),
        "multi_users_select",
        True,
    ),
    (
        "UserSelectMenu",
        lambda: UserSelectMenu(action_id="a", placeholder="p"),
        "users_select",
        True,
    ),
    (
        "WorkflowButton",
        lambda: WorkflowButton(
            text="Run",
            workflow=Workflow.from_url("https://slack.com/x", env="prod"),
        ),
        "workflow_button",
        True,
    ),
]


_OBJECT_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    ("Text_markdown", lambda: Text("hi", type_=TextType.MARKDOWN), "mrkdwn", True),
    (
        "Text_plaintext",
        lambda: Text("hi", type_=TextType.PLAINTEXT, emoji=True),
        "plain_text",
        True,
    ),
    ("PlainText", lambda: PlainText("hi", emoji=True), "plain_text", True),
    ("Markdown_alias", lambda: Markdown("hi", verbatim=True), "mrkdwn", True),
    ("Option", lambda: Option(text="A", value="a"), "", False),
    (
        "Option_with_description_and_url",
        lambda: Option(text="A", value="a", description="desc", url="https://x.com"),
        "",
        False,
    ),
    (
        "OptionGroup",
        lambda: OptionGroup(label="G", options=[Option("A", "a")]),
        "",
        False,
    ),
    (
        "ConfirmationDialogue",
        lambda: ConfirmationDialogue(title="T", text="x", confirm="y", deny="n"),
        "",
        False,
    ),
    (
        "Confirm_alias",
        lambda: Confirm(title="T", text="x", confirm="y", deny="n"),
        "",
        False,
    ),
    (
        "ConversationFilter",
        lambda: ConversationFilter(include=["public"], exclude_bot_users=True),
        "",
        False,
    ),
    (
        "DispatchActionConfiguration",
        lambda: DispatchActionConfiguration(trigger_actions_on=["on_enter_pressed"]),
        "",
        False,
    ),
    ("InputParameter", lambda: InputParameter("a", "1"), "", False),
    ("SlackFile_url", lambda: SlackFile(url="https://x.png", id=None), "", False),
    ("SlackFile_id", lambda: SlackFile(url=None, id="F123"), "", False),
    (
        "Trigger_no_params",
        lambda: Trigger(url="https://slack.com/x", customizable_input_parameters=None),
        "",
        False,
    ),
    (
        "Trigger_with_params",
        lambda: Trigger(
            url="https://slack.com/x",
            customizable_input_parameters=[InputParameter("a", "1")],
        ),
        "",
        False,
    ),
    (
        "Workflow",
        lambda: Workflow(
            trigger=Trigger(url="https://slack.com/x", customizable_input_parameters=None)
        ),
        "",
        False,
    ),
    (
        "Workflow_from_url",
        lambda: Workflow.from_url("https://slack.com/x", a="1", b="2"),
        "",
        False,
    ),
    ("RawText", lambda: RawText("hi", emoji=True), "raw_text", True),
    (
        "ColumnSettings",
        lambda: ColumnSettings(align="left", is_wrapped=False),
        "",
        False,
    ),
]


_RICH_TEXT_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    ("RichText", lambda: RichText("hi", bold=True), "text", True),
    ("RichTextChannel", lambda: RichTextChannel(channel_id="C1"), "channel", True),
    ("RichTextEmoji", lambda: RichTextEmoji(name="wave"), "emoji", True),
    (
        "RichTextLink",
        lambda: RichTextLink(url="https://x.com", text="hi"),
        "link",
        True,
    ),
    ("RichTextUser", lambda: RichTextUser(user_id="U1"), "user", True),
    (
        "RichTextUserGroup",
        lambda: RichTextUserGroup(user_group_id="S1"),
        "usergroup",
        True,
    ),
    (
        "RichTextSection",
        lambda: RichTextSection(elements=[RichText("hi")]),
        "rich_text_section",
        True,
    ),
    (
        "RichTextList",
        lambda: RichTextList(
            style="bullet",
            elements=[RichTextSection(elements=[RichText("hi")])],
        ),
        "rich_text_list",
        True,
    ),
    (
        "RichTextCodeBlock",
        lambda: RichTextCodeBlock(elements=[RichText("code")]),
        "rich_text_preformatted",
        True,
    ),
    (
        "RichTextQuote",
        lambda: RichTextQuote(elements=[RichText("quote")]),
        "rich_text_quote",
        True,
    ),
]


_SURFACE_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    (
        "Message",
        lambda: Message(channel="#g", blocks=SectionBlock("Hi")),
        "",
        False,
    ),
    (
        "MessageResponse",
        lambda: MessageResponse(text="hi", ephemeral=True, replace_original=True),
        "",
        False,
    ),
    (
        "WebhookMessage",
        lambda: WebhookMessage(text="hi", blocks=[SectionBlock("Hi")]),
        "",
        False,
    ),
    (
        "ModalView",
        lambda: ModalView(title="My modal", blocks=[SectionBlock("Hi")]),
        "modal",
        True,
    ),
    (
        "Modal_alias",
        lambda: Modal(title="My modal", blocks=[SectionBlock("Hi")]),
        "modal",
        True,
    ),
    (
        "HomeTabView",
        lambda: HomeTabView(blocks=[SectionBlock("Hi")]),
        "home",
        True,
    ),
]


_ATTACHMENT_CASES: list[tuple[str, Callable[[], Any], str, bool]] = [
    (
        "Attachment_with_color",
        lambda: Attachment(blocks=[SectionBlock("Hi")], color=Color.GOOD),
        "",
        False,
    ),
    (
        "Attachment_minimal",
        lambda: Attachment(blocks=[SectionBlock("Hi")]),
        "",
        False,
    ),
]


# Pytest parametrize ids must be unique strings.
_ALL_CASES = (
    [(f"block-{name}", c, t, h) for name, c, t, h in _BLOCK_CASES]
    + [(f"element-{name}", c, t, h) for name, c, t, h in _ELEMENT_CASES]
    + [(f"object-{name}", c, t, h) for name, c, t, h in _OBJECT_CASES]
    + [(f"rich_text-{name}", c, t, h) for name, c, t, h in _RICH_TEXT_CASES]
    + [(f"surface-{name}", c, t, h) for name, c, t, h in _SURFACE_CASES]
    + [(f"attachment-{name}", c, t, h) for name, c, t, h in _ATTACHMENT_CASES]
)


@pytest.mark.parametrize(
    "name,constructor,expected_type,has_type_field",
    _ALL_CASES,
    ids=[case[0] for case in _ALL_CASES],
)
def test_resolves_and_serializes_to_json(
    name: str,
    constructor: Callable[[], Any],
    expected_type: str,
    has_type_field: bool,
) -> None:
    """Every public class:

    1. Constructs successfully with minimal-but-valid arguments.
    2. ``._resolve()`` returns a dict.
    3. ``json.dumps`` of that dict succeeds (the Phase 1 bug class).
    4. If applicable, the dict's ``type`` field matches the documented
       Slack value.
    """
    obj = constructor()
    _serializability_check(
        obj,
        expected_type=expected_type if expected_type else None,
        has_type_field=has_type_field,
    )


def test_message_json_method_round_trip_through_json_module() -> None:
    """Slightly stronger than the parametrized check: the public ``Message.json()``
    string must parse cleanly with ``json.loads``."""
    msg = Message(channel="#g", blocks=[SectionBlock("Hi")])
    parsed = json.loads(msg.json())
    assert parsed["channel"] == "#g"
    assert isinstance(parsed["blocks"], list)


def test_webhook_message_json_method_round_trip_through_json_module() -> None:
    msg = WebhookMessage(text="hi", blocks=[SectionBlock("Hi")])
    parsed = json.loads(msg.json())
    assert parsed["text"] == "hi"


def test_modal_view_json_method_round_trip_through_json_module() -> None:
    """``ModalView`` exposes ``__repr__`` (an indented JSON string) and
    ``to_dict()``; both must be parseable as JSON-equivalent."""
    view = ModalView(title="My modal", blocks=[SectionBlock("Hi")])
    # __repr__ returns the indented JSON.
    parsed = json.loads(repr(view))
    assert parsed["type"] == "modal"
    assert parsed["title"]["text"] == "My modal"
    # to_dict equals the parsed-then-loaded payload.
    assert view.to_dict() == parsed


def test_dunder_unpacking_through_keys_and_getitem() -> None:
    """``**msg`` unpacking is the documented integration point with the
    Slack SDKs. Verify it still works end-to-end by unpacking into a dict."""
    msg = Message(channel="#g", blocks=[SectionBlock("Hi")])
    unpacked = {**msg}
    assert "channel" in unpacked
    assert unpacked["channel"] == "#g"
