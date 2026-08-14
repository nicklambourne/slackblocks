import type { KnownBlock } from "@slack/types";

/** JSON scalar accepted by Slack payloads. */
export type JsonPrimitive = boolean | number | string | null;

/** Recursive JSON value accepted by Slack payloads. */
export type JsonValue = JsonPrimitive | JsonObject | JsonValue[];

/** Object with JSON-compatible values and Slack-shaped string keys. */
export interface JsonObject {
  /** JSON field value by wire-format key. */
  [key: string]: JsonValue;
}

/** Per-call behavior supported by every public factory. */
export interface FactorySettings {
  /**
   * Whether to validate the constructed object immediately. Defaults to `true`.
   * Disable only when intentionally creating an intermediate partial object.
   */
  validate?: boolean;
}

/** Generic validated Block Kit object. */
export type BlockKitPayload = JsonObject;

/** Slack-shaped JSON object whose `type` field is known. */
export type SlackObject<Type extends string> = JsonObject & {
  /** Discriminator identifying the Block Kit object on Slack's wire format. */
  type: Type;
};

/** Official Slack SDK type intersected with its JSON wire representation. */
export type SlackWire<Type> = Type & JsonObject;

/** Compatibility helper for call sites that accept Slack's official block types. */
export type SlackCompatibleBlock = SlackWire<KnownBlock>;
