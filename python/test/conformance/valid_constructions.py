"""Registry mapping every valid-fixture ID in ``spec/manifest.json`` to a
construction of that fixture through the public ``slackblocks`` API.

The conformance suite parametrises over the manifest and asserts that the
JSON rendered by each construction here is semantically identical to the
fixture on disk. A manifest entry without a registered construction fails
the suite, as does a construction that renders different JSON.

The constructions are ported from the unit tests that originally pinned each
fixture (see ``test/unit`` and ``test/integration``).
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from slackblocks import (
    ActionsBlock,
    AlertBlock,
    AreaChart,
    Attachment,
    AxisConfig,
    BarChart,
    Button,
    CardBlock,
    CarouselBlock,
    ChannelMultiSelectMenu,
    ChannelSelectMenu,
    ChartSegment,
    CheckboxGroup,
    Color,
    ColumnSettings,
    ConfirmationDialogue,
    ContainerBlock,
    ContextActionsBlock,
    ContextBlock,
    ConversationFilter,
    ConversationMultiSelectMenu,
    ConversationSelectMenu,
    DataPoint,
    DataSeries,
    DataTableBlock,
    DataVisualizationBlock,
    DatePicker,
    DateTimePicker,
    DispatchActionConfiguration,
    DividerBlock,
    EmailInput,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    FeedbackButton,
    FeedbackButtons,
    FileBlock,
    FileInput,
    HeaderBlock,
    HomeTabView,
    IconButton,
    Image,
    ImageBlock,
    InputBlock,
    InputParameter,
    LineChart,
    MarkdownBlock,
    Message,
    MessageResponse,
    Modal,
    NumberInput,
    Option,
    OptionGroup,
    OverflowMenu,
    PieChart,
    PlainTextInput,
    PlanBlock,
    RadioButtonGroup,
    RawNumber,
    RawText,
    ResponseType,
    RichTextBlock,
    RichTextInput,
    SectionBlock,
    SlackFile,
    SlackIcon,
    StaticMultiSelectMenu,
    StaticSelectMenu,
    TableBlock,
    TaskCardBlock,
    Text,
    TextType,
    TimePicker,
    Trigger,
    URLInput,
    URLSource,
    UserMultiSelectMenu,
    UserSelectMenu,
    VideoBlock,
    WebhookMessage,
    Workflow,
    WorkflowButton,
)
from slackblocks.elements import ButtonStyle
from slackblocks.rich_text import (
    ListType,
    RichText,
    RichTextChannel,
    RichTextCodeBlock,
    RichTextEmoji,
    RichTextLink,
    RichTextList,
    RichTextQuote,
    RichTextSection,
    RichTextUser,
    RichTextUserGroup,
)

if TYPE_CHECKING:
    from collections.abc import Callable


def _option(label: str) -> Option:
    return Option(text=Text(label, type_=TextType.PLAINTEXT), value=label)


def _two_options() -> list[Option]:
    return [_option("A"), _option("B")]


def _three_options() -> list[Option]:
    return [_option("A"), _option("B"), _option("C")]


def _input_parameters() -> list[InputParameter]:
    return [InputParameter(name="A", value="A"), InputParameter(name="B", value="B")]


def _trigger() -> Trigger:
    return Trigger(
        url="https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
        customizable_input_parameters=_input_parameters(),
    )


def _task_output(text: str, block_id: str) -> RichTextBlock:
    return RichTextBlock(RichTextSection(RichText(text)), block_id=block_id)


def _radio_button_group() -> RadioButtonGroup:
    options = _three_options()
    return RadioButtonGroup(action_id="radio_buttons", initial_option=options[0], options=options)


def _compound_message() -> Message:
    block_1 = SectionBlock("Block, One", block_id="fake_block1")
    block_2 = SectionBlock("Block, Two", block_id="fake_block2")
    block_3 = ImageBlock(
        title=" ",
        image_url="http://bit.ly/slack-block-test-image",
        alt_text="crash",
        block_id="fake_block3",
    )
    return Message(
        channel="#slackblocks",
        blocks=[block_1, block_3],
        attachments=[
            Attachment(blocks=block_1, color=Color.PURPLE),
            Attachment(blocks=[block_2, block_3], color=Color.YELLOW),
        ],
    )


def _rich_text_list(style: ListType, **arguments: int) -> Callable[[], RichTextList]:
    words = ["Oh", "Hi", "Mark"] if style is ListType.BULLET else ["Oh", "Hi"]
    return lambda: RichTextList(
        elements=[RichTextSection(elements=[RichText(text=word)]) for word in words],
        style=style,
        **arguments,
    )


CONSTRUCTIONS: dict[str, Callable[[], object]] = {
    "attachments/attachment_multi_block": lambda: Attachment(
        blocks=[
            SectionBlock("I like pretty colours", block_id="fake_block_id_0"),
            SectionBlock("I don't like pretty colours", block_id="fake_block_id_1"),
        ],
        color=Color.PURPLE,
    ),
    "attachments/attachment_simple": lambda: Attachment(
        blocks=SectionBlock("I like pretty colours", block_id="fake_block_id"),
        color=Color.BLACK,
        fallback="Colours preference",
    ),
    "blocks/actions_block_checkboxes": lambda: ActionsBlock(
        block_id="fake_block_id",
        elements=CheckboxGroup(
            action_id="actionId-0",
            options=[
                Option(text="*a*", value="a", description="*a*"),
                Option(text="*b*", value="b", description="*b*"),
                Option(text="*c*", value="c", description="*c*"),
            ],
        ),
    ),
    "blocks/alert_block": lambda: AlertBlock(
        "The work is mysterious and important.",
        level="info",
        block_id="fake_block_id",
    ),
    "blocks/card_block": lambda: CardBlock(
        block_id="fake_block_id",
        hero_image=Image(
            image_url="https://picsum.photos/400/300",
            alt_text="Sample hero image",
        ),
        title="Lumon Industries",
        subtitle="Committed to work-life balance",
        body="Please enjoy each card equally.",
        actions=Button(text="Action Button", action_id="button_action"),
        slack_icon=SlackIcon("bot"),
        subtext="A card assembled by slackblocks.",
    ),
    "blocks/carousel_block": lambda: CarouselBlock(
        block_id="fake_block_id",
        elements=[
            CardBlock(title="First result", block_id="card_1"),
            CardBlock(title="Second result", block_id="card_2"),
        ],
    ),
    "blocks/container_block": lambda: ContainerBlock(
        block_id="fake_block_id",
        title="Deployment summary",
        subtitle="Production is healthy",
        child_blocks=[SectionBlock("All systems operational.", block_id="child_1")],
        has_header_divider=True,
    ),
    "blocks/context_actions_feedback_buttons": lambda: ContextActionsBlock(
        block_id="fake_block_id",
        elements=[
            FeedbackButtons(
                action_id="feedback_buttons_1",
                positive_button=FeedbackButton(
                    "Good",
                    "positive_feedback",
                    "Mark this response as good",
                ),
                negative_button=FeedbackButton(
                    "Bad",
                    "negative_feedback",
                    "Mark this response as bad",
                ),
            )
        ],
    ),
    "blocks/context_actions_icon_button": lambda: ContextActionsBlock(
        block_id="fake_block_id",
        elements=[
            IconButton(
                text="Delete",
                action_id="delete_button",
                value="delete_item",
            )
        ],
    ),
    "blocks/context_block_text_only": lambda: ContextBlock(
        elements=[Text("Hello, world!")], block_id="fake_block_id"
    ),
    "blocks/data_table_block": lambda: DataTableBlock(
        block_id="fake_block_id",
        rows=[
            [RawText("Name"), RawText("Score")],
            [RawText("Alice"), RawNumber(42, "42")],
        ],
        caption="Team scores",
    ),
    "blocks/data_visualization_area": lambda: DataVisualizationBlock(
        block_id="fake_block_id",
        title="Daily Active Users",
        chart=AreaChart(
            [
                DataSeries(
                    "Free Tier",
                    [DataPoint("Mon", 12000), DataPoint("Tue", 13500)],
                )
            ],
            AxisConfig(["Mon", "Tue"], x_label="Day", y_label="Users"),
        ),
    ),
    "blocks/data_visualization_bar": lambda: DataVisualizationBlock(
        block_id="fake_block_id",
        title="Pies by Tastiness",
        chart=BarChart(
            [
                DataSeries(
                    "Pies",
                    [DataPoint("Pumpkin", 70), DataPoint("Blueberry", 90)],
                )
            ],
            AxisConfig(
                ["Pumpkin", "Blueberry"],
                x_label="Pies",
                y_label="Tastiness",
            ),
        ),
    ),
    "blocks/data_visualization_line": lambda: DataVisualizationBlock(
        block_id="fake_block_id",
        title="Weekly Paper Sales",
        chart=LineChart(
            [
                DataSeries(
                    "Website",
                    [DataPoint("Week 1", 32000), DataPoint("Week 2", 35000)],
                ),
                DataSeries(
                    "In-store",
                    [DataPoint("Week 1", 28000), DataPoint("Week 2", 31000)],
                ),
            ],
            AxisConfig(
                ["Week 1", "Week 2"],
                x_label="Week",
                y_label="Paper Sales (USD)",
            ),
        ),
    ),
    "blocks/data_visualization_pie": lambda: DataVisualizationBlock(
        block_id="fake_block_id",
        title="My Favorite Candy Bars",
        chart=PieChart(
            [
                ChartSegment("Kit Kat", 45),
                ChartSegment("Twix", 28),
                ChartSegment("Crunch", 18),
                ChartSegment("Milky Way", 9),
            ]
        ),
    ),
    "blocks/divider_block_only": lambda: DividerBlock(block_id="fake_block_id"),
    "blocks/file_block_only": lambda: FileBlock(
        external_id="external_id", block_id="fake_block_id"
    ),
    "blocks/header_block_emoji_at_limit": lambda: HeaderBlock(
        text="\U0001f600" * 150, block_id="fake_block_id"
    ),
    "blocks/header_block_only": lambda: HeaderBlock(text="AloHa!", block_id="fake_block_id"),
    "blocks/header_block_text_at_limit": lambda: HeaderBlock(
        text="x" * 150, block_id="fake_block_id"
    ),
    "blocks/image_block_only": lambda: ImageBlock(
        image_url="https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
        alt_text="image1",
        title="image1",
        block_id="fake_block_id",
    ),
    "blocks/input_block_only": lambda: InputBlock(
        label=Text("Label", type_=TextType.PLAINTEXT, emoji=True),
        hint=Text("Hint", type_=TextType.PLAINTEXT, emoji=True),
        element=PlainTextInput(action_id="action"),
        block_id="fake_block_id",
        optional=True,
    ),
    "blocks/markdown_block_basic": lambda: MarkdownBlock(
        text="**Hello**, _world_!", block_id="fake_block_id"
    ),
    "blocks/plan_block": lambda: PlanBlock(
        block_id="fake_block_id",
        title="Thinking completed",
        tasks=[
            TaskCardBlock(
                task_id="call_001",
                title="Fetched user profile information",
                status="complete",
                output=_task_output("Profile data loaded", "plan_output"),
            ),
            TaskCardBlock(
                task_id="call_002",
                title="Checked user permissions",
                status="pending",
            ),
        ],
    ),
    "blocks/rich_text_block_basic": lambda: RichTextBlock(
        RichTextSection(
            [
                RichText(
                    "You 'bout to witness hip-hop in its most purest",
                    bold=True,
                ),
                RichText(
                    "Most rawest form, flow almost flawless",
                    strike=True,
                ),
                RichText(
                    "Most hardest, most honest known artist",
                    italic=True,
                ),
            ]
        ),
        block_id="fake_block_id",
    ),
    "blocks/section_block_both_text_and_fields": lambda: SectionBlock(
        text="Hello",
        block_id="fake_block_id",
        fields=[
            Text("Are you", type_=TextType.MARKDOWN),
            Text("There?", type_=TextType.PLAINTEXT, emoji=True),
        ],
    ),
    "blocks/section_block_empty_text_field_value": lambda: SectionBlock(
        block_id="fake_block_id",
        fields=[
            Text("Highly", type_=TextType.MARKDOWN),
            Text("Strung", type_=TextType.PLAINTEXT, emoji=True),
        ],
    ),
    "blocks/section_block_fields": lambda: SectionBlock(
        "Test:",
        fields=[Text(text="foo", type_=TextType.PLAINTEXT), Text(text="bar")],
        block_id="fake_block_id",
    ),
    "blocks/section_block_single_field_value_coercion": lambda: SectionBlock(
        block_id="fake_block_id",
        fields="Lowly",
    ),
    "blocks/section_block_text_only": lambda: SectionBlock(
        "Hello, world!", block_id="fake_block_id"
    ),
    "blocks/table_block": lambda: TableBlock(
        block_id="fake_block_id",
        column_settings=[
            ColumnSettings(is_wrapped=True),
            ColumnSettings(align="right"),
        ],
        rows=[
            [
                RawText(text="Header A"),
                RawText(text="Header B"),
            ],
            [
                RawText(text="Data 1A"),
                RichTextSection(elements=RichTextLink(url="https://slack.com", text="Data 1B")),
            ],
            [
                RawText(text="Data 2A"),
                RichTextSection(elements=RichTextLink(url="https://slack.com", text="Data 2B")),
            ],
        ],
    ),
    "blocks/task_card_block": lambda: TaskCardBlock(
        block_id="fake_block_id",
        task_id="task_1",
        title="Fetching weather data",
        output=_task_output("Found weather data for Chicago from 2 sources", "task_output"),
        sources=[
            URLSource("https://weather.com/", "weather.com"),
            URLSource("https://www.accuweather.com/", "accuweather.com"),
        ],
        status="pending",
    ),
    "blocks/video_block_basic": lambda: VideoBlock(
        alt_text="alt",
        thumbnail_url="https://example.com/t.png",
        title="Title",
        video_url="https://example.com/v.mp4",
        block_id="b1",
    ),
    "blocks/video_block_full": lambda: VideoBlock(
        alt_text="How to use Slack",
        thumbnail_url="https://example.com/thumb.png",
        title="Getting Started",
        video_url="https://example.com/video.mp4",
        author_name="Slack",
        description="A short intro",
        provider_icon_url="https://example.com/icon.png",
        provider_name="YouTube",
        title_url="https://example.com",
        block_id="video_1",
    ),
    "elements/button_basic": lambda: Button(text="Click Me", value="click_me", action_id="button"),
    "elements/button_link": lambda: Button(text="Link!", url="https://ndl.im/", action_id="button"),
    "elements/button_style": lambda: Button(
        text="Load",
        style=ButtonStyle.PRIMARY,
        value="im_a_style_button",
        action_id="button",
    ),
    "elements/button_text_at_limit": lambda: Button(text="x" * 75, action_id="button"),
    "elements/checkbox_basic": lambda: CheckboxGroup(
        options=_two_options(), action_id="and...action", initial_options=_option("A")
    ),
    "elements/date_picker_basic": lambda: DatePicker(
        action_id="datepicker", initial_date="1970-01-01", placeholder="Pick a date"
    ),
    "elements/datetime_picker_basic": lambda: DateTimePicker(
        action_id="datetime_picker", initial_datetime=1628633830
    ),
    "elements/email_input_basic": lambda: EmailInput(
        action_id="email_input", placeholder="Enter your email"
    ),
    "elements/file_input_basic": lambda: FileInput(
        action_id="file_input_action_id_1",
        filetypes=["jpg", "png"],
        max_files=5,
    ),
    "elements/image_basic": lambda: Image(
        image_url="https://ndl.im/img/logo.png", alt_text="Logo for ndl.im"
    ),
    "elements/image_slack_file_id": lambda: Image(
        alt_text="An incredibly cute kitten.",
        slack_file=SlackFile(url=None, id="F0123456"),
    ),
    "elements/image_slack_file_url": lambda: Image(
        alt_text="An incredibly cute kitten.",
        slack_file=SlackFile(
            url="https://files.slack.com/files-pri/T0123456-F0123456/xyz.png",
            id=None,
        ),
    ),
    "elements/multi_select_channel": lambda: ChannelMultiSelectMenu(
        action_id="multi_channels_select",
        placeholder=Text("Select channels", type_=TextType.PLAINTEXT),
    ),
    "elements/multi_select_conversation": lambda: ConversationMultiSelectMenu(
        action_id="multi_conversations_select",
        placeholder=Text("Select conversations", type_=TextType.PLAINTEXT),
    ),
    "elements/multi_select_external": lambda: ExternalMultiSelectMenu(
        action_id="multi_external_select",
        placeholder=Text("Select items", type_=TextType.PLAINTEXT),
        min_query_length=3,
    ),
    "elements/multi_select_static": lambda: StaticMultiSelectMenu(
        action_id="multi_static_select",
        placeholder=Text("Select one or more", type_=TextType.PLAINTEXT),
        options=_two_options(),
    ),
    "elements/multi_select_user": lambda: UserMultiSelectMenu(
        action_id="multi_users_select",
        placeholder=Text("Select one or more users", type_=TextType.PLAINTEXT),
    ),
    "elements/multi_select_user_with_initial_users": lambda: UserMultiSelectMenu(
        action_id="multi_users_select",
        placeholder=Text("Select one or more users", type_=TextType.PLAINTEXT),
        initial_users=["U064B5H1309", "U063JR973UP"],
    ),
    "elements/number_input_basic": lambda: NumberInput(
        action_id="number_input", is_decimal_allowed=False
    ),
    "elements/overflow_menu_basic": lambda: OverflowMenu(
        options=_three_options(), action_id="overflow"
    ),
    "elements/plaintext_input_basic": lambda: PlainTextInput(
        action_id="plaintext_input", placeholder="Enter your plain text"
    ),
    "elements/radio_button_group_basic": _radio_button_group,
    "elements/rich_text_input_basic": lambda: RichTextInput(
        action_id="action_id",
        initial_value=RichText("I'm rich"),
        focus_on_load=False,
        placeholder="Hello",
    ),
    "elements/select_menu_channel": lambda: ChannelSelectMenu(
        action_id="channels_select", placeholder="Select a channel"
    ),
    "elements/select_menu_conversation": lambda: ConversationSelectMenu(
        action_id="conversations_select", placeholder="Select one conversation"
    ),
    "elements/select_menu_external": lambda: ExternalSelectMenu(
        action_id="external_select", placeholder="Select one item", min_query_length=4
    ),
    "elements/select_menu_static": lambda: StaticSelectMenu(
        action_id="static_select", placeholder="Select one item", options=_three_options()
    ),
    "elements/select_menu_user": lambda: UserSelectMenu(
        action_id="users_select", placeholder="Select one user"
    ),
    "elements/timepicker_basic": lambda: TimePicker(
        timezone="Australia/Sydney",
        action_id="timepicker",
        initial_time="12:00",
        placeholder="Select your time",
    ),
    "elements/url_input_basic": lambda: URLInput(action_id="url_text_input"),
    "elements/url_source_basic": lambda: URLSource("https://docs.slack.dev/", "Slack API docs"),
    "elements/workflow_button_basic": lambda: WorkflowButton(
        text=Text("Run Your Workflow", type_=TextType.PLAINTEXT),
        workflow=Workflow(
            trigger=Trigger(
                url="https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                customizable_input_parameters=[
                    InputParameter(name="name_a", value="value_a"),
                    InputParameter(name="name_b", value="value_b"),
                ],
            )
        ),
    ),
    "messages/message_basic": lambda: Message(
        channel="#slackblocks",
        blocks=SectionBlock("Hello, world!", block_id="fake_block_id"),
    ),
    "messages/message_basic_attachment": lambda: Message(
        channel="#slackblocks",
        attachments=[
            Attachment(
                blocks=SectionBlock("Hello, world!", block_id="block1"),
                color=Color.BLACK,
            )
        ],
    ),
    "messages/message_compound": _compound_message,
    "messages/message_response": lambda: MessageResponse(
        blocks=SectionBlock("Hello, world!", block_id="fake_block_id"), ephemeral=True
    ),
    "messages/message_with_attachments": lambda: Message(
        channel="#slackblocks",
        attachments=[
            Attachment(
                blocks=SectionBlock("Hello, world!", block_id="fake_block_id"),
                color=Color.YELLOW,
            )
        ],
    ),
    "messages/message_with_optional_arguments": lambda: Message(
        channel="#slackblocks",
        blocks=SectionBlock("Hello, world!", block_id="fake_block_id"),
        unfurl_links=False,
        unfurl_media=False,
    ),
    "messages/webhook_message_basic": lambda: WebhookMessage(
        blocks=[
            SectionBlock(
                Text("You wouldn't do ol' Hook in now, would you, lad?"),
                block_id="fake_block_id",
            ),
            SectionBlock(
                Text("Well, all right... if you... say you're a codfish."),
                block_id="fake_block_id",
            ),
        ],
        response_type=ResponseType.EPHEMERAL,
        replace_original=True,
        unfurl_links=False,
        unfurl_media=False,
        metadata={"sender": "Walt"},
    ),
    "messages/webhook_message_delete": lambda: WebhookMessage(
        attachments=[
            Attachment(blocks=[SectionBlock(Text("I'M A CODFISH!"), block_id="fake_block_id")])
        ],
        blocks=[
            SectionBlock(Text("I'm a codfish."), block_id="fake_block_id"),
            SectionBlock(Text("Louder!"), block_id="fake_block_id"),
        ],
        response_type="in_channel",
        delete_original=True,
        unfurl_links=True,
        unfurl_media=True,
        metadata={"sender": "Walt"},
    ),
    "objects/confirmation_dialogue_basic": lambda: ConfirmationDialogue(
        title=Text("Maybe?", type_=TextType.PLAINTEXT),
        text=Text("Would you like to play checkers?", type_=TextType.PLAINTEXT),
        confirm=Text("Yes", type_=TextType.PLAINTEXT),
        deny=Text("Nope!", type_=TextType.PLAINTEXT),
    ),
    "objects/conversation_filter_basic": lambda: ConversationFilter(
        include=["public", "mpim"],
        exclude_bot_users=True,
    ),
    "objects/dispatch_action_configuration_basic": lambda: DispatchActionConfiguration(
        trigger_actions_on=["on_character_entered"]
    ),
    "objects/input_parameter_basic": lambda: InputParameter(name="name", value="value"),
    "objects/option_basic": lambda: Option(
        text=Text(text="Canberra", type_=TextType.PLAINTEXT),
        value="canberra",
    ),
    "objects/option_group_basic": lambda: OptionGroup(
        label="Group A",
        options=_three_options(),
    ),
    "objects/option_value_at_limit": lambda: Option(
        text=Text(text="At limit", type_=TextType.PLAINTEXT),
        value="x" * 150,
    ),
    "objects/text_markdown_basic": lambda: Text(text="hi", type_=TextType.MARKDOWN),
    "objects/text_markdown_verbatim": lambda: Text(
        text="hi", type_=TextType.MARKDOWN, verbatim=True
    ),
    "objects/text_plaintext_basic": lambda: Text(text="hi", type_=TextType.PLAINTEXT),
    "objects/text_plaintext_emoji": lambda: Text(text="hi", type_=TextType.PLAINTEXT, emoji=True),
    "objects/trigger_basic": _trigger,
    "objects/workflow_basic": lambda: Workflow(trigger=_trigger()),
    "rich_text/rich_text_basic": lambda: RichText(
        text="I am a bold rich text block!",
        bold=True,
        italic=True,
        strike=False,
    ),
    "rich_text/rich_text_channel_basic": lambda: RichTextChannel(
        channel_id="C0261C65XNY",
        bold=True,
        italic=False,
        strike=True,
        highlight=True,
        client_highlight=True,
        unlink=False,
    ),
    "rich_text/rich_text_code_block_basic": lambda: RichTextCodeBlock(
        elements=[RichText(text="\ndef hello_world():\n    print('hello, world')")],
        border=0,
    ),
    "rich_text/rich_text_emoji_basic": lambda: RichTextEmoji(name="wave"),
    "rich_text/rich_text_link_basic": lambda: RichTextLink(
        url="https://google.com/",
        text="Google",
        unsafe=False,
        bold=True,
        italic=False,
        strike=True,
        code=True,
    ),
    "rich_text/rich_text_list_basic": _rich_text_list(
        ListType.BULLET, indent=0, offset=0, border=1
    ),
    "rich_text/rich_text_list_ordered": _rich_text_list(
        ListType.ORDERED, indent=1, offset=2, border=3
    ),
    "rich_text/rich_text_quote_basic": lambda: RichTextQuote(
        elements=[RichText(text="Great and good are seldom the same man")], border=1
    ),
    "rich_text/rich_text_section_basic": lambda: RichTextSection(
        elements=[RichText(text="The only true wisdom is in knowing you know nothing")]
    ),
    "rich_text/rich_text_user_basic": lambda: RichTextUser(
        user_id="DR36TNNLA",
        bold=True,
        italic=False,
        strike=True,
        highlight=True,
        client_highlight=True,
        unlink=False,
    ),
    "rich_text/rich_text_user_group_basic": lambda: RichTextUserGroup(
        user_group_id="C01RGRU0RUK",
        bold=True,
        italic=False,
        strike=True,
        highlight=True,
        client_highlight=True,
        unlink=False,
    ),
    "views/hometab_view": lambda: HomeTabView(
        blocks=[SectionBlock(text="Example Block", block_id="fake_id")]
    ),
    "views/modal_with_blocks": lambda: Modal(
        title="Hello, world!",
        close="Close button",
        submit="Submit button",
        blocks=[
            SectionBlock(text="first section block", block_id="1"),
            DividerBlock(block_id="2"),
            SectionBlock(text="second section block", block_id="3"),
        ],
    ),
}
