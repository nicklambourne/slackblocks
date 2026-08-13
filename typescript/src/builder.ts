import type { JsonObject } from "./types.js";

const BUILDER_URL = "https://app.slack.com/block-kit-builder/";

export function blockKitBuilderUrl(
  payload: JsonObject | JsonObject[],
  teamId?: string,
): string {
  const body = Array.isArray(payload) ? { blocks: payload } : payload;
  const prefix = teamId === undefined ? BUILDER_URL : `${BUILDER_URL}${teamId}`;
  return `${prefix}#${encodeURIComponent(JSON.stringify(body))}`;
}
