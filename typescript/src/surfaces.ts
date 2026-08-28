import { TypeMismatchError } from "./errors.js";
import type { JsonValue } from "./types.js";

export type BlockSurface = "home" | "message" | "modal";

const BLOCK_TYPES_BY_SURFACE: Record<BlockSurface, ReadonlySet<string>> = {
  message: new Set([
    "actions",
    "card",
    "carousel",
    "container",
    "context",
    "context_actions",
    "data_table",
    "data_visualization",
    "divider",
    "file",
    "header",
    "image",
    "markdown",
    "plan",
    "rich_text",
    "section",
    "table",
    "task_card",
    "video",
  ]),
  modal: new Set([
    "actions",
    "alert",
    "card",
    "context",
    "divider",
    "header",
    "image",
    "input",
    "rich_text",
    "section",
    "video",
  ]),
  home: new Set([
    "actions",
    "card",
    "carousel",
    "container",
    "context",
    "data_table",
    "divider",
    "header",
    "image",
    "input",
    "rich_text",
    "section",
    "table",
    "video",
  ]),
};

export function validateSurfaceBlocks(
  blocks: readonly JsonValue[],
  surface: BlockSurface,
  path: string,
): void {
  const allowed = BLOCK_TYPES_BY_SURFACE[surface];
  blocks.forEach((block, index) => {
    if (block === null || Array.isArray(block) || typeof block !== "object") {
      throw new TypeMismatchError(`${path}[${index}]`, "expected a block object");
    }
    const type = block.type;
    if (typeof type !== "string" || !allowed.has(type)) {
      throw new TypeMismatchError(
        `${path}[${index}].type`,
        `block type ${String(type)} is not supported on ${surface} surfaces`,
      );
    }
  });
}
