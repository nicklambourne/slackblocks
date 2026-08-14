import { createObject } from "./internal.js";
import type { FactorySettings, JsonObject } from "./types.js";

export function attachment(
  input: { blocks: JsonObject[]; color?: string; fallback?: string },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}

export interface MessageInput {
  channel: string;
  blocks?: JsonObject[];
  attachments?: JsonObject[];
  text?: string;
  mrkdwn?: boolean;
  unfurlLinks?: boolean;
  unfurlMedia?: boolean;
  metadata?: JsonObject;
}

export function message(
  input: MessageInput,
  settings: FactorySettings = {},
): JsonObject {
  return createObject({ mrkdwn: true, text: "", ...input }, settings);
}

export function messageResponse(
  input: {
    blocks?: JsonObject[];
    attachments?: JsonObject[];
    text?: string;
    mrkdwn?: boolean;
    replaceOriginal?: boolean;
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

export function webhookMessage(
  input: {
    blocks?: JsonObject[];
    attachments?: JsonObject[];
    text?: string;
    responseType?: "ephemeral" | "in_channel";
    replaceOriginal?: boolean;
    deleteOriginal?: boolean;
    unfurlLinks?: boolean;
    unfurlMedia?: boolean;
    metadata?: JsonObject;
  },
  settings: FactorySettings = {},
): JsonObject {
  return createObject(input, settings);
}
