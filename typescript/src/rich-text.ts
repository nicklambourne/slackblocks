import { create } from "./internal.js";
import type { FactorySettings, JsonObject } from "./types.js";

export interface RichTextStyle {
  bold?: boolean;
  italic?: boolean;
  strike?: boolean;
  code?: boolean;
}

export function richText(
  text: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("text", { text, style }, settings);
}

export function richTextChannel(
  channelId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("channel", { channelId, style }, settings);
}

export function richTextEmoji(
  name: string,
  settings: FactorySettings = {},
) {
  return create("emoji", { name }, settings);
}

export function richTextLink(
  input: { url: string; text?: string; unsafe?: boolean; style?: RichTextStyle },
  settings: FactorySettings = {},
) {
  return create("link", input, settings);
}

export function richTextUser(
  userId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("user", { userId, style }, settings);
}

export function richTextUserGroup(
  usergroupId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("usergroup", { usergroupId, style }, settings);
}

export function richTextSection(
  elements: JsonObject[],
  settings: FactorySettings = {},
) {
  return create("rich_text_section", { elements }, settings);
}

export function richTextList(
  input: {
    elements: JsonObject[];
    style?: "bullet" | "ordered";
    indent?: number;
    offset?: number;
    border?: number;
  },
  settings: FactorySettings = {},
) {
  return create("rich_text_list", input, settings);
}

export function richTextCodeBlock(
  elements: JsonObject[],
  options: { border?: number } = {},
  settings: FactorySettings = {},
) {
  return create("rich_text_preformatted", { elements, ...options }, settings);
}

export function richTextQuote(
  elements: JsonObject[],
  options: { border?: number } = {},
  settings: FactorySettings = {},
) {
  return create("rich_text_quote", { elements, ...options }, settings);
}
