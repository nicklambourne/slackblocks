/** Fluent builders for Block Kit composition and rich-text objects. */
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

interface PlainTextBuilderInput extends PlainTextOptions {
  text: string;
}

/** Starts a fluent plain-text composition object. */
export function PlainText(): FluentBuilder<PlainTextBuilderInput, TextObject> {
  return createFluentBuilder(({ text, ...options }, settings) =>
    createPlainText(text, options, settings),
  );
}

interface MarkdownBuilderInput extends MarkdownOptions {
  text: string;
}

/** Starts a fluent Slack mrkdwn composition object. */
export function Markdown(): FluentBuilder<MarkdownBuilderInput, TextObject> {
  return createFluentBuilder(({ text, ...options }, settings) =>
    createMarkdown(text, options, settings),
  );
}

/** Starts a fluent confirmation-dialog object. */
export function Confirmation(): FluentBuilder<ConfirmationInput, JsonObject> {
  return createFluentBuilder(createConfirmation);
}

/** Starts a fluent selectable option. */
export function Option(): FluentBuilder<OptionInput, JsonObject> {
  return createFluentBuilder(createOption);
}

/** Starts a fluent labelled option group. */
export function OptionGroup(): FluentBuilder<OptionGroupInput, JsonObject> {
  return createFluentBuilder(createOptionGroup, { collections: { options: "flat" } });
}

/** Starts a fluent conversation filter. */
export function ConversationFilter(): FluentBuilder<
  FirstInput<typeof createConversationFilter>,
  JsonObject
> {
  return createFluentBuilder(createConversationFilter, { collections: { include: "flat" } });
}

/** Starts a fluent dispatch-action configuration. */
export function DispatchActionConfiguration(): FluentBuilder<
  FirstInput<typeof createDispatchActionConfiguration>,
  JsonObject
> {
  return createFluentBuilder(createDispatchActionConfiguration, {
    collections: { triggerActionsOn: "flat" },
  });
}

/** Starts a fluent workflow input parameter. */
export function InputParameter(): FluentBuilder<
  FirstInput<typeof createInputParameter>,
  JsonObject
> {
  return createFluentBuilder(createInputParameter);
}

/** Starts a fluent workflow trigger. */
export function Trigger(): FluentBuilder<FirstInput<typeof createTrigger>, JsonObject> {
  return createFluentBuilder(createTrigger, {
    collections: { customizableInputParameters: "flat" },
  });
}

/** Starts a fluent workflow object. */
export function Workflow(): FluentBuilder<FirstInput<typeof createWorkflow>, JsonObject> {
  return createFluentBuilder(createWorkflow);
}

/** Starts a fluent Slack-hosted file reference. */
export function SlackFile(): FluentBuilder<FirstInput<typeof createSlackFile>, JsonObject> {
  return createFluentBuilder(createSlackFile);
}

/** Starts fluent display settings for one table column. */
export function ColumnSettings(): FluentBuilder<
  FirstInput<typeof createColumnSettings>,
  JsonObject
> {
  return createFluentBuilder(createColumnSettings);
}

/** Starts a fluent raw-text table cell. */
export function RawText(): FluentBuilder<{ text: string }, SlackObject<"raw_text">> {
  return createFluentBuilder(({ text }, settings) => createRawText(text, settings));
}

/** Starts a fluent sortable numeric table cell. */
export function RawNumber(): FluentBuilder<
  { value: number; text: string },
  SlackObject<"raw_number">
> {
  return createFluentBuilder(({ value, text }, settings) =>
    createRawNumber(value, text, settings),
  );
}

/** Starts a fluent Slack-provided icon. */
export function SlackIcon(): FluentBuilder<{ name: SlackIconName }, SlackObject<"icon">> {
  return createFluentBuilder(({ name }, settings) => createSlackIcon(name, settings));
}

/** Starts a fluent pie-chart segment. */
export function ChartSegment(): FluentBuilder<ChartSegmentInput, JsonObject> {
  return createFluentBuilder(createChartSegment);
}

/** Starts a fluent axis-chart data point. */
export function DataPoint(): FluentBuilder<DataPointInput, JsonObject> {
  return createFluentBuilder(createDataPoint);
}

/** Starts a fluent named chart series. */
export function DataSeries(): FluentBuilder<DataSeriesInput, JsonObject> {
  return createFluentBuilder(createDataSeries, { collections: { data: "flat" } });
}

/** Starts fluent axis labels and categories. */
export function AxisConfig(): FluentBuilder<AxisConfigInput, JsonObject> {
  return createFluentBuilder(createAxisConfig, { collections: { categories: "flat" } });
}

/** Starts a fluent pie chart. */
export function PieChart(): FluentBuilder<
  { segments: JsonObject[] },
  SlackObject<"pie">
> {
  return createFluentBuilder<{ segments: JsonObject[] }, SlackObject<"pie">>(
    ({ segments }, settings) => createPieChart(segments, settings),
    { collections: { segments: "flat" } },
  );
}

interface AxisChartBuilderInput {
  series: JsonObject[];
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

/** Starts a fluent grouped bar chart. */
export function BarChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"bar">> {
  return axisChart(createBarChart);
}

/** Starts a fluent area chart. */
export function AreaChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"area">> {
  return axisChart(createAreaChart);
}

/** Starts a fluent line chart. */
export function LineChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"line">> {
  return axisChart(createLineChart);
}

interface RichTextBuilderInput {
  text: string;
  style?: RichTextStyle;
}

/** Starts a fluent styled rich-text run. */
export function RichText(): FluentBuilder<RichTextBuilderInput, JsonObject> {
  return createFluentBuilder(({ text, style }, settings) =>
    createRichText(text, style, settings),
  );
}

interface RichTextMentionBuilderInput {
  id: string;
  style?: RichTextStyle;
}

/** Starts a fluent rich-text channel mention. */
export function RichTextChannel(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextChannel(id, style, settings),
  );
}

/** Starts a fluent rich-text emoji. */
export function RichTextEmoji(): FluentBuilder<{ name: string }, JsonObject> {
  return createFluentBuilder(({ name }, settings) => createRichTextEmoji(name, settings));
}

/** Starts a fluent rich-text link. */
export function RichTextLink(): FluentBuilder<FirstInput<typeof createRichTextLink>, JsonObject> {
  return createFluentBuilder(createRichTextLink);
}

/** Starts a fluent rich-text user mention. */
export function RichTextUser(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextUser(id, style, settings),
  );
}

/** Starts a fluent rich-text user-group mention. */
export function RichTextUserGroup(): FluentBuilder<RichTextMentionBuilderInput, JsonObject> {
  return createFluentBuilder(({ id, style }, settings) =>
    createRichTextUserGroup(id, style, settings),
  );
}

/** Starts a fluent paragraph-like rich-text section. */
export function RichTextSection(): FluentBuilder<{ elements: JsonObject[] }, JsonObject> {
  return createFluentBuilder<{ elements: JsonObject[] }, JsonObject>(
    ({ elements }, settings) => createRichTextSection(elements, settings),
    { collections: { elements: "flat" } },
  );
}

/** Starts a fluent ordered or bulleted rich-text list. */
export function RichTextList(): FluentBuilder<FirstInput<typeof createRichTextList>, JsonObject> {
  return createFluentBuilder(createRichTextList, { collections: { elements: "flat" } });
}

interface RichTextLayoutBuilderInput {
  elements: JsonObject[];
  border?: number;
}

/** Starts a fluent preformatted rich-text code block. */
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

/** Starts a fluent rich-text block quote. */
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
