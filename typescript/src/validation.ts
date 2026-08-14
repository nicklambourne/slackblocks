import limits from "../../spec/limits.json" with { type: "json" };

import {
  InvalidUsageError,
  LengthError,
  MissingRequiredError,
  MutualExclusivityError,
  OutOfRangeError,
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

function codePointLength(value: string): number {
  let length = 0;
  for (const _ of value) {
    length += 1;
  }
  return length;
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
  const size = typeof value === "string" ? codePointLength(value) : value.length;
  if (minimum !== undefined && size < minimum) {
    throw new LengthError(path, `${size} is less than minimum ${minimum}`);
  }
  if (maximum !== undefined && size > maximum) {
    throw new LengthError(path, `${size} exceeds maximum ${maximum}`);
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
    throw new OutOfRangeError(path, `${value} is less than minimum ${minimum}`);
  }
  if (maximum !== undefined && value > maximum) {
    throw new OutOfRangeError(path, `${value} exceeds maximum ${maximum}`);
  }
}

function textCharacterCount(value: JsonValue): number {
  if (Array.isArray(value)) {
    return value.reduce<number>((total, item) => total + textCharacterCount(item), 0);
  }
  if (value !== null && typeof value === "object") {
    return Object.entries(value).reduce<number>(
      (total, [key, nested]) =>
        total +
        (key === "text" && typeof nested === "string"
          ? codePointLength(nested)
          : textCharacterCount(nested)),
      0,
    );
  }
  return 0;
}

function validateSeriesChart(object: JsonObject, path: string): void {
  if (!Array.isArray(object.series)) {
    throw new TypeMismatchError(child(path, "series"), "expected an array");
  }
  length(
    object.series,
    child(path, "series"),
    limits.data_visualization.series.min_items,
    limits.data_visualization.series.max_items,
  );
  const axis = objectAt(object.axis_config, child(path, "axis_config"));
  if (!Array.isArray(axis.categories)) {
    throw new TypeMismatchError(child(path, "axis_config.categories"), "expected an array");
  }
  const categories = axis.categories;
  length(
    categories,
    child(path, "axis_config.categories"),
    limits.data_visualization.categories.min_items,
    limits.data_visualization.categories.max_items,
  );
  categories.forEach((category, index) => {
    if (typeof category !== "string") {
      throw new TypeMismatchError(`${child(path, "axis_config.categories")}[${index}]`, "expected a string");
    }
    length(
      category,
      `${child(path, "axis_config.categories")}[${index}]`,
      undefined,
      limits.data_visualization.category_label.max_length,
    );
  });
  if (new Set(categories).size !== categories.length) {
    throw new InvalidUsageError(child(path, "axis_config.categories"), "expected unique labels");
  }
  for (const field of ["x_label", "y_label"] as const) {
    if (axis[field] !== undefined && typeof axis[field] !== "string") {
      throw new TypeMismatchError(child(path, `axis_config.${field}`), "expected a string");
    }
    length(
      axis[field] as string | undefined,
      child(path, `axis_config.${field}`),
      undefined,
      limits.data_visualization.axis_label.max_length,
    );
  }
  const names: string[] = [];
  object.series.forEach((rawSeries, seriesIndex) => {
    const seriesPath = `${child(path, "series")}[${seriesIndex}]`;
    const series = objectAt(rawSeries, seriesPath);
    if (typeof series.name !== "string") {
      throw new TypeMismatchError(child(seriesPath, "name"), "expected a string");
    }
    length(
      series.name,
      child(seriesPath, "name"),
      undefined,
      limits.data_visualization.series_name.max_length,
    );
    names.push(series.name);
    if (!Array.isArray(series.data)) {
      throw new TypeMismatchError(child(seriesPath, "data"), "expected an array");
    }
    length(
      series.data,
      child(seriesPath, "data"),
      limits.data_visualization.data.min_items,
      limits.data_visualization.data.max_items,
    );
    const labels = series.data.map((rawPoint, pointIndex) => {
      const pointPath = `${child(seriesPath, "data")}[${pointIndex}]`;
      const point = objectAt(rawPoint, pointPath);
      if (typeof point.label !== "string") {
        throw new TypeMismatchError(child(pointPath, "label"), "expected a string");
      }
      length(
        point.label,
        child(pointPath, "label"),
        undefined,
        limits.data_visualization.point_label.max_length,
      );
      if (typeof point.value !== "number" || !Number.isFinite(point.value)) {
        throw new TypeMismatchError(child(pointPath, "value"), "expected a finite number");
      }
      return point.label;
    });
    if (
      labels.length !== categories.length ||
      new Set(labels).size !== categories.length ||
      labels.some((label) => !categories.includes(label))
    ) {
      throw new InvalidUsageError(
        child(seriesPath, "data"),
        "expected exactly one point for every axis category",
      );
    }
  });
  if (new Set(names).size !== names.length) {
    throw new InvalidUsageError(child(path, "series"), "series names must be unique");
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
  "timepicker",
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
  "file_input",
]);

const REQUIRED_FIELDS: Record<string, readonly string[]> = {
  actions: ["elements"],
  alert: ["text"],
  area: ["series", "axis_config"],
  bar: ["series", "axis_config"],
  button: ["text", "action_id"],
  carousel: ["elements"],
  channel: ["channel_id"],
  channels_select: ["action_id"],
  checkboxes: ["action_id", "options"],
  container: ["child_blocks"],
  context: ["elements"],
  context_actions: ["elements"],
  conversations_select: ["action_id"],
  data_table: ["rows", "caption"],
  data_visualization: ["title", "chart"],
  datepicker: ["action_id"],
  datetimepicker: ["action_id"],
  email_text_input: ["action_id"],
  emoji: ["name"],
  external_select: ["action_id"],
  feedback_buttons: ["positive_button", "negative_button"],
  file: ["external_id"],
  file_input: ["action_id"],
  header: ["text"],
  home: ["blocks"],
  icon_button: ["text"],
  image: ["alt_text"],
  input: ["label", "element"],
  line: ["series", "axis_config"],
  link: ["url"],
  markdown: ["text"],
  modal: ["title", "blocks"],
  multi_channels_select: ["action_id"],
  multi_conversations_select: ["action_id"],
  multi_external_select: ["action_id"],
  multi_static_select: ["action_id"],
  multi_users_select: ["action_id"],
  number_input: ["action_id"],
  overflow: ["action_id", "options"],
  pie: ["segments"],
  plain_text_input: ["action_id"],
  plan: ["title"],
  radio_buttons: ["action_id", "options"],
  rich_text: ["elements"],
  rich_text_input: ["action_id"],
  rich_text_list: ["style", "elements"],
  rich_text_preformatted: ["elements"],
  rich_text_quote: ["elements"],
  rich_text_section: ["elements"],
  static_select: ["action_id"],
  table: ["rows"],
  task_card: ["task_id", "title"],
  text: ["text"],
  timepicker: ["action_id"],
  url: ["url", "text"],
  url_text_input: ["action_id"],
  user: ["user_id"],
  usergroup: ["usergroup_id"],
  users_select: ["action_id"],
  video: ["alt_text", "thumbnail_url", "title", "video_url"],
  workflow_button: ["text", "workflow"],
};

const CONFIRM_SUPPORTING_TYPES = new Set([
  "button",
  "channels_select",
  "checkboxes",
  "conversations_select",
  "datepicker",
  "datetimepicker",
  "external_select",
  "icon_button",
  "multi_channels_select",
  "multi_conversations_select",
  "multi_external_select",
  "multi_static_select",
  "multi_users_select",
  "overflow",
  "radio_buttons",
  "static_select",
  "timepicker",
  "users_select",
  "workflow_button",
]);

const OPTION_URL_MAX_LENGTH = 3000;

const TABLE_MAX_ROWS = 100;
const TABLE_MAX_COLUMNS = 20;

function validateOptionEntry(value: JsonValue, path: string): void {
  const option = objectAt(value, path);
  for (const field of ["text", "value"] as const) {
    if (option[field] === undefined) {
      throw new MissingRequiredError(path, `expected ${field}`);
    }
  }
  length(
    textValue(option.text),
    child(path, "text.text"),
    undefined,
    limits.option.text.max_length,
  );
  if (typeof option.value === "string") {
    length(option.value, child(path, "value"), undefined, limits.option.value.max_length);
  }
  if (option.description !== undefined) {
    length(
      textValue(option.description),
      child(path, "description.text"),
      undefined,
      limits.option.description.max_length,
    );
  }
  if (typeof option.url === "string") {
    length(option.url, child(path, "url"), undefined, OPTION_URL_MAX_LENGTH);
  }
}

function validateOptionEntries(object: JsonObject, path: string): void {
  if (Array.isArray(object.options)) {
    object.options.forEach((option, index) =>
      validateOptionEntry(option, `${child(path, "options")}[${index}]`),
    );
  }
  if (Array.isArray(object.option_groups)) {
    object.option_groups.forEach((rawGroup, index) => {
      const groupPath = `${child(path, "option_groups")}[${index}]`;
      const group = objectAt(rawGroup, groupPath);
      for (const field of ["label", "options"] as const) {
        if (group[field] === undefined) {
          throw new MissingRequiredError(groupPath, `expected ${field}`);
        }
      }
      length(
        textValue(group.label),
        child(groupPath, "label.text"),
        undefined,
        limits.option_group.label.max_length,
      );
      if (Array.isArray(group.options)) {
        length(
          group.options,
          child(groupPath, "options"),
          limits.option_group.options.min_items,
          limits.option_group.options.max_items,
        );
        group.options.forEach((option, optionIndex) =>
          validateOptionEntry(option, `${child(groupPath, "options")}[${optionIndex}]`),
        );
      }
    });
  }
}

function validateConfirmObject(value: JsonValue, path: string): void {
  const confirm = objectAt(value, path);
  for (const [field, maximum] of [
    ["title", limits.confirmation.title.max_length],
    ["text", limits.confirmation.text.max_length],
    ["confirm", limits.confirmation.confirm.max_length],
    ["deny", limits.confirmation.deny.max_length],
  ] as const) {
    if (confirm[field] === undefined) {
      throw new MissingRequiredError(path, `expected ${field}`);
    }
    length(textValue(confirm[field]), child(path, `${field}.text`), undefined, maximum);
  }
}

function validateKnownObject(object: JsonObject, path: string): void {
  const type = object.type;
  const blockId = object.block_id;
  if (typeof blockId === "string") {
    length(blockId, child(path, "block_id"), undefined, limits.block_id.max_length);
  }
  const actionId = object.action_id;
  if (typeof actionId === "string") {
    length(actionId, child(path, "action_id"), undefined, limits.action_id.max_length);
  }
  if (typeof type === "string") {
    for (const field of REQUIRED_FIELDS[type] ?? []) {
      if (object[field] === undefined) {
        throw new MissingRequiredError(path, `expected ${field}`);
      }
    }
    if (CONFIRM_SUPPORTING_TYPES.has(type) && object.confirm !== undefined) {
      validateConfirmObject(object.confirm, child(path, "confirm"));
    }
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
      if (typeof object.accessibility_label === "string") {
        length(
          object.accessibility_label,
          child(path, "accessibility_label"),
          undefined,
          limits.button.accessibility_label.max_length,
        );
      }
      break;
    case "icon_button":
      if (object.icon !== "trash") {
        throw new TypeMismatchError(child(path, "icon"), "expected trash");
      }
      if (typeof object.value === "string") {
        length(object.value, child(path, "value"), undefined, limits.button.value.max_length);
      }
      if (typeof object.accessibility_label === "string") {
        length(
          object.accessibility_label,
          child(path, "accessibility_label"),
          undefined,
          limits.button.accessibility_label.max_length,
        );
      }
      if (Array.isArray(object.visible_to_user_ids)) {
        length(
          object.visible_to_user_ids,
          child(path, "visible_to_user_ids"),
          undefined,
          limits.icon_button.visible_to_user_ids.max_items,
        );
      }
      break;
    case "feedback_buttons": {
      for (const field of ["positive_button", "negative_button"] as const) {
        const buttonPath = child(path, field);
        const feedback = objectAt(object[field], buttonPath);
        length(
          textValue(feedback.text),
          child(buttonPath, "text.text"),
          undefined,
          limits.feedback_button.text.max_length,
        );
        length(
          typeof feedback.value === "string" ? feedback.value : undefined,
          child(buttonPath, "value"),
          undefined,
          limits.feedback_button.value.max_length,
        );
        if (typeof feedback.accessibility_label === "string") {
          length(
            feedback.accessibility_label,
            child(buttonPath, "accessibility_label"),
            undefined,
            limits.feedback_button.accessibility_label.max_length,
          );
        }
      }
      break;
    }
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
      validateOptionEntries(object, path);
      break;
    case "checkboxes":
    case "radio_buttons":
      if (Array.isArray(object.options)) {
        length(object.options, child(path, "options"), 1, 10);
      }
      validateOptionEntries(object, path);
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
      validateOptionEntries(object, path);
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
        throw new OutOfRangeError(path, "min_value cannot exceed max_value");
      }
      break;
    case "image":
      if (object.image_url === undefined && object.slack_file === undefined) {
        throw new MissingRequiredError(path, "expected image_url or slack_file");
      }
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
    case "alert":
      length(
        textValue(object.text),
        child(path, "text.text"),
        undefined,
        limits.alert.text.max_length,
      );
      if (![undefined, "default", "info", "warning", "error", "success"].includes(object.level as never)) {
        throw new TypeMismatchError(child(path, "level"), "unknown alert level");
      }
      break;
    case "card":
      if (
        object.hero_image === undefined &&
        object.title === undefined &&
        object.actions === undefined &&
        object.body === undefined
      ) {
        throw new MissingRequiredError(path, "expected hero_image, title, actions, or body");
      }
      if (object.icon !== undefined && object.slack_icon !== undefined) {
        throw new MutualExclusivityError(path, "icon and slack_icon cannot be provided together");
      }
      for (const [field, maximum] of [
        ["title", limits.card.title.max_length],
        ["subtitle", limits.card.subtitle.max_length],
        ["body", limits.card.body.max_length],
        ["subtext", limits.card.subtext.max_length],
      ] as const) {
        if (object[field] !== undefined) {
          length(textValue(object[field]), child(path, `${field}.text`), undefined, maximum);
        }
      }
      if (Array.isArray(object.actions)) {
        length(object.actions, child(path, "actions"), undefined, limits.card.actions.max_items);
        object.actions.forEach((action, index) => {
          if (objectAt(action, `${child(path, "actions")}[${index}]`).type !== "button") {
            throw new TypeMismatchError(`${child(path, "actions")}[${index}]`, "expected a button");
          }
        });
      }
      break;
    case "carousel":
      if (!Array.isArray(object.elements)) {
        throw new TypeMismatchError(child(path, "elements"), "expected an array");
      }
      length(
        object.elements,
        child(path, "elements"),
        limits.carousel.elements.min_items,
        limits.carousel.elements.max_items,
      );
      object.elements.forEach((element, index) => {
        if (objectAt(element, `${child(path, "elements")}[${index}]`).type !== "card") {
          throw new TypeMismatchError(`${child(path, "elements")}[${index}]`, "expected a card");
        }
      });
      break;
    case "container": {
      if (object.title === undefined && object.rich_text_title === undefined) {
        throw new MissingRequiredError(path, "expected title or rich_text_title");
      }
      if (object.title !== undefined) {
        length(
          textValue(object.title),
          child(path, "title.text"),
          undefined,
          limits.container.title.max_length,
        );
      }
      if (object.subtitle !== undefined) {
        length(
          textValue(object.subtitle),
          child(path, "subtitle.text"),
          undefined,
          limits.container.subtitle.max_length,
        );
      }
      if (!Array.isArray(object.child_blocks)) {
        throw new TypeMismatchError(child(path, "child_blocks"), "expected an array");
      }
      length(
        object.child_blocks,
        child(path, "child_blocks"),
        1,
        limits.container.child_blocks.max_items,
      );
      const allowed = new Set([
        "actions", "context", "divider", "file", "header", "image", "input",
        "rich_text", "section", "table", "video",
      ]);
      object.child_blocks.forEach((block, index) => {
        if (!allowed.has(String(objectAt(block, `${child(path, "child_blocks")}[${index}]`).type))) {
          throw new TypeMismatchError(`${child(path, "child_blocks")}[${index}]`, "unsupported child block");
        }
      });
      if (![undefined, "narrow", "standard", "wide", "full"].includes(object.width as never)) {
        throw new TypeMismatchError(child(path, "width"), "unknown container width");
      }
      if (object.default_collapsed === true && object.is_collapsible !== true) {
        throw new InvalidUsageError(child(path, "default_collapsed"), "requires is_collapsible");
      }
      if (object.has_header_divider === true && object.is_collapsible === true) {
        throw new InvalidUsageError(child(path, "has_header_divider"), "requires a non-collapsible container");
      }
      break;
    }
    case "context_actions":
      if (!Array.isArray(object.elements)) {
        throw new TypeMismatchError(child(path, "elements"), "expected an array");
      }
      length(object.elements, child(path, "elements"), 1, limits.context_actions.elements.max_items);
      object.elements.forEach((element, index) => {
        const elementType = objectAt(element, `${child(path, "elements")}[${index}]`).type;
        if (elementType !== "feedback_buttons" && elementType !== "icon_button") {
          throw new TypeMismatchError(`${child(path, "elements")}[${index}]`, "unsupported context action");
        }
      });
      break;
    case "data_table": {
      if (!Array.isArray(object.rows)) {
        throw new TypeMismatchError(child(path, "rows"), "expected an array");
      }
      length(
        object.rows,
        child(path, "rows"),
        limits.data_table.rows.min_items,
        limits.data_table.rows.max_items,
      );
      let columns: number | undefined;
      object.rows.forEach((rawRow, rowIndex) => {
        if (!Array.isArray(rawRow)) {
          throw new TypeMismatchError(`${child(path, "rows")}[${rowIndex}]`, "expected an array");
        }
        length(
          rawRow,
          `${child(path, "rows")}[${rowIndex}]`,
          limits.data_table.columns.min_items,
          limits.data_table.columns.max_items,
        );
        columns ??= rawRow.length;
        if (rawRow.length !== columns) {
          throw new InvalidUsageError(`${child(path, "rows")}[${rowIndex}]`, "column count differs");
        }
        rawRow.forEach((rawCell, cellIndex) => {
          const cellPath = `${child(path, "rows")}[${rowIndex}][${cellIndex}]`;
          const cell = objectAt(rawCell, cellPath);
          if (!["raw_text", "raw_number", "rich_text"].includes(String(cell.type))) {
            throw new TypeMismatchError(cellPath, "unsupported data-table cell");
          }
          if (rowIndex === 0 && cell.type === "rich_text") {
            throw new TypeMismatchError(cellPath, "header cells cannot contain rich text");
          }
          if ((cell.type === "raw_text" || cell.type === "raw_number") && typeof cell.text === "string") {
            length(cell.text, child(cellPath, "text"), limits.data_table.cell_text.min_length);
          }
        });
      });
      range(
        typeof object.page_size === "number" ? object.page_size : undefined,
        child(path, "page_size"),
        limits.data_table.page_size.min,
        limits.data_table.page_size.max,
      );
      if (typeof object.row_header_column_index === "number") {
        range(object.row_header_column_index, child(path, "row_header_column_index"), 0, (columns ?? 1) - 1);
      }
      if (typeof object.caption !== "string") {
        throw new TypeMismatchError(child(path, "caption"), "expected a string");
      }
      length(object.caption, child(path, "caption"), 1);
      const contentLength = textCharacterCount(object.rows);
      if (contentLength > limits.data_table.content.max_length) {
        throw new LengthError(child(path, "rows"), `${contentLength} exceeds maximum ${limits.data_table.content.max_length}`);
      }
      break;
    }
    case "table": {
      if (!Array.isArray(object.rows)) {
        throw new TypeMismatchError(child(path, "rows"), "expected an array");
      }
      length(object.rows, child(path, "rows"), 1, TABLE_MAX_ROWS);
      let columns: number | undefined;
      object.rows.forEach((rawRow, rowIndex) => {
        const rowPath = `${child(path, "rows")}[${rowIndex}]`;
        if (!Array.isArray(rawRow)) {
          throw new TypeMismatchError(rowPath, "expected an array");
        }
        length(rawRow, rowPath, undefined, TABLE_MAX_COLUMNS);
        columns ??= rawRow.length;
        if (rawRow.length !== columns) {
          throw new InvalidUsageError(rowPath, "column count differs");
        }
        rawRow.forEach((rawCell, cellIndex) => {
          const cellPath = `${rowPath}[${cellIndex}]`;
          const cell = objectAt(rawCell, cellPath);
          if (!["raw_text", "rich_text"].includes(String(cell.type))) {
            throw new TypeMismatchError(cellPath, "unsupported table cell");
          }
        });
      });
      if (object.column_settings !== undefined) {
        if (!Array.isArray(object.column_settings)) {
          throw new TypeMismatchError(child(path, "column_settings"), "expected an array");
        }
        length(
          object.column_settings,
          child(path, "column_settings"),
          undefined,
          TABLE_MAX_COLUMNS,
        );
        if (object.column_settings.length !== columns) {
          throw new InvalidUsageError(
            child(path, "column_settings"),
            "expected one entry for every column",
          );
        }
      }
      break;
    }
    case "data_visualization":
      length(
        typeof object.title === "string" ? object.title : undefined,
        child(path, "title"),
        undefined,
        limits.data_visualization.title.max_length,
      );
      objectAt(object.chart, child(path, "chart"));
      break;
    case "pie":
      if (!Array.isArray(object.segments)) {
        throw new TypeMismatchError(child(path, "segments"), "expected an array");
      }
      length(
        object.segments,
        child(path, "segments"),
        limits.data_visualization.segments.min_items,
        limits.data_visualization.segments.max_items,
      );
      object.segments.forEach((rawSegment, index) => {
        const segmentPath = `${child(path, "segments")}[${index}]`;
        const segment = objectAt(rawSegment, segmentPath);
        length(
          typeof segment.label === "string" ? segment.label : undefined,
          child(segmentPath, "label"),
          undefined,
          limits.data_visualization.segment.label.max_length,
        );
        if (typeof segment.value !== "number" || !Number.isFinite(segment.value)) {
          throw new TypeMismatchError(child(segmentPath, "value"), "expected a finite number");
        }
        if (segment.value <= limits.data_visualization.segment.value.exclusive_min) {
          throw new OutOfRangeError(child(segmentPath, "value"), "expected a value greater than 0");
        }
      });
      break;
    case "bar":
    case "area":
    case "line":
      validateSeriesChart(object, path);
      break;
    case "task_card":
      if (typeof object.task_id !== "string" || typeof object.title !== "string") {
        throw new MissingRequiredError(path, "expected task_id and title");
      }
      if (![undefined, "pending", "in_progress", "complete", "error"].includes(object.status as never)) {
        throw new TypeMismatchError(child(path, "status"), "unknown task status");
      }
      if (Array.isArray(object.sources)) {
        object.sources.forEach((source, index) => {
          if (objectAt(source, `${child(path, "sources")}[${index}]`).type !== "url") {
            throw new TypeMismatchError(`${child(path, "sources")}[${index}]`, "expected a URL source");
          }
        });
      }
      break;
    case "plan":
      if (typeof object.title !== "string") {
        throw new MissingRequiredError(child(path, "title"), "expected a title");
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
  if (typeof value === "number" && !Number.isFinite(value)) {
    throw new TypeMismatchError(path, "expected a finite number");
  }
  if (Array.isArray(value)) {
    value.forEach((nested, index) => visit(nested, `${path}[${index}]`));
    return;
  }
  if (value !== null && typeof value === "object") {
    validateKnownObject(value, path);
    for (const [key, nested] of Object.entries(value)) {
      if (key === "event_payload") continue;
      visit(nested, path ? `${path}.${key}` : key);
    }
  }
}

/**
 * Asserts that an object is a valid Block Kit payload.
 *
 * Validation walks nested blocks, elements, views, and composition objects and
 * reports the first failing field through a typed validation error.
 *
 * @param payload - JSON value to validate.
 * @throws InvalidUsageError when the payload violates a supported Block Kit constraint.
 */
export function assertValid(payload: JsonValue): asserts payload is BlockKitPayload {
  if (payload === null || Array.isArray(payload) || typeof payload !== "object") {
    throw new TypeMismatchError("payload", "expected a Block Kit object");
  }
  visit(payload, "");
}

/**
 * Checks whether a value is a valid Block Kit payload without throwing for validation failures.
 *
 * Validation identifies objects by their `type` field, so it enforces required
 * fields and limits for every typed block, element, view, and rich-text object,
 * and it validates type-less `options`, `option_groups`, and `confirm`
 * composition objects contextually through their typed parents. Known
 * asymmetries with factory validation remain for type-less objects that
 * appear without a typed parent: standalone confirmation dialogs, options,
 * option groups, attachments, message payloads, workflow objects, and chart
 * axis configurations pass unchecked, and one-of rules enforced only by
 * factory signatures (for example `slackFile` requiring exactly one source)
 * are not rediscovered from raw JSON. The contents of message metadata
 * `event_payload` objects are always treated as opaque user data and skipped.
 *
 * @param payload - Unknown value to validate.
 * @returns `true` for a valid payload; otherwise `false`.
 */
export function validate(payload: unknown): payload is BlockKitPayload {
  try {
    assertValid(payload as JsonValue);
    return true;
  } catch (error) {
    if (error instanceof InvalidUsageError) return false;
    throw error;
  }
}
