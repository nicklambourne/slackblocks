import type { KnownBlock } from "@slack/types";

export type JsonPrimitive = boolean | number | string | null;
export type JsonValue = JsonPrimitive | JsonObject | JsonValue[];

export interface JsonObject {
  [key: string]: JsonValue;
}

export interface FactorySettings {
  /** Disable eager validation for this construction only. */
  validate?: boolean;
}

export type BlockKitPayload = JsonObject;
export type SlackObject<Type extends string> = JsonObject & { type: Type };
export type SlackWire<Type> = Type & JsonObject;

/** Compatibility helper for call sites that accept Slack's official block types. */
export type SlackCompatibleBlock = SlackWire<KnownBlock>;
