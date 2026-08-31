/**
 * Fluent builders for Block Kit composition and rich-text objects.
 *
 * @module objects
 */
import {
  areaChart as createAreaChart,
  axisConfig as createAxisConfig,
  barChart as createBarChart,
  chartSegment as createChartSegment,
  columnSettings as createColumnSettings,
  confirmation as createConfirmation,
  conversationFilter as createConversationFilter,
  dataPoint as createDataPoint,
  dataSeries as createDataSeries,
  dispatchActionConfiguration as createDispatchActionConfiguration,
  inputParameter as createInputParameter,
  lineChart as createLineChart,
  mrkdwn as createMarkdown,
  option as createOption,
  optionGroup as createOptionGroup,
  pieChart as createPieChart,
  plainText as createPlainText,
  rawNumber as createRawNumber,
  rawText as createRawText,
  slackFile as createSlackFile,
  slackIcon as createSlackIcon,
  trigger as createTrigger,
  workflow as createWorkflow,
  type AxisConfigInput,
  type ChartSegmentInput,
  type ConfirmationInput,
  type DataPointInput,
  type DataSeriesInput,
  type MarkdownOptions,
  type OptionGroupInput,
  type OptionInput,
  type PlainTextOptions,
  type SlackIconName,
  type TextObject,
} from "../legacy/objects.js";
import {
  richText as createRichText,
  richTextChannel as createRichTextChannel,
  richTextCodeBlock as createRichTextCodeBlock,
  richTextEmoji as createRichTextEmoji,
  richTextLink as createRichTextLink,
  richTextList as createRichTextList,
  richTextQuote as createRichTextQuote,
  richTextSection as createRichTextSection,
  richTextUser as createRichTextUser,
  richTextUserGroup as createRichTextUserGroup,
  type RichTextStyle,
} from "../legacy/rich-text.js";
import type { FactorySettings, JsonObject, SlackObject } from "../types.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];

/** Values configured through {@link PlainText}. */
interface PlainTextBuilderInput extends PlainTextOptions {
  /** Text to display. */
  text: string;
}

/**
 * Creates a plain-text composition object with no Slack formatting. Use it for
 * labels and other fields that require `plain_text`; emoji shortcodes can be
 * converted by enabling the optional `emoji` setting.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>.
 */
export function PlainText(): FluentBuilder<PlainTextBuilderInput, TextObject> {
  return createFluentBuilder(({ text, ...options }, settings) =>
    createPlainText(text, options, settings),
  );
}

/** Values configured through {@link Markdown}. */
interface MarkdownBuilderInput extends MarkdownOptions {
  /** Slack mrkdwn text to display. */
  text: string;
}

/**
 * Creates a text composition object rendered with Slack's `mrkdwn` syntax. Use
 * `verbatim()` when links, mentions, and other tokens should remain literal
 * rather than being parsed automatically.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>.
 */
export function Markdown(): FluentBuilder<MarkdownBuilderInput, TextObject> {
  return createFluentBuilder(({ text, ...options }, settings) =>
    createMarkdown(text, options, settings),
  );
}

/**
 * Creates a confirmation dialog that asks a user to approve or cancel an
 * interactive action. Configure its title, explanatory text, and the labels on
 * both the confirm and deny buttons.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object>.
 */
export function Confirmation(): FluentBuilder<ConfirmationInput, JsonObject> {
  return createFluentBuilder(createConfirmation);
}

/**
 * Creates one selectable item for a select menu, multi-select menu, checkbox
 * group, radio-button group, or overflow menu. The application-defined `value`
 * is returned when the user chooses the option.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/option-object>.
 */
export function Option(): FluentBuilder<OptionInput, JsonObject> {
  return createFluentBuilder(createOption);
}

/**
 * Creates a labelled group containing between one and 100 selectable options.
 * Option groups can organize choices in static single-select and multi-select
 * menus.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object>.
 */
export function OptionGroup(): FluentBuilder<OptionGroupInput, JsonObject> {
  return createFluentBuilder(createOptionGroup, { collections: { options: "flat" } });
}

/**
 * Creates a filter for conversation single-select and multi-select elements.
 * Include selected conversation kinds and optionally exclude externally shared
 * channels or direct messages with bots; at least one filter field is required.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object>.
 */
export function ConversationFilter(): FluentBuilder<
  FirstInput<typeof createConversationFilter>,
  JsonObject
> {
  return createFluentBuilder(createConversationFilter, { collections: { include: "flat" } });
}

/**
 * Creates a dispatch-action configuration for an input element. Select the input
 * events, such as Enter being pressed or a character being entered, that should
 * immediately send a `block_actions` payload to the application.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object>.
 */
export function DispatchActionConfiguration(): FluentBuilder<
  FirstInput<typeof createDispatchActionConfiguration>,
  JsonObject
> {
  return createFluentBuilder(createDispatchActionConfiguration, {
    collections: { triggerActionsOn: "flat" },
  });
}

/**
 * Creates one customizable input parameter passed to a Slack workflow trigger.
 * The parameter name must match an input defined by the workflow, and its value
 * is supplied when the workflow button is used.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.
 */
export function InputParameter(): FluentBuilder<
  FirstInput<typeof createInputParameter>,
  JsonObject
> {
  return createFluentBuilder(createInputParameter);
}

/**
 * Creates the link-trigger definition nested inside a workflow object. Supply the
 * trigger URL generated by Slack and optionally add customizable input parameters.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.
 */
export function Trigger(): FluentBuilder<FirstInput<typeof createTrigger>, JsonObject> {
  return createFluentBuilder(createTrigger, {
    collections: { customizableInputParameters: "flat" },
  });
}

/**
 * Creates a workflow composition object for a workflow button. It wraps a trigger
 * built with {@link Trigger}, including any customizable values the application
 * wants to pass when the user launches the workflow.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.
 */
export function Workflow(): FluentBuilder<FirstInput<typeof createWorkflow>, JsonObject> {
  return createFluentBuilder(createWorkflow);
}

/**
 * Creates a Slack-hosted image reference for an image block or image element.
 * Supply exactly one Slack file ID or Slack-hosted file URL; the two source forms
 * are mutually exclusive.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object>.
 */
export function SlackFile(): FluentBuilder<FirstInput<typeof createSlackFile>, JsonObject> {
  return createFluentBuilder(createSlackFile);
}

/**
 * Creates display settings for one column in a table block. Configure horizontal
 * alignment and whether long cell content should wrap within the column.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.
 */
export function ColumnSettings(): FluentBuilder<
  FirstInput<typeof createColumnSettings>,
  JsonObject
> {
  return createFluentBuilder(createColumnSettings);
}

/**
 * Creates an unformatted `raw_text` cell for a table or data table. Slack displays
 * the supplied text literally, without applying `mrkdwn` or rich-text formatting.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.
 */
export function RawText(): FluentBuilder<{ text: string }, SlackObject<"raw_text">> {
  return createFluentBuilder(({ text }, settings) => createRawText(text, settings));
}

/**
 * Creates a numeric data-table cell with separate machine-sortable and
 * human-readable values. The numeric value must be finite, while `text()` controls
 * what Slack displays to the reader.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>.
 */
export function RawNumber(): FluentBuilder<
  { value: number; text: string },
  SlackObject<"raw_number">
> {
  return createFluentBuilder(({ value, text }, settings) =>
    createRawNumber(value, text, settings),
  );
}

/**
 * Creates a named icon supplied and rendered by Slack for use in a card block.
 * Choose one of the supported {@link SlackIconName} values instead of supplying
 * an image URL.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/card-block>.
 */
export function SlackIcon(): FluentBuilder<{ name: SlackIconName }, SlackObject<"icon">> {
  return createFluentBuilder(({ name }, settings) => createSlackIcon(name, settings));
}

/**
 * Creates one labelled, positive-valued segment in a pie chart. Segment labels
 * are limited to 20 characters and each value must be a finite number greater
 * than zero.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function ChartSegment(): FluentBuilder<ChartSegmentInput, JsonObject> {
  return createFluentBuilder(createChartSegment);
}

/**
 * Creates one labelled numeric point in a bar, area, or line chart series. Its
 * label must correspond to one of the chart's axis categories and its value must
 * be finite.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function DataPoint(): FluentBuilder<DataPointInput, JsonObject> {
  return createFluentBuilder(createDataPoint);
}

/**
 * Creates a named series containing between one and 20 data points for a bar,
 * area, or line chart. Add points in display order with repeated `data()` calls.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function DataSeries(): FluentBuilder<DataSeriesInput, JsonObject> {
  return createFluentBuilder(createDataSeries, { collections: { data: "flat" } });
}

/**
 * Creates category labels and optional axis titles for a bar, area, or line chart.
 * Categories must be unique and match the labels represented in every data
 * series.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function AxisConfig(): FluentBuilder<AxisConfigInput, JsonObject> {
  return createFluentBuilder(createAxisConfig, { collections: { categories: "flat" } });
}

/**
 * Creates a pie chart containing between one and 12 labelled segments for a data
 * visualization block. Add built segments or {@link ChartSegment} builders with
 * repeated `segments()` calls.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function PieChart(): FluentBuilder<
  { segments: JsonObject[] },
  SlackObject<"pie">
> {
  return createFluentBuilder<{ segments: JsonObject[] }, SlackObject<"pie">>(
    ({ segments }, settings) => createPieChart(segments, settings),
    { collections: { segments: "flat" } },
  );
}

/** Values configured through {@link BarChart}, {@link AreaChart}, or {@link LineChart}. */
interface AxisChartBuilderInput {
  /** One or more named data series to plot. */
  series: JsonObject[];
  /** Axis labels and category configuration. */
  axisConfig: JsonObject;
}

function axisChart<Type extends "bar" | "area" | "line">(
  factory: (
    series: JsonObject[],
    axis: JsonObject,
    settings?: FactorySettings,
  ) => SlackObject<Type>,
): FluentBuilder<AxisChartBuilderInput, SlackObject<Type>> {
  return createFluentBuilder<AxisChartBuilderInput, SlackObject<Type>>(
    ({ series, axisConfig }, settings) => factory(series, axisConfig, settings),
    { collections: { series: "flat" } },
  );
}

/**
 * Creates a grouped bar chart for a data visualization block. Supply one or more
 * named series and an axis configuration whose categories match every point in
 * those series.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function BarChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"bar">> {
  return axisChart(createBarChart);
}

/**
 * Creates a layered area chart for a data visualization block. Supply one or more
 * named series and an axis configuration whose categories match every point in
 * those series.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function AreaChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"area">> {
  return axisChart(createAreaChart);
}

/**
 * Creates a line chart for a data visualization block. Supply one or more named
 * series and an axis configuration whose categories match every point in those
 * series.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function LineChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"line">> {
  return axisChart(createLineChart);
}

/** Values configured through {@link RichText}. */
interface RichTextBuilderInput {
  /** Text content for this run. */
  text: string;
  /** Optional Slack rich-text styling. */
  style?: RichTextStyle;
}

/**
 * Creates the core text run used by Slack's structured rich-text API. Apply bold,
 * italic, strikethrough, or code styling and place the result inside a rich-text
 * section, list, code block, or quote.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#text-element-type>.
 */
export function RichText(): FluentBuilder<RichTextBuilderInput, JsonObject> {
  return createFluentBuilder(({ text, style }, settings) =>
    createRichText(text, style, settings),
  );
}

/** Values configured through rich-text mention builders. */
interface RichTextMentionBuilderInput {
  /** Slack channel, user, or user-group identifier. */
  id: string;
  /** Optional Slack rich-text styling. */
  style?: RichTextStyle;
}

/**
 * Creates a structured rich-text mention for a Slack channel, such as `#general`.
 * Slack resolves the supplied channel ID when rendering the containing rich-text
 * block.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#channel-element-type>.
 */
export function RichTextChannel(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextChannel(id, style, settings),
  );
}

/**
 * Creates a structured rich-text emoji using a built-in Slack name or a custom
 * workspace emoji name. Supply the name without surrounding colon characters.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#emoji-element-type>.
 */
export function RichTextEmoji(): FluentBuilder<{ name: string }, JsonObject> {
  return createFluentBuilder(({ name }, settings) => createRichTextEmoji(name, settings));
}

/**
 * Creates a structured rich-text link with a destination URL and optional display
 * text or style. When text is omitted, Slack displays the URL itself.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#link-element-type>.
 */
export function RichTextLink(): FluentBuilder<FirstInput<typeof createRichTextLink>, JsonObject> {
  return createFluentBuilder(createRichTextLink);
}

/**
 * Creates a structured rich-text mention for one Slack user. Slack resolves the
 * supplied user ID to the appropriate display name when rendering the containing
 * rich-text block.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-element-type>.
 */
export function RichTextUser(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextUser(id, style, settings),
  );
}

/**
 * Creates a structured rich-text mention for a Slack user group. Slack resolves
 * the supplied user-group ID when rendering the containing rich-text block.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#usergroup-element-type>.
 */
export function RichTextUserGroup(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextUserGroup(id, style, settings),
  );
}

/**
 * Creates the basic paragraph-like container for structured rich-text elements.
 * Add text runs, links, emoji, and mentions with `elements()` before placing the
 * section in a rich-text block or higher-level rich-text layout.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_section>.
 */
export function RichTextSection(): FluentBuilder<{ elements: JsonObject[] }, JsonObject> {
  return createFluentBuilder<{ elements: JsonObject[] }, JsonObject>(
    ({ elements }, settings) => createRichTextSection(elements, settings),
    { collections: { elements: "flat" } },
  );
}

/**
 * Creates an ordered or bulleted list of rich-text sections. Configure indentation
 * and list style, then add each section with `elements()` in display order.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_list>.
 */
export function RichTextList(): FluentBuilder<FirstInput<typeof createRichTextList>, JsonObject> {
  return createFluentBuilder(createRichTextList, { collections: { elements: "flat" } });
}

/** Values configured through rich-text layout builders. */
interface RichTextLayoutBuilderInput {
  /** Rich-text elements displayed inside the layout. */
  elements: JsonObject[];
  /** Optional border width. */
  border?: number;
}

/**
 * Creates a preformatted rich-text code block, roughly equivalent to a fenced code
 * block in Markdown. Add structured rich-text elements as its content and
 * optionally configure the surrounding border.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_preformatted>.
 */
export function RichTextCodeBlock(): FluentBuilder<RichTextLayoutBuilderInput, JsonObject> {
  return createFluentBuilder<RichTextLayoutBuilderInput, JsonObject>(
    ({ elements, border }, settings) =>
      createRichTextCodeBlock(
        elements,
        border === undefined ? {} : { border },
        settings,
      ),
    { collections: { elements: "flat" } },
  );
}

/**
 * Creates a rich-text quotation rendered with a vertical bar beside its content.
 * Add structured rich-text elements in display order and optionally configure the
 * surrounding border.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_quote>.
 */
export function RichTextQuote(): FluentBuilder<RichTextLayoutBuilderInput, JsonObject> {
  return createFluentBuilder<RichTextLayoutBuilderInput, JsonObject>(
    ({ elements, border }, settings) =>
      createRichTextQuote(
        elements,
        border === undefined ? {} : { border },
        settings,
      ),
    { collections: { elements: "flat" } },
  );
}
