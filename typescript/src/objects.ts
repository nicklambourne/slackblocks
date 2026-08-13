import { LengthError, MissingRequiredError, MutualExclusivityError } from "./errors.js";
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
  return createObject(input, settings);
}

export function dispatchActionConfiguration(
  input: { triggerActionsOn: string[] },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

export function inputParameter(
  input: { name: string; value: string },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

export function trigger(
  input: { url: string; customizableInputParameters?: JsonObject[] },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
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
