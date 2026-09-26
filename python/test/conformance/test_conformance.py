from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

import slackblocks
from slackblocks import (
    ActionsBlock,
    AlertBlock,
    Attachment,
    AxisConfig,
    Button,
    CardBlock,
    CarouselBlock,
    ChartSegment,
    CheckboxGroup,
    ColumnSettings,
    ConfirmationDialogue,
    ContainerBlock,
    ContextActionsBlock,
    ContextBlock,
    ConversationFilter,
    DataPoint,
    DataSeries,
    DataTableBlock,
    DataVisualizationBlock,
    DatePicker,
    DispatchActionConfiguration,
    DividerBlock,
    EmailInput,
    FeedbackButton,
    FeedbackButtons,
    FileInput,
    HeaderBlock,
    HomeTabView,
    IconButton,
    Image,
    ImageBlock,
    InputBlock,
    LengthError,
    LineChart,
    MarkdownBlock,
    Message,
    MissingRequiredError,
    ModalView,
    MutualExclusivityError,
    NumberInput,
    Option,
    OptionGroup,
    OverflowMenu,
    PieChart,
    PlainText,
    PlainTextInput,
    PlanBlock,
    RadioButtonGroup,
    RangeError,
    RawText,
    RichTextInput,
    SectionBlock,
    SlackFile,
    SlackIcon,
    StaticSelectMenu,
    TableBlock,
    TaskCardBlock,
    Text,
    TimePicker,
    TypeMismatchError,
    URLInput,
    URLSource,
    UserMultiSelectMenu,
    VideoBlock,
    WorkflowButton,
)
from slackblocks.errors import InvalidUsageError
from slackblocks.rich_text import (
    RichText,
    RichTextCodeBlock,
    RichTextList,
    RichTextQuote,
    RichTextSection,
)

from .valid_constructions import CONSTRUCTIONS

if TYPE_CHECKING:
    from collections.abc import Callable

REPO_ROOT = Path(__file__).resolve().parents[3]
SPEC_ROOT = REPO_ROOT / "spec"


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def test_valid_manifest_covers_the_entire_fixture_corpus() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    fixture_ids = {fixture["id"] for fixture in manifest["fixtures"]}
    files = {
        path.relative_to(SPEC_ROOT / "fixtures" / "valid").with_suffix("").as_posix()
        for path in (SPEC_ROOT / "fixtures" / "valid").rglob("*.json")
    }
    assert manifest["spec_version"] == slackblocks.SPEC_VERSION
    assert fixture_ids
    assert len(fixture_ids) == len(manifest["fixtures"])
    assert fixture_ids == files


def test_every_shared_json_capability_has_an_official_fixture() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    coverage = load_json(SPEC_ROOT / "coverage.json")
    fixtures = {fixture["id"]: fixture for fixture in manifest["fixtures"]}
    assert coverage["spec_version"] == manifest["spec_version"]
    for capability, fixture_ids in coverage["capabilities"].items():
        assert fixture_ids, capability
        for fixture_id in fixture_ids:
            assert fixture_id in fixtures, f"{capability} -> {fixture_id}"
            assert fixtures[fixture_id]["slack_docs"].startswith("https://docs.slack.dev/")


def test_construction_registry_matches_the_manifest_exactly() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    assert set(CONSTRUCTIONS) == {fixture["id"] for fixture in manifest["fixtures"]}


@pytest.mark.parametrize(
    "fixture_id",
    [fixture["id"] for fixture in load_json(SPEC_ROOT / "manifest.json")["fixtures"]],
)
def test_valid_fixture_constructs_identically_through_the_public_api(
    fixture_id: str,
) -> None:
    construction = CONSTRUCTIONS.get(fixture_id)
    assert construction is not None, f"No registered Python construction for {fixture_id}"
    expected = load_json(SPEC_ROOT / "fixtures" / "valid" / f"{fixture_id}.json")
    assert json.loads(repr(construction())) == expected


def test_every_valid_fixture_contains_json() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    for fixture in manifest["fixtures"]:
        path = SPEC_ROOT / "fixtures" / "valid" / f"{fixture['id']}.json"
        json.loads(path.read_text())


def option(value: str = "a") -> Option:
    return Option(text=PlainText("A"), value=value)


LIMITS = load_json(SPEC_ROOT / "limits.json")


def video(**overrides: object) -> VideoBlock:
    inputs: dict[str, object] = {
        "alt_text": "Video",
        "thumbnail_url": "https://example.com/thumbnail.png",
        "title": "Title",
        "video_url": "https://example.com/video.mp4",
        **overrides,
    }
    return VideoBlock(**inputs)  # type: ignore[arg-type]


def valid_card() -> CardBlock:
    return CardBlock(title="Card")


def valid_feedback_button() -> FeedbackButton:
    return FeedbackButton("Good", "good")


def valid_table_rows() -> list[list[RawText]]:
    return [[RawText("Name")], [RawText("Alice")]]


def valid_axis() -> AxisConfig:
    return AxisConfig(["A"])


def valid_series(name: str = "Series") -> DataSeries:
    return DataSeries(name, [DataPoint("A", 1)])


def valid_task(task_id: str = "task", status: str = "complete") -> TaskCardBlock:
    return TaskCardBlock(task_id=task_id, title="Task", status=status)  # type: ignore[arg-type]


def valid_rich_text_list(**overrides: int) -> RichTextList:
    return RichTextList(style="bullet", elements=[RichTextSection(RichText("A"))], **overrides)


def data_table_with_content(length: int) -> DataTableBlock:
    """A data table whose cells total ``length`` characters."""
    header = "Name"
    return DataTableBlock(
        [[RawText(header)], [RawText("x" * (length - len(header)))]], caption="Names"
    )


INVALID_CASES: dict[str, Callable[[], object]] = {
    "text-empty": lambda: Text(""),
    "text-too-long": lambda: Text("x" * 3001),
    "button-action-id-too-long": lambda: Button(
        text="A", action_id="x" * (LIMITS["action_id"]["max_length"] + 1)
    ),
    "button-text-too-long": lambda: Button(
        text="x" * (LIMITS["button"]["text"]["max_length"] + 1), action_id="a"
    ),
    "button-url-too-long": lambda: Button(
        text="A",
        action_id="a",
        url="x" * (LIMITS["button"]["url"]["max_length"] + 1),
    ),
    "button-value-too-long": lambda: Button(
        text="A",
        action_id="a",
        value="x" * (LIMITS["button"]["value"]["max_length"] + 1),
    ),
    "confirmation-title-too-long": lambda: ConfirmationDialogue(
        title="x" * (LIMITS["confirmation"]["title"]["max_length"] + 1),
        text="Text",
        confirm="Yes",
        deny="No",
    ),
    "confirmation-text-too-long": lambda: ConfirmationDialogue(
        title="Title",
        text="x" * (LIMITS["confirmation"]["text"]["max_length"] + 1),
        confirm="Yes",
        deny="No",
    ),
    "confirmation-confirm-too-long": lambda: ConfirmationDialogue(
        title="Title",
        text="Text",
        confirm="x" * (LIMITS["confirmation"]["confirm"]["max_length"] + 1),
        deny="No",
    ),
    "confirmation-deny-too-long": lambda: ConfirmationDialogue(
        title="Title",
        text="Text",
        confirm="Yes",
        deny="x" * (LIMITS["confirmation"]["deny"]["max_length"] + 1),
    ),
    "option-text-too-long": lambda: Option(
        text="x" * (LIMITS["option"]["text"]["max_length"] + 1), value="a"
    ),
    "option-value-too-long": lambda: option("x" * (LIMITS["option"]["value"]["max_length"] + 1)),
    "option-description-too-long": lambda: Option(
        text="A",
        value="a",
        description="x" * (LIMITS["option"]["description"]["max_length"] + 1),
    ),
    "option-group-label-too-long": lambda: OptionGroup(
        label="x" * (LIMITS["option_group"]["label"]["max_length"] + 1),
        options=[option()],
    ),
    "option-group-empty": lambda: OptionGroup(label="Group", options=[]),
    "option-group-too-many-options": lambda: OptionGroup(
        label="Group",
        options=[
            option(str(index))
            for index in range(LIMITS["option_group"]["options"]["max_items"] + 1)
        ],
    ),
    "select-placeholder-too-long": lambda: StaticSelectMenu(
        action_id="a",
        options=[option()],
        placeholder="x" * (LIMITS["select"]["placeholder"]["max_length"] + 1),
    ),
    "plain-text-input-placeholder-too-long": lambda: PlainTextInput(
        action_id="a",
        placeholder="x" * (LIMITS["plain_text_input"]["placeholder"]["max_length"] + 1),
    ),
    "email-input-placeholder-too-long": lambda: EmailInput(
        action_id="a", placeholder="x" * (LIMITS["email_input"]["placeholder"]["max_length"] + 1)
    ),
    "url-input-placeholder-too-long": lambda: URLInput(
        action_id="a", placeholder="x" * (LIMITS["url_input"]["placeholder"]["max_length"] + 1)
    ),
    "number-input-placeholder-too-long": lambda: NumberInput(
        is_decimal_allowed=False,
        action_id="a",
        placeholder="x" * (LIMITS["number_input"]["placeholder"]["max_length"] + 1),
    ),
    "date-picker-placeholder-too-long": lambda: DatePicker(
        action_id="a", placeholder="x" * (LIMITS["date_picker"]["placeholder"]["max_length"] + 1)
    ),
    "time-picker-placeholder-too-long": lambda: TimePicker(
        action_id="a", placeholder="x" * (LIMITS["time_picker"]["placeholder"]["max_length"] + 1)
    ),
    "rich-text-input-placeholder-too-long": lambda: RichTextInput(
        action_id="a",
        placeholder="x" * (LIMITS["rich_text_input"]["placeholder"]["max_length"] + 1),
    ),
    "select-too-many-options": lambda: StaticSelectMenu(
        action_id="a",
        options=[
            option(str(index)) for index in range(LIMITS["select"]["options"]["max_items"] + 1)
        ],
    ),
    "select-too-many-option-groups": lambda: StaticSelectMenu(
        action_id="a",
        option_groups=[
            OptionGroup(label=f"Group {index}", options=[option()])
            for index in range(LIMITS["select"]["option_groups"]["max_items"] + 1)
        ],
    ),
    "overflow-empty": lambda: OverflowMenu("a", []),
    "overflow-too-many-options": lambda: OverflowMenu(
        "a",
        [option(str(index)) for index in range(LIMITS["overflow"]["options"]["max_items"] + 1)],
    ),
    "checkboxes-empty": lambda: CheckboxGroup(action_id="a", options=[]),
    "checkboxes-too-many-options": lambda: CheckboxGroup(
        action_id="a",
        options=[
            option(str(index)) for index in range(LIMITS["checkboxes"]["options"]["max_items"] + 1)
        ],
    ),
    "radio-buttons-empty": lambda: RadioButtonGroup(action_id="a", options=[]),
    "radio-buttons-too-many-options": lambda: RadioButtonGroup(
        action_id="a",
        options=[
            option(str(index))
            for index in range(LIMITS["radio_buttons"]["options"]["max_items"] + 1)
        ],
    ),
    # Astral-plane emoji pin that the limit counts Unicode code points.
    "option-url-too-long": lambda: Option(
        text=PlainText("A"),
        value="a",
        url="\U0001f642" * (LIMITS["option"]["url"]["max_length"] + 1),
    ),
    "url-source-url-empty": lambda: URLSource("", "text"),
    "url-source-url-too-long": lambda: URLSource(
        "x" * (LIMITS["url_source"]["url"]["max_length"] + 1), "text"
    ),
    "table-too-many-rows": lambda: TableBlock(
        [[RawText("A")] for _ in range(LIMITS["table"]["rows"]["max_items"] + 1)]
    ),
    "table-too-many-columns": lambda: TableBlock(
        [[RawText("A") for _ in range(LIMITS["table"]["columns"]["max_items"] + 1)]]
    ),
    "table-ragged-rows": lambda: TableBlock([[RawText("A"), RawText("B")], [RawText("C")]]),
    "file-input-max-files-too-small": lambda: FileInput(
        action_id="a", max_files=LIMITS["file_input"]["max_files"]["min"] - 1
    ),
    "file-input-max-files-too-large": lambda: FileInput(
        action_id="a", max_files=LIMITS["file_input"]["max_files"]["max"] + 1
    ),
    "dispatch-action-no-triggers": lambda: DispatchActionConfiguration([]),
    "dispatch-action-missing-triggers": lambda: DispatchActionConfiguration(),
    "dispatch-action-too-many-triggers": lambda: DispatchActionConfiguration(
        ["on_enter_pressed", "on_character_entered", "on_enter_pressed"]
    ),
    "plain-text-input-max-length-too-large": lambda: PlainTextInput(
        action_id="a", max_length=LIMITS["plain_text_input"]["max_length"]["max"] + 1
    ),
    "actions-too-many-elements": lambda: ActionsBlock(
        elements=[
            Button(text="A", action_id=f"a-{index}")
            for index in range(LIMITS["actions"]["elements"]["max_items"] + 1)
        ]
    ),
    "context-too-many-elements": lambda: ContextBlock(
        elements=[Text("A") for _ in range(LIMITS["context"]["elements"]["max_items"] + 1)]
    ),
    "header-text-too-long": lambda: HeaderBlock("x" * (LIMITS["header"]["text"]["max_length"] + 1)),
    "image-url-too-long": lambda: ImageBlock(
        image_url="x" * (LIMITS["image"]["image_url"]["max_length"] + 1),
        alt_text="Alt",
    ),
    "image-alt-text-too-long": lambda: ImageBlock(
        image_url="https://example.com/image.png",
        alt_text="x" * (LIMITS["image"]["alt_text"]["max_length"] + 1),
    ),
    "input-label-too-long": lambda: InputBlock(
        label="x" * (LIMITS["input"]["label"]["max_length"] + 1),
        element=PlainTextInput(action_id="a"),
    ),
    "input-hint-too-long": lambda: InputBlock(
        label="Label",
        hint="x" * (LIMITS["input"]["hint"]["max_length"] + 1),
        element=PlainTextInput(action_id="a"),
    ),
    "markdown-empty": lambda: MarkdownBlock(""),
    "markdown-too-long": lambda: MarkdownBlock(
        "x" * (LIMITS["markdown"]["text"]["max_length"] + 1)
    ),
    "section-text-too-long": lambda: SectionBlock(
        "x" * (LIMITS["section"]["text"]["max_length"] + 1)
    ),
    "section-too-many-fields": lambda: SectionBlock(
        fields=["x"] * (LIMITS["section"]["fields"]["max_items"] + 1)
    ),
    "section-field-too-long": lambda: SectionBlock(
        fields=["x" * (LIMITS["section"]["fields"]["item_max_length"] + 1)]
    ),
    "video-alt-text-too-long": lambda: video(
        alt_text="x" * (LIMITS["video"]["alt_text"]["max_length"] + 1)
    ),
    "video-title-too-long": lambda: video(title="x" * (LIMITS["video"]["title"]["max_length"] + 1)),
    "video-author-name-too-long": lambda: video(
        author_name="x" * (LIMITS["video"]["author_name"]["max_length"] + 1)
    ),
    "video-description-too-long": lambda: video(
        description="x" * (LIMITS["video"]["description"]["max_length"] + 1)
    ),
    "video-provider-name-too-long": lambda: video(
        provider_name="x" * (LIMITS["video"]["provider_name"]["max_length"] + 1)
    ),
    "message-channel-empty": lambda: Message(channel=""),
    "message-too-many-blocks": lambda: Message(
        channel="C123",
        blocks=[DividerBlock() for _ in range(LIMITS["message"]["blocks"]["max_items"] + 1)],
    ),
    "message-too-many-attachments": lambda: Message(
        channel="C123",
        attachments=[
            Attachment(blocks=[DividerBlock()])
            for _ in range(LIMITS["message"]["attachments"]["max_items"] + 1)
        ],
    ),
    "message-invalid-block-surface": lambda: Message(
        channel="C123", blocks=[AlertBlock("Modal only")]
    ),
    "modal-invalid-block-surface": lambda: ModalView(
        title="Invalid", blocks=[MarkdownBlock("Message only")]
    ),
    "home-invalid-block-surface": lambda: HomeTabView(blocks=[AlertBlock("Modal only")]),
    "modal-input-requires-submit": lambda: ModalView(
        title="Missing submit",
        blocks=[InputBlock(label="Name", element=PlainTextInput(action_id="name"))],
    ),
    "view-missing-blocks": lambda: HomeTabView(blocks=[]),
    "view-too-many-blocks": lambda: HomeTabView(
        blocks=[DividerBlock() for _ in range(LIMITS["view"]["blocks"]["max_items"] + 1)]
    ),
    "view-private-metadata-too-long": lambda: HomeTabView(
        blocks=[DividerBlock()],
        private_metadata="x" * (LIMITS["view"]["private_metadata"]["max_length"] + 1),
    ),
    "view-callback-id-too-long": lambda: HomeTabView(
        blocks=[DividerBlock()],
        callback_id="x" * (LIMITS["view"]["callback_id"]["max_length"] + 1),
    ),
    "view-title-too-long": lambda: ModalView(
        title="x" * (LIMITS["view"]["title"]["max_length"] + 1),
        blocks=[DividerBlock()],
    ),
    "view-close-too-long": lambda: ModalView(
        title="Title",
        close="x" * (LIMITS["view"]["close"]["max_length"] + 1),
        blocks=[DividerBlock()],
    ),
    "view-submit-too-long": lambda: ModalView(
        title="Title",
        submit="x" * (LIMITS["view"]["submit"]["max_length"] + 1),
        blocks=[DividerBlock()],
    ),
    "section-missing-content": lambda: SectionBlock(),
    "section-empty-fields": lambda: SectionBlock(fields=[]),
    "static-select-options-and-groups": lambda: StaticSelectMenu(
        "a",
        options=[option()],
        option_groups=[object()],  # type: ignore[list-item]
    ),
    "image-url-and-slack-file": lambda: Image(
        image_url="https://example.com/image.png", slack_file=SlackFile(url=None, id="F0123ABC456")
    ),
    "number-input-inverted-range": lambda: NumberInput(
        is_decimal_allowed=True, min_value=2, max_value=1
    ),
    "context-invalid-element": lambda: ContextBlock(elements=[DividerBlock()]),
    "input-invalid-element": lambda: InputBlock(label="Label", element=Button("A", "a")),
    "block-id-too-long": lambda: DividerBlock(
        block_id="x" * (LIMITS["block_id"]["max_length"] + 1)
    ),
    "button-accessibility-label-too-long": lambda: Button(
        text="A",
        action_id="a",
        accessibility_label="x" * (LIMITS["button"]["accessibility_label"]["max_length"] + 1),
    ),
    "workflow-button-text-too-long": lambda: WorkflowButton(
        "x" * (LIMITS["workflow_button"]["text"]["max_length"] + 1), action_id="a"
    ),
    "workflow-button-accessibility-label-too-long": lambda: WorkflowButton(
        "Run",
        accessibility_label="x"
        * (LIMITS["workflow_button"]["accessibility_label"]["max_length"] + 1),
        action_id="a",
    ),
    "alert-text-too-long": lambda: AlertBlock("x" * (LIMITS["alert"]["text"]["max_length"] + 1)),
    "card-title-too-long": lambda: CardBlock(
        title="x" * (LIMITS["card"]["title"]["max_length"] + 1)
    ),
    "card-subtitle-too-long": lambda: CardBlock(
        title="Card",
        subtitle="x" * (LIMITS["card"]["subtitle"]["max_length"] + 1),
    ),
    "card-body-too-long": lambda: CardBlock(body="x" * (LIMITS["card"]["body"]["max_length"] + 1)),
    "card-too-many-actions": lambda: CardBlock(
        actions=[
            Button("A", f"a-{index}") for index in range(LIMITS["card"]["actions"]["max_items"] + 1)
        ]
    ),
    "card-subtext-too-long": lambda: CardBlock(
        title="Card",
        subtext="x" * (LIMITS["card"]["subtext"]["max_length"] + 1),
    ),
    "carousel-empty": lambda: CarouselBlock([]),
    "carousel-too-many-cards": lambda: CarouselBlock(
        [valid_card() for _ in range(LIMITS["carousel"]["elements"]["max_items"] + 1)]
    ),
    "container-title-too-long": lambda: ContainerBlock(
        title="x" * (LIMITS["container"]["title"]["max_length"] + 1),
        child_blocks=[DividerBlock()],
    ),
    "container-subtitle-too-long": lambda: ContainerBlock(
        title="Container",
        subtitle="x" * (LIMITS["container"]["subtitle"]["max_length"] + 1),
        child_blocks=[DividerBlock()],
    ),
    "container-too-many-child-blocks": lambda: ContainerBlock(
        title="Container",
        child_blocks=[
            DividerBlock() for _ in range(LIMITS["container"]["child_blocks"]["max_items"] + 1)
        ],
    ),
    "context-actions-too-many-elements": lambda: ContextActionsBlock(
        [
            IconButton("Delete", action_id=f"delete-{index}")
            for index in range(LIMITS["context_actions"]["elements"]["max_items"] + 1)
        ]
    ),
    "feedback-button-text-too-long": lambda: FeedbackButtons(
        positive_button=FeedbackButton(
            "x" * (LIMITS["feedback_button"]["text"]["max_length"] + 1), "good"
        ),
        negative_button=valid_feedback_button(),
    ),
    "feedback-button-value-too-long": lambda: FeedbackButtons(
        positive_button=FeedbackButton(
            "Good", "x" * (LIMITS["feedback_button"]["value"]["max_length"] + 1)
        ),
        negative_button=valid_feedback_button(),
    ),
    "feedback-button-accessibility-label-too-long": lambda: FeedbackButtons(
        positive_button=FeedbackButton(
            "Good",
            "good",
            "x" * (LIMITS["feedback_button"]["accessibility_label"]["max_length"] + 1),
        ),
        negative_button=valid_feedback_button(),
    ),
    "icon-button-too-many-visible-users": lambda: IconButton(
        "Delete",
        visible_to_user_ids=[
            f"U{index}"
            for index in range(LIMITS["icon_button"]["visible_to_user_ids"]["max_items"] + 1)
        ],
    ),
    "icon-button-value-too-long": lambda: IconButton(
        "Delete", value="x" * (LIMITS["icon_button"]["value"]["max_length"] + 1)
    ),
    "icon-button-accessibility-label-too-long": lambda: IconButton(
        "Delete",
        accessibility_label="x" * (LIMITS["icon_button"]["accessibility_label"]["max_length"] + 1),
    ),
    "data-table-too-few-rows": lambda: DataTableBlock([[RawText("Name")]], caption="Names"),
    "data-table-too-many-rows": lambda: DataTableBlock(
        [[RawText("A")] for _ in range(LIMITS["data_table"]["rows"]["max_items"] + 1)],
        caption="Names",
    ),
    "data-table-too-few-columns": lambda: DataTableBlock([[], []], caption="Empty"),
    "data-table-too-many-columns": lambda: DataTableBlock(
        [
            [RawText("A") for _ in range(LIMITS["data_table"]["columns"]["max_items"] + 1)],
            [RawText("A") for _ in range(LIMITS["data_table"]["columns"]["max_items"] + 1)],
        ],
        caption="Wide",
    ),
    "data-table-page-size-too-small": lambda: DataTableBlock(
        valid_table_rows(),
        caption="Names",
        page_size=LIMITS["data_table"]["page_size"]["min"] - 1,
    ),
    "data-table-page-size-too-large": lambda: DataTableBlock(
        valid_table_rows(),
        caption="Names",
        page_size=LIMITS["data_table"]["page_size"]["max"] + 1,
    ),
    "data-table-cell-text-empty": lambda: DataTableBlock(
        [[RawText("Name")], [RawText("")]], caption="Names"
    ),
    "data-table-content-too-long": lambda: DataTableBlock(
        [
            [RawText("Name")],
            [RawText("x" * LIMITS["data_table"]["content"]["max_length"])],
        ],
        caption="Names",
    ),
    "data-visualization-title-too-long": lambda: DataVisualizationBlock(
        "x" * (LIMITS["data_visualization"]["title"]["max_length"] + 1),
        PieChart([ChartSegment("A", 1)]),
    ),
    "pie-chart-empty": lambda: PieChart([]),
    "pie-chart-too-many-segments": lambda: PieChart(
        [
            ChartSegment(f"S{index}", 1)
            for index in range(LIMITS["data_visualization"]["segments"]["max_items"] + 1)
        ]
    ),
    "chart-segment-label-too-long": lambda: ChartSegment(
        "x" * (LIMITS["data_visualization"]["segment"]["label"]["max_length"] + 1),
        1,
    ),
    "chart-segment-value-not-positive": lambda: ChartSegment("A", 0),
    "chart-series-empty": lambda: LineChart([], valid_axis()),
    "chart-duplicate-point-labels": lambda: LineChart(
        [DataSeries("Series", [DataPoint("A", 1), DataPoint("A", 2)])],
        AxisConfig(["A", "B"]),
    ),
    "chart-too-many-series": lambda: LineChart(
        [
            valid_series(f"S{index}")
            for index in range(LIMITS["data_visualization"]["series"]["max_items"] + 1)
        ],
        valid_axis(),
    ),
    "data-series-name-too-long": lambda: DataSeries(
        "x" * (LIMITS["data_visualization"]["series_name"]["max_length"] + 1),
        [DataPoint("A", 1)],
    ),
    "data-series-empty": lambda: DataSeries("Series", []),
    "data-series-too-many-points": lambda: DataSeries(
        "Series",
        [
            DataPoint(f"P{index}", index)
            for index in range(LIMITS["data_visualization"]["data"]["max_items"] + 1)
        ],
    ),
    "data-point-label-too-long": lambda: DataPoint(
        "x" * (LIMITS["data_visualization"]["point_label"]["max_length"] + 1), 1
    ),
    "axis-categories-empty": lambda: AxisConfig([]),
    "axis-too-many-categories": lambda: AxisConfig(
        [
            f"C{index}"
            for index in range(LIMITS["data_visualization"]["categories"]["max_items"] + 1)
        ]
    ),
    "axis-category-label-too-long": lambda: AxisConfig(
        ["x" * (LIMITS["data_visualization"]["category_label"]["max_length"] + 1)]
    ),
    "axis-label-too-long": lambda: AxisConfig(
        ["A"], x_label="x" * (LIMITS["data_visualization"]["axis_label"]["max_length"] + 1)
    ),
    "plain-text-input-min-length-negative": lambda: PlainTextInput(
        action_id="a", min_length=LIMITS["plain_text_input"]["min_length"]["min"] - 1
    ),
    "plain-text-input-min-length-too-large": lambda: PlainTextInput(
        action_id="a", min_length=LIMITS["plain_text_input"]["min_length"]["max"] + 1
    ),
    "plain-text-input-max-length-too-small": lambda: PlainTextInput(
        action_id="a", max_length=LIMITS["plain_text_input"]["max_length"]["min"] - 1
    ),
    "rich-text-input-min-lines-too-small": lambda: RichTextInput(
        action_id="a", min_lines=LIMITS["rich_text_input"]["min_lines"]["min"] - 1
    ),
    "rich-text-input-min-lines-too-large": lambda: RichTextInput(
        action_id="a", min_lines=LIMITS["rich_text_input"]["min_lines"]["max"] + 1
    ),
    "rich-text-input-max-lines-too-small": lambda: RichTextInput(
        action_id="a", max_lines=LIMITS["rich_text_input"]["max_lines"]["min"] - 1
    ),
    "rich-text-input-max-lines-too-large": lambda: RichTextInput(
        action_id="a", max_lines=LIMITS["rich_text_input"]["max_lines"]["max"] + 1
    ),
    "multi-select-max-selected-items-too-small": lambda: UserMultiSelectMenu(
        action_id="a",
        max_selected_items=LIMITS["multi_select"]["max_selected_items"]["min"] - 1,
    ),
    "conversation-filter-include-empty": lambda: ConversationFilter(include=[]),
    "conversation-filter-unknown-include": lambda: ConversationFilter(
        include=["channels"]  # type: ignore[list-item]
    ),
    "workflow-button-missing-action-id": lambda: WorkflowButton("Run"),
    "number-input-missing-decimal-flag": lambda: NumberInput(
        is_decimal_allowed=None,  # type: ignore[arg-type]
        action_id="a",
    ),
    "slack-icon-missing-name": lambda: SlackIcon(None),  # type: ignore[arg-type]
    "slack-file-id-malformed": lambda: SlackFile(url=None, id="F0123456"),
    "image-element-url-too-long": lambda: Image(
        image_url="x" * (LIMITS["image_element"]["image_url"]["max_length"] + 1),
        alt_text="Alt",
    ),
    "image-element-alt-text-too-long": lambda: Image(
        image_url="https://example.com/image.png",
        alt_text="x" * (LIMITS["image_element"]["alt_text"]["max_length"] + 1),
    ),
    "image-block-title-too-long": lambda: ImageBlock(
        image_url="https://example.com/image.png",
        alt_text="Alt",
        title="x" * (LIMITS["image"]["title"]["max_length"] + 1),
    ),
    "image-block-url-and-slack-file": lambda: ImageBlock(
        image_url="https://example.com/image.png",
        alt_text="Alt",
        slack_file=SlackFile(url=None, id="F0123ABC456"),
    ),
    "image-block-missing-source": lambda: ImageBlock(alt_text="Alt"),
    "container-no-child-blocks": lambda: ContainerBlock(title="Container", child_blocks=[]),
    "table-too-many-column-settings": lambda: TableBlock(
        [[RawText("A")]],
        column_settings=[
            ColumnSettings(is_wrapped=True)
            for _ in range(LIMITS["table"]["column_settings"]["max_items"] + 1)
        ],
    ),
    "data-table-row-header-index-negative": lambda: DataTableBlock(
        valid_table_rows(),
        caption="Names",
        row_header_column_index=LIMITS["data_table"]["row_header_column_index"]["min"] - 1,
    ),
    "data-table-total-content-too-long": lambda: Message(
        channel="C123",
        blocks=[
            data_table_with_content(LIMITS["data_table"]["total_content"]["max_length"] // 2 + 1)
            for _ in range(2)
        ],
    ),
    "markdown-total-too-long": lambda: Message(
        channel="C123",
        blocks=[
            MarkdownBlock("x" * (LIMITS["markdown"]["total_text"]["max_length"] // 2 + 1))
            for _ in range(2)
        ],
    ),
    "plan-missing-tasks": lambda: PlanBlock(title="Plan"),
    "plan-too-many-tasks": lambda: PlanBlock(
        title="Plan",
        tasks=[
            valid_task(f"task-{index}") for index in range(LIMITS["plan"]["tasks"]["max_items"] + 1)
        ],
    ),
    "plan-duplicate-task-ids": lambda: PlanBlock(
        title="Plan", tasks=[valid_task("task"), valid_task("task")]
    ),
    "task-card-missing-status": lambda: TaskCardBlock(task_id="task", title="Task"),
    "task-card-pending-status": lambda: Message(
        channel="C123", blocks=[valid_task(status="pending")]
    ),
    "rich-text-list-indent-negative": lambda: valid_rich_text_list(
        indent=LIMITS["rich_text_list"]["indent"]["min"] - 1
    ),
    "rich-text-list-indent-too-large": lambda: valid_rich_text_list(
        indent=LIMITS["rich_text_list"]["indent"]["max"] + 1
    ),
    "rich-text-list-offset-negative": lambda: valid_rich_text_list(
        offset=LIMITS["rich_text_list"]["offset"]["min"] - 1
    ),
    "rich-text-list-border-negative": lambda: valid_rich_text_list(
        border=LIMITS["rich_text_list"]["border"]["min"] - 1
    ),
    "rich-text-list-border-too-large": lambda: valid_rich_text_list(
        border=LIMITS["rich_text_list"]["border"]["max"] + 1
    ),
    "rich-text-quote-border-negative": lambda: RichTextQuote(
        [RichText("A")], border=LIMITS["rich_text_quote"]["border"]["min"] - 1
    ),
    "rich-text-quote-border-too-large": lambda: RichTextQuote(
        [RichText("A")], border=LIMITS["rich_text_quote"]["border"]["max"] + 1
    ),
    "rich-text-preformatted-border-negative": lambda: RichTextCodeBlock(
        [RichText("A")], border=LIMITS["rich_text_preformatted"]["border"]["min"] - 1
    ),
    "rich-text-preformatted-border-too-large": lambda: RichTextCodeBlock(
        [RichText("A")], border=LIMITS["rich_text_preformatted"]["border"]["max"] + 1
    ),
    "video-thumbnail-url-too-long": lambda: video(
        thumbnail_url="x" * (LIMITS["video"]["thumbnail_url"]["max_length"] + 1)
    ),
    "video-url-too-long": lambda: video(
        video_url="x" * (LIMITS["video"]["video_url"]["max_length"] + 1)
    ),
    "video-title-url-too-long": lambda: video(
        title_url="x" * (LIMITS["video"]["title_url"]["max_length"] + 1)
    ),
    "video-provider-icon-url-too-long": lambda: video(
        provider_icon_url="x" * (LIMITS["video"]["provider_icon_url"]["max_length"] + 1)
    ),
    "view-external-id-too-long": lambda: HomeTabView(
        blocks=[DividerBlock()],
        external_id="x" * (LIMITS["view"]["external_id"]["max_length"] + 1),
    ),
    "attachment-missing-blocks": lambda: Attachment(),
}


def scalar_paths(value: object, prefix: tuple[str, ...] = ()) -> set[str]:
    if not isinstance(value, dict):
        return {".".join(prefix)}
    return {path for key, nested in value.items() for path in scalar_paths(nested, (*prefix, key))}


ERROR_CATEGORIES = {
    LengthError: "length-exceeded",
    RangeError: "out-of-range",
    MutualExclusivityError: "mutually-exclusive",
    TypeMismatchError: "type-mismatch",
    MissingRequiredError: "missing-required",
    InvalidUsageError: "invalid-usage",
}


@pytest.mark.parametrize(
    ("case_id", "expected_category"),
    [
        (case["id"], case["category"])
        for case in load_json(SPEC_ROOT / "fixtures" / "invalid" / "manifest.json")["cases"]
    ],
)
def test_invalid_case_category(case_id: str, expected_category: str) -> None:
    construction = INVALID_CASES[case_id]
    with pytest.raises(InvalidUsageError) as caught:
        construction()
    category = next(
        name for error_type, name in ERROR_CATEGORIES.items() if type(caught.value) is error_type
    )
    assert category == expected_category


def test_invalid_manifest_covers_every_scalar_limit() -> None:
    cases = load_json(SPEC_ROOT / "fixtures" / "invalid" / "manifest.json")["cases"]
    covered = {case["constraint"] for case in cases}
    assert scalar_paths(LIMITS) <= covered


def test_invalid_manifest_has_unique_case_ids_and_constraints() -> None:
    cases = load_json(SPEC_ROOT / "fixtures" / "invalid" / "manifest.json")["cases"]
    assert len({case["id"] for case in cases}) == len(cases)
    assert len({case["constraint"] for case in cases}) == len(cases)


def test_every_invalid_case_has_a_python_construction() -> None:
    cases = load_json(SPEC_ROOT / "fixtures" / "invalid" / "manifest.json")["cases"]
    assert set(INVALID_CASES) == {case["id"] for case in cases}


def test_declared_spec_version_matches_the_manifest() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    assert manifest["spec_version"] == slackblocks.SPEC_VERSION


def test_python_skiplist_is_empty() -> None:
    entries = [
        line
        for line in (REPO_ROOT / "python" / "conformance" / "skiplist.txt")
        .read_text(encoding="utf-8")
        .splitlines()
        if line and not line.startswith("#")
    ]
    assert entries == []
