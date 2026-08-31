/**
 * Fluent builders for Block Kit interactive and visual elements.
 *
 * @module elements
 */
import {
  button as createButton,
  channelMultiSelect as createChannelMultiSelect,
  channelSelect as createChannelSelect,
  checkboxes as createCheckboxes,
  conversationMultiSelect as createConversationMultiSelect,
  conversationSelect as createConversationSelect,
  datePicker as createDatePicker,
  dateTimePicker as createDateTimePicker,
  emailInput as createEmailInput,
  externalMultiSelect as createExternalMultiSelect,
  externalSelect as createExternalSelect,
  feedbackButton as createFeedbackButton,
  feedbackButtons as createFeedbackButtons,
  fileInput as createFileInput,
  iconButton as createIconButton,
  imageElement as createImageElement,
  numberInput as createNumberInput,
  overflow as createOverflow,
  plainTextInput as createPlainTextInput,
  radioButtons as createRadioButtons,
  richTextInput as createRichTextInput,
  staticMultiSelect as createStaticMultiSelect,
  staticSelect as createStaticSelect,
  timePicker as createTimePicker,
  urlInput as createUrlInput,
  urlSource as createUrlSource,
  userMultiSelect as createUserMultiSelect,
  userSelect as createUserSelect,
  workflowButton as createWorkflowButton,
  type ButtonInput,
  type ChannelMultiSelectInput,
  type ChannelSelectInput,
  type CheckboxesInput,
  type ConversationMultiSelectInput,
  type ConversationSelectInput,
  type DatePickerInput,
  type DateTimePickerInput,
  type EmailElementInput,
  type ExternalMultiSelectInput,
  type ExternalSelectInput,
  type FeedbackButtonInput,
  type ImageElementInput,
  type NumberElementInput,
  type OverflowInput,
  type PlainTextElementInput,
  type RadioButtonsInput,
  type RichTextElementInput,
  type StaticMultiSelectInput,
  type StaticSelectInput,
  type TimePickerInput,
  type UrlElementInput,
  type UserMultiSelectInput,
  type UserSelectInput,
  type WorkflowButtonInput,
} from "../legacy/elements.js";
import type { JsonObject, SlackObject } from "../types.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/**
 * Creates a fluent interactive button that can submit an action, open a URL, or
 * carry an application-defined value. Slack returns `actionId` and `value` in the
 * interaction payload when the user selects it.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/button-element>.
 */
export function Button(): FluentBuilder<ButtonInput, SlackObject<"button">> {
  return createFluentBuilder(createButton);
}

/**
 * Creates one labelled positive or negative choice for a {@link FeedbackButtons}
 * control. The choice's value is returned to the application when the user gives
 * feedback.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>.
 */
export function FeedbackButton(): FluentBuilder<FeedbackButtonInput, JsonObject> {
  return createFluentBuilder(createFeedbackButton);
}

/**
 * Creates a paired positive and negative feedback control for a context-actions
 * block. Build each choice with {@link FeedbackButton} so its visible text,
 * returned value, and accessibility label are validated.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>.
 */
export function FeedbackButtons(): FluentBuilder<
  FirstInput<typeof createFeedbackButtons>,
  SlackObject<"feedback_buttons">
> {
  return createFluentBuilder(createFeedbackButtons);
}

/**
 * Creates a compact icon-only action for a context-actions block. Slack currently
 * supports the `trash` icon, and the control can optionally be restricted to a
 * list of up to ten visible users.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element>.
 */
export function IconButton(): FluentBuilder<
  FirstInput<typeof createIconButton>,
  SlackObject<"icon_button">
> {
  return createFluentBuilder(createIconButton, {
    collections: { visibleToUserIds: "flat" },
  });
}

/**
 * Creates a labelled URL source for a task card. Use source links to identify the
 * external documents, tickets, or other resources from which a task originated.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/task-card-block>.
 */
export function UrlSource(): FluentBuilder<
  FirstInput<typeof createUrlSource>,
  SlackObject<"url">
> {
  return createFluentBuilder(createUrlSource);
}

/**
 * Creates a checkbox group that lets a user choose multiple items from a list of
 * up to ten options. Initial selections must correspond to options included in
 * the same element.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element>.
 */
export function Checkboxes(): FluentBuilder<CheckboxesInput, SlackObject<"checkboxes">> {
  return createFluentBuilder(createCheckboxes, {
    collections: { options: "flat", initialOptions: "flat" },
  });
}

/**
 * Creates an interactive calendar control for selecting one date. The optional
 * initial value uses `YYYY-MM-DD`, and Slack returns the selected date with the
 * configured action identifier.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element>.
 */
export function DatePicker(): FluentBuilder<DatePickerInput, SlackObject<"datepicker">> {
  return createFluentBuilder(createDatePicker);
}

/**
 * Creates an interactive control for selecting both a date and a time of day.
 * Initial and submitted values are represented as Unix timestamps in seconds.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element>.
 */
export function DateTimePicker(): FluentBuilder<
  DateTimePickerInput,
  SlackObject<"datetimepicker">
> {
  return createFluentBuilder(createDateTimePicker);
}

/**
 * Creates a single-line input specialized for email addresses. It can start with
 * an existing value and optionally dispatch interaction payloads while the user
 * edits the field.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/email-input-element>.
 */
export function EmailInput(): FluentBuilder<
  EmailElementInput,
  SlackObject<"email_text_input">
> {
  return createFluentBuilder(createEmailInput);
}

/**
 * Creates an interactive input that lets users upload files to Slack. Restrict
 * accepted formats with `filetypes()` and control the permitted number of files
 * with `maxFiles()`.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/file-input-element>.
 */
export function FileInput(): FluentBuilder<
  FirstInput<typeof createFileInput>,
  SlackObject<"file_input">
> {
  return createFluentBuilder(createFileInput, { collections: { filetypes: "flat" } });
}

/** Values configured through {@link ImageElement}. */
interface ImageElementBuilderInput {
  /** Alternative text for screen readers and unavailable images. */
  altText: string;
  /** Public URL of the image. Mutually exclusive with `slackFile`. */
  imageUrl?: string;
  /** Slack-hosted file object. Mutually exclusive with `imageUrl`. */
  slackFile?: JsonObject;
}

/**
 * Creates an image element for use inside section and context blocks. Supply
 * accessible alternative text and exactly one public image URL or Slack-hosted
 * file reference; use `ImageBlock()` for a standalone image.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/image-element>.
 */
export function ImageElement(): FluentBuilder<
  ImageElementBuilderInput,
  SlackObject<"image">
> {
  return createFluentBuilder<ImageElementBuilderInput, SlackObject<"image">>(
    (input, settings) => createImageElement(input as ImageElementInput, settings),
  );
}

/**
 * Creates a multi-select populated with public channels visible to the current
 * user. It can preselect channel IDs, limit the number selected, and show a
 * confirmation dialog before submitting the change.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select>.
 */
export function ChannelMultiSelect(): FluentBuilder<
  ChannelMultiSelectInput,
  Output<typeof createChannelMultiSelect>
> {
  return createFluentBuilder(createChannelMultiSelect, {
    collections: { initialChannels: "flat" },
  });
}

/**
 * Creates a multi-select populated with conversations visible to the current
 * user, including the conversation from which a view was opened when requested.
 * Apply a conversation filter to control which channel and DM types appear.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select>.
 */
export function ConversationMultiSelect(): FluentBuilder<
  ConversationMultiSelectInput,
  Output<typeof createConversationMultiSelect>
> {
  return createFluentBuilder(createConversationMultiSelect, {
    collections: { initialConversations: "flat" },
  });
}

/**
 * Creates a dynamic multi-select whose options are supplied by your application.
 * Slack requests matching options after the user types the configured minimum
 * number of characters.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select>.
 */
export function ExternalMultiSelect(): FluentBuilder<
  ExternalMultiSelectInput,
  Output<typeof createExternalMultiSelect>
> {
  return createFluentBuilder(createExternalMultiSelect, {
    collections: { initialOptions: "flat" },
  });
}

/**
 * Creates a multi-select from options defined directly in the Block Kit payload.
 * Supply either individual options or option groups, not both, and optionally
 * mark matching options as initially selected.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select>.
 */
export function StaticMultiSelect(): FluentBuilder<
  StaticMultiSelectInput,
  Output<typeof createStaticMultiSelect>
> {
  return createFluentBuilder(createStaticMultiSelect, {
    collections: {
      options: "flat",
      optionGroups: "flat",
      initialOptions: "flat",
    },
  });
}

/**
 * Creates a multi-select populated automatically with workspace users visible to
 * the current user. It can preselect user IDs and enforce a maximum number of
 * selections.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#user_multi_select>.
 */
export function UserMultiSelect(): FluentBuilder<
  UserMultiSelectInput,
  Output<typeof createUserMultiSelect>
> {
  return createFluentBuilder(createUserMultiSelect, {
    collections: { initialUsers: "flat" },
  });
}

/**
 * Creates an input that accepts whole numbers and, when enabled, decimal values
 * such as `0.25`, `5.5`, or `-10`. Optional minimum and maximum values constrain
 * what the user may submit.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/number-input-element>.
 */
export function NumberInput(): FluentBuilder<
  NumberElementInput,
  Output<typeof createNumberInput>
> {
  return createFluentBuilder(createNumberInput);
}

/**
 * Creates a compact overflow menu, conventionally displayed as an ellipsis, for
 * secondary actions. Slack requires between two and five options and returns the
 * selected option with the action identifier.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element>.
 */
export function Overflow(): FluentBuilder<OverflowInput, Output<typeof createOverflow>> {
  return createFluentBuilder(createOverflow, { collections: { options: "flat" } });
}

/**
 * Creates a free-form plain-text field similar to an HTML `<input>` or textarea.
 * Configure single-line or multiline display, initial text, character limits,
 * and when editing should dispatch an interaction payload.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element>.
 */
export function PlainTextInput(): FluentBuilder<
  PlainTextElementInput,
  Output<typeof createPlainTextInput>
> {
  return createFluentBuilder(createPlainTextInput);
}

/**
 * Creates a radio-button group that lets a user choose exactly one item from up
 * to ten options. An initial option may be selected before the element is shown.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element>.
 */
export function RadioButtons(): FluentBuilder<
  RadioButtonsInput,
  Output<typeof createRadioButtons>
> {
  return createFluentBuilder(createRadioButtons, { collections: { options: "flat" } });
}

/**
 * Creates a single-select populated with public channels visible to the current
 * user. It may start with one channel selected and can expose a response URL when
 * used inside a modal.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select>.
 */
export function ChannelSelect(): FluentBuilder<
  ChannelSelectInput,
  Output<typeof createChannelSelect>
> {
  return createFluentBuilder(createChannelSelect);
}

/**
 * Creates a single-select populated with visible public channels, private
 * channels, direct messages, and group DMs. Apply a conversation filter to limit
 * the available conversation types.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select>.
 */
export function ConversationSelect(): FluentBuilder<
  ConversationSelectInput,
  Output<typeof createConversationSelect>
> {
  return createFluentBuilder(createConversationSelect);
}

/**
 * Creates a dynamic single-select whose options are supplied by your application.
 * Slack requests matching options after the user types the configured minimum
 * number of characters.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select>.
 */
export function ExternalSelect(): FluentBuilder<
  ExternalSelectInput,
  Output<typeof createExternalSelect>
> {
  return createFluentBuilder(createExternalSelect);
}

/**
 * Creates a single-select from options defined directly in the Block Kit payload.
 * Supply either individual options or option groups, not both, and optionally set
 * one matching initial option.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select>.
 */
export function StaticSelect(): FluentBuilder<
  StaticSelectInput,
  Output<typeof createStaticSelect>
> {
  return createFluentBuilder(createStaticSelect, {
    collections: { options: "flat", optionGroups: "flat" },
  });
}

/**
 * Creates a single-select populated automatically with workspace users visible
 * to the current user. It can begin with one user ID already selected.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select>.
 */
export function UserSelect(): FluentBuilder<
  UserSelectInput,
  Output<typeof createUserSelect>
> {
  return createFluentBuilder(createUserSelect);
}

/**
 * Creates an interactive control for selecting a time of day. Initial values use
 * 24-hour `HH:mm` format, and an optional IANA timezone is displayed as supporting
 * text and returned with interactions.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element>.
 */
export function TimePicker(): FluentBuilder<
  TimePickerInput,
  Output<typeof createTimePicker>
> {
  return createFluentBuilder(createTimePicker);
}

/**
 * Creates a single-line field specialized for collecting a URL. It can start with
 * an existing value and optionally dispatch interaction payloads as the user
 * edits the field.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/url-input-element>.
 */
export function UrlInput(): FluentBuilder<
  UrlElementInput,
  Output<typeof createUrlInput>
> {
  return createFluentBuilder(createUrlInput);
}

/**
 * Creates a button that launches a Slack link trigger with optional customizable
 * inputs. Build the nested workflow, trigger, and input parameters with the
 * corresponding composition-object builders.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element>.
 */
export function WorkflowButton(): FluentBuilder<
  WorkflowButtonInput,
  Output<typeof createWorkflowButton>
> {
  return createFluentBuilder(createWorkflowButton);
}

/**
 * Creates a WYSIWYG rich-text editor similar to Slack's message composer. It can
 * start with structured rich text, dispatch changes as the user edits, and limit
 * the editor's visible height with minimum and maximum line counts.
 *
 * See: <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element>.
 */
export function RichTextInput(): FluentBuilder<
  RichTextElementInput,
  Output<typeof createRichTextInput>
> {
  return createFluentBuilder(createRichTextInput);
}
