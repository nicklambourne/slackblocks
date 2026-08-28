/**
 * Interactive and visual element factories used inside Block Kit blocks.
 *
 * Factories accept camelCase input and return validated Slack wire objects.
 *
 * @module elements
 */
import limits from "../../../spec/limits.json" with { type: "json" };

import { LengthError, OutOfRangeError, TypeMismatchError } from "../errors.js";
import { create, createObject, dropEmpty } from "../internal.js";
import { asText, type TextLike } from "./objects.js";
import type { FactorySettings, JsonObject, SlackObject } from "../types.js";

/** Fields accepted by {@link checkboxes}. */
export interface CheckboxesInput {
  /** Identifier returned when the checkbox selection changes, up to 255 characters. */
  actionId: string;
  /** Up to ten option objects displayed as checkboxes. */
  options: JsonObject[];
  /** Options from `options` that are selected when the element first loads. */
  initialOptions?: JsonObject[];
  /** Optional confirmation dialog shown before the changed selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
}

/** Fields accepted by {@link datePicker}. */
export interface DatePickerInput {
  /** Identifier returned when a date is selected, up to 255 characters. */
  actionId: string;
  /** Initially selected date in `YYYY-MM-DD` format. */
  initialDate?: string;
  /** Optional confirmation dialog shown after a date is selected. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a date is selected, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link dateTimePicker}. */
export interface DateTimePickerInput {
  /** Identifier returned when a date and time are selected, up to 255 characters. */
  actionId: string;
  /** Initially selected date and time as a Unix timestamp in seconds. */
  initialDateTime?: number;
  /** Optional confirmation dialog shown after a date and time are selected. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
}

/** Fields accepted by {@link emailInput}. */
export interface EmailElementInput {
  /** Identifier used to find the submitted email value, up to 255 characters. */
  actionId: string;
  /** Email address present when the input first loads. */
  initialValue?: string;
  /** Configuration controlling when typing dispatches a `block_actions` payload. */
  dispatchActionConfig?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown in the empty input, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link channelMultiSelect}. */
export interface ChannelMultiSelectInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** Public channel IDs selected when the menu first loads. */
  initialChannels?: string[];
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Maximum number of channels that may be selected; the minimum is one. */
  maxSelectedItems?: number;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link conversationMultiSelect}. */
export interface ConversationMultiSelectInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** Conversation IDs selected when the menu first loads. */
  initialConversations?: string[];
  /** Select the conversation from which the view was opened by default. */
  defaultToCurrentConversation?: boolean;
  /** Filter controlling which public channels, private channels, DMs, and group DMs appear. */
  filter?: JsonObject;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Maximum number of conversations that may be selected; the minimum is one. */
  maxSelectedItems?: number;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link externalMultiSelect}. */
export interface ExternalMultiSelectInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** Minimum typed characters before Slack requests options; defaults to three. */
  minQueryLength?: number;
  /** Options selected when the menu first loads. */
  initialOptions?: JsonObject[];
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Maximum number of options that may be selected; the minimum is one. */
  maxSelectedItems?: number;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link staticMultiSelect}. */
export interface StaticMultiSelectInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** Up to 100 directly supplied options; mutually exclusive with `optionGroups`. */
  options?: JsonObject[];
  /** Up to 100 groups of options; mutually exclusive with `options`. */
  optionGroups?: JsonObject[];
  /** Options selected when the menu first loads. */
  initialOptions?: JsonObject[];
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Maximum number of options that may be selected; the minimum is one. */
  maxSelectedItems?: number;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link userMultiSelect}. */
export interface UserMultiSelectInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** User IDs selected when the menu first loads. */
  initialUsers?: string[];
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Maximum number of users that may be selected; the minimum is one. */
  maxSelectedItems?: number;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link numberInput}. */
export interface NumberElementInput {
  /** Identifier used to find the submitted numeric value, up to 255 characters. */
  actionId: string;
  /** Whether the input accepts decimal values as well as whole numbers. */
  isDecimalAllowed?: boolean;
  /** Numeric text present when the input first loads. */
  initialValue?: string;
  /** Minimum accepted value; it cannot exceed `maxValue`. */
  minValue?: number;
  /** Maximum accepted value; it cannot be less than `minValue`. */
  maxValue?: number;
  /** Configuration controlling when typing dispatches a `block_actions` payload. */
  dispatchActionConfig?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown in the empty input, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link overflow}. */
export interface OverflowInput {
  /** Identifier returned when an option is selected, up to 255 characters. */
  actionId: string;
  /** Between two and five option objects displayed in the compact menu. */
  options: JsonObject[];
  /** Optional confirmation dialog shown after an option is selected. */
  confirm?: JsonObject;
}

/** Fields accepted by {@link plainTextInput}. */
export interface PlainTextElementInput {
  /** Identifier used to find the submitted text value, up to 255 characters. */
  actionId: string;
  /** Text present when the input first loads. */
  initialValue?: string;
  /** Whether the input is a multi-line textarea instead of a single line. */
  multiline?: boolean;
  /** Minimum number of characters the user must enter, between 0 and 3000. */
  minLength?: number;
  /** Maximum number of characters the user may enter, between 1 and 3000. */
  maxLength?: number;
  /** Configuration controlling when typing dispatches a `block_actions` payload. */
  dispatchActionConfig?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown in the empty input, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link radioButtons}. */
export interface RadioButtonsInput {
  /** Identifier returned when the selection changes, up to 255 characters. */
  actionId: string;
  /** Up to ten options displayed as radio buttons. */
  options: JsonObject[];
  /** Option from `options` selected when the element first loads. */
  initialOption?: JsonObject;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
}

/** Fields accepted by {@link channelSelect}. */
export interface ChannelSelectInput {
  /** Identifier returned when a channel is selected, up to 255 characters. */
  actionId: string;
  /** Public channel ID selected when the menu first loads. */
  initialChannel?: string;
  /** Include a `response_url` in a parent modal's submission payload. */
  responseUrlEnabled?: boolean;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link conversationSelect}. */
export interface ConversationSelectInput {
  /** Identifier returned when a conversation is selected, up to 255 characters. */
  actionId: string;
  /** Conversation ID selected when the menu first loads. */
  initialConversation?: string;
  /** Select the conversation from which the view was opened by default. */
  defaultToCurrentConversation?: boolean;
  /** Filter controlling which public channels, private channels, DMs, and group DMs appear. */
  filter?: JsonObject;
  /** Include a `response_url` in a parent modal's submission payload. */
  responseUrlEnabled?: boolean;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link externalSelect}. */
export interface ExternalSelectInput {
  /** Identifier returned when an option is selected, up to 255 characters. */
  actionId: string;
  /** Minimum typed characters before Slack requests options; defaults to three. */
  minQueryLength?: number;
  /** Option selected when the menu first loads. */
  initialOption?: JsonObject;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link staticSelect}. */
export interface StaticSelectInput {
  /** Identifier returned when an option is selected, up to 255 characters. */
  actionId: string;
  /** Up to 100 directly supplied options; mutually exclusive with `optionGroups`. */
  options?: JsonObject[];
  /** Up to 100 groups of options; mutually exclusive with `options`. */
  optionGroups?: JsonObject[];
  /** Option selected when the menu first loads. */
  initialOption?: JsonObject;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link userSelect}. */
export interface UserSelectInput {
  /** Identifier returned when a user is selected, up to 255 characters. */
  actionId: string;
  /** User ID selected when the menu first loads. */
  initialUser?: string;
  /** Optional confirmation dialog shown before the selection is submitted. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a selection, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link timePicker}. */
export interface TimePickerInput {
  /** Identifier returned when a time is selected, up to 255 characters. */
  actionId: string;
  /** Initially selected time in 24-hour `HH:mm` format. */
  initialTime?: string;
  /** IANA timezone displayed as supporting text and returned with interactions. */
  timezone?: string;
  /** Optional confirmation dialog shown after a time is selected. */
  confirm?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown before a time is selected, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link urlInput}. */
export interface UrlElementInput {
  /** Identifier used to find the submitted URL, up to 255 characters. */
  actionId: string;
  /** URL present when the input first loads. */
  initialValue?: string;
  /** Configuration controlling when typing dispatches a `block_actions` payload. */
  dispatchActionConfig?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown in the empty input, up to 150 characters. */
  placeholder?: TextLike;
}

/** Fields accepted by {@link richTextInput}. */
export interface RichTextElementInput {
  /** Identifier used to find the submitted rich-text value, up to 255 characters. */
  actionId: string;
  /** Rich-text content present when the editor first loads. */
  initialValue?: JsonObject;
  /** Configuration controlling when editing dispatches a `block_actions` payload. */
  dispatchActionConfig?: JsonObject;
  /** Whether this element receives focus when its containing view opens. */
  focusOnLoad?: boolean;
  /** Plain-text prompt shown in the empty editor, up to 150 characters. */
  placeholder?: TextLike;
  /** Minimum visible editor lines before scrolling, between 1 and 100. */
  minLines?: number;
  /** Maximum visible editor lines before scrolling, between 1 and 100. */
  maxLines?: number;
}

function withPlaceholder(
  input: Record<string, unknown>,
  settings: FactorySettings,
): Record<string, unknown> {
  if (typeof input.placeholder !== "string") return input;
  return { ...input, placeholder: asText(input.placeholder, "plain_text", settings) };
}

function element<Type extends string>(
  type: Type,
  input: Record<string, unknown>,
  settings: FactorySettings,
): SlackObject<Type> {
  return create(type, withPlaceholder(input, settings), settings);
}

/** Fields accepted by {@link button}. */
export interface ButtonInput {
  /** Plain-text label displayed on the button. */
  text: TextLike;
  /** Identifier returned when the button is selected. */
  actionId: string;
  /** Optional URL opened by the button. */
  url?: string;
  /** Optional application-defined value returned with the interaction. */
  value?: string;
  /** Optional visual emphasis. */
  style?: "primary" | "danger";
  /** Optional confirmation dialog created with `confirmation`. */
  confirm?: JsonObject;
  /** Accessible label when the visible text is insufficient. */
  accessibilityLabel?: string;
}

/** Fields accepted by {@link workflowButton}. */
export interface WorkflowButtonInput {
  /** Plain-text label displayed on the button. */
  text: TextLike;
  /** Workflow object created with `workflow`. */
  workflow: JsonObject;
  /** Optional interaction identifier. */
  actionId?: string;
  /** Optional confirmation dialog. */
  confirm?: JsonObject;
  /** Optional visual emphasis. */
  style?: "primary" | "danger";
  /** Accessible label when the visible text is insufficient. */
  accessibilityLabel?: string;
}

/** URL-backed or Slack-file-backed image fields accepted by {@link imageElement}. */
export type ImageElementInput =
  | {
      /** Accessible plain-text summary of the image. */
      altText: string;
      /** Public image URL, up to 3,000 characters. */
      imageUrl: string;
      /** A Slack file cannot be combined with `imageUrl`. */
      slackFile?: never;
    }
  | {
      /** Accessible plain-text summary of the image. */
      altText: string;
      /** An image URL cannot be combined with `slackFile`. */
      imageUrl?: never;
      /** Slack-hosted file reference created with `slackFile`. */
      slackFile: JsonObject;
    };

/**
 * Creates an interactive button.
 *
 * @param input - Button label, action identifier, and optional behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `button` element.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
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

/** Fields accepted by {@link feedbackButton}. */
export interface FeedbackButtonInput {
  /** Plain-text feedback choice. */
  text: TextLike;
  /** Application-defined value returned with the feedback. */
  value: string;
  /** Accessible label when the visible text is insufficient. */
  accessibilityLabel?: string;
}

/**
 * Creates one choice used by a feedback-buttons element.
 *
 * @param input - Choice text, returned value, and optional accessible label.
 * @param settings - Per-call validation settings.
 * @returns A validated feedback-button object.
 * @throws InvalidUsageError when a field exceeds Slack's limits.
 */
export function feedbackButton(
  input: FeedbackButtonInput,
  settings: FactorySettings = {},
): JsonObject {
  return createObject(
    { ...input, text: asText(input.text, "plain_text", settings) },
    settings,
  );
}

/**
 * Creates a paired positive/negative feedback control.
 *
 * @param input - Positive and negative feedback buttons plus an optional action identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `feedback_buttons` element.
 * @throws InvalidUsageError when either feedback choice is invalid.
 */
export function feedbackButtons(
  input: {
    /** Positive choice created with `feedbackButton`. */
    positiveButton: JsonObject;
    /** Negative choice created with `feedbackButton`. */
    negativeButton: JsonObject;
    /** Optional identifier returned with the interaction. */
    actionId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"feedback_buttons"> {
  return element("feedback_buttons", input, settings);
}

/**
 * Creates a compact icon action for a context-actions block.
 *
 * @param input - Accessible text, icon behavior, and optional visibility restrictions.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `icon_button` element.
 * @throws InvalidUsageError when the icon or visible-user list is invalid.
 */
export function iconButton(
  input: {
    /** Plain-text description of the icon action. */
    text: TextLike;
    /** Icon name. Slack currently accepts only `trash`. */
    icon?: "trash";
    /** Optional identifier returned with the interaction. */
    actionId?: string;
    /** Optional application-defined interaction value. */
    value?: string;
    /** Optional confirmation dialog. */
    confirm?: JsonObject;
    /** Accessible label when the text is insufficient. */
    accessibilityLabel?: string;
    /** Up to ten user IDs allowed to see the action. */
    visibleToUserIds?: string[];
  },
  settings: FactorySettings = {},
): SlackObject<"icon_button"> {
  if (input.icon !== undefined && input.icon !== "trash") {
    throw new TypeMismatchError("iconButton.icon", "expected trash");
  }
  if (
    input.visibleToUserIds !== undefined &&
    input.visibleToUserIds.length > limits.icon_button.visible_to_user_ids.max_items
  ) {
    throw new LengthError(
      "iconButton.visibleToUserIds",
      `exceeds maximum ${limits.icon_button.visible_to_user_ids.max_items}`,
    );
  }
  return element(
    "icon_button",
    {
      ...input,
      icon: input.icon ?? "trash",
      text: asText(input.text, "plain_text", settings),
    },
    settings,
  );
}

/**
 * Creates a source link for a task card.
 *
 * @param input - Public URL and human-readable link text.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `url` source element.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
export function urlSource(
  input: {
    /** Public source URL. */
    url: string;
    /** Human-readable source label. */
    text: string;
  },
  settings: FactorySettings = {},
): SlackObject<"url"> {
  return element("url", input, settings);
}

/**
 * Creates a checkbox group.
 *
 * @param input - Action identifier, options, and optional Slack checkbox fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `checkboxes` element.
 * @throws InvalidUsageError when options or identifiers violate Slack's constraints.
 */
export function checkboxes(
  input: CheckboxesInput,
  settings: FactorySettings = {},
): SlackObject<"checkboxes"> {
  return element(
    "checkboxes",
    { ...input, initialOptions: dropEmpty(input.initialOptions) },
    settings,
  );
}

/**
 * Creates a date picker.
 *
 * @param input - Action identifier and optional Slack date-picker fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `datepicker` element.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
export function datePicker(
  input: DatePickerInput,
  settings: FactorySettings = {},
): SlackObject<"datepicker"> {
  return element("datepicker", { ...input }, settings);
}

/**
 * Creates a date-and-time picker.
 *
 * @param input - Action identifier and optional Slack date-time-picker fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `datetimepicker` element.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
export function dateTimePicker(
  input: DateTimePickerInput,
  settings: FactorySettings = {},
): SlackObject<"datetimepicker"> {
  return element("datetimepicker", { ...input }, settings);
}

/**
 * Creates an email-address input.
 *
 * @param input - Action identifier and optional Slack email-input fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `email_text_input` element.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
export function emailInput(
  input: EmailElementInput,
  settings: FactorySettings = {},
): SlackObject<"email_text_input"> {
  return element("email_text_input", { ...input }, settings);
}

/**
 * Creates a file-upload input.
 *
 * @param input - Action identifier, allowed extensions, and maximum file count.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `file_input` element.
 * @throws InvalidUsageError when `maxFiles` is outside 1–10.
 */
export function fileInput(
  input: {
    /** Identifier returned with submitted files. */
    actionId: string;
    /** Optional allowed file extensions. */
    filetypes?: string[];
    /** Maximum files accepted, between 1 and 10. */
    maxFiles?: number;
  },
  settings: FactorySettings = {},
): SlackObject<"file_input"> {
  if (
    input.maxFiles !== undefined &&
    (input.maxFiles < limits.file_input.max_files.min ||
      input.maxFiles > limits.file_input.max_files.max)
  ) {
    throw new OutOfRangeError(
      "fileInput.maxFiles",
      `expected a value between ${limits.file_input.max_files.min} and ${limits.file_input.max_files.max}`,
    );
  }
  return create("file_input", input, settings);
}

/**
 * Creates an image element from a URL or Slack-hosted file.
 *
 * @param input - Accessible text and exactly one image source.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `image` element.
 * @throws InvalidUsageError when both or neither image source is provided.
 */
export function imageElement(
  input: ImageElementInput,
  settings: FactorySettings = {},
): SlackObject<"image"> {
  return element("image", input, settings);
}

/**
 * Creates a multi-select menu populated with public channels visible to the current user.
 *
 * @param input - Channel selection, confirmation, focus, and placeholder behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `multi_channels_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, or selection constraint is invalid.
 */
export function channelMultiSelect(
  input: ChannelMultiSelectInput,
  settings: FactorySettings = {},
) {
  return element(
    "multi_channels_select",
    { ...input, initialChannels: dropEmpty(input.initialChannels) },
    settings,
  );
}

/**
 * Creates a multi-select menu of public channels, private channels, DMs, and group DMs.
 *
 * @param input - Conversation filters, initial selection, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `multi_conversations_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, filter, or selection is invalid.
 */
export function conversationMultiSelect(
  input: ConversationMultiSelectInput,
  settings: FactorySettings = {},
) {
  return element(
    "multi_conversations_select",
    { ...input, initialConversations: dropEmpty(input.initialConversations) },
    settings,
  );
}

/**
 * Creates a multi-select menu whose options are loaded from the app's configured Options Load URL.
 *
 * @param input - Query threshold, initial options, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `multi_external_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, query, or selection is invalid.
 */
export function externalMultiSelect(
  input: ExternalMultiSelectInput,
  settings: FactorySettings = {},
) {
  return element(
    "multi_external_select",
    { ...input, initialOptions: dropEmpty(input.initialOptions) },
    settings,
  );
}

/**
 * Creates a multi-select menu from options embedded directly in the Block Kit payload.
 *
 * @param input - Options or option groups, initial selection, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `multi_static_select` element.
 * @throws InvalidUsageError when options, identifiers, or selection constraints are invalid.
 */
export function staticMultiSelect(
  input: StaticMultiSelectInput,
  settings: FactorySettings = {},
) {
  return element(
    "multi_static_select",
    {
      ...input,
      options: dropEmpty(input.options),
      optionGroups: dropEmpty(input.optionGroups),
      initialOptions: dropEmpty(input.initialOptions),
    },
    settings,
  );
}

/**
 * Creates a multi-select menu populated with users visible in the active workspace.
 *
 * @param input - Initial users, confirmation, selection limit, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `multi_users_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, or selection constraint is invalid.
 */
export function userMultiSelect(
  input: UserMultiSelectInput,
  settings: FactorySettings = {},
) {
  return element(
    "multi_users_select",
    { ...input, initialUsers: dropEmpty(input.initialUsers) },
    settings,
  );
}

/**
 * Creates a single-line input for whole or decimal numeric values.
 *
 * @param input - Decimal behavior, initial value, range, dispatch, and display settings.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `number_input` element.
 * @throws InvalidUsageError when an identifier, placeholder, or numeric range is invalid.
 */
export function numberInput(input: NumberElementInput, settings: FactorySettings = {}) {
  return element("number_input", { ...input }, settings);
}

/**
 * Creates an overflow menu.
 *
 * @param input - Action identifier, two to five options, and optional Slack fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `overflow` element.
 * @throws InvalidUsageError when the option list violates Slack's constraints.
 */
export function overflow(
  input: OverflowInput,
  settings: FactorySettings = {},
) {
  return element("overflow", { ...input }, settings);
}

/**
 * Creates a single- or multi-line freeform text input for a modal or App Home tab.
 *
 * @param input - Initial text, length limits, multiline, dispatch, and display settings.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `plain_text_input` element.
 * @throws InvalidUsageError when an identifier, placeholder, or length constraint is invalid.
 */
export function plainTextInput(
  input: PlainTextElementInput,
  settings: FactorySettings = {},
) {
  return element("plain_text_input", { ...input }, settings);
}

/**
 * Creates a radio-button group.
 *
 * @param input - Action identifier, options, and optional Slack fields.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `radio_buttons` element.
 * @throws InvalidUsageError when options or identifiers violate Slack's constraints.
 */
export function radioButtons(
  input: RadioButtonsInput,
  settings: FactorySettings = {},
) {
  return element("radio_buttons", { ...input }, settings);
}

/**
 * Creates a single-select menu populated with public channels visible to the current user.
 *
 * @param input - Initial channel, response URL, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `channels_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, or initial selection is invalid.
 */
export function channelSelect(input: ChannelSelectInput, settings: FactorySettings = {}) {
  return element("channels_select", { ...input }, settings);
}

/**
 * Creates a single-select menu of public channels, private channels, DMs, and group DMs.
 *
 * @param input - Conversation filter, initial selection, response URL, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `conversations_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, filter, or selection is invalid.
 */
export function conversationSelect(
  input: ConversationSelectInput,
  settings: FactorySettings = {},
) {
  return element("conversations_select", { ...input }, settings);
}

/**
 * Creates a single-select menu whose options are loaded from the app's Options Load URL.
 *
 * @param input - Query threshold, initial option, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `external_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, query, or option is invalid.
 */
export function externalSelect(input: ExternalSelectInput, settings: FactorySettings = {}) {
  return element("external_select", { ...input }, settings);
}

/**
 * Creates a single-select menu from options embedded directly in the Block Kit payload.
 *
 * @param input - Options or option groups, initial option, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `static_select` element.
 * @throws InvalidUsageError when options, identifiers, or selection constraints are invalid.
 */
export function staticSelect(input: StaticSelectInput, settings: FactorySettings = {}) {
  return element(
    "static_select",
    {
      ...input,
      options: dropEmpty(input.options),
      optionGroups: dropEmpty(input.optionGroups),
    },
    settings,
  );
}

/**
 * Creates a single-select menu populated with users visible in the active workspace.
 *
 * @param input - Initial user, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `users_select` element.
 * @throws InvalidUsageError when an identifier, placeholder, or initial user is invalid.
 */
export function userSelect(input: UserSelectInput, settings: FactorySettings = {}) {
  return element("users_select", { ...input }, settings);
}

/**
 * Creates a time picker.
 *
 * @param input - Initial time, timezone, confirmation, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `timepicker` element.
 * @throws InvalidUsageError when an identifier, placeholder, time, or timezone is invalid.
 */
export function timePicker(input: TimePickerInput, settings: FactorySettings = {}) {
  return element("timepicker", { ...input }, settings);
}

/**
 * Creates a URL input.
 *
 * @param input - Initial URL, dispatch configuration, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `url_text_input` element.
 * @throws InvalidUsageError when an identifier, placeholder, or initial URL is invalid.
 */
export function urlInput(input: UrlElementInput, settings: FactorySettings = {}) {
  return element("url_text_input", { ...input }, settings);
}

/**
 * Creates a button that launches a Slack workflow.
 *
 * @param input - Button label, workflow trigger, and optional interaction behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `workflow_button` element.
 * @throws InvalidUsageError when text or identifiers violate Slack's constraints.
 */
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

/**
 * Creates a rich-text input for a modal or App Home tab.
 *
 * @param input - Initial content, dispatch configuration, line limits, and display behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text_input` element.
 * @throws InvalidUsageError when an identifier, placeholder, content, or line limit is invalid.
 */
export function richTextInput(
  input: RichTextElementInput,
  settings: FactorySettings = {},
) {
  return element("rich_text_input", { ...input }, settings);
}
