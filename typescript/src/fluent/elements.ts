/** Fluent builders for Block Kit interactive and visual elements. */
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
} from "../elements.js";
import type { JsonObject, SlackObject } from "../types.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/** Starts a fluent interactive button. */
export function Button(): FluentBuilder<ButtonInput, SlackObject<"button">> {
  return createFluentBuilder(createButton);
}

/** Starts one fluent choice for a feedback control. */
export function FeedbackButton(): FluentBuilder<FeedbackButtonInput, JsonObject> {
  return createFluentBuilder(createFeedbackButton);
}

/** Starts a fluent paired positive/negative feedback control. */
export function FeedbackButtons(): FluentBuilder<
  FirstInput<typeof createFeedbackButtons>,
  SlackObject<"feedback_buttons">
> {
  return createFluentBuilder(createFeedbackButtons);
}

/** Starts a fluent compact icon action. */
export function IconButton(): FluentBuilder<
  FirstInput<typeof createIconButton>,
  SlackObject<"icon_button">
> {
  return createFluentBuilder(createIconButton, {
    collections: { visibleToUserIds: "flat" },
  });
}

/** Starts a fluent source link for a task card. */
export function UrlSource(): FluentBuilder<
  FirstInput<typeof createUrlSource>,
  SlackObject<"url">
> {
  return createFluentBuilder(createUrlSource);
}

/** Starts a fluent checkbox group. */
export function Checkboxes(): FluentBuilder<CheckboxesInput, SlackObject<"checkboxes">> {
  return createFluentBuilder(createCheckboxes, {
    collections: { options: "flat", initialOptions: "flat" },
  });
}

/** Starts a fluent date picker. */
export function DatePicker(): FluentBuilder<DatePickerInput, SlackObject<"datepicker">> {
  return createFluentBuilder(createDatePicker);
}

/** Starts a fluent date-and-time picker. */
export function DateTimePicker(): FluentBuilder<
  DateTimePickerInput,
  SlackObject<"datetimepicker">
> {
  return createFluentBuilder(createDateTimePicker);
}

/** Starts a fluent email-address input. */
export function EmailInput(): FluentBuilder<
  EmailElementInput,
  SlackObject<"email_text_input">
> {
  return createFluentBuilder(createEmailInput);
}

/** Starts a fluent file-upload input. */
export function FileInput(): FluentBuilder<
  FirstInput<typeof createFileInput>,
  SlackObject<"file_input">
> {
  return createFluentBuilder(createFileInput, { collections: { filetypes: "flat" } });
}

interface ImageElementBuilderInput {
  altText: string;
  imageUrl?: string;
  slackFile?: JsonObject;
}

/** Starts a fluent image element backed by a URL or Slack file. */
export function ImageElement(): FluentBuilder<
  ImageElementBuilderInput,
  SlackObject<"image">
> {
  return createFluentBuilder<ImageElementBuilderInput, SlackObject<"image">>(
    (input, settings) => createImageElement(input as ImageElementInput, settings),
  );
}

/** Starts a fluent multi-select populated with public channels. */
export function ChannelMultiSelect(): FluentBuilder<
  ChannelMultiSelectInput,
  Output<typeof createChannelMultiSelect>
> {
  return createFluentBuilder(createChannelMultiSelect, {
    collections: { initialChannels: "flat" },
  });
}

/** Starts a fluent multi-select populated with conversations. */
export function ConversationMultiSelect(): FluentBuilder<
  ConversationMultiSelectInput,
  Output<typeof createConversationMultiSelect>
> {
  return createFluentBuilder(createConversationMultiSelect, {
    collections: { initialConversations: "flat" },
  });
}

/** Starts a fluent multi-select populated by an external data source. */
export function ExternalMultiSelect(): FluentBuilder<
  ExternalMultiSelectInput,
  Output<typeof createExternalMultiSelect>
> {
  return createFluentBuilder(createExternalMultiSelect, {
    collections: { initialOptions: "flat" },
  });
}

/** Starts a fluent multi-select populated with static options. */
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

/** Starts a fluent multi-select populated with users. */
export function UserMultiSelect(): FluentBuilder<
  UserMultiSelectInput,
  Output<typeof createUserMultiSelect>
> {
  return createFluentBuilder(createUserMultiSelect, {
    collections: { initialUsers: "flat" },
  });
}

/** Starts a fluent numeric input. */
export function NumberInput(): FluentBuilder<
  NumberElementInput,
  Output<typeof createNumberInput>
> {
  return createFluentBuilder(createNumberInput);
}

/** Starts a fluent overflow menu. */
export function Overflow(): FluentBuilder<OverflowInput, Output<typeof createOverflow>> {
  return createFluentBuilder(createOverflow, { collections: { options: "flat" } });
}

/** Starts a fluent plain-text input. */
export function PlainTextInput(): FluentBuilder<
  PlainTextElementInput,
  Output<typeof createPlainTextInput>
> {
  return createFluentBuilder(createPlainTextInput);
}

/** Starts a fluent radio-button group. */
export function RadioButtons(): FluentBuilder<
  RadioButtonsInput,
  Output<typeof createRadioButtons>
> {
  return createFluentBuilder(createRadioButtons, { collections: { options: "flat" } });
}

/** Starts a fluent single-select populated with public channels. */
export function ChannelSelect(): FluentBuilder<
  ChannelSelectInput,
  Output<typeof createChannelSelect>
> {
  return createFluentBuilder(createChannelSelect);
}

/** Starts a fluent single-select populated with conversations. */
export function ConversationSelect(): FluentBuilder<
  ConversationSelectInput,
  Output<typeof createConversationSelect>
> {
  return createFluentBuilder(createConversationSelect);
}

/** Starts a fluent single-select populated by an external data source. */
export function ExternalSelect(): FluentBuilder<
  ExternalSelectInput,
  Output<typeof createExternalSelect>
> {
  return createFluentBuilder(createExternalSelect);
}

/** Starts a fluent single-select populated with static options. */
export function StaticSelect(): FluentBuilder<
  StaticSelectInput,
  Output<typeof createStaticSelect>
> {
  return createFluentBuilder(createStaticSelect, {
    collections: { options: "flat", optionGroups: "flat" },
  });
}

/** Starts a fluent single-select populated with users. */
export function UserSelect(): FluentBuilder<
  UserSelectInput,
  Output<typeof createUserSelect>
> {
  return createFluentBuilder(createUserSelect);
}

/** Starts a fluent time picker. */
export function TimePicker(): FluentBuilder<
  TimePickerInput,
  Output<typeof createTimePicker>
> {
  return createFluentBuilder(createTimePicker);
}

/** Starts a fluent URL input. */
export function UrlInput(): FluentBuilder<
  UrlElementInput,
  Output<typeof createUrlInput>
> {
  return createFluentBuilder(createUrlInput);
}

/** Starts a fluent button that launches a Slack workflow. */
export function WorkflowButton(): FluentBuilder<
  WorkflowButtonInput,
  Output<typeof createWorkflowButton>
> {
  return createFluentBuilder(createWorkflowButton);
}

/** Starts a fluent rich-text editor input. */
export function RichTextInput(): FluentBuilder<
  RichTextElementInput,
  Output<typeof createRichTextInput>
> {
  return createFluentBuilder(createRichTextInput);
}
