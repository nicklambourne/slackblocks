// Checks for type-less composition objects, shared by their factories and by
// the raw-JSON walker, which reaches them through their typed parents.
import limits from "../../spec/limits.json" with { type: "json" };

import { LengthError, TypeMismatchError } from "./errors.js";
import type { JsonObject, JsonValue } from "./types.js";

const CONVERSATION_FILTER_INCLUDES = new Set(["im", "mpim", "private", "public"]);

const SLACK_FILE_ID_PATTERN = /^F[A-Z0-9]{8,}$/;

function objectAt(value: JsonValue, path: string): JsonObject {
  if (value === null || Array.isArray(value) || typeof value !== "object") {
    throw new TypeMismatchError(path, "expected an object");
  }
  return value;
}

export function validateConversationFilter(value: JsonValue, path: string): void {
  const include = objectAt(value, path).include;
  if (include === undefined) return;
  const includePath = `${path}.include`;
  if (!Array.isArray(include)) {
    throw new TypeMismatchError(includePath, "expected an array");
  }
  const minimum = limits.conversation_filter.include.min_items;
  if (include.length < minimum) {
    throw new LengthError(includePath, `${include.length} is less than minimum ${minimum}`);
  }
  include.forEach((entry, index) => {
    if (typeof entry !== "string" || !CONVERSATION_FILTER_INCLUDES.has(entry)) {
      throw new TypeMismatchError(
        `${includePath}[${index}]`,
        "expected im, mpim, private, or public",
      );
    }
  });
}

export function validateSlackFile(value: JsonValue, path: string): void {
  const id = objectAt(value, path).id;
  if (typeof id === "string" && !SLACK_FILE_ID_PATTERN.test(id)) {
    throw new TypeMismatchError(
      `${path}.id`,
      `expected a Slack file ID matching ${SLACK_FILE_ID_PATTERN.source}`,
    );
  }
}
