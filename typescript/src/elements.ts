import { RangeError } from "./errors.js";
import { create, createObject } from "./internal.js";
import { asText, type TextLike } from "./objects.js";
import type { FactorySettings, JsonObject, SlackObject } from "./types.js";

export type ElementInput = Record<string, unknown>;

export interface ActionInput extends ElementInput {
  actionId: string;
}

function withPlaceholder(input: ElementInput, settings: FactorySettings): ElementInput {
  if (typeof input.placeholder !== "string") return input;
  return { ...input, placeholder: asText(input.placeholder, "plain_text", settings) };
}

function element<Type extends string>(
  type: Type,
  input: ElementInput,
  settings: FactorySettings,
): SlackObject<Type> {
  return create(type, withPlaceholder(input, settings), settings);
}

export interface ButtonInput extends ElementInput {
  text: TextLike;
  actionId: string;
  url?: string;
  value?: string;
  style?: "primary" | "danger";
  confirm?: JsonObject;
  accessibilityLabel?: string;
}

export interface WorkflowButtonInput extends ElementInput {
  text: TextLike;
  workflow: JsonObject;
  actionId?: string;
  confirm?: JsonObject;
  accessibilityLabel?: string;
}

export function button(
  input: ButtonInput,
  settings: FactorySettings = {},
): SlackObject<"button"> {
  return element(
    "button",
    { ...input, text: asText(input.text, "plain_text", settings) },
    settings,
  );
}

export function checkboxes(
  input: ActionInput & { options: JsonObject[] },
  settings: FactorySettings = {},
): SlackObject<"checkboxes"> {
  return element("checkboxes", input, settings);
}

export function datePicker(
  input: ActionInput,
  settings: FactorySettings = {},
): SlackObject<"datepicker"> {
  return element("datepicker", input, settings);
}

export function dateTimePicker(
  input: ActionInput,
  settings: FactorySettings = {},
): SlackObject<"datetimepicker"> {
  return element("datetimepicker", input, settings);
}

export function emailInput(
  input: ActionInput,
  settings: FactorySettings = {},
): SlackObject<"email_text_input"> {
  return element("email_text_input", input, settings);
}

export function fileInput(
  input: { actionId: string; filetypes?: string[]; maxFiles?: number },
  settings: FactorySettings = {},
): JsonObject {
  if (input.maxFiles !== undefined && (input.maxFiles < 1 || input.maxFiles > 10)) {
    throw new RangeError("fileInput.maxFiles", "expected a value between 1 and 10");
  }
  // Python 2.0 fixtures preserve this historical no-type wire shape.
  return createObject(input, settings);
}

export function imageElement(
  input: { altText: string } & (
    | { imageUrl: string; slackFile?: never }
    | { imageUrl?: never; slackFile: JsonObject }
  ),
  settings: FactorySettings = {},
): SlackObject<"image"> {
  return element("image", input, settings);
}

export function channelMultiSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("multi_channels_select", input, settings);
}

export function conversationMultiSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("multi_conversations_select", input, settings);
}

export function externalMultiSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("multi_external_select", input, settings);
}

export function staticMultiSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("multi_static_select", input, settings);
}

export function userMultiSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("multi_users_select", input, settings);
}

export function numberInput(input: ActionInput, settings: FactorySettings = {}) {
  return element("number_input", input, settings);
}

export function overflow(
  input: ActionInput & { options: JsonObject[] },
  settings: FactorySettings = {},
) {
  return element("overflow", input, settings);
}

export function plainTextInput(input: ActionInput, settings: FactorySettings = {}) {
  return element("plain_text_input", input, settings);
}

export function radioButtons(
  input: ActionInput & { options: JsonObject[] },
  settings: FactorySettings = {},
) {
  return element("radio_buttons", input, settings);
}

export function channelSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("channels_select", input, settings);
}

export function conversationSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("conversations_select", input, settings);
}

export function externalSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("external_select", input, settings);
}

export function staticSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("static_select", input, settings);
}

export function userSelect(input: ActionInput, settings: FactorySettings = {}) {
  return element("users_select", input, settings);
}

export function timePicker(input: ActionInput, settings: FactorySettings = {}) {
  return element("timepicker", input, settings);
}

export function urlInput(input: ActionInput, settings: FactorySettings = {}) {
  return element("url_text_input", input, settings);
}

export function workflowButton(
  input: WorkflowButtonInput,
  settings: FactorySettings = {},
) {
  return element(
    "workflow_button",
    { ...input, text: asText(input.text, "plain_text", settings) },
    settings,
  );
}

export function richTextInput(input: ActionInput, settings: FactorySettings = {}) {
  return element("rich_text_input", input, settings);
}
