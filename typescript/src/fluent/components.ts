/** Higher-level fluent components that render ordinary Block Kit blocks. */
import { actionsBlock, contextBlock } from "../blocks.js";
import { button } from "../elements.js";
import { MissingRequiredError, OutOfRangeError } from "../errors.js";
import { mrkdwn } from "../objects.js";
import type { FactorySettings, JsonObject } from "../types.js";
import {
  createFluentGroupBuilder,
  type FluentGroupBuilder,
} from "./core.js";

/** Configuration accepted by {@link Paginator}. */
export interface PaginatorInput {
  /** Blocks to paginate, in display order. */
  blocks: JsonObject[];
  /** Prefix used for the generated previous and next action identifiers. */
  actionIdPrefix: string;
  /** One-based page to render. Defaults to 1. */
  page?: number;
  /** Blocks displayed per page. Defaults to 5. */
  pageSize?: number;
  /** Label for the previous-page button. Defaults to `Previous`. */
  previousText?: string;
  /** Label for the next-page button. Defaults to `Next`. */
  nextText?: string;
  /** Whether to render `Page n of m` above the controls. Defaults to `true`. */
  showPageIndicator?: boolean;
  /** Optional identifier for the generated actions block. */
  blockId?: string;
}

function positiveInteger(path: string, value: number): void {
  if (!Number.isInteger(value) || value < 1) {
    throw new OutOfRangeError(path, "expected a positive integer");
  }
}

function renderPaginator(
  input: PaginatorInput,
  settings: FactorySettings = {},
): JsonObject[] {
  if (input.blocks.length === 0) {
    throw new MissingRequiredError("Paginator.blocks", "expected at least one block");
  }
  if (input.actionIdPrefix.length === 0) {
    throw new MissingRequiredError(
      "Paginator.actionIdPrefix",
      "expected a non-empty prefix",
    );
  }

  const page = input.page ?? 1;
  const pageSize = input.pageSize ?? 5;
  positiveInteger("Paginator.page", page);
  positiveInteger("Paginator.pageSize", pageSize);

  const pageCount = Math.ceil(input.blocks.length / pageSize);
  if (page > pageCount) {
    throw new OutOfRangeError(
      "Paginator.page",
      `expected a value between 1 and ${pageCount}`,
    );
  }

  const visible = input.blocks.slice((page - 1) * pageSize, page * pageSize);
  if (pageCount === 1) return visible;

  const controls: JsonObject[] = [];
  if (page > 1) {
    controls.push(
      button(
        {
          text: input.previousText ?? "Previous",
          actionId: `${input.actionIdPrefix}.previous`,
          value: String(page - 1),
        },
        settings,
      ),
    );
  }
  if (page < pageCount) {
    controls.push(
      button(
        {
          text: input.nextText ?? "Next",
          actionId: `${input.actionIdPrefix}.next`,
          value: String(page + 1),
        },
        settings,
      ),
    );
  }

  return [
    ...visible,
    ...(input.showPageIndicator === false
      ? []
      : [
          contextBlock(
            { elements: [mrkdwn(`Page ${page} of ${pageCount}`, {}, settings)] },
            settings,
          ),
        ]),
    actionsBlock(
      {
        elements: controls,
        ...(input.blockId === undefined ? {} : { blockId: input.blockId }),
      },
      settings,
    ),
  ];
}

/**
 * Starts a pure paginator component.
 *
 * The component renders the selected page plus ordinary context and action
 * blocks. Generated button values contain the one-based page to render next;
 * interaction handling remains in the application.
 */
export function Paginator(): FluentGroupBuilder<PaginatorInput, JsonObject> {
  return createFluentGroupBuilder(renderPaginator, {
    collections: { blocks: "flat" },
  });
}
