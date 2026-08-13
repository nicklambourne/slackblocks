import limits from "../../spec/limits.json" with { type: "json" };

import {
  InvalidUsageError,
  LengthError,
  MissingRequiredError,
  MutualExclusivityError,
  RangeError,
  TypeMismatchError,
} from "./errors.js";
import type { BlockKitPayload, JsonObject, JsonValue } from "./types.js";

function textValue(value: unknown): string | undefined {
  if (typeof value === "string") return value;
  if (
    value !== null &&
    typeof value === "object" &&
    "text" in value &&
    typeof value.text === "string"
  ) {
    return value.text;
  }
  return undefined;
}

function objectAt(value: JsonValue | undefined, path: string): JsonObject {
  if (value === null || Array.isArray(value) || typeof value !== "object") {
    throw new TypeMismatchError(path, "expected an object");
  }
  return value;
}

function child(path: string, field: string): string {
  return path ? `${path}.${field}` : field;
}

function length(
  value: string | unknown[] | undefined,
  path: string,
  minimum?: number,
  maximum?: number,
): void {
  if (value === undefined) return;
  if (minimum !== undefined && value.length < minimum) {
    throw new LengthError(path, `${value.length} is less than minimum ${minimum}`);
  }
  if (maximum !== undefined && value.length > maximum) {
    throw new LengthError(path, `${value.length} exceeds maximum ${maximum}`);
  }
}

function validateTextObject(object: JsonObject, path: string): void {
  const value = textValue(object);
  if (value === undefined) {
    throw new TypeMismatchError(child(path, "text"), "expected a string");
  }
  length(value, child(path, "text"), limits.text.min_length, limits.text.max_length);
}

const INPUT_ELEMENT_TYPES = new Set([
  "plain_text_input",
  "number_input",
  "checkboxes",
  "radio_buttons",
  "datepicker",
  "datetimepicker",
  "channels_select",
  "multi_channels_select",
  "conversations_select",
  "multi_conversations_select",
  "external_select",
  "multi_external_select",
  "static_select",
  "multi_static_select",
  "users_select",
  "multi_users_select",
  "rich_text_input",
  "email_text_input",
  "url_text_input",
]);

function validateKnownObject(object: JsonObject, path: string): void {
  const type = object.type;
  const actionId = object.action_id;
  if (typeof actionId === "string") {
    length(actionId, child(path, "action_id"), undefined, limits.action_id.max_length);
  }

  switch (type) {
    case "plain_text":
    case "mrkdwn":
      validateTextObject(object, path);
      break;
    case "section": {
      const text = object.text;
      const fields = object.fields;
      if (text === undefined && fields === undefined) {
        throw new MissingRequiredError(path, "expected text, fields, or both");
      }
      if (text !== undefined) {
        length(
          textValue(text),
          child(path, "text.text"),
          undefined,
          limits.section.text.max_length,
        );
      }
      if (Array.isArray(fields)) {
        length(fields, child(path, "fields"), undefined, limits.section.fields.max_items);
        fields.forEach((field, index) =>
          length(
            textValue(field),
            `${child(path, "fields")}[${index}].text`,
            undefined,
            limits.section.fields.item_max_length,
          ),
        );
      }
      break;
    }
    case "header":
      length(
        textValue(object.text),
        child(path, "text.text"),
        undefined,
        limits.header.text.max_length,
      );
      break;
    case "button":
    case "workflow_button":
      length(
        textValue(object.text),
        child(path, "text.text"),
        undefined,
        limits.button.text.max_length,
      );
      if (typeof object.url === "string") {
        length(object.url, child(path, "url"), undefined, limits.button.url.max_length);
      }
      if (typeof object.value === "string") {
        length(object.value, child(path, "value"), undefined, limits.button.value.max_length);
      }
      break;
    case "overflow":
      if (Array.isArray(object.options)) {
        length(
          object.options,
          child(path, "options"),
          limits.overflow.options.min_items,
          limits.overflow.options.max_items,
        );
      }
      break;
    case "static_select":
    case "multi_static_select":
      if (object.options !== undefined && object.option_groups !== undefined) {
        throw new MutualExclusivityError(
          path,
          "options and option_groups cannot be provided together",
        );
      }
      break;
    case "number_input":
      if (
        typeof object.min_value === "number" &&
        typeof object.max_value === "number" &&
        object.min_value > object.max_value
      ) {
        throw new RangeError(path, "min_value cannot exceed max_value");
      }
      break;
    case "image":
      if (object.image_url !== undefined && object.slack_file !== undefined) {
        throw new MutualExclusivityError(
          path,
          "image_url and slack_file cannot be provided together",
        );
      }
      break;
    case "context":
      if (Array.isArray(object.elements)) {
        length(object.elements, child(path, "elements"), undefined, limits.context.elements.max_items);
        object.elements.forEach((element, index) => {
          const elementPath = `${child(path, "elements")}[${index}]`;
          const nested = objectAt(element, elementPath);
          if (!["plain_text", "mrkdwn", "image"].includes(String(nested.type))) {
            throw new TypeMismatchError(
              elementPath,
              "expected text or image element",
            );
          }
        });
      }
      break;
    case "actions":
      if (Array.isArray(object.elements)) {
        length(object.elements, child(path, "elements"), undefined, limits.actions.elements.max_items);
      }
      break;
    case "input": {
      const elementPath = child(path, "element");
      const element = objectAt(object.element, elementPath);
      if (!INPUT_ELEMENT_TYPES.has(String(element.type))) {
        throw new TypeMismatchError(elementPath, "expected an input-compatible element");
      }
      break;
    }
    case "modal":
    case "home":
      if (!Array.isArray(object.blocks)) {
        throw new TypeMismatchError(child(path, "blocks"), "expected an array");
      }
      length(
        object.blocks,
        child(path, "blocks"),
        limits.view.blocks.min_items,
        limits.view.blocks.max_items,
      );
      break;
    default:
      break;
  }
}

function visit(value: JsonValue, path: string): void {
  if (Array.isArray(value)) {
    value.forEach((nested, index) => visit(nested, `${path}[${index}]`));
    return;
  }
  if (value !== null && typeof value === "object") {
    validateKnownObject(value, path);
    for (const [key, nested] of Object.entries(value)) {
      visit(nested, path ? `${path}.${key}` : key);
    }
  }
}

export function assertValid(payload: JsonValue): asserts payload is BlockKitPayload {
  if (payload === null || Array.isArray(payload) || typeof payload !== "object") {
    throw new TypeMismatchError("payload", "expected a Block Kit object");
  }
  visit(payload, "");
}

export function validate(payload: unknown): payload is BlockKitPayload {
  try {
    assertValid(payload as JsonValue);
    return true;
  } catch (error) {
    if (error instanceof InvalidUsageError) return false;
    throw error;
  }
}
