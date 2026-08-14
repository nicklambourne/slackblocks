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

import { create } from "./internal.js";
import { asText, type TextLike } from "./objects.js";
import type { FactorySettings, JsonObject, SlackWire } from "./types.js";
import type { SlackObject } from "./types.js";

export type AlertLevel = "default" | "info" | "warning" | "error" | "success";
export type ContainerWidth = "narrow" | "standard" | "wide" | "full";
export type TaskStatus = "pending" | "in_progress" | "complete" | "error";

export function alertBlock(
  input: { text: TextLike; level?: AlertLevel; blockId?: string },
  settings: FactorySettings = {},
): SlackObject<"alert"> {
  return create(
    "alert",
    { ...input, text: asText(input.text, "mrkdwn", settings) },
    settings,
  );
}

export interface CardBlockInput {
  heroImage?: JsonObject;
  icon?: JsonObject;
  title?: TextLike;
  subtitle?: TextLike;
  body?: TextLike;
  actions?: JsonObject[];
  slackIcon?: JsonObject;
  subtext?: TextLike;
  blockId?: string;
}

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

export function carouselBlock(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackObject<"carousel"> {
  return create("carousel", input, settings);
}

export function containerBlock(
  input: {
    childBlocks: JsonObject[];
    title?: TextLike;
    richTextTitle?: JsonObject;
    subtitle?: TextLike;
    width?: ContainerWidth;
    icon?: JsonObject;
    isCollapsible?: boolean;
    defaultCollapsed?: boolean;
    hasHeaderDivider?: boolean;
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

export function contextActionsBlock(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackObject<"context_actions"> {
  return create("context_actions", input, settings);
}

export function dataTableBlock(
  input: {
    rows: JsonObject[][];
    caption: string;
    pageSize?: number;
    rowHeaderColumnIndex?: number;
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"data_table"> {
  return create("data_table", { pageSize: 5, rowHeaderColumnIndex: 0, ...input }, settings);
}

export function dataVisualizationBlock(
  input: { title: string; chart: JsonObject; blockId?: string },
  settings: FactorySettings = {},
): SlackObject<"data_visualization"> {
  return create("data_visualization", input, settings);
}

export function taskCardBlock(
  input: {
    taskId: string;
    title: string;
    details?: JsonObject;
    output?: JsonObject;
    sources?: JsonObject[];
    status?: TaskStatus;
    blockId?: string;
  },
  settings: FactorySettings = {},
): SlackObject<"task_card"> {
  return create("task_card", input, settings);
}

export function planBlock(
  input: { title: string; tasks?: JsonObject[]; blockId?: string },
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

export interface SectionBlockInput {
  text?: TextLike;
  blockId?: string;
  fields?: TextLike[];
  accessory?: JsonObject;
}

export function sectionBlock(
  input: SectionBlockInput,
  settings: FactorySettings = {},
): SlackWire<SectionBlock> {
  return create(
    "section",
    {
      ...input,
      text: input.text === undefined ? undefined : asText(input.text, "mrkdwn", settings),
      fields: input.fields?.map((field) => asText(field, "mrkdwn", settings)),
    },
    settings,
  ) as SlackWire<SectionBlock>;
}

export function actionsBlock(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<ActionsBlock> {
  return create("actions", input, settings) as SlackWire<ActionsBlock>;
}

export function contextBlock(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<ContextBlock> {
  return create("context", input, settings) as SlackWire<ContextBlock>;
}

export function dividerBlock(
  input: { blockId?: string } = {},
  settings: FactorySettings = {},
): SlackWire<DividerBlock> {
  return create("divider", input, settings) as SlackWire<DividerBlock>;
}

export function fileBlock(
  input: { externalId: string; source?: "remote"; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<FileBlock> {
  return create("file", { source: "remote", ...input }, settings) as SlackWire<FileBlock>;
}

export function headerBlock(
  input: { text: TextLike; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<HeaderBlock> {
  return create(
    "header",
    { ...input, text: asText(input.text, "plain_text", settings) },
    settings,
  ) as SlackWire<HeaderBlock>;
}

export function imageBlock(
  input: { imageUrl: string; altText?: string; title?: TextLike; blockId?: string },
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

export function inputBlock(
  value: {
    label: TextLike;
    element: JsonObject;
    dispatchAction?: boolean;
    blockId?: string;
    hint?: TextLike;
    optional?: boolean;
  },
  settings: FactorySettings = {},
): SlackWire<InputBlock> {
  return create(
    "input",
    {
      ...value,
      label: asText(value.label, "plain_text", settings),
      hint: value.hint === undefined ? undefined : asText(value.hint, "plain_text", settings),
    },
    settings,
  ) as SlackWire<InputBlock>;
}

export function markdownBlock(
  input: { text: string; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<MarkdownBlock> {
  return create("markdown", input, settings) as SlackWire<MarkdownBlock>;
}

export function richTextBlock(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<RichTextBlock> {
  return create("rich_text", input, settings) as SlackWire<RichTextBlock>;
}

export function tableBlock(
  input: { rows: JsonObject[][]; columnSettings?: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<TableBlock> {
  return create("table", input, settings) as SlackWire<TableBlock>;
}

export function videoBlock(
  input: {
    altText: string;
    thumbnailUrl: string;
    title: TextLike;
    videoUrl: string;
    blockId?: string;
    authorName?: string;
    description?: TextLike;
    providerIconUrl?: string;
    providerName?: string;
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
