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
    InputParameter,
    Option,
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
    assert fetch_sample(path="test/samples/elements/button_link.json") == repr(
        link_button
    )


def test_button_style() -> None:
    style_button = Button(
        text="Load",
        style=ButtonStyle.PRIMARY,
        value="im_a_style_button",
        action_id="button",
    )
    assert fetch_sample(path="test/samples/elements/button_style.json") == repr(
        style_button
    )


def test_checkbox_basic() -> None:
    checkbox = CheckboxGroup(
        options=TWO_OPTIONS, action_id="and...action", initial_options=OPTION_A
    )
    assert fetch_sample(path="test/samples/elements/checkbox_basic.json") == repr(
        checkbox
    )


def test_datepicker_basic() -> None:
    datepicker = DatePicker(
        action_id="datepicker", initial_date="1970-01-01", placeholder="Pick a date"
    )
    assert fetch_sample(path="test/samples/elements/date_picker_basic.json") == repr(
        datepicker
    )


def test_datetime_picker_basic() -> None:
    datetime_picker = DateTimePicker(
        action_id="datetime_picker", initial_datetime=1628633830
    )
    assert fetch_sample(
        path="test/samples/elements/datetime_picker_basic.json"
    ) == repr(datetime_picker)


def test_email_input_basic() -> None:
    email_input = EmailInput(action_id="email_input", placeholder="Enter your email")
    assert fetch_sample(path="test/samples/elements/email_input_basic.json") == repr(
        email_input
    )


def test_image_basic() -> None:
    image = Image(image_url="https://ndl.im/img/logo.png", alt_text="Logo for ndl.im")
    assert fetch_sample(path="test/samples/elements/image_basic.json") == repr(image)


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
    assert fetch_sample(
        path="test/samples/elements/multi_select_conversation.json"
    ) == repr(multi_select_conversation)


def test_multi_select_external() -> None:
    multi_select_external = ExternalMultiSelectMenu(
        action_id="multi_external_select",
        placeholder=Text("Select items", type_=TextType.PLAINTEXT),
        min_query_length=3,
    )
    assert fetch_sample(
        path="test/samples/elements/multi_select_external.json"
    ) == repr(multi_select_external)


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
        multi_select_static = StaticMultiSelectMenu(
            action_id="multi_static_select",
            placeholder=Text("Select one or more", type_=TextType.PLAINTEXT),
            options=TWO_OPTIONS
            + [Option(text=Text("C", type_=TextType.MARKDOWN), value="X")],
        )


def test_multi_select_user() -> None:
    multi_select_user = UserMultiSelectMenu(
        action_id="multi_users_select",
        placeholder=Text("Select one or more users", type_=TextType.PLAINTEXT),
    )
    assert fetch_sample(path="test/samples/elements/multi_select_user.json") == repr(
        multi_select_user
    )


def test_number_input_basic() -> None:
    number_input = NumberInput(action_id="number_input", is_decimal_allowed=False)
    assert fetch_sample(path="test/samples/elements/number_input_basic.json") == repr(
        number_input
    )


def test_overflow_menu_basic() -> None:
    overflow_menu = OverflowMenu(options=THREE_OPTIONS, action_id="overflow")
    assert fetch_sample(path="test/samples/elements/overflow_menu_basic.json") == repr(
        overflow_menu
    )


def test_plaintext_input_menu_basic() -> None:
    plaintext_input = PlainTextInput(
        action_id="plaintext_input", placeholder="Enter your plain text"
    )
    assert fetch_sample(
        path="test/samples/elements/plaintext_input_basic.json"
    ) == repr(plaintext_input)


def test_radio_button_group_basic() -> None:
    radio_button_group = RadioButtonGroup(
        action_id="radio_buttons", initial_option=OPTION_A, options=THREE_OPTIONS
    )
    assert fetch_sample(
        path="test/samples/elements/radio_button_group_basic.json"
    ) == repr(radio_button_group)


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
    assert fetch_sample(
        path="test/samples/elements/select_menu_conversation.json"
    ) == repr(select_menu_conversation)


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
        select_menu_static = StaticSelectMenu(
            action_id="static_select",
            placeholder="Select one item",
            options=THREE_OPTIONS
            + [Option(text=Text("C", type_=TextType.MARKDOWN), value="X")],
        )


def test_select_menu_user() -> None:
    select_menu_user = UserSelectMenu(
        action_id="users_select", placeholder="Select one user"
    )
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
    assert fetch_sample(path="test/samples/elements/timepicker_basic.json") == repr(
        timepicker
    )


def test_url_input_basic() -> None:
    url_input = URLInput(action_id="url_text_input")
    assert fetch_sample(path="test/samples/elements/url_input_basic.json") == repr(
        url_input
    )


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
    assert fetch_sample(
        path="test/samples/elements/workflow_button_basic.json"
    ) == repr(workflow_button)


def test_rich_text_input_basic() -> None:
    assert fetch_sample(
        path="test/samples/elements/rich_text_input_basic.json"
    ) == repr(
        RichTextInput(
            action_id="action_id",
            initial_value=RichText("I'm rich"),
            focus_on_load=False,
            placeholder="Hello",
        )
    )
