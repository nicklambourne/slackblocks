/**
 * Higher-level fluent components that render ordinary Block Kit blocks.
 *
 * @module components
 */
import {
  actionsBlock,
  containerBlock,
  contextBlock,
  type ContainerWidth,
} from "../legacy/blocks.js";
import { button } from "../legacy/elements.js";
import {
  MissingRequiredError,
  OutOfRangeError,
  TypeMismatchError,
} from "../errors.js";
import { mrkdwn, type TextLike } from "../legacy/objects.js";
import type { FactorySettings, JsonObject } from "../types.js";
import {
  createFluentBuilder,
  createFluentGroupBuilder,
  type FluentBuilder,
  type FluentGroupBuilder,
} from "./core.js";

/** Configuration accepted by {@link AccordionSection}. */
export interface AccordionSectionInput {
  /** Plain-text heading shown above the collapsible content. */
  title: TextLike;
  /** Blocks revealed when the section is expanded. */
  blocks: JsonObject[];
  /** Optional supporting copy below the heading. */
  subtitle?: TextLike;
  /** Optional image displayed in the header. */
  icon?: JsonObject;
  /** Whether this section starts expanded. Defaults to `false`. */
  expanded?: boolean;
  /** Horizontal width of this section. Defaults to `standard`. */
  width?: ContainerWidth;
  /** Whether Slack draws a divider below the header. */
  hasHeaderDivider?: boolean;
  /** Optional deterministic identifier for this section. */
  blockId?: string;
}

function renderAccordionSection(
  input: AccordionSectionInput,
  settings: FactorySettings = {},
): JsonObject {
  const { blocks, expanded, ...container } = input;
  return containerBlock(
    {
      ...container,
      childBlocks: blocks,
      isCollapsible: true,
      defaultCollapsed: !(expanded ?? false),
    },
    settings,
  );
}

/** Starts one native collapsible section for an {@link Accordion}. */
export function AccordionSection(): FluentBuilder<
  AccordionSectionInput,
  JsonObject
> {
  return createFluentBuilder(renderAccordionSection, {
    collections: { blocks: "flat" },
  });
}

/** Configuration accepted by {@link Accordion}. */
export interface AccordionInput {
  /** Sections created with {@link AccordionSection}, in display order. */
  sections: JsonObject[];
}

function renderAccordion(input: AccordionInput): JsonObject[] {
  if (!Array.isArray(input.sections) || input.sections.length === 0) {
    throw new MissingRequiredError(
      "Accordion.sections",
      "expected at least one section",
    );
  }
  input.sections.forEach((section, index) => {
    if (section.type !== "container" || section.is_collapsible !== true) {
      throw new TypeMismatchError(
        `Accordion.sections[${index}]`,
        "expected an AccordionSection",
      );
    }
  });
  return input.sections;
}

/**
 * Starts an accordion made from Slack-native collapsible containers.
 *
 * Each section expands independently in Slack and requires no application-side
 * interaction handler.
 */
export function Accordion(): FluentGroupBuilder<AccordionInput, JsonObject> {
  return createFluentGroupBuilder(renderAccordion, {
    collections: { sections: "flat" },
  });
}

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
  if (!Array.isArray(input.blocks) || input.blocks.length === 0) {
    throw new MissingRequiredError("Paginator.blocks", "expected at least one block");
  }
  if (typeof input.actionIdPrefix !== "string" || input.actionIdPrefix.length === 0) {
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
