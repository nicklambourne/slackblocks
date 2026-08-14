from __future__ import annotations

import json

import pytest

from slackblocks import (
    ActionsBlock,
    AlertBlock,
    AreaChart,
    AxisConfig,
    BarChart,
    Button,
    CardBlock,
    CarouselBlock,
    ChartSegment,
    CheckboxGroup,
    ColumnSettings,
    ContainerBlock,
    ContextActionsBlock,
    ContextBlock,
    DataPoint,
    DataSeries,
    DataTableBlock,
    DataVisualizationBlock,
    DividerBlock,
    FeedbackButton,
    FeedbackButtons,
    FileBlock,
    HeaderBlock,
    IconButton,
    Image,
    ImageBlock,
    InputBlock,
    InvalidUsageError,
    LengthError,
    LineChart,
    MarkdownBlock,
    Option,
    PieChart,
    PlainTextInput,
    PlanBlock,
    RawNumber,
    RawText,
    RichText,
    RichTextBlock,
    RichTextSection,
    SectionBlock,
    SlackIcon,
    TableBlock,
    TaskCardBlock,
    Text,
    TextType,
    URLSource,
    VideoBlock,
)
from slackblocks.rich_text import RichTextLink

from .utils import fetch_sample


def test_basic_section_block() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    assert fetch_sample(path="blocks/section_block_text_only.json") == repr(block)


def test_basic_section_fields() -> None:
    block = SectionBlock(
        "Test:",
        fields=[Text(text="foo", type_=TextType.PLAINTEXT), Text(text="bar")],
        block_id="fake_block_id",
    )
    assert fetch_sample(path="blocks/section_block_fields.json") == repr(block)


def test_section_empty_text_field_value() -> None:
    block = SectionBlock(
        block_id="fake_block_id",
        fields=[
            Text("Highly", type_=TextType.MARKDOWN),
            Text("Strung", type_=TextType.PLAINTEXT, emoji=True),
        ],
    )
    assert fetch_sample(path="blocks/section_block_empty_text_field_value.json") == repr(block)


def test_section_neither_fields_nor_text() -> None:
    with pytest.raises(InvalidUsageError):
        SectionBlock(
            block_id="fake_block_id",
            text=None,
            fields=None,
        )


def test_section_invalid_field_content() -> None:
    with pytest.raises(InvalidUsageError):
        SectionBlock(
            block_id="fake_block_id",
            fields=[
                None,
            ],
        )


def test_section_single_field_value_coercion() -> None:
    block = SectionBlock(
        block_id="fake_block_id",
        fields="Lowly",
    )
    assert fetch_sample(path="blocks/section_block_single_field_value_coercion.json") == repr(block)


def test_section_both_text_and_fields() -> None:
    block = SectionBlock(
        text="Hello",
        block_id="fake_block_id",
        fields=[
            Text("Are you", type_=TextType.MARKDOWN),
            Text("There?", type_=TextType.PLAINTEXT, emoji=True),
        ],
    )
    assert fetch_sample(path="blocks/section_block_both_text_and_fields.json") == repr(block)


def test_basic_context_block() -> None:
    block = ContextBlock(elements=[Text("Hello, world!")], block_id="fake_block_id")
    assert fetch_sample(path="blocks/context_block_text_only.json") == repr(block)


def test_basic_divider_block() -> None:
    block = DividerBlock(block_id="fake_block_id")
    assert fetch_sample(path="blocks/divider_block_only.json") == repr(block)


def test_basic_image_block() -> None:
    block = ImageBlock(
        image_url="https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
        alt_text="image1",
        title="image1",
        block_id="fake_block_id",
    )
    assert fetch_sample(path="blocks/image_block_only.json") == repr(block)


def test_basic_header_block() -> None:
    block = HeaderBlock(text="AloHa!", block_id="fake_block_id")
    assert fetch_sample(path="blocks/header_block_only.json") == repr(block)


def test_checkbox_action_block() -> None:
    options = [
        Option(text="*a*", value="a", description="*a*"),
        Option(text="*b*", value="b", description="*b*"),
        Option(text="*c*", value="c", description="*c*"),
    ]
    block = ActionsBlock(
        block_id="fake_block_id",
        elements=CheckboxGroup(action_id="actionId-0", options=options),
    )
    assert fetch_sample(path="blocks/actions_block_checkboxes.json") == repr(block)


def test_basic_input_block() -> None:
    block = InputBlock(
        label=Text("Label", type_=TextType.PLAINTEXT, emoji=True),
        hint=Text("Hint", type_=TextType.PLAINTEXT, emoji=True),
        element=PlainTextInput(action_id="action"),
        block_id="fake_block_id",
        optional=True,
    )
    assert fetch_sample(path="blocks/input_block_only.json") == repr(block)


def test_input_block_invalid_element() -> None:
    with pytest.raises(InvalidUsageError):
        InputBlock(
            label=Text("Label", type_=TextType.PLAINTEXT, emoji=True),
            hint=Text("Hint", type_=TextType.PLAINTEXT, emoji=True),
            element=Text("hello"),
            block_id="fake_block_id",
        )


def test_input_block_invalid_label_type() -> None:
    with pytest.raises(InvalidUsageError):
        InputBlock(
            label=Text("Label", type_=TextType.MARKDOWN),
            hint=Text("Hint", type_=TextType.PLAINTEXT, emoji=True),
            element=Text("hello"),
            block_id="fake_block_id",
        )


def test_basic_rich_text_block() -> None:
    assert fetch_sample(path="blocks/rich_text_block_basic.json") == repr(
        RichTextBlock(
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
        )
    )


def test_basic_table_block() -> None:
    block = TableBlock(
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
                RichTextSection(
                    elements=RichTextLink(
                        url="https://slack.com",
                        text="Data 1B",
                    )
                ),
            ],
            [
                RawText(text="Data 2A"),
                RichTextSection(
                    elements=RichTextLink(
                        url="https://slack.com",
                        text="Data 2B",
                    )
                ),
            ],
        ],
    )
    # Add block_id to the sample as it is auto-generated
    sample = json.loads(fetch_sample(path="blocks/table_block.json"))
    sample["block_id"] = block.block_id
    assert sample == json.loads(repr(block))


def test_basic_file_block() -> None:
    """Renamed from ``text_basic_file_block`` (typo) so pytest collects it.

    Compare via parsed JSON to be robust to key-ordering differences in the
    rendered output. See #156."""
    block = FileBlock(
        external_id="external_id",
        block_id="fake_block_id",
    )
    sample = json.loads(fetch_sample(path="blocks/file_block_only.json"))
    assert sample == json.loads(repr(block))


def test_file_block_block_id_is_optional() -> None:
    """Regression test for #152: FileBlock must accept ``block_id`` as
    optional and synthesize a UUID like every other block, not raise
    ``TypeError``."""
    block = FileBlock(external_id="external_id")
    resolved = block._resolve()
    assert resolved["external_id"] == "external_id"
    assert resolved["source"] == "remote"
    # block_id auto-generated by parent class.
    assert resolved["block_id"] is not None and len(resolved["block_id"]) > 0


def test_markdown_block_basic() -> None:
    """``MarkdownBlock`` renders Slack's documented ``markdown`` block JSON."""
    block = MarkdownBlock(text="**Hello**, _world_!", block_id="fake_block_id")
    sample = json.loads(fetch_sample(path="blocks/markdown_block_basic.json"))
    assert sample == json.loads(repr(block))


def test_markdown_block_block_id_is_optional() -> None:
    """Like other blocks, ``MarkdownBlock`` synthesizes a UUID block_id when
    one is not supplied."""
    block = MarkdownBlock(text="# Heading")
    resolved = block._resolve()
    assert resolved["type"] == "markdown"
    assert resolved["text"] == "# Heading"
    assert resolved["block_id"] is not None and len(resolved["block_id"]) > 0


def test_markdown_block_rejects_empty_text() -> None:
    with pytest.raises(LengthError):
        MarkdownBlock(text="")


def test_markdown_block_rejects_text_over_12000_chars() -> None:
    with pytest.raises(LengthError):
        MarkdownBlock(text="x" * 12001)


def test_markdown_block_accepts_exactly_12000_chars() -> None:
    """Boundary case: the documented max is 12000 characters inclusive."""
    block = MarkdownBlock(text="x" * 12000)
    assert len(block.text) == 12000


def test_video_block_basic() -> None:
    """Minimal ``VideoBlock`` (required fields only) matches the documented
    Slack JSON shape."""
    block = VideoBlock(
        alt_text="alt",
        thumbnail_url="https://example.com/t.png",
        title="Title",
        video_url="https://example.com/v.mp4",
        block_id="b1",
    )
    sample = json.loads(fetch_sample(path="blocks/video_block_basic.json"))
    assert sample == json.loads(repr(block))


def test_video_block_full() -> None:
    """Full ``VideoBlock`` with every optional field populated."""
    block = VideoBlock(
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
    )
    sample = json.loads(fetch_sample(path="blocks/video_block_full.json"))
    assert sample == json.loads(repr(block))


def test_video_block_accepts_text_title_and_description() -> None:
    """``title`` and ``description`` accept ``Text`` objects directly,
    with markdown text coerced to plaintext via ``force_plaintext``."""
    block = VideoBlock(
        alt_text="alt",
        thumbnail_url="https://example.com/t.png",
        title=Text("Title", type_=TextType.MARKDOWN),
        video_url="https://example.com/v.mp4",
        description=Text("Desc", type_=TextType.MARKDOWN),
        block_id="b1",
    )
    resolved = block._resolve()
    assert resolved["title"] == {"type": "plain_text", "text": "Title"}
    assert resolved["description"] == {"type": "plain_text", "text": "Desc"}


def test_video_block_block_id_is_optional() -> None:
    block = VideoBlock(
        alt_text="alt",
        thumbnail_url="https://example.com/t.png",
        title="Title",
        video_url="https://example.com/v.mp4",
    )
    resolved = block._resolve()
    assert resolved["block_id"] is not None and len(resolved["block_id"]) > 0


def test_video_block_alt_text_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        VideoBlock(
            alt_text="x" * 201,
            thumbnail_url="https://example.com/t.png",
            title="Title",
            video_url="https://example.com/v.mp4",
        )


def test_video_block_title_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        VideoBlock(
            alt_text="alt",
            thumbnail_url="https://example.com/t.png",
            title="x" * 201,
            video_url="https://example.com/v.mp4",
        )


def test_video_block_author_name_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        VideoBlock(
            alt_text="alt",
            thumbnail_url="https://example.com/t.png",
            title="Title",
            video_url="https://example.com/v.mp4",
            author_name="x" * 51,
        )


def test_video_block_provider_name_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        VideoBlock(
            alt_text="alt",
            thumbnail_url="https://example.com/t.png",
            title="Title",
            video_url="https://example.com/v.mp4",
            provider_name="x" * 51,
        )


def test_video_block_description_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        VideoBlock(
            alt_text="alt",
            thumbnail_url="https://example.com/t.png",
            title="Title",
            video_url="https://example.com/v.mp4",
            description="x" * 201,
        )


def test_alert_block() -> None:
    block = AlertBlock(
        "The work is mysterious and important.",
        level="info",
        block_id="fake_block_id",
    )
    assert fetch_sample(path="blocks/alert_block.json") == repr(block)


def test_card_block() -> None:
    block = CardBlock(
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
    )
    assert fetch_sample(path="blocks/card_block.json") == repr(block)


def test_carousel_block() -> None:
    block = CarouselBlock(
        block_id="fake_block_id",
        elements=[
            CardBlock(title="First result", block_id="card_1"),
            CardBlock(title="Second result", block_id="card_2"),
        ],
    )
    assert fetch_sample(path="blocks/carousel_block.json") == repr(block)


def test_container_block() -> None:
    block = ContainerBlock(
        block_id="fake_block_id",
        title="Deployment summary",
        subtitle="Production is healthy",
        child_blocks=[SectionBlock("All systems operational.", block_id="child_1")],
        has_header_divider=True,
    )
    assert fetch_sample(path="blocks/container_block.json") == repr(block)


def test_context_actions_feedback_buttons() -> None:
    block = ContextActionsBlock(
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
    )
    assert fetch_sample(path="blocks/context_actions_feedback_buttons.json") == repr(block)


def test_context_actions_icon_button() -> None:
    block = ContextActionsBlock(
        block_id="fake_block_id",
        elements=[
            IconButton(
                text="Delete",
                action_id="delete_button",
                value="delete_item",
            )
        ],
    )
    assert fetch_sample(path="blocks/context_actions_icon_button.json") == repr(block)


def test_data_table_block() -> None:
    block = DataTableBlock(
        block_id="fake_block_id",
        rows=[
            [RawText("Name"), RawText("Score")],
            [RawText("Alice"), RawNumber(42, "42")],
        ],
        caption="Team scores",
    )
    assert fetch_sample(path="blocks/data_table_block.json") == repr(block)


def test_data_visualization_pie() -> None:
    block = DataVisualizationBlock(
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
    )
    assert fetch_sample(path="blocks/data_visualization_pie.json") == repr(block)


def test_data_visualization_line() -> None:
    axis = AxisConfig(
        ["Week 1", "Week 2"],
        x_label="Week",
        y_label="Paper Sales (USD)",
    )
    block = DataVisualizationBlock(
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
            axis,
        ),
    )
    assert fetch_sample(path="blocks/data_visualization_line.json") == repr(block)


def test_data_visualization_bar() -> None:
    block = DataVisualizationBlock(
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
    )
    assert fetch_sample(path="blocks/data_visualization_bar.json") == repr(block)


def test_data_visualization_area() -> None:
    block = DataVisualizationBlock(
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
    )
    assert fetch_sample(path="blocks/data_visualization_area.json") == repr(block)


def task_output(block_id: str) -> RichTextBlock:
    return RichTextBlock(
        RichTextSection(RichText("Profile data loaded")),
        block_id=block_id,
    )


def test_task_card_block() -> None:
    output = RichTextBlock(
        RichTextSection(RichText("Found weather data for Chicago from 2 sources")),
        block_id="task_output",
    )
    block = TaskCardBlock(
        block_id="fake_block_id",
        task_id="task_1",
        title="Fetching weather data",
        output=output,
        sources=[
            URLSource("https://weather.com/", "weather.com"),
            URLSource("https://www.accuweather.com/", "accuweather.com"),
        ],
        status="pending",
    )
    assert fetch_sample(path="blocks/task_card_block.json") == repr(block)


def test_plan_block() -> None:
    block = PlanBlock(
        block_id="fake_block_id",
        title="Thinking completed",
        tasks=[
            TaskCardBlock(
                task_id="call_001",
                title="Fetched user profile information",
                status="complete",
                output=task_output("plan_output"),
            ),
            TaskCardBlock(
                task_id="call_002",
                title="Checked user permissions",
                status="pending",
            ),
        ],
    )
    assert fetch_sample(path="blocks/plan_block.json") == repr(block)


def test_url_source() -> None:
    source = URLSource("https://docs.slack.dev/", "Slack API docs")
    assert fetch_sample(path="elements/url_source_basic.json") == repr(source)
