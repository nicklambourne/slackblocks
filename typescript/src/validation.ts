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

function range(
  value: number | undefined,
  path: string,
  minimum?: number,
  maximum?: number,
): void {
  if (value === undefined) return;
  if (minimum !== undefined && value < minimum) {
    throw new RangeError(path, `${value} is less than minimum ${minimum}`);
  }
  if (maximum !== undefined && value > maximum) {
    throw new RangeError(path, `${value} exceeds maximum ${maximum}`);
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
    case "file_input":
      range(
        typeof object.max_files === "number" ? object.max_files : undefined,
        child(path, "max_files"),
        limits.file_input.max_files.min,
        limits.file_input.max_files.max,
      );
      break;
    case "plain_text_input":
      range(
        typeof object.max_length === "number" ? object.max_length : undefined,
        child(path, "max_length"),
        undefined,
        limits.plain_text_input.max_length.max,
      );
      if (object.placeholder !== undefined) {
        length(
          textValue(object.placeholder),
          child(path, "placeholder.text"),
          undefined,
          limits.select.placeholder.max_length,
        );
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
      if (Array.isArray(object.options)) {
        length(
          object.options,
          child(path, "options"),
          undefined,
          limits.select.options.max_items,
        );
      }
      if (Array.isArray(object.option_groups)) {
        length(
          object.option_groups,
          child(path, "option_groups"),
          undefined,
          limits.select.option_groups.max_items,
        );
      }
      if (object.placeholder !== undefined) {
        length(
          textValue(object.placeholder),
          child(path, "placeholder.text"),
          undefined,
          limits.select.placeholder.max_length,
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
      if (typeof object.image_url === "string") {
        length(
          object.image_url,
          child(path, "image_url"),
          undefined,
          limits.image.image_url.max_length,
        );
      }
      if (typeof object.alt_text === "string") {
        length(
          object.alt_text,
          child(path, "alt_text"),
          undefined,
          limits.image.alt_text.max_length,
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
      length(
        textValue(object.label),
        child(path, "label.text"),
        undefined,
        limits.input.label.max_length,
      );
      if (object.hint !== undefined) {
        length(
          textValue(object.hint),
          child(path, "hint.text"),
          undefined,
          limits.input.hint.max_length,
        );
      }
      const elementPath = child(path, "element");
      const element = objectAt(object.element, elementPath);
      if (!INPUT_ELEMENT_TYPES.has(String(element.type))) {
        throw new TypeMismatchError(elementPath, "expected an input-compatible element");
      }
      break;
    }
    case "markdown":
      length(
        typeof object.text === "string" ? object.text : undefined,
        child(path, "text"),
        limits.markdown.text.min_length,
        limits.markdown.text.max_length,
      );
      break;
    case "video":
      length(
        typeof object.alt_text === "string" ? object.alt_text : undefined,
        child(path, "alt_text"),
        limits.video.alt_text.min_length,
        limits.video.alt_text.max_length,
      );
      length(
        textValue(object.title),
        child(path, "title.text"),
        undefined,
        limits.video.title.max_length,
      );
      length(
        typeof object.author_name === "string" ? object.author_name : undefined,
        child(path, "author_name"),
        undefined,
        limits.video.author_name.max_length,
      );
      if (object.description !== undefined) {
        length(
          textValue(object.description),
          child(path, "description.text"),
          undefined,
          limits.video.description.max_length,
        );
      }
      length(
        typeof object.provider_name === "string" ? object.provider_name : undefined,
        child(path, "provider_name"),
        undefined,
        limits.video.provider_name.max_length,
      );
      break;
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
      length(
        typeof object.private_metadata === "string" ? object.private_metadata : undefined,
        child(path, "private_metadata"),
        undefined,
        limits.view.private_metadata.max_length,
      );
      length(
        typeof object.callback_id === "string" ? object.callback_id : undefined,
        child(path, "callback_id"),
        undefined,
        limits.view.callback_id.max_length,
      );
      if (type === "modal") {
        length(
          textValue(object.title),
          child(path, "title.text"),
          undefined,
          limits.view.title.max_length,
        );
        if (object.close !== undefined) {
          length(
            textValue(object.close),
            child(path, "close.text"),
            undefined,
            limits.view.close.max_length,
          );
        }
        if (object.submit !== undefined) {
          length(
            textValue(object.submit),
            child(path, "submit.text"),
            undefined,
            limits.view.submit.max_length,
          );
        }
      }
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
