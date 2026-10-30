import limits from "../../spec/limits.json" with { type: "json" };
import { LengthError, TypeMismatchError } from "./errors.js";
import { blockCost, compositeDescendants } from "./layout.js";
import { requireArray } from "./internal.js";
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
    "input",
    "layout",
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
    "layout",
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
    "data_visualization",
    "divider",
    "header",
    "image",
    "input",
    "layout",
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
  requireArray(blocks, path);
  const allowed = BLOCK_TYPES_BY_SURFACE[surface];
  const maximum = surface === "message" ? limits.message.blocks.max_items : limits.view.blocks.max_items;
  const cost = blocks.reduce<number>((n,b) => n + (b !== null && typeof b === "object" && !Array.isArray(b) ? blockCost(b) : 1),0);
  if (cost > maximum) throw new LengthError(path,"layout content exceeds the surface block budget");
  compositeDescendants(blocks).forEach((block, index) => {
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
