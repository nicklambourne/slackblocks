/**
 * Inline and layout factories for Slack rich-text blocks.
 *
 * Build inline elements first, combine them in a section, list, quote, or code
 * block, then pass those layout objects to `richTextBlock`.
 *
 * @module rich-text
 */
import { create } from "../internal.js";
import type { FactorySettings, JsonObject } from "../types.js";

/** Inline formatting supported by rich-text text, links, users, and channels. */
export interface RichTextStyle {
  /** Render the inline content in bold. */
  bold?: boolean;
  /** Render the inline content in italics. */
  italic?: boolean;
  /** Render the inline content with a strikethrough. */
  strike?: boolean;
  /** Render the inline content as code. */
  code?: boolean;
}

/**
 * Creates a styled rich-text text run.
 *
 * @param text - Text content.
 * @param style - Optional inline formatting.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `text` element.
 */
export function richText(
  text: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("text", { text, style }, settings);
}

/**
 * Creates a rich-text channel mention.
 *
 * @param channelId - Slack channel identifier.
 * @param style - Optional inline formatting.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `channel` element.
 */
export function richTextChannel(
  channelId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("channel", { channelId, style }, settings);
}

/**
 * Creates a rich-text emoji.
 *
 * @param name - Emoji name without surrounding colons.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `emoji` element.
 */
export function richTextEmoji(
  name: string,
  settings: FactorySettings = {},
) {
  return create("emoji", { name }, settings);
}

/**
 * Creates a rich-text link.
 *
 * @param input - Destination URL, optional label, safety flag, and formatting.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `link` element.
 */
export function richTextLink(
  input: {
    /** Destination URL. */
    url: string;
    /** Optional visible label. Slack displays the URL when omitted. */
    text?: string;
    /** Mark a URL as unsafe when mirroring a Slack-provided payload. */
    unsafe?: boolean;
    /** Optional inline formatting. */
    style?: RichTextStyle;
  },
  settings: FactorySettings = {},
) {
  return create("link", input, settings);
}

/**
 * Creates a rich-text user mention.
 *
 * @param userId - Slack user identifier.
 * @param style - Optional inline formatting.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `user` element.
 */
export function richTextUser(
  userId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("user", { userId, style }, settings);
}

/**
 * Creates a rich-text user-group mention.
 *
 * @param usergroupId - Slack user-group identifier.
 * @param style - Optional inline formatting.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack rich-text `usergroup` element.
 */
export function richTextUserGroup(
  usergroupId: string,
  style?: RichTextStyle,
  settings: FactorySettings = {},
) {
  return create("usergroup", { usergroupId, style }, settings);
}

/**
 * Creates a paragraph-like rich-text section.
 *
 * @param elements - Inline rich-text elements in display order.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text_section` object.
 */
export function richTextSection(
  elements: JsonObject[],
  settings: FactorySettings = {},
) {
  return create("rich_text_section", { elements }, settings);
}

/**
 * Creates an ordered or bulleted rich-text list.
 *
 * @param input - List items, marker style, and optional list layout.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text_list` object.
 * @throws MissingRequiredError when `style` is not provided.
 */
export function richTextList(
  input: {
    /** Rich-text section objects used as list items. */
    elements: JsonObject[];
    /** List marker style. */
    style: "bullet" | "ordered";
    /** Nesting depth. */
    indent?: number;
    /** Starting number for an ordered list. */
    offset?: number;
    /** Optional border thickness. */
    border?: number;
  },
  settings: FactorySettings = {},
) {
  return create("rich_text_list", input, settings);
}

/**
 * Creates a preformatted rich-text code block.
 *
 * @param elements - Inline rich-text elements displayed as code.
 * @param options - Optional border thickness.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text_preformatted` object.
 */
export function richTextCodeBlock(
  elements: JsonObject[],
  options: {
    /** Optional border thickness. */
    border?: number;
  } = {},
  settings: FactorySettings = {},
) {
  return create("rich_text_preformatted", { elements, ...options }, settings);
}

/**
 * Creates a rich-text block quote.
 *
 * @param elements - Inline rich-text elements displayed in the quote.
 * @param options - Optional border thickness.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `rich_text_quote` object.
 */
export function richTextQuote(
  elements: JsonObject[],
  options: {
    /** Optional border thickness. */
    border?: number;
  } = {},
  settings: FactorySettings = {},
) {
  return create("rich_text_quote", { elements, ...options }, settings);
}
