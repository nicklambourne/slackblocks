from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from slackblocks import (
    ActionsBlock,
    AlertBlock,
    AxisConfig,
    Button,
    CardBlock,
    CarouselBlock,
    ChartSegment,
    ConfirmationDialogue,
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
    RangeError,
    RawText,
    SectionBlock,
    SlackFile,
    StaticSelectMenu,
    Text,
    TypeMismatchError,
    VideoBlock,
)
from slackblocks.errors import InvalidUsageError

if TYPE_CHECKING:
    from collections.abc import Callable

REPO_ROOT = Path(__file__).resolve().parents[3]
SPEC_ROOT = REPO_ROOT / "spec"


def load_json(path: Path) -> dict:
    return json.loads(path.read_text())


def test_valid_manifest_covers_the_entire_fixture_corpus() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    fixture_ids = {fixture["id"] for fixture in manifest["fixtures"]}
    files = {
        path.relative_to(SPEC_ROOT / "fixtures" / "valid").with_suffix("").as_posix()
        for path in (SPEC_ROOT / "fixtures" / "valid").rglob("*.json")
    }
    assert manifest["spec_version"] == "1.0.0"
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


def test_every_valid_fixture_is_exercised_by_a_python_construction_test() -> None:
    manifest = load_json(SPEC_ROOT / "manifest.json")
    test_source = "\n".join(
        path.read_text() for path in (REPO_ROOT / "python" / "test").rglob("test_*.py")
    )
    missing = [
        fixture["id"]
        for fixture in manifest["fixtures"]
        if f"{fixture['id']}.json" not in test_source
    ]
    assert missing == []


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
    "file-input-max-files-too-small": lambda: FileInput(
        action_id="a", max_files=LIMITS["file_input"]["max_files"]["min"] - 1
    ),
    "file-input-max-files-too-large": lambda: FileInput(
        action_id="a", max_files=LIMITS["file_input"]["max_files"]["max"] + 1
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
    "video-alt-text-empty": lambda: video(alt_text=""),
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
    "static-select-options-and-groups": lambda: StaticSelectMenu(
        "a",
        options=[option()],
        option_groups=[object()],  # type: ignore[list-item]
    ),
    "image-url-and-slack-file": lambda: Image(
        image_url="https://example.com/image.png", slack_file=SlackFile(url=None, id="F123")
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


def test_python_skiplist_is_empty() -> None:
    entries = [
        line
        for line in (REPO_ROOT / "python" / "conformance" / "skiplist.txt").read_text().splitlines()
        if line and not line.startswith("#")
    ]
    assert entries == []
