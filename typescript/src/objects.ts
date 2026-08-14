import {
  InvalidUsageError,
  LengthError,
  MissingRequiredError,
  MutualExclusivityError,
  RangeError,
  TypeMismatchError,
} from "./errors.js";
import { create, createObject, textValue } from "./internal.js";
import type { FactorySettings, JsonObject, SlackObject } from "./types.js";

export type TextObject = SlackObject<"plain_text" | "mrkdwn">;
export type TextLike = string | TextObject;

export interface PlainTextOptions {
  emoji?: boolean;
}

export interface MarkdownOptions {
  verbatim?: boolean;
}

function ensureLength(value: string, path: string, maximum: number): void {
  if (value.length > maximum) {
    throw new LengthError(path, `${value.length} exceeds maximum ${maximum}`);
  }
}

export function plainText(
  text: string,
  options: PlainTextOptions = {},
  settings: FactorySettings = {},
): TextObject {
  return create("plain_text", { text, ...options }, settings) as TextObject;
}

export function mrkdwn(
  text: string,
  options: MarkdownOptions = {},
  settings: FactorySettings = {},
): TextObject {
  return create("mrkdwn", { text, ...options }, settings) as TextObject;
}

export function asText(
  value: TextLike,
  kind: "mrkdwn" | "plain_text" = "mrkdwn",
  settings: FactorySettings = {},
): TextObject {
  if (typeof value !== "string") return value;
  return kind === "plain_text" ? plainText(value, {}, settings) : mrkdwn(value, {}, settings);
}

export interface ConfirmationInput {
  title: TextLike;
  text: TextLike;
  confirm: TextLike;
  deny: TextLike;
}

export function confirmation(
  input: ConfirmationInput,
  settings: FactorySettings = {},
): JsonObject {
  for (const [field, maximum] of [
    ["title", 100],
    ["text", 300],
    ["confirm", 30],
    ["deny", 30],
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

export interface OptionInput {
  text: TextLike;
  value: string;
  description?: TextLike;
  url?: string;
}

export function option(input: OptionInput, settings: FactorySettings = {}): JsonObject {
  const text = asText(input.text, "plain_text", settings);
  ensureLength(textValue(text) ?? "", "option.text", 75);
  ensureLength(input.value, "option.value", 75);
  if (input.description !== undefined) {
    ensureLength(textValue(input.description) ?? "", "option.description", 75);
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

export interface OptionGroupInput {
  label: TextLike;
  options: JsonObject[];
}

export function optionGroup(
  input: OptionGroupInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(
    textValue(asText(input.label, "plain_text", settings)) ?? "",
    "optionGroup.label",
    75,
  );
  if (input.options.length < 1 || input.options.length > 100) {
    throw new LengthError(
      "optionGroup.options",
      `expected between 1 and 100 options, received ${input.options.length}`,
    );
  }
  return createObject(
    { ...input, label: asText(input.label, "plain_text", settings) },
    settings,
  );
}

export function conversationFilter(
  input: { include?: string[]; excludeExternalSharedChannels?: boolean; excludeBotUsers?: boolean },
  settings: FactorySettings = {},
): JsonObject {
  if (Object.values(input).every((value) => value === undefined)) {
    throw new MissingRequiredError("conversationFilter", "expected at least one filter");
  }
  return createObject({ ...input }, settings);
}

export function dispatchActionConfiguration(
  input: { triggerActionsOn: string[] },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

export function inputParameter(
  input: { name: string; value: string },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

export function trigger(
  input: { url: string; customizableInputParameters?: JsonObject[] },
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ ...input }, settings);
}

export function workflow(
  input: { trigger: JsonObject },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

export function slackFile(
  input: { id?: string; url?: string },
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

export function columnSettings(
  input: { align?: "left" | "center" | "right"; isWrapped?: boolean },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

export function rawText(
  text: string,
  settings: FactorySettings = {},
): SlackObject<"raw_text"> {
  return create("raw_text", { text }, settings);
}

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

export function slackIcon(
  name: SlackIconName,
  settings: FactorySettings = {},
): SlackObject<"icon"> {
  if (!SLACK_ICON_NAMES.has(name)) {
    throw new TypeMismatchError("slackIcon.name", `unknown icon ${name}`);
  }
  return create("icon", { name }, settings);
}

export interface ChartSegmentInput {
  label: string;
  value: number;
}

export function chartSegment(
  input: ChartSegmentInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(input.label, "chartSegment.label", 20);
  if (!Number.isFinite(input.value)) {
    throw new TypeMismatchError("chartSegment.value", "expected a finite number");
  }
  if (input.value <= 0) {
    throw new RangeError("chartSegment.value", "expected a value greater than 0");
  }
  return createObject({ ...input }, settings);
}

export interface DataPointInput {
  label: string;
  value: number;
}

export function dataPoint(
  input: DataPointInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(input.label, "dataPoint.label", 20);
  if (!Number.isFinite(input.value)) {
    throw new TypeMismatchError("dataPoint.value", "expected a finite number");
  }
  return createObject({ ...input }, settings);
}

export interface DataSeriesInput {
  name: string;
  data: JsonObject[];
}

export function dataSeries(
  input: DataSeriesInput,
  settings: FactorySettings = {},
): JsonObject {
  ensureLength(input.name, "dataSeries.name", 20);
  if (input.data.length < 1 || input.data.length > 20) {
    throw new LengthError("dataSeries.data", "expected between 1 and 20 data points");
  }
  return createObject({ ...input }, settings);
}

export interface AxisConfigInput {
  categories: string[];
  xLabel?: string;
  yLabel?: string;
}

export function axisConfig(
  input: AxisConfigInput,
  settings: FactorySettings = {},
): JsonObject {
  if (input.categories.length < 1 || input.categories.length > 20) {
    throw new LengthError("axisConfig.categories", "expected between 1 and 20 categories");
  }
  input.categories.forEach((category) => ensureLength(category, "axisConfig.category", 20));
  if (new Set(input.categories).size !== input.categories.length) {
    throw new InvalidUsageError("axisConfig.categories", "expected unique labels");
  }
  if (input.xLabel !== undefined) ensureLength(input.xLabel, "axisConfig.xLabel", 50);
  if (input.yLabel !== undefined) ensureLength(input.yLabel, "axisConfig.yLabel", 50);
  return createObject({ ...input }, settings);
}

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
  const names = series.map((item) => item.name);
  if (new Set(names).size !== names.length) {
    throw new InvalidUsageError(`${type}Chart.series`, "series names must be unique");
  }
  const categories = axis.categories;
  if (!Array.isArray(categories)) {
    throw new TypeMismatchError(`${type}Chart.axisConfig.categories`, "expected an array");
  }
  for (const item of series) {
    const data = item.data;
    if (!Array.isArray(data)) {
      throw new TypeMismatchError(`${type}Chart.series.data`, "expected an array");
    }
    const labels = data.map((point) =>
      point !== null && typeof point === "object" && !Array.isArray(point)
        ? point.label
        : undefined,
    );
    if (
      labels.length !== categories.length ||
      new Set(labels).size !== categories.length ||
      labels.some((label) => !categories.includes(label as never))
    ) {
      throw new InvalidUsageError(
        `${type}Chart.series.data`,
        "each series must contain one point for every axis category",
      );
    }
  }
  return create(type, { series, axisConfig: axis }, settings);
}

export function barChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"bar"> {
  return axisChart("bar", series, axis, settings);
}

export function areaChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"area"> {
  return axisChart("area", series, axis, settings);
}

export function lineChart(
  series: JsonObject[],
  axis: JsonObject,
  settings: FactorySettings = {},
): SlackObject<"line"> {
  return axisChart("line", series, axis, settings);
}
