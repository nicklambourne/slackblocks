import pytest

from slackblocks.elements import (
    Button,
    ButtonStyle,
    ChannelMultiSelectMenu,
    ChannelSelectMenu,
    CheckboxGroup,
    ConversationMultiSelectMenu,
    ConversationSelectMenu,
    DatePicker,
    DateTimePicker,
    EmailInput,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    Image,
    NumberInput,
    OverflowMenu,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    TimePicker,
    URLInput,
    UserMultiSelectMenu,
    UserSelectMenu,
    WorkflowButton,
)
from slackblocks.errors import InvalidUsageError
from slackblocks.objects import (
    ConfirmationDialogue,
    ConversationFilter,
    DispatchActionConfiguration,
    InputParameter,
    Option,
    OptionGroup,
    SlackFile,
    Text,
    TextType,
    Trigger,
    Workflow,
)
from slackblocks.rich_text import RichText

from .utils import OPTION_A, THREE_OPTIONS, TWO_OPTIONS, fetch_sample


def test_button_basic() -> None:
    button = Button(text="Click Me", value="click_me", action_id="button")
    assert fetch_sample(path="test/samples/elements/button_basic.json") == repr(button)


def test_button_link() -> None:
    link_button = Button(text="Link!", url="https://ndl.im/", action_id="button")
    assert fetch_sample(path="test/samples/elements/button_link.json") == repr(link_button)


def test_button_style() -> None:
    style_button = Button(
        text="Load",
        style=ButtonStyle.PRIMARY,
        value="im_a_style_button",
        action_id="button",
    )
    assert fetch_sample(path="test/samples/elements/button_style.json") == repr(style_button)


def test_checkbox_basic() -> None:
    checkbox = CheckboxGroup(
        options=TWO_OPTIONS, action_id="and...action", initial_options=OPTION_A
    )
    assert fetch_sample(path="test/samples/elements/checkbox_basic.json") == repr(checkbox)


def test_datepicker_basic() -> None:
    datepicker = DatePicker(
        action_id="datepicker", initial_date="1970-01-01", placeholder="Pick a date"
    )
    assert fetch_sample(path="test/samples/elements/date_picker_basic.json") == repr(datepicker)


def test_datepicker_without_initial_date() -> None:
    """Regression test for #135: DatePicker without ``initial_date`` must not
    raise AttributeError on ``_resolve``."""
    from json import dumps

    datepicker = DatePicker(action_id="datepicker")
    resolved = datepicker._resolve()
    dumps(resolved)  # Must not raise.
    assert "initial_date" not in resolved


def test_datepicker_with_confirm_resolves() -> None:
    """Regression test for #135: DatePicker must call ``_resolve()`` on its
    nested ``confirm`` so the result is JSON-serializable."""
    from json import dumps

    datepicker = DatePicker(
        action_id="datepicker",
        confirm=ConfirmationDialogue(
            title=Text("Sure?", type_=TextType.PLAINTEXT),
            text=Text("Pick this date?", type_=TextType.PLAINTEXT),
            confirm=Text("Yes", type_=TextType.PLAINTEXT),
            deny=Text("No", type_=TextType.PLAINTEXT),
        ),
    )
    resolved = datepicker._resolve()
    dumps(resolved)
    assert resolved["confirm"]["title"]["text"] == "Sure?"


def test_datetime_picker_basic() -> None:
    datetime_picker = DateTimePicker(action_id="datetime_picker", initial_datetime=1628633830)
    assert fetch_sample(path="test/samples/elements/datetime_picker_basic.json") == repr(
        datetime_picker
    )


def test_datetime_picker_without_initial_datetime() -> None:
    """Regression test for #136: DateTimePicker without ``initial_datetime``
    must not raise AttributeError on ``_resolve``."""
    from json import dumps

    datetime_picker = DateTimePicker(action_id="datetime_picker")
    resolved = datetime_picker._resolve()
    dumps(resolved)
    assert "initial_date_time" not in resolved


def test_datetime_picker_with_confirm_resolves() -> None:
    """Regression test for #136: DateTimePicker must call ``_resolve()`` on
    its nested ``confirm`` so the result is JSON-serializable."""
    from json import dumps

    datetime_picker = DateTimePicker(
        action_id="datetime_picker",
        confirm=ConfirmationDialogue(
            title=Text("Sure?", type_=TextType.PLAINTEXT),
            text=Text("Pick this datetime?", type_=TextType.PLAINTEXT),
            confirm=Text("Yes", type_=TextType.PLAINTEXT),
            deny=Text("No", type_=TextType.PLAINTEXT),
        ),
    )
    resolved = datetime_picker._resolve()
    dumps(resolved)
    assert resolved["confirm"]["title"]["text"] == "Sure?"


def test_email_input_basic() -> None:
    email_input = EmailInput(action_id="email_input", placeholder="Enter your email")
    assert fetch_sample(path="test/samples/elements/email_input_basic.json") == repr(email_input)


def test_image_basic() -> None:
    image = Image(image_url="https://ndl.im/img/logo.png", alt_text="Logo for ndl.im")
    assert fetch_sample(path="test/samples/elements/image_basic.json") == repr(image)


def test_image_with_slack_file_resolves() -> None:
    """Regression test for #130: Image element must call ``_resolve()`` on
    its nested ``slack_file`` so the result is JSON-serializable."""
    from json import dumps

    image = Image(
        alt_text="alt",
        slack_file=SlackFile(url="https://example.com/img.png", id=None),
    )
    resolved = image._resolve()
    # Should not raise.
    dumps(resolved)
    assert resolved["slack_file"] == {"url": "https://example.com/img.png"}


def test_multi_select_channel() -> None:
    multi_select_channel = ChannelMultiSelectMenu(
        action_id="multi_channels_select",
        placeholder=Text("Select channels", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(path="test/samples/elements/multi_select_channel.json") == repr(
        multi_select_channel
    )


def test_multi_select_conversation() -> None:
    multi_select_conversation = ConversationMultiSelectMenu(
        action_id="multi_conversations_select",
        placeholder=Text("Select conversations", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(path="test/samples/elements/multi_select_conversation.json") == repr(
        multi_select_conversation
    )


def test_multi_select_conversation_initial_conversations_key() -> None:
    """Regression test for #144: ConversationMultiSelectMenu must emit the
    correctly-spelled ``initial_conversations`` JSON key (Slack silently
    ignores the previous typo'd ``intial_conversations``)."""
    multi_select = ConversationMultiSelectMenu(
        action_id="multi_conversations_select",
        initial_conversations=["C123", "C456"],
    )
    resolved = multi_select._resolve()
    assert "initial_conversations" in resolved
    assert resolved["initial_conversations"] == ["C123", "C456"]
    assert "intial_conversations" not in resolved


def test_multi_select_external() -> None:
    multi_select_external = ExternalMultiSelectMenu(
        action_id="multi_external_select",
        placeholder=Text("Select items", type_=TextType.PLAINTEXT),
        min_query_length=3,
    )
    assert fetch_sample(path="test/samples/elements/multi_select_external.json") == repr(
        multi_select_external
    )


def test_multi_select_static() -> None:
    multi_select_static = StaticMultiSelectMenu(
        action_id="multi_static_select",
        placeholder=Text("Select one or more", type_=TextType.PLAINTEXT),
        options=TWO_OPTIONS,
    )
    assert fetch_sample(path="test/samples/elements/multi_select_static.json") == repr(
        multi_select_static
    )


def test_multi_select_static_invalid_option() -> None:
    with pytest.raises(InvalidUsageError):
        StaticMultiSelectMenu(
            action_id="multi_static_select",
            placeholder=Text("Select one or more", type_=TextType.PLAINTEXT),
            options=TWO_OPTIONS + [Option(text=Text("C", type_=TextType.MARKDOWN), value="X")],
        )


def test_static_select_menu_without_options_or_groups_does_not_raise() -> None:
    """Regression test for #150: StaticSelectMenu must not raise
    UnboundLocalError when neither ``options`` nor ``option_groups`` is set."""
    menu = StaticSelectMenu(action_id="action", placeholder="Select")
    resolved = menu._resolve()
    assert resolved["type"] == "static_select"


def test_static_multi_select_menu_without_options_or_groups_does_not_raise() -> None:
    """Regression test for #150: StaticMultiSelectMenu must not raise
    UnboundLocalError when ``options`` is None and ``option_groups`` is unset."""
    menu = StaticMultiSelectMenu(action_id="action", placeholder="Select", options=None)
    resolved = menu._resolve()
    assert resolved["type"] == "multi_static_select"


def test_static_multi_select_with_option_groups_validates_text() -> None:
    """Regression test for #148: option_groups flattening must continue to
    visit every option for plaintext validation after the migration from
    sum([list], []) to itertools.chain."""
    valid = StaticMultiSelectMenu(
        action_id="multi_static_select",
        placeholder="Pick",
        options=None,
        option_groups=[
            OptionGroup(label="Group 1", options=TWO_OPTIONS),
            OptionGroup(label="Group 2", options=TWO_OPTIONS),
        ],
    )
    assert "option_groups" in valid._resolve()
    # Markdown-typed option in a later group still raises.
    with pytest.raises(InvalidUsageError):
        StaticMultiSelectMenu(
            action_id="multi_static_select",
            placeholder="Pick",
            options=None,
            option_groups=[
                OptionGroup(label="Group 1", options=TWO_OPTIONS),
                OptionGroup(
                    label="Group 2",
                    options=[Option(text=Text("Bad", type_=TextType.MARKDOWN), value="bad")],
                ),
            ],
        )


def test_multi_select_user() -> None:
    multi_select_user = UserMultiSelectMenu(
        action_id="multi_users_select",
        placeholder=Text("Select one or more users", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(path="test/samples/elements/multi_select_user.json") == repr(
        multi_select_user
    )


def test_multi_select_user_with_initial_users() -> None:
    multi_select_user = UserMultiSelectMenu(
        action_id="multi_users_select",
        placeholder=Text("Select one or more users", type_=TextType.PLAINTEXT),
        initial_users=["U064B5H1309", "U063JR973UP"],
    )
    assert fetch_sample(
        path="test/samples/elements/multi_select_user_with_initial_users.json"
    ) == repr(multi_select_user)


def test_number_input_basic() -> None:
    number_input = NumberInput(action_id="number_input", is_decimal_allowed=False)
    assert fetch_sample(path="test/samples/elements/number_input_basic.json") == repr(number_input)


def test_number_input_zero_min_value_emitted() -> None:
    """Regression test for #146: NumberInput must emit ``min_value=0`` and
    ``max_value=0`` (legitimate constraints), not silently drop them."""
    number_input = NumberInput(
        is_decimal_allowed=False,
        action_id="number_input",
        min_value=0,
        max_value=0,
    )
    resolved = number_input._resolve()
    assert resolved["min_value"] == 0
    assert resolved["max_value"] == 0


def test_number_input_min_greater_than_max_error_message() -> None:
    """Regression test for #146: cross-validation error message must include
    both min_value and max_value (previously interpolated min_value twice)."""
    with pytest.raises(InvalidUsageError) as excinfo:
        NumberInput(
            is_decimal_allowed=False,
            action_id="number_input",
            min_value=10,
            max_value=5,
        )
    msg = str(excinfo.value)
    assert "10" in msg and "5" in msg


def test_overflow_menu_basic() -> None:
    overflow_menu = OverflowMenu(options=THREE_OPTIONS, action_id="overflow")
    assert fetch_sample(path="test/samples/elements/overflow_menu_basic.json") == repr(
        overflow_menu
    )


def test_plaintext_input_menu_basic() -> None:
    plaintext_input = PlainTextInput(
        action_id="plaintext_input", placeholder="Enter your plain text"
    )
    assert fetch_sample(path="test/samples/elements/plaintext_input_basic.json") == repr(
        plaintext_input
    )


def test_radio_button_group_basic() -> None:
    radio_button_group = RadioButtonGroup(
        action_id="radio_buttons", initial_option=OPTION_A, options=THREE_OPTIONS
    )
    assert fetch_sample(path="test/samples/elements/radio_button_group_basic.json") == repr(
        radio_button_group
    )


def test_select_menu_channel() -> None:
    select_menu_channel = ChannelSelectMenu(
        action_id="channels_select", placeholder="Select a channel"
    )
    assert fetch_sample(path="test/samples/elements/select_menu_channel.json") == repr(
        select_menu_channel
    )


def test_select_menu_conversation() -> None:
    select_menu_conversation = ConversationSelectMenu(
        action_id="conversations_select", placeholder="Select one conversation"
    )
    assert fetch_sample(path="test/samples/elements/select_menu_conversation.json") == repr(
        select_menu_conversation
    )


def test_conversation_select_menu_with_filter_resolves() -> None:
    """Regression test for #140: ConversationSelectMenu must call
    ``_resolve()`` on its nested ``filter`` so the result is JSON-serializable."""
    from json import dumps

    select_menu = ConversationSelectMenu(
        action_id="conversations_select",
        placeholder="Pick a conversation",
        filter=ConversationFilter(include=["public"], exclude_bot_users=True),
    )
    resolved = select_menu._resolve()
    dumps(resolved)
    assert resolved["filter"] == {
        "include": ["public"],
        "exclude_bot_users": True,
    }


def test_select_menu_external() -> None:
    select_menu_external = ExternalSelectMenu(
        action_id="external_select", placeholder="Select one item", min_query_length=4
    )
    assert fetch_sample(path="test/samples/elements/select_menu_external.json") == repr(
        select_menu_external
    )


def test_select_menu_static() -> None:
    select_menu_static = StaticSelectMenu(
        action_id="static_select", placeholder="Select one item", options=THREE_OPTIONS
    )
    assert fetch_sample(path="test/samples/elements/select_menu_static.json") == repr(
        select_menu_static
    )


def test_select_menu_static_invalid_option() -> None:
    with pytest.raises(InvalidUsageError):
        StaticSelectMenu(
            action_id="static_select",
            placeholder="Select one item",
            options=THREE_OPTIONS + [Option(text=Text("C", type_=TextType.MARKDOWN), value="X")],
        )


def test_select_menu_user() -> None:
    select_menu_user = UserSelectMenu(action_id="users_select", placeholder="Select one user")
    assert fetch_sample(path="test/samples/elements/select_menu_user.json") == repr(
        select_menu_user
    )


def test_timepicker_basic() -> None:
    timepicker = TimePicker(
        timezone="Australia/Sydney",
        action_id="timepicker",
        initial_time="12:00",
        placeholder="Select your time",
    )
    assert fetch_sample(path="test/samples/elements/timepicker_basic.json") == repr(timepicker)


def test_timepicker_with_confirm_resolves() -> None:
    """Regression test for #134: TimePicker must call ``_resolve()`` on its
    nested ``confirm`` so the result is JSON-serializable."""
    from json import dumps

    timepicker = TimePicker(
        action_id="timepicker",
        confirm=ConfirmationDialogue(
            title=Text("Sure?", type_=TextType.PLAINTEXT),
            text=Text("Pick a time?", type_=TextType.PLAINTEXT),
            confirm=Text("Yes", type_=TextType.PLAINTEXT),
            deny=Text("No", type_=TextType.PLAINTEXT),
        ),
    )
    resolved = timepicker._resolve()
    dumps(resolved)  # Must not raise.
    assert resolved["confirm"]["title"]["text"] == "Sure?"


def test_url_input_basic() -> None:
    url_input = URLInput(action_id="url_text_input")
    assert fetch_sample(path="test/samples/elements/url_input_basic.json") == repr(url_input)


def test_url_input_with_placeholder_resolves() -> None:
    """Regression test for #132: URLInput must call ``_resolve()`` on its
    nested ``placeholder`` so the result is JSON-serializable."""
    from json import dumps

    url_input = URLInput(action_id="url_text_input", placeholder="Enter a URL")
    resolved = url_input._resolve()
    dumps(resolved)  # Must not raise.
    assert resolved["placeholder"] == {"type": "plain_text", "text": "Enter a URL"}


def test_workflow_button_basic() -> None:
    workflow_button = WorkflowButton(
        text=Text("Run Your Workflow", type_=TextType.PLAINTEXT),
        workflow=Workflow(
            trigger=Trigger(
                url="https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                customizable_input_parameters=[
                    InputParameter(
                        name="name_a",
                        value="value_a",
                    ),
                    InputParameter(
                        name="name_b",
                        value="value_b",
                    ),
                ],
            )
        ),
    )
    assert fetch_sample(path="test/samples/elements/workflow_button_basic.json") == repr(
        workflow_button
    )


def test_rich_text_input_basic() -> None:
    assert fetch_sample(path="test/samples/elements/rich_text_input_basic.json") == repr(
        RichTextInput(
            action_id="action_id",
            initial_value=RichText("I'm rich"),
            focus_on_load=False,
            placeholder="Hello",
        )
    )


def test_rich_text_input_dispatch_action_config_resolves() -> None:
    """Regression test for #142: RichTextInput must call ``_resolve()`` on
    its nested ``dispatch_action_config`` and coerce a ``placeholder`` string
    to a ``Text`` object so the result is JSON-serializable."""
    from json import dumps

    rich_text_input = RichTextInput(
        action_id="rt",
        placeholder="Type something",
        dispatch_action_config=DispatchActionConfiguration(trigger_actions_on=["on_enter_pressed"]),
    )
    resolved = rich_text_input._resolve()
    dumps(resolved)
    assert resolved["dispatch_action_config"] == {"trigger_actions_on": ["on_enter_pressed"]}
    assert resolved["placeholder"] == {"type": "plain_text", "text": "Type something"}
