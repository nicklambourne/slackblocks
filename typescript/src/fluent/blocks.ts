/**
 * Fluent builders for messages, modal, and App Home blocks.
 *
 * @module blocks
 */
import {
  actionsBlock as createActionsBlock,
  alertBlock as createAlertBlock,
  cardBlock as createCardBlock,
  carouselBlock as createCarouselBlock,
  containerBlock as createContainerBlock,
  contextActionsBlock as createContextActionsBlock,
  contextBlock as createContextBlock,
  dataTableBlock as createDataTableBlock,
  dataVisualizationBlock as createDataVisualizationBlock,
  dividerBlock as createDividerBlock,
  fileBlock as createFileBlock,
  headerBlock as createHeaderBlock,
  imageBlock as createImageBlock,
  inputBlock as createInputBlock,
  markdownBlock as createMarkdownBlock,
  planBlock as createPlanBlock,
  richTextBlock as createRichTextBlock,
  sectionBlock as createSectionBlock,
  tableBlock as createTableBlock,
  taskCardBlock as createTaskCardBlock,
  videoBlock as createVideoBlock,
  type CardBlockInput,
  type SectionBlockInput,
} from "../legacy/blocks.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/**
 * Creates a fluent severity-labelled alert for a modal. Supply the alert copy as
 * a string or text object and choose one of Slack's supported severity levels.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/alert-block>.
 */
export function AlertBlock(): FluentBuilder<
  FirstInput<typeof createAlertBlock>,
  Output<typeof createAlertBlock>
> {
  return createFluentBuilder(createAlertBlock);
}

/**
 * Creates a fluent compact card containing text, images, and up to three button
 * actions. A card may stand alone or appear in a {@link CarouselBlock}; at least
 * one visible content field must be set before calling `.build()`.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/card-block>.
 */
export function CardBlock(): FluentBuilder<CardBlockInput, Output<typeof createCardBlock>> {
  return createFluentBuilder(createCardBlock, { collections: { actions: "flat" } });
}

/**
 * Creates a fluent horizontally scrolling group of between one and ten cards.
 * Add each card with `elements()` using a built card, a {@link CardBlock}
 * builder, or an array containing either form.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/carousel-block>.
 */
export function CarouselBlock(): FluentBuilder<
  FirstInput<typeof createCarouselBlock>,
  Output<typeof createCarouselBlock>
> {
  return createFluentBuilder(createCarouselBlock, {
    collections: { elements: "flat" },
  });
}

/**
 * Creates a fluent titled container that groups up to ten supported child
 * blocks. Set either `title()` or `richTextTitle()` and optionally make the
 * container collapsible, choose its width, or add supporting header content.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/container-block>.
 */
export function ContainerBlock(): FluentBuilder<
  FirstInput<typeof createContainerBlock>,
  Output<typeof createContainerBlock>
> {
  return createFluentBuilder(createContainerBlock, {
    collections: { childBlocks: "flat" },
  });
}

/**
 * Creates a fluent row of up to five contextual controls for feedback or compact
 * icon actions. Its elements must be built with {@link FeedbackButtons} or
 * {@link IconButton} and the block is intended for contextual actions.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/context-actions-block>.
 */
export function ContextActionsBlock(): FluentBuilder<
  FirstInput<typeof createContextActionsBlock>,
  Output<typeof createContextActionsBlock>
> {
  return createFluentBuilder(createContextActionsBlock, {
    collections: { elements: "flat" },
  });
}

/**
 * Creates a fluent sortable data table containing raw text, raw numbers, or rich
 * text. The first row supplies the headers and cannot contain rich text; add each
 * complete row with a separate `rows()` call.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>.
 */
export function DataTableBlock(): FluentBuilder<
  FirstInput<typeof createDataTableBlock>,
  Output<typeof createDataTableBlock>
> {
  return createFluentBuilder(createDataTableBlock, { collections: { rows: "nested" } });
}

/**
 * Creates a fluent data visualization rendered natively by Slack. Set a title
 * and a pie, bar, area, or line chart built with the corresponding composition
 * object builder.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.
 */
export function DataVisualizationBlock(): FluentBuilder<
  FirstInput<typeof createDataVisualizationBlock>,
  Output<typeof createDataVisualizationBlock>
> {
  return createFluentBuilder(createDataVisualizationBlock);
}

/**
 * Creates a fluent task card containing a stable identifier, title, lifecycle
 * state, rich-text details or output, and source links. Task cards may stand
 * alone or be collected in a {@link PlanBlock}.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/task-card-block>.
 */
export function TaskCardBlock(): FluentBuilder<
  FirstInput<typeof createTaskCardBlock>,
  Output<typeof createTaskCardBlock>
> {
  return createFluentBuilder(createTaskCardBlock, { collections: { sources: "flat" } });
}

/**
 * Creates a fluent titled sequence of task cards. Add tasks with `tasks()` using
 * built task cards, {@link TaskCardBlock} builders, or arrays containing either
 * form.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/plan-block>.
 */
export function PlanBlock(): FluentBuilder<
  FirstInput<typeof createPlanBlock>,
  Output<typeof createPlanBlock>
> {
  return createFluentBuilder(createPlanBlock, { collections: { tasks: "flat" } });
}

/**
 * Creates one of Block Kit's most flexible blocks: a section can show main text,
 * arrange short fields into columns, and display an interactive or visual
 * accessory beside the content. Set at least `text()` or `fields()`.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/section-block>.
 */
export function SectionBlock(): FluentBuilder<
  SectionBlockInput,
  Output<typeof createSectionBlock>
> {
  return createFluentBuilder(createSectionBlock, { collections: { fields: "flat" } });
}

/**
 * Creates a fluent block that holds interactive controls such as buttons, select
 * menus, and date pickers. Add up to 25 supported elements with `elements()`;
 * Slack sends their action identifiers back in interaction payloads.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/actions-block>.
 */
export function ActionsBlock(): FluentBuilder<
  FirstInput<typeof createActionsBlock>,
  Output<typeof createActionsBlock>
> {
  return createFluentBuilder(createActionsBlock, { collections: { elements: "flat" } });
}

/**
 * Creates a fluent block for compact contextual information beneath or beside
 * primary content. Add up to ten text objects or image elements with
 * `elements()`.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/context-block>.
 */
export function ContextBlock(): FluentBuilder<
  FirstInput<typeof createContextBlock>,
  Output<typeof createContextBlock>
> {
  return createFluentBuilder(createContextBlock, { collections: { elements: "flat" } });
}

/**
 * Creates a visual divider between adjacent blocks, similar to an HTML `<hr>`.
 * The block has no visible content; its only optional field is a deterministic
 * block identifier.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/divider-block>.
 */
export function DividerBlock(): FluentBuilder<
  NonNullable<FirstInput<typeof createDividerBlock>>,
  Output<typeof createDividerBlock>
> {
  return createFluentBuilder((input, settings) => createDividerBlock(input, settings));
}

/**
 * Creates a block that displays a remote file already registered with Slack.
 * Supply the external identifier returned by Slack's remote-files API; local or
 * directly uploaded files cannot be embedded with this block.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/file-block>.
 */
export function FileBlock(): FluentBuilder<
  FirstInput<typeof createFileBlock>,
  Output<typeof createFileBlock>
> {
  return createFluentBuilder(createFileBlock);
}

/**
 * Creates a prominent plain-text heading rendered in a larger, bold font. Header
 * text is limited to 150 characters and Slack does not apply mrkdwn formatting.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/header-block>.
 */
export function HeaderBlock(): FluentBuilder<
  FirstInput<typeof createHeaderBlock>,
  Output<typeof createHeaderBlock>
> {
  return createFluentBuilder(createHeaderBlock);
}

/**
 * Creates a block containing one image with accessible alternative text and an
 * optional title. Use {@link ImageElement} instead when the image must sit inside
 * a section or context block.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/image-block>.
 */
export function ImageBlock(): FluentBuilder<
  FirstInput<typeof createImageBlock>,
  Output<typeof createImageBlock>
> {
  return createFluentBuilder(createImageBlock);
}

/**
 * Creates a labelled form control for collecting information in a modal or App
 * Home view. Set the required label and one supported input element, then
 * optionally add a hint, allow omission, or dispatch changes immediately.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/input-block>.
 */
export function InputBlock(): FluentBuilder<
  FirstInput<typeof createInputBlock>,
  Output<typeof createInputBlock>
> {
  return createFluentBuilder(createInputBlock);
}

/**
 * Creates a block rendered with GitHub-flavored Markdown, including tables and
 * fenced code blocks. This differs from Slack `mrkdwn` used by section text and
 * is intended for richer AI or agent-generated output.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/markdown-block>.
 */
export function MarkdownBlock(): FluentBuilder<
  FirstInput<typeof createMarkdownBlock>,
  Output<typeof createMarkdownBlock>
> {
  return createFluentBuilder(createMarkdownBlock);
}

/**
 * Creates a rich-text block from Slack's structured rich-text sections, lists,
 * code blocks, and quotes. Use it when text needs formatting or nesting that is
 * unavailable through ordinary section `mrkdwn`.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block>.
 */
export function RichTextBlock(): FluentBuilder<
  FirstInput<typeof createRichTextBlock>,
  Output<typeof createRichTextBlock>
> {
  return createFluentBuilder(createRichTextBlock, { collections: { elements: "flat" } });
}

/**
 * Creates a table block for structured rows and optional column display settings.
 * Add each complete row with a separate `rows()` call so nested cell arrays retain
 * their row boundaries.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.
 */
export function TableBlock(): FluentBuilder<
  FirstInput<typeof createTableBlock>,
  Output<typeof createTableBlock>
> {
  return createFluentBuilder(createTableBlock, {
    collections: { rows: "nested", columnSettings: "flat" },
  });
}

/**
 * Creates a block that embeds video content in a message, modal, or App Home tab.
 * Slack enforces its own provider allow-list when accepting the payload, so an
 * unsupported video URL can still produce a Slack API error after local validation.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks/video-block>.
 */
export function VideoBlock(): FluentBuilder<
  FirstInput<typeof createVideoBlock>,
  Output<typeof createVideoBlock>
> {
  return createFluentBuilder(createVideoBlock);
}
