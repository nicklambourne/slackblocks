import type {
  FactorySettings,
  JsonObject,
  JsonValue,
  SlackObject,
} from "./types.js";
import { assertValid } from "./validation.js";

function snakeCase(key: string): string {
  return key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}

function copyVerbatim(value: unknown): JsonValue {
  if (value === null || typeof value === "string" || typeof value === "boolean") {
    return value;
  }
  if (typeof value === "number") {
    return value;
  }
  if (Array.isArray(value)) {
    return value.map(copyVerbatim);
  }
  if (typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .filter(([, nested]) => nested !== undefined)
        .map(([key, nested]) => [key, copyVerbatim(nested)]),
    );
  }
  throw new TypeError(`Unsupported Block Kit value: ${String(value)}`);
}

export function codePointLength(value: string): number {
  let length = 0;
  for (const _ of value) {
    length += 1;
  }
  return length;
}

export function dropEmpty<Item>(
  value: readonly Item[] | undefined,
): readonly Item[] | undefined {
  return value !== undefined && value.length === 0 ? undefined : value;
}

export function toWire(value: unknown): JsonValue {
  if (value === null || typeof value === "string" || typeof value === "boolean") {
    return value;
  }
  if (typeof value === "number") {
    return value;
  }
  if (Array.isArray(value)) {
    return value.map(toWire);
  }
  if (typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .filter(([, nested]) => nested !== undefined && nested !== null)
        .map(([key, nested]) => {
          const wireKey = snakeCase(key);
          return [
            wireKey,
            wireKey === "event_payload" ? copyVerbatim(nested) : toWire(nested),
          ];
        }),
    );
  }
  throw new TypeError(`Unsupported Block Kit value: ${String(value)}`);
}

export function createObject(
  input: Record<string, unknown>,
  settings: FactorySettings = {},
): JsonObject {
  const output = toWire(input) as JsonObject;
  if (settings.validate !== false) {
    assertValid(output);
  }
  return output;
}

export function create<Type extends string>(
  type: Type,
  input: Record<string, unknown>,
  settings: FactorySettings = {},
): SlackObject<Type> {
  return createObject({ ...input, type }, settings) as SlackObject<Type>;
}

export function textValue(value: unknown): string | undefined {
  if (typeof value === "string") {
    return value;
  }
  if (
    value !== null &&
    typeof value === "object" &&
    "text" in value &&
    typeof value.text === "string"
  ) {
    return value.text;
  }
  return undefined;
}
