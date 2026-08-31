/**
 * Block factories for messages, modals, and App Home tabs.
 *
 * Every factory accepts camelCase input, returns Slack's snake_case wire shape,
 * and validates the result unless `settings.validate` is `false`.
 *
 * @module blocks
 */
import type {
  ActionsBlock,
  ContextBlock,
  DividerBlock,
  FileBlock,
  HeaderBlock,
  ImageBlock,
  InputBlock,
  MarkdownBlock,
  RichTextBlock,
  SectionBlock,
  TableBlock,
  VideoBlock,
} from "@slack/types";

import { create, dropEmpty } from "../internal.js";
import { asText, type TextLike } from "./objects.js";
import type { FactorySettings, JsonObject, SlackWire } from "../types.js";
import type { SlackObject } from "../types.js";

/** Severity shown by an alert block. */
export type AlertLevel = "default" | "info" | "warning" | "error" | "success";

/** Horizontal width used by a container block. */
export type ContainerWidth = "narrow" | "standard" | "wide" | "full";

/** Lifecycle state shown by a task card. */
export type TaskStatus = "pending" | "in_progress" | "complete" | "error";

/**
 * Creates a severity-labelled notice for a modal.
 *
 * @param input - Alert content, severity, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `alert` block.
 * @throws InvalidUsageError when a field violates Slack's constraints.
 */
export function alertBlock(
  input: {
    /** Alert copy. Strings are converted to mrkdwn text. */
    text: TextLike;
    /** Visual severity. Defaults to `default`. */
    level?: AlertLevel;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"alert"> {
  return create(
    "alert",
    { ...input, text: asText(input.text, "mrkdwn", settings) },
    settings,
  );
}

/**
 * Content and presentation fields configured by `CardBlock()`. A card must set at
 * least one of `heroImage`, `title`, `actions`, or `body` before it is built.
 */
export interface CardBlockInput {
  /** Large image displayed above the card content. */
  heroImage?: JsonObject;
  /** Small image displayed beside the card heading. */
  icon?: JsonObject;
  /** Primary heading, up to 150 characters. */
  title?: TextLike;
  /** Secondary heading, up to 150 characters. */
  subtitle?: TextLike;
  /** Main card copy, up to 200 characters. */
  body?: TextLike;
  /** Up to three button actions. */
  actions?: JsonObject[];
  /** Slack-hosted icon created with `slackIcon`. */
  slackIcon?: JsonObject;
  /** Supporting copy displayed below the body. */
  subtext?: TextLike;
  /** Deterministic identifier, up to 255 characters. */
  blockId?: string;
}

/**
 * Creates a compact content card with text, imagery, and optional actions.
 *
 * @param input - Card fields. At least one visible content field must be supplied.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `card` block.
 * @throws InvalidUsageError when content is missing or exceeds Slack's limits.
 */
export function cardBlock(
  input: CardBlockInput,
  settings: FactorySettings = {},
): SlackObject<"card"> {
  return create(
    "card",
    {
      ...input,
      title: input.title === undefined ? undefined : asText(input.title, "mrkdwn", settings),
      subtitle:
        input.subtitle === undefined ? undefined : asText(input.subtitle, "mrkdwn", settings),
      body: input.body === undefined ? undefined : asText(input.body, "mrkdwn", settings),
      subtext:
        input.subtext === undefined ? undefined : asText(input.subtext, "mrkdwn", settings),
    },
    settings,
  );
}

/**
 * Creates a horizontally scrolling collection of cards.
 *
 * @param input - One to ten card blocks and an optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `carousel` block.
 * @throws InvalidUsageError when the card count is outside Slack's limits.
 */
export function carouselBlock(
  input: {
    /** Between one and ten objects returned by `cardBlock`. */
    elements: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"carousel"> {
  return create("carousel", input, settings);
}

/**
 * Creates a titled container that groups related child blocks.
 *
 * @param input - Child blocks plus optional heading, width, icon, and collapse behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `container` block.
 * @throws InvalidUsageError when child content or collapse options are invalid.
 */
export function containerBlock(
  input: {
    /** Up to ten blocks supported by Slack containers. */
    childBlocks: JsonObject[];
    /** Plain-text title. Mutually exclusive with `richTextTitle`. */
    title?: TextLike;
    /** Rich-text title block. Mutually exclusive with `title`. */
    richTextTitle?: JsonObject;
    /** Optional supporting copy below the title. */
    subtitle?: TextLike;
    /** Container width. Defaults to `standard`. */
    width?: ContainerWidth;
    /** Optional image displayed in the header. */
    icon?: JsonObject;
    /** Whether readers can expand and collapse the container. */
    isCollapsible?: boolean;
    /** Whether a collapsible container starts collapsed. */
    defaultCollapsed?: boolean;
    /** Whether Slack draws a divider below the header. */
    hasHeaderDivider?: boolean;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"container"> {
  return create(
    "container",
    {
      ...input,
      title: input.title === undefined ? undefined : asText(input.title, "plain_text", settings),
      subtitle:
        input.subtitle === undefined ? undefined : asText(input.subtitle, "mrkdwn", settings),
    },
    settings,
  );
}

/**
 * Creates contextual feedback or icon controls.
 *
 * @param input - Up to five feedback-buttons or icon-button elements.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `context_actions` block.
 * @throws InvalidUsageError when an element is unsupported or the limit is exceeded.
 */
export function contextActionsBlock(
  input: {
    /** Feedback-buttons or icon-button elements. */
    elements: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"context_actions"> {
  return create("context_actions", input, settings);
}

/**
 * Creates a sortable data table.
 *
 * @param input - Caption, rows, pagination size, and row-header configuration.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `data_table` block.
 * @throws InvalidUsageError when row dimensions, cells, or pagination are invalid.
 */
export function dataTableBlock(
  input: {
    /** Two to 201 equally sized rows containing raw text, raw numbers, or rich text. */
    rows: JsonObject[][];
    /** Accessible table caption. */
    caption: string;
    /** Rows per page, between 1 and 100. Defaults to 5. */
    pageSize?: number;
    /** Zero-based column used as the row header. Defaults to 0. */
    rowHeaderColumnIndex?: number;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"data_table"> {
  return create("data_table", { pageSize: 5, rowHeaderColumnIndex: 0, ...input }, settings);
}

/**
 * Creates a chart rendered by Slack.
 *
 * @param input - Chart title, chart object, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `data_visualization` block.
 * @throws InvalidUsageError when chart data or labels violate Slack's limits.
 */
export function dataVisualizationBlock(
  input: {
    /** Chart heading, up to 50 characters. */
    title: string;
    /** Object returned by `pieChart`, `barChart`, `areaChart`, or `lineChart`. */
    chart: JsonObject;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"data_visualization"> {
  return create("data_visualization", input, settings);
}

/**
 * Creates one task card for a plan.
 *
 * @param input - Task identity, title, rich content, sources, and optional status.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `task_card` block.
 * @throws InvalidUsageError when identifiers, content, or source links are invalid.
 */
export function taskCardBlock(
  input: {
    /** Stable task identifier. */
    taskId: string;
    /** Human-readable task title. */
    title: string;
    /** Optional rich-text task details. */
    details?: JsonObject;
    /** Optional rich-text task output. */
    output?: JsonObject;
    /** Optional source links created with `urlSource`. */
    sources?: JsonObject[];
    /** Current task lifecycle state. */
    status?: TaskStatus;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"task_card"> {
  return create("task_card", input, settings);
}

/**
 * Creates a titled sequence of task cards.
 *
 * @param input - Plan title, tasks, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `plan` block.
 * @throws InvalidUsageError when task content violates Slack's constraints.
 */
export function planBlock(
  input: {
    /** Human-readable plan title. */
    title: string;
    /** Task-card blocks. Their outer `type` and `block_id` fields are omitted in the plan. */
    tasks?: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"plan"> {
  return create(
    "plan",
    {
      ...input,
      tasks: input.tasks?.map(({ type: _type, block_id: _blockId, ...task }) => task),
    },
    settings,
  );
}

/**
 * Text, field, accessory, and identity fields configured by `SectionBlock()`.
 * Every section must contain main text, one or more fields, or both.
 */
export interface SectionBlockInput {
  /** Main copy. Strings are converted to mrkdwn text. */
  text?: TextLike;
  /** Deterministic identifier, up to 255 characters. */
  blockId?: string;
  /** Up to ten text fields displayed in columns. */
  fields?: TextLike[];
  /** Optional interactive or visual element displayed beside the text. */
  accessory?: JsonObject;
}

/**
 * Creates a flexible text block with optional fields or an accessory.
 *
 * @param input - Section text, fields, accessory, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `section` block.
 * @throws InvalidUsageError when text is missing or a field exceeds Slack's limits.
 */
export function sectionBlock(
  input: SectionBlockInput,
  settings: FactorySettings = {},
): SlackWire<SectionBlock> {
  return create(
    "section",
    {
      ...input,
      text: input.text === undefined ? undefined : asText(input.text, "mrkdwn", settings),
      fields: dropEmpty(input.fields)?.map((field) => asText(field, "mrkdwn", settings)),
    },
    settings,
  ) as SlackWire<SectionBlock>;
}

/**
 * Creates a row of interactive elements.
 *
 * @param input - Up to 25 buttons, select menus, or other action elements.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `actions` block.
 * @throws InvalidUsageError when an element is unsupported or the limit is exceeded.
 */
export function actionsBlock(
  input: {
    /** Interactive elements displayed in the row. */
    elements: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<ActionsBlock> {
  return create("actions", input, settings) as SlackWire<ActionsBlock>;
}

/**
 * Creates compact contextual text and images.
 *
 * @param input - Up to ten text objects or image elements.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `context` block.
 * @throws InvalidUsageError when an element is unsupported or the limit is exceeded.
 */
export function contextBlock(
  input: {
    /** Text objects and image elements. */
    elements: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<ContextBlock> {
  return create("context", input, settings) as SlackWire<ContextBlock>;
}

/**
 * Creates a visual divider between blocks.
 *
 * @param input - Optional deterministic block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `divider` block.
 * @throws InvalidUsageError when the block identifier is too long.
 */
export function dividerBlock(
  input: {
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  } = {},
  settings: FactorySettings = {},
): SlackWire<DividerBlock> {
  return create("divider", input, settings) as SlackWire<DividerBlock>;
}

/**
 * Creates a block that displays a Slack remote file.
 *
 * @param input - Remote-file identifier, source, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `file` block.
 * @throws InvalidUsageError when required file data is missing.
 */
export function fileBlock(
  input: {
    /** Identifier assigned when the remote file was added to Slack. */
    externalId: string;
    /** Remote-file source. Slack currently accepts only `remote`. */
    source?: "remote";
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<FileBlock> {
  return create("file", { source: "remote", ...input }, settings) as SlackWire<FileBlock>;
}

/**
 * Creates a prominent plain-text heading.
 *
 * @param input - Heading text and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `header` block.
 * @throws InvalidUsageError when the heading exceeds Slack's limit.
 */
export function headerBlock(
  input: {
    /** Heading copy. Strings are converted to plain text. */
    text: TextLike;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<HeaderBlock> {
  return create(
    "header",
    { ...input, text: asText(input.text, "plain_text", settings) },
    settings,
  ) as SlackWire<HeaderBlock>;
}

/**
 * Creates an image block with optional title text.
 *
 * @param input - Image URL, accessible alternative, title, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `image` block.
 * @throws InvalidUsageError when text or URL fields violate Slack's constraints.
 */
export function imageBlock(
  input: {
    /** Public URL of the image. */
    imageUrl: string;
    /** Accessible description of the image. */
    altText: string;
    /** Optional plain-text title. */
    title?: TextLike;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<ImageBlock> {
  return create(
    "image",
    {
      ...input,
      title: input.title === undefined ? undefined : asText(input.title, "plain_text", settings),
    },
    settings,
  ) as SlackWire<ImageBlock>;
}

/**
 * Creates a labelled form control for a modal or App Home tab.
 *
 * @param input - Label, input-compatible element, and optional form behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `input` block.
 * @throws InvalidUsageError when the element is unsupported or text exceeds Slack's limits.
 */
export function inputBlock(
  input: {
    /** Plain-text label displayed above the control. */
    label: TextLike;
    /** Input-compatible element such as a text input, picker, or select menu. */
    element: JsonObject;
    /** Whether changes dispatch an interaction immediately. */
    dispatchAction?: boolean;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
    /** Optional plain-text help shown below the control. */
    hint?: TextLike;
    /** Whether the user may submit without completing this input. */
    optional?: boolean;
  },
  settings: FactorySettings = {},
): SlackWire<InputBlock> {
  return create(
    "input",
    {
      ...input,
      label: asText(input.label, "plain_text", settings),
      hint: input.hint === undefined ? undefined : asText(input.hint, "plain_text", settings),
    },
    settings,
  ) as SlackWire<InputBlock>;
}

/**
 * Creates a block rendered from GitHub-flavored Markdown.
 *
 * @param input - Markdown source and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `markdown` block.
 * @throws InvalidUsageError when the Markdown source violates Slack's limits.
 */
export function markdownBlock(
  input: {
    /** GitHub-flavored Markdown source. */
    text: string;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<MarkdownBlock> {
  return create("markdown", input, settings) as SlackWire<MarkdownBlock>;
}

/**
 * Creates a rich-text block from rich-text layout objects.
 *
 * @param input - Rich-text sections, lists, quotes, or code blocks.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text` block.
 * @throws InvalidUsageError when an element is not a supported rich-text object.
 */
export function richTextBlock(
  input: {
    /** Rich-text layout objects. */
    elements: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<RichTextBlock> {
  return create("rich_text", input, settings) as SlackWire<RichTextBlock>;
}

/**
 * Creates a table block from raw-text or rich-text cells.
 *
 * @param input - Rows, optional column display settings, and optional block identifier.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `table` block.
 * @throws InvalidUsageError when dimensions, cells, or column settings are invalid.
 */
export function tableBlock(
  input: {
    /** Up to 100 equally sized rows of raw-text or rich-text cells. */
    rows: JsonObject[][];
    /** Optional display settings for each column. */
    columnSettings?: JsonObject[];
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackWire<TableBlock> {
  return create("table", input, settings) as SlackWire<TableBlock>;
}

/**
 * Creates an embedded video block.
 *
 * Slack, rather than this library, enforces its provider allowlist when the
 * payload is submitted.
 *
 * @param input - Video URL, thumbnail, accessible text, title, and optional metadata.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `video` block.
 * @throws InvalidUsageError when a text field violates Slack's constraints.
 */
export function videoBlock(
  input: {
    /** Accessible summary, up to 200 characters. */
    altText: string;
    /** Public preview-image URL. */
    thumbnailUrl: string;
    /** Plain-text video title, up to 200 characters. */
    title: TextLike;
    /** URL of a video hosted by a Slack-supported provider. */
    videoUrl: string;
    /** Deterministic identifier, up to 255 characters. */
    blockId?: string;
    /** Optional author name, up to 50 characters. */
    authorName?: string;
    /** Optional plain-text description, up to 200 characters. */
    description?: TextLike;
    /** Optional provider icon URL. */
    providerIconUrl?: string;
    /** Optional provider name, up to 50 characters. */
    providerName?: string;
    /** Optional destination when the title is selected. */
    titleUrl?: string;
  },
  settings: FactorySettings = {},
): SlackWire<VideoBlock> {
  return create(
    "video",
    {
      ...input,
      title: asText(input.title, "plain_text", settings),
      description:
        input.description === undefined
          ? undefined
          : asText(input.description, "plain_text", settings),
    },
    settings,
  ) as SlackWire<VideoBlock>;
}
