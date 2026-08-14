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

export interface SectionInput {
  text?: TextLike;
  blockId?: string;
  fields?: TextLike[];
  accessory?: JsonObject;
}

export function section(
  input: SectionInput,
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

export function actions(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<ActionsBlock> {
  return create("actions", input, settings) as SlackWire<ActionsBlock>;
}

export function context(
  input: { elements: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<ContextBlock> {
  return create("context", input, settings) as SlackWire<ContextBlock>;
}

export function divider(
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

export function header(
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

export function input(
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

export function table(
  input: { rows: JsonObject[][]; columnSettings?: JsonObject[]; blockId?: string },
  settings: FactorySettings = {},
): SlackWire<TableBlock> {
  return create("table", input, settings) as SlackWire<TableBlock>;
}

export function video(
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
