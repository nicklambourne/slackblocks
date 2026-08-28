/**
 * Payload factories for Web API messages, webhooks, and interaction responses.
 *
 * @module messages
 */
import limits from "../../../spec/limits.json" with { type: "json" };

import { LengthError, TypeMismatchError } from "../errors.js";
import { createObject, dropEmpty } from "../internal.js";
import { validateSurfaceBlocks } from "../surfaces.js";
import type { FactorySettings, JsonObject } from "../types.js";

/**
 * Preset side-border colors for legacy attachments.
 *
 * The values mirror the Python `Color` enum: three Slack-recognized aliases
 * (`good`, `warning`, `danger`) plus common hex colors.
 */
export const Color = {
  GOOD: "good",
  WARNING: "warning",
  DANGER: "danger",
  RED: "#ff0000",
  BLUE: "#0000ff",
  YELLOW: "#ffff00",
  GREEN: "#00ff00",
  ORANGE: "#ff8800",
  PURPLE: "#8800ff",
  BLACK: "#000000",
} as const;

const COLOR_ALIASES = new Set<string>([Color.GOOD, Color.WARNING, Color.DANGER]);

function validateMessageCollections(
  input: { blocks?: JsonObject[]; attachments?: JsonObject[] },
  path: string,
): void {
  if (input.blocks !== undefined) {
    if (!Array.isArray(input.blocks)) {
      throw new TypeMismatchError(`${path}.blocks`, "expected an array");
    }
    if (input.blocks.length > limits.message.blocks.max_items) {
      throw new LengthError(
        `${path}.blocks`,
        `exceeds maximum ${limits.message.blocks.max_items}`,
      );
    }
    validateSurfaceBlocks(input.blocks, "message", `${path}.blocks`);
  }
  if (
    input.attachments !== undefined &&
    !Array.isArray(input.attachments)
  ) {
    throw new TypeMismatchError(`${path}.attachments`, "expected an array");
  }
  if (
    input.attachments !== undefined &&
    input.attachments.length > limits.message.attachments.max_items
  ) {
    throw new LengthError(
      `${path}.attachments`,
      `exceeds maximum ${limits.message.attachments.max_items}`,
    );
  }
}

function normalizeColor(color: string): string {
  if (COLOR_ALIASES.has(color)) {
    return color;
  }
  if (/^#[0-9a-fA-F]{6}$/.test(color)) {
    return color;
  }
  if (/^[0-9a-fA-F]{6}$/.test(color)) {
    return `#${color}`;
  }
  throw new TypeMismatchError(
    "attachment.color",
    "expected a hex color such as #ffffff",
  );
}

/**
 * Creates lower-priority supporting content using Slack's legacy secondary-attachment format.
 *
 * Attachments can add context beneath a message, while `fallback` supplies a
 * plain-text summary for notifications and clients that cannot display Block Kit.
 *
 * @param input - Attachment blocks plus optional color and fallback text.
 * @param settings - Per-call validation settings.
 * @returns A Slack attachment object.
 * @throws InvalidUsageError when the color is not a valid hex code or a nested
 *   block violates a supported Block Kit constraint.
 * @see https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments
 */
export function attachment(
  input: {
    /** Blocks displayed inside the attachment. */
    blocks: JsonObject[];
    /** Optional side-border color: a `Color` value or a six-digit hex code. */
    color?: string;
    /** Plain-text fallback for notifications and clients without Block Kit support. */
    fallback?: string;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(
    {
      ...input,
      blocks: dropEmpty(input.blocks),
      color: input.color === undefined ? undefined : normalizeColor(input.color),
    },
    settings,
  );
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
  if (typeof input.channel !== "string") {
    throw new TypeMismatchError("message.channel", "expected a string");
  }
  if (input.channel.length < limits.message.channel.min_length) {
    throw new LengthError(
      "message.channel",
      `expected at least ${limits.message.channel.min_length} character`,
    );
  }
  validateMessageCollections(input, "message");
  return createObject(
    {
      mrkdwn: true,
      text: "",
      ...input,
      blocks: dropEmpty(input.blocks),
      attachments: dropEmpty(input.attachments),
    },
    settings,
  );
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
  validateMessageCollections(input, "messageResponse");
  return createObject(
    {
      mrkdwn: true,
      text: "",
      replaceOriginal: false,
      responseType: "in_channel",
      ...input,
      blocks: dropEmpty(input.blocks),
      attachments: dropEmpty(input.attachments),
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
  validateMessageCollections(input, "webhookMessage");
  return createObject(input, settings);
}
