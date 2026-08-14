import type { JsonObject } from "./types.js";

const BUILDER_URL = "https://app.slack.com/block-kit-builder/";

/**
 * Builds a Block Kit Builder URL containing a serialized payload.
 *
 * @param payload - A complete payload or a list of blocks.
 * @param teamId - Optional workspace ID used in the Builder URL.
 * @returns A URL that opens the payload in Slack's Block Kit Builder.
 */
export function blockKitBuilderUrl(
  payload: JsonObject | JsonObject[],
  teamId?: string,
): string {
  const body = Array.isArray(payload) ? { blocks: payload } : payload;
  const prefix = teamId === undefined ? BUILDER_URL : `${BUILDER_URL}${teamId}`;
  return `${prefix}#${encodeURIComponent(JSON.stringify(body))}`;
}
