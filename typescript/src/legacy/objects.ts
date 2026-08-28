/**
 * Composition-object factories used as fields inside blocks and elements.
 *
 * These helpers cover text, options, confirmations, workflow metadata, files,
 * table cells, icons, and chart data.
 *
 * @module objects
 */
import limits from "../../../spec/limits.json" with { type: "json" };

import {
  InvalidUsageError,
  LengthError,
  MissingRequiredError,
  MutualExclusivityError,
  OutOfRangeError,
  TypeMismatchError,
} from "../errors.js";
import { codePointLength, create, createObject, textValue } from "../internal.js";
import type { FactorySettings, JsonObject, SlackObject } from "../types.js";

/** A Slack plain-text or mrkdwn composition object. */
export type TextObject = SlackObject<"plain_text" | "mrkdwn">;

/** Text accepted by factories: a string or an existing Slack text object. */
export type TextLike = string | TextObject;

/** Optional behavior for {@link plainText}. */
export interface PlainTextOptions {
  /** Whether Slack should render emoji shortcodes. */
  emoji?: boolean;
}

/** Optional behavior for {@link mrkdwn}. */
export interface MarkdownOptions {
  /** Whether Slack should treat the text literally instead of auto-parsing links and mentions. */
  verbatim?: boolean;
}

const OPTION_URL_MAX_LENGTH = 3000;

function ensureLength(value: string, path: string, maximum: number): void {
  const size = codePointLength(value);
  if (size > maximum) {
    throw new LengthError(path, `${size} exceeds maximum ${maximum}`);
  }
}

/**
 * Creates a plain-text composition object.
 *
 * @param text - Text content.
 * @param options - Optional emoji behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `plain_text` object.
 * @throws InvalidUsageError when the text violates Slack's length constraints.
 */
export function plainText(
  text: string,
  options: PlainTextOptions = {},
  settings: FactorySettings = {},
): TextObject {
  return create("plain_text", { text, ...options }, settings) as TextObject;
}

/**
 * Creates a Slack mrkdwn composition object.
 *
 * @param text - Slack mrkdwn content.
 * @param options - Optional parsing behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `mrkdwn` object.
 * @throws InvalidUsageError when the text violates Slack's length constraints.
 */
export function mrkdwn(
  text: string,
  options: MarkdownOptions = {},
  settings: FactorySettings = {},
): TextObject {
  return create("mrkdwn", { text, ...options }, settings) as TextObject;
}

/**
 * Converts a string to a text object while preserving existing text objects.
 *
 * @param value - String or existing text object.
 * @param kind - Text kind used for strings. Defaults to `mrkdwn`.
 * @param settings - Per-call validation settings.
 * @returns A Slack text composition object.
 * @throws InvalidUsageError when the text violates Slack's length constraints.
 */
export function asText(
  value: TextLike,
  kind: "mrkdwn" | "plain_text" = "mrkdwn",
  settings: FactorySettings = {},
): TextObject {
  if (typeof value !== "string") return value;
  return kind === "plain_text" ? plainText(value, {}, settings) : mrkdwn(value, {}, settings);
}

/** Fields accepted by {@link confirmation}. */
export interface ConfirmationInput {
  /** Plain-text dialog title, up to 100 characters. */
  title: TextLike;
  /** Confirmation question, up to 300 characters. */
  text: TextLike;
  /** Plain-text confirm-button label, up to 30 characters. */
  confirm: TextLike;
  /** Plain-text cancel-button label, up to 30 characters. */
  deny: TextLike;
}

/**
 * Creates a confirmation dialog for an interactive element.
 *
 * @param input - Dialog title, question, and button labels.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack confirmation object.
 * @throws InvalidUsageError when a text field exceeds Slack's limit.
 */
export function confirmation(
  input: ConfirmationInput,
  settings: FactorySettings = {},
): JsonObject {
  for (const [field, maximum] of [
    ["title", limits.confirmation.title.max_length],
    ["text", limits.confirmation.text.max_length],
    ["confirm", limits.confirmation.confirm.max_length],
    ["deny", limits.confirmation.deny.max_length],
  ] as const) {
    const value = textValue(input[field]);
    if (value !== undefined) ensureLength(value, field, maximum);
  }
  return createObject(
    {
      title: asText(input.title, "plain_text", settings),
      text: asText(input.text, "mrkdwn", settings),
      confirm: asText(input.confirm, "plain_text", settings),
      deny: asText(input.deny, "plain_text", settings),
    },
    settings,
  );
}

/** Fields accepted by {@link option}. */
export interface OptionInput {
  /** Plain-text option label, up to 75 characters. */
  text: TextLike;
  /** Application-defined value, up to 150 characters. */
  value: string;
  /** Optional plain-text supporting copy, up to 75 characters. */
  description?: TextLike;
  /** Optional destination URL for overflow menus. */
  url?: string;
}

/**
 * Creates one option for a select, checkbox, radio, or overflow element.
 *
 * @param input - Option label, value, and optional description or URL.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack option object.
 * @throws InvalidUsageError when text or values exceed Slack's limits.
 */
export function option(input: OptionInput, settings: FactorySettings = {}): JsonObject {
  const text = asText(input.text, "plain_text", settings);
  ensureLength(textValue(text) ?? "", "option.text", limits.option.text.max_length);
  ensureLength(input.value, "option.value", limits.option.value.max_length);
  if (input.description !== undefined) {
    ensureLength(
      textValue(input.description) ?? "",
      "option.description",
      limits.option.description.max_length,
    );
  }
  if (input.url !== undefined) {
    ensureLength(input.url, "option.url", OPTION_URL_MAX_LENGTH);
  }
  return createObject(
    {
      ...input,
      text,
      description:
        input.description === undefined
          ? undefined
          : asText(input.description, "plain_text", settings),
    },
    settings,
  );
}

/** Fields accepted by {@link optionGroup}. */
export interface OptionGroupInput {
  /** Plain-text group label, up to 75 characters. */
  label: TextLike;
  /** Between one and 100 option objects. */
  options: JsonObject[];
}

/**
 * Creates a labelled group of options for a static select menu.
 *
 * @param input - Group label and option list.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack option-group object.
 * @throws InvalidUsageError when the label or option count violates Slack's limits.
 */
export function optionGroup(
  input: OptionGroupInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(
    textValue(asText(input.label, "plain_text", settings)) ?? "",
    "optionGroup.label",
    limits.option_group.label.max_length,
  );
  if (
    input.options.length < limits.option_group.options.min_items ||
    input.options.length > limits.option_group.options.max_items
  ) {
    throw new LengthError(
      "optionGroup.options",
      `expected between ${limits.option_group.options.min_items} and ` +
        `${limits.option_group.options.max_items} options, received ${input.options.length}`,
    );
  }
  return createObject(
    { ...input, label: asText(input.label, "plain_text", settings) },
    settings,
  );
}

/**
 * Creates filters for a conversation select menu.
 *
 * @param input - Conversation types and optional exclusions. At least one field is required.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack conversation-filter object.
 * @throws MissingRequiredError when no filter field is supplied.
 */
export function conversationFilter(
  input: {
    /** Conversation kinds to include, such as `im`, `mpim`, `private`, or `public`. */
    include?: string[];
    /** Exclude externally shared conversations. */
    excludeExternalSharedChannels?: boolean;
    /** Exclude direct messages with bots. */
    excludeBotUsers?: boolean;
  },
  settings: FactorySettings = {},
): JsonObject {
  if (Object.values(input).every((value) => value === undefined)) {
    throw new MissingRequiredError("conversationFilter", "expected at least one filter");
  }
  return createObject({ ...input }, settings);
}

/**
 * Creates a dispatch-action configuration for an input element.
 *
 * @param input - Interaction events that should dispatch immediately.
 * @param settings - Per-call validation settings.
 * @returns A Slack dispatch-action configuration object.
 */
export function dispatchActionConfiguration(
  input: {
    /** Events such as `on_enter_pressed` or `on_character_entered`. */
    triggerActionsOn: string[];
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

/**
 * Creates a customizable workflow input parameter.
 *
 * @param input - Workflow parameter name and value.
 * @param settings - Per-call validation settings.
 * @returns A Slack workflow input-parameter object.
 */
export function inputParameter(
  input: {
    /** Workflow parameter name. */
    name: string;
    /** Value passed to the workflow. */
    value: string;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

/**
 * Creates a workflow trigger definition.
 *
 * @param input - Trigger URL and optional customizable parameters.
 * @param settings - Per-call validation settings.
 * @returns A Slack workflow-trigger object.
 */
export function trigger(
  input: {
    /** Slack workflow trigger URL. */
    url: string;
    /** Optional parameters created with `inputParameter`. */
    customizableInputParameters?: JsonObject[];
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

/**
 * Wraps a trigger for use by a workflow button.
 *
 * @param input - Workflow trigger object.
 * @param settings - Per-call validation settings.
 * @returns A Slack workflow object.
 */
export function workflow(
  input: {
    /** Trigger created with `trigger`. */
    trigger: JsonObject;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

/**
 * Creates a Slack-hosted image reference.
 *
 * @param input - Exactly one Slack file ID or Slack file URL.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack file-reference object.
 * @throws InvalidUsageError when both or neither source is supplied.
 */
export function slackFile(
  input: {
    /** Slack file identifier. */
    id?: string;
    /** Slack-hosted file URL. */
    url?: string;
  },
  settings: FactorySettings = {},
): JsonObject {
  if (input.id === undefined && input.url === undefined) {
    throw new MissingRequiredError("slackFile", "expected id or url");
  }
  if (input.id !== undefined && input.url !== undefined) {
    throw new MutualExclusivityError("slackFile", "id and url cannot be provided together");
  }
  return createObject(input, settings);
}

/**
 * Creates display settings for one table column.
 *
 * @param input - Horizontal alignment and optional wrapping behavior.
 * @param settings - Per-call validation settings.
 * @returns A Slack table column-settings object.
 */
export function columnSettings(
  input: {
    /** Horizontal cell alignment. */
    align?: "left" | "center" | "right";
    /** Whether long cell content wraps. */
    isWrapped?: boolean;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

/**
 * Creates an unformatted text cell for a table or data table.
 *
 * @param text - Cell content.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `raw_text` object.
 * @throws InvalidUsageError when the cell text violates Slack's constraints.
 */
export function rawText(
  text: string,
  settings: FactorySettings = {},
): SlackObject<"raw_text"> {
  return create("raw_text", { text }, settings);
}

/**
 * Creates a sortable numeric cell for a data table.
 *
 * @param value - Finite number used for sorting.
 * @param text - Human-readable cell text.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `raw_number` object.
 * @throws TypeMismatchError when `value` is not finite.
 */
export function rawNumber(
  value: number,
  text: string,
  settings: FactorySettings = {},
): SlackObject<"raw_number"> {
  if (!Number.isFinite(value)) {
    throw new TypeMismatchError("rawNumber.value", "expected a finite number");
  }
  return create("raw_number", { value, text }, settings);
}

/** Slack-provided icon name accepted by {@link slackIcon}. */
export type SlackIconName =
  | "archive"
  | "book"
  | "bookmark"
  | "bot"
  | "bug"
  | "calendar"
  | "call"
  | "caret-left"
  | "caret-right"
  | "check"
  | "clipboard"
  | "code"
  | "comment"
  | "compass"
  | "copy"
  | "cube"
  | "download"
  | "edit"
  | "email"
  | "eye-closed"
  | "eye-open"
  | "file"
  | "flag"
  | "folder"
  | "gear"
  | "globe"
  | "heart"
  | "help"
  | "image"
  | "info"
  | "key"
  | "lightbulb"
  | "link"
  | "map"
  | "mobile"
  | "new-window"
  | "pin"
  | "plus"
  | "refine"
  | "refresh"
  | "rocket"
  | "save"
  | "screen"
  | "share"
  | "sparkle"
  | "star"
  | "star-filled"
  | "tag"
  | "thumbs-down"
  | "thumbs-up"
  | "trash"
  | "upload"
  | "user"
  | "warning";

const SLACK_ICON_NAMES = new Set<SlackIconName>([
  "archive", "book", "bookmark", "bot", "bug", "calendar", "call", "caret-left",
  "caret-right", "check", "clipboard", "code", "comment", "compass", "copy", "cube",
  "download", "edit", "email", "eye-closed", "eye-open", "file", "flag", "folder",
  "gear", "globe", "heart", "help", "image", "info", "key", "lightbulb", "link",
  "map", "mobile", "new-window", "pin", "plus", "refine", "refresh", "rocket",
  "save", "screen", "share", "sparkle", "star", "star-filled", "tag", "thumbs-down",
  "thumbs-up", "trash", "upload", "user", "warning",
]);

/**
 * Creates a Slack-provided icon for a card.
 *
 * @param name - One of Slack's supported icon names.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `icon` object.
 * @throws TypeMismatchError when the icon name is unsupported at runtime.
 */
export function slackIcon(
  name: SlackIconName,
  settings: FactorySettings = {},
): SlackObject<"icon"> {
  if (!SLACK_ICON_NAMES.has(name)) {
    throw new TypeMismatchError("slackIcon.name", `unknown icon ${name}`);
  }
  return create("icon", { name }, settings);
}

/** Fields accepted by {@link chartSegment}. */
export interface ChartSegmentInput {
  /** Segment label, up to 20 characters. */
  label: string;
  /** Positive finite segment value. */
  value: number;
}

/**
 * Creates one labelled segment for a pie chart.
 *
 * @param input - Segment label and positive value.
 * @param settings - Per-call validation settings.
 * @returns A validated chart-segment object.
 * @throws InvalidUsageError when the label or value violates chart constraints.
 */
export function chartSegment(
  input: ChartSegmentInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(
    input.label,
    "chartSegment.label",
    limits.data_visualization.segment.label.max_length,
  );
  if (!Number.isFinite(input.value)) {
    throw new TypeMismatchError("chartSegment.value", "expected a finite number");
  }
  if (input.value <= limits.data_visualization.segment.value.exclusive_min) {
    throw new OutOfRangeError("chartSegment.value", "expected a value greater than 0");
  }
  return createObject({ ...input }, settings);
}

/** Fields accepted by {@link dataPoint}. */
export interface DataPointInput {
  /** Category label, up to 20 characters. */
  label: string;
  /** Finite numeric value. */
  value: number;
}

/**
 * Creates one labelled data point for an axis-based chart.
 *
 * @param input - Category label and finite value.
 * @param settings - Per-call validation settings.
 * @returns A validated data-point object.
 * @throws InvalidUsageError when the label or value violates chart constraints.
 */
export function dataPoint(
  input: DataPointInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(input.label, "dataPoint.label", limits.data_visualization.point_label.max_length);
  if (!Number.isFinite(input.value)) {
    throw new TypeMismatchError("dataPoint.value", "expected a finite number");
  }
  return createObject({ ...input }, settings);
}

/** Fields accepted by {@link dataSeries}. */
export interface DataSeriesInput {
  /** Unique series name, up to 20 characters. */
  name: string;
  /** Between one and 20 points created with `dataPoint`. */
  data: JsonObject[];
}

/**
 * Creates one named series for an axis-based chart.
 *
 * @param input - Series name and ordered data points.
 * @param settings - Per-call validation settings.
 * @returns A validated chart-series object.
 * @throws InvalidUsageError when the name or point count violates chart constraints.
 */
export function dataSeries(
  input: DataSeriesInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(input.name, "dataSeries.name", limits.data_visualization.series_name.max_length);
  if (
    input.data.length < limits.data_visualization.data.min_items ||
    input.data.length > limits.data_visualization.data.max_items
  ) {
    throw new LengthError(
      "dataSeries.data",
      `expected between ${limits.data_visualization.data.min_items} and ` +
        `${limits.data_visualization.data.max_items} data points`,
    );
  }
  return createObject({ ...input }, settings);
}

/** Fields accepted by {@link axisConfig}. */
export interface AxisConfigInput {
  /** Unique category labels, in display order. */
  categories: string[];
  /** Optional horizontal-axis label, up to 50 characters. */
  xLabel?: string;
  /** Optional vertical-axis label, up to 50 characters. */
  yLabel?: string;
}

/**
 * Creates category and label configuration for an axis-based chart.
 *
 * @param input - Ordered categories and optional axis labels.
 * @param settings - Per-call validation settings.
 * @returns A validated chart-axis configuration object.
 * @throws InvalidUsageError when labels are duplicated or exceed Slack's limits.
 */
export function axisConfig(
  input: AxisConfigInput,
  settings: FactorySettings = {},
): JsonObject {
  if (
    input.categories.length < limits.data_visualization.categories.min_items ||
    input.categories.length > limits.data_visualization.categories.max_items
  ) {
    throw new LengthError(
      "axisConfig.categories",
      `expected between ${limits.data_visualization.categories.min_items} and ` +
        `${limits.data_visualization.categories.max_items} categories`,
    );
  }
  input.categories.forEach((category) =>
    ensureLength(
      category,
      "axisConfig.category",
      limits.data_visualization.category_label.max_length,
    ),
  );
  if (new Set(input.categories).size !== input.categories.length) {
    throw new InvalidUsageError("axisConfig.categories", "expected unique labels");
  }
  if (input.xLabel !== undefined) {
    ensureLength(input.xLabel, "axisConfig.xLabel", limits.data_visualization.axis_label.max_length);
  }
  if (input.yLabel !== undefined) {
    ensureLength(input.yLabel, "axisConfig.yLabel", limits.data_visualization.axis_label.max_length);
  }
  return createObject({ ...input }, settings);
}

/**
 * Creates a pie chart.
 *
 * @param segments - One to 12 segments created with `chartSegment`.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `pie` chart object.
 * @throws InvalidUsageError when segment data violates Slack's chart constraints.
 */
export function pieChart(
  segments: JsonObject[],
  settings: FactorySettings = {},
): SlackObject<"pie"> {
  return create("pie", { segments }, settings);
}

function axisChart<Type extends "bar" | "area" | "line">(
  type: Type,
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings,
): SlackObject<Type> {
  // Series shape, axis categories, and name uniqueness are enforced by the
  // shared series-chart validator invoked through `create`.
  return create(type, { series, axisConfig: axis }, settings);
}

/**
 * Creates a grouped bar chart.
 *
 * @param series - One to 12 uniquely named data series.
 * @param axis - Axis configuration whose categories match every series.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `bar` chart object.
 * @throws InvalidUsageError when series and axis categories do not match.
 */
export function barChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"bar"> {
  return axisChart("bar", series, axis, settings);
}

/**
 * Creates an area chart.
 *
 * @param series - One to 12 uniquely named data series.
 * @param axis - Axis configuration whose categories match every series.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `area` chart object.
 * @throws InvalidUsageError when series and axis categories do not match.
 */
export function areaChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"area"> {
  return axisChart("area", series, axis, settings);
}

/**
 * Creates a line chart.
 *
 * @param series - One to 12 uniquely named data series.
 * @param axis - Axis configuration whose categories match every series.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `line` chart object.
 * @throws InvalidUsageError when series and axis categories do not match.
 */
export function lineChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"line"> {
  return axisChart("line", series, axis, settings);
}
