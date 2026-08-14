"""Enforcement that the package's public API and ``spec/coverage.json`` agree.

Every public symbol exported from ``slackblocks`` must either map to a
capability registered in the shared coverage registry or appear in the
explicit exclusions list below. Adding a new public block, element, or
composition object without registering a capability (and fixture) for it
fails this suite.
"""

from __future__ import annotations

import inspect
import json
from pathlib import Path

import slackblocks

SPEC_ROOT = Path(__file__).resolve().parents[3] / "spec"

# Public symbols that intentionally have no entry in coverage.json because
# they do not, by themselves, produce Slack JSON.
EXCLUSIONS = {
    # Error types (validation outcomes, not JSON producers).
    "InvalidUsageError",
    "LengthError",
    "MissingRequiredError",
    "MutualExclusivityError",
    "RangeError",
    "TypeMismatchError",
    # Enums and Literal type aliases used in public signatures.
    "AlertLevel",
    "Color",
    "ContainerWidth",
    "ResponseType",
    "SlackIconName",
    "TaskStatus",
    "TextType",
    # Abstract bases / union aliases; their concrete subclasses are mapped.
    "Block",
    "Chart",
    "Element",
    "RichTextElement",
    "RichTextObject",
    "View",
    # Text helper: its concrete serialized forms are covered through the
    # PlainText and Markdown aliases (objects.plain_text / objects.markdown_text).
    "Text",
    # Python-only legacy attachment field helper; not part of the shared API.
    "Field",
    # Utilities and metadata.
    "block_kit_builder_url",
    "name",
    "SPEC_VERSION",
    # Artifact of `from __future__ import annotations` in __init__.py.
    "annotations",
}

# Mapping from public symbol to its capability key in coverage.json.
CAPABILITY_BY_SYMBOL = {
    "ActionsBlock": "blocks.actions",
    "AlertBlock": "blocks.alert",
    "CardBlock": "blocks.card",
    "CarouselBlock": "blocks.carousel",
    "ContainerBlock": "blocks.container",
    "ContextActionsBlock": "blocks.context_actions",
    "ContextBlock": "blocks.context",
    "DataTableBlock": "blocks.data_table",
    "DataVisualizationBlock": "blocks.data_visualization",
    "DividerBlock": "blocks.divider",
    "FileBlock": "blocks.file",
    "HeaderBlock": "blocks.header",
    "ImageBlock": "blocks.image",
    "InputBlock": "blocks.input",
    "MarkdownBlock": "blocks.markdown",
    "PlanBlock": "blocks.plan",
    "RichTextBlock": "blocks.rich_text",
    "SectionBlock": "blocks.section",
    "TableBlock": "blocks.table",
    "TaskCardBlock": "blocks.task_card",
    "VideoBlock": "blocks.video",
    # Chart constructors serialize inside the data-visualization block.
    "AreaChart": "blocks.data_visualization",
    "BarChart": "blocks.data_visualization",
    "LineChart": "blocks.data_visualization",
    "PieChart": "blocks.data_visualization",
    "Button": "elements.button",
    "ChannelMultiSelectMenu": "elements.multi_select_channels",
    "ChannelSelectMenu": "elements.select_channels",
    "CheckboxGroup": "elements.checkboxes",
    "ConversationMultiSelectMenu": "elements.multi_select_conversations",
    "ConversationSelectMenu": "elements.select_conversations",
    "DatePicker": "elements.date_picker",
    "DateTimePicker": "elements.datetime_picker",
    "EmailInput": "elements.email_input",
    "ExternalMultiSelectMenu": "elements.multi_select_external",
    "ExternalSelectMenu": "elements.select_external",
    "FeedbackButton": "objects.feedback_button",
    "FeedbackButtons": "elements.feedback_buttons",
    "FileInput": "elements.file_input",
    "IconButton": "elements.icon_button",
    "Image": "elements.image",
    "NumberInput": "elements.number_input",
    "OverflowMenu": "elements.overflow",
    "PlainTextInput": "elements.plain_text_input",
    "RadioButtonGroup": "elements.radio_buttons",
    "RichTextInput": "elements.rich_text_input",
    "StaticMultiSelectMenu": "elements.multi_select_static",
    "StaticSelectMenu": "elements.select_static",
    "TimePicker": "elements.time_picker",
    "URLInput": "elements.url_input",
    "URLSource": "elements.url_source",
    "UserMultiSelectMenu": "elements.multi_select_users",
    "UserSelectMenu": "elements.select_users",
    "WorkflowButton": "elements.workflow_button",
    "Attachment": "messages.attachment",
    "Message": "messages.message",
    "MessageResponse": "messages.message_response",
    "WebhookMessage": "messages.webhook_message",
    "AxisConfig": "objects.axis_config",
    "ChartSegment": "objects.chart_segment",
    "ColumnSettings": "objects.column_settings",
    "Confirm": "objects.confirmation",
    "ConfirmationDialogue": "objects.confirmation",
    "ConversationFilter": "objects.conversation_filter",
    "DataPoint": "objects.data_point",
    "DataSeries": "objects.data_series",
    "DispatchActionConfiguration": "objects.dispatch_action_configuration",
    "InputParameter": "objects.input_parameter",
    "Markdown": "objects.markdown_text",
    "Option": "objects.option",
    "OptionGroup": "objects.option_group",
    "PlainText": "objects.plain_text",
    "RawNumber": "objects.raw_number",
    "RawText": "objects.raw_text",
    "SlackFile": "objects.slack_file",
    "SlackIcon": "objects.slack_icon",
    "Trigger": "objects.trigger",
    "Workflow": "objects.workflow",
    "RichText": "rich_text.text",
    "RichTextChannel": "rich_text.channel",
    "RichTextCodeBlock": "rich_text.code_block",
    "RichTextEmoji": "rich_text.emoji",
    "RichTextLink": "rich_text.link",
    "RichTextList": "rich_text.list",
    "RichTextQuote": "rich_text.quote",
    "RichTextSection": "rich_text.section",
    "RichTextUser": "rich_text.user",
    "RichTextUserGroup": "rich_text.user_group",
    "HomeTabView": "views.home_tab",
    "Modal": "views.modal",
    "ModalView": "views.modal",
}


def public_symbols() -> set[str]:
    return {
        symbol
        for symbol in vars(slackblocks)
        if not symbol.startswith("_") and not inspect.ismodule(getattr(slackblocks, symbol))
    }


def test_every_public_symbol_is_mapped_or_explicitly_excluded() -> None:
    assert public_symbols() == set(CAPABILITY_BY_SYMBOL) | EXCLUSIONS


def test_no_symbol_is_both_mapped_and_excluded() -> None:
    assert not set(CAPABILITY_BY_SYMBOL) & EXCLUSIONS


def test_mapped_capabilities_and_coverage_registry_agree() -> None:
    coverage = json.loads((SPEC_ROOT / "coverage.json").read_text())["capabilities"]
    for symbol, capability in CAPABILITY_BY_SYMBOL.items():
        assert capability in coverage, f"{symbol} -> {capability} missing from coverage.json"
    assert set(coverage) == set(CAPABILITY_BY_SYMBOL.values())
