/**
 * Payload factories for Web API messages, webhooks, and interaction responses.
 *
 * @module messages
 */
import { createObject } from "./internal.js";
import type { FactorySettings, JsonObject } from "./types.js";

/**
 * Creates lower-priority supporting content using Slack's legacy secondary-attachment format.
 *
 * Attachments can add context beneath a message, while `fallback` supplies a
 * plain-text summary for notifications and clients that cannot display Block Kit.
 *
 * @param input - Attachment blocks plus optional color and fallback text.
 * @param settings - Per-call validation settings.
 * @returns A Slack attachment object.
 * @throws InvalidUsageError when a nested block violates a supported Block Kit constraint.
 * @see https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments
 */
export function attachment(
  input: {
    /** Blocks displayed inside the attachment. */
    blocks: JsonObject[];
    /** Optional side-border color. */
    color?: string;
    /** Plain-text fallback for notifications and clients without Block Kit support. */
    fallback?: string;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

/** Fields accepted by {@link message}. */
export interface MessageInput {
  /** Channel, group, or direct-message conversation identifier. */
  channel: string;
  /** Block Kit blocks displayed in the message. */
  blocks?: JsonObject[];
  /** Optional secondary attachments. */
  attachments?: JsonObject[];
  /** Notification and accessibility fallback text. */
  text?: string;
  /** Whether Slack parses `text` as mrkdwn. Defaults to `true`. */
  mrkdwn?: boolean;
  /** Whether Slack unfurls links. */
  unfurlLinks?: boolean;
  /** Whether Slack unfurls media. */
  unfurlMedia?: boolean;
  /** Optional message metadata. */
  metadata?: JsonObject;
}

/**
 * Creates a payload for Slack Web API message methods such as `chat.postMessage`.
 *
 * @param input - Destination channel and message content.
 * @param settings - Per-call validation settings.
 * @returns A Slack-shaped message payload ready to spread into an SDK call.
 * @throws InvalidUsageError when nested Block Kit content is invalid.
 */
export function message(
  input: MessageInput,
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ mrkdwn: true, text: "", ...input }, settings);
}

/**
 * Creates a response payload for slash commands and interactive requests.
 *
 * @param input - Response content, visibility, and replacement behavior.
 * @param settings - Per-call validation settings.
 * @returns A Slack interaction-response payload.
 * @throws InvalidUsageError when nested Block Kit content is invalid.
 */
export function messageResponse(
  input: {
    /** Block Kit blocks displayed in the response. */
    blocks?: JsonObject[];
    /** Optional secondary attachments. */
    attachments?: JsonObject[];
    /** Notification and accessibility fallback text. */
    text?: string;
    /** Whether Slack parses `text` as mrkdwn. Defaults to `true`. */
    mrkdwn?: boolean;
    /** Replace the original interaction message. Defaults to `false`. */
    replaceOriginal?: boolean;
    /** Response visibility. Defaults to `in_channel`. */
    responseType?: "ephemeral" | "in_channel";
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(
    {
      mrkdwn: true,
      text: "",
      replaceOriginal: false,
      responseType: "in_channel",
      ...input,
    },
    settings,
  );
}

/**
 * Creates a payload for an incoming webhook or response URL.
 *
 * @param input - Message content, visibility, replacement behavior, and unfurl settings.
 * @param settings - Per-call validation settings.
 * @returns A Slack webhook-message payload.
 * @throws InvalidUsageError when nested Block Kit content is invalid.
 */
export function webhookMessage(
  input: {
    /** Block Kit blocks displayed in the message. */
    blocks?: JsonObject[];
    /** Optional secondary attachments. */
    attachments?: JsonObject[];
    /** Notification and accessibility fallback text. */
    text?: string;
    /** Response visibility for response URLs. */
    responseType?: "ephemeral" | "in_channel";
    /** Replace the original interaction message. */
    replaceOriginal?: boolean;
    /** Delete the original interaction message. */
    deleteOriginal?: boolean;
    /** Whether Slack unfurls links. */
    unfurlLinks?: boolean;
    /** Whether Slack unfurls media. */
    unfurlMedia?: boolean;
    /** Optional message metadata. */
    metadata?: JsonObject;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}
