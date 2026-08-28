/** Fluent builders for messages, modal, and App Home blocks. */
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
} from "../blocks.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/** Starts a fluent severity-labelled notice. */
export function AlertBlock(): FluentBuilder<
  FirstInput<typeof createAlertBlock>,
  Output<typeof createAlertBlock>
> {
  return createFluentBuilder(createAlertBlock);
}

/** Starts a fluent compact content card. */
export function CardBlock(): FluentBuilder<CardBlockInput, Output<typeof createCardBlock>> {
  return createFluentBuilder(createCardBlock, { collections: { actions: "flat" } });
}

/** Starts a fluent horizontally scrolling collection of cards. */
export function CarouselBlock(): FluentBuilder<
  FirstInput<typeof createCarouselBlock>,
  Output<typeof createCarouselBlock>
> {
  return createFluentBuilder(createCarouselBlock, {
    collections: { elements: "flat" },
  });
}

/** Starts a fluent titled container of related blocks. */
export function ContainerBlock(): FluentBuilder<
  FirstInput<typeof createContainerBlock>,
  Output<typeof createContainerBlock>
> {
  return createFluentBuilder(createContainerBlock, {
    collections: { childBlocks: "flat" },
  });
}

/** Starts a fluent row of contextual feedback or icon controls. */
export function ContextActionsBlock(): FluentBuilder<
  FirstInput<typeof createContextActionsBlock>,
  Output<typeof createContextActionsBlock>
> {
  return createFluentBuilder(createContextActionsBlock, {
    collections: { elements: "flat" },
  });
}

/** Starts a fluent sortable data table. Add each row with one `rows()` call. */
export function DataTableBlock(): FluentBuilder<
  FirstInput<typeof createDataTableBlock>,
  Output<typeof createDataTableBlock>
> {
  return createFluentBuilder(createDataTableBlock, { collections: { rows: "nested" } });
}

/** Starts a fluent Slack-rendered data visualization. */
export function DataVisualizationBlock(): FluentBuilder<
  FirstInput<typeof createDataVisualizationBlock>,
  Output<typeof createDataVisualizationBlock>
> {
  return createFluentBuilder(createDataVisualizationBlock);
}

/** Starts a fluent task card used inside a plan. */
export function TaskCardBlock(): FluentBuilder<
  FirstInput<typeof createTaskCardBlock>,
  Output<typeof createTaskCardBlock>
> {
  return createFluentBuilder(createTaskCardBlock, { collections: { sources: "flat" } });
}

/** Starts a fluent titled sequence of task cards. */
export function PlanBlock(): FluentBuilder<
  FirstInput<typeof createPlanBlock>,
  Output<typeof createPlanBlock>
> {
  return createFluentBuilder(createPlanBlock, { collections: { tasks: "flat" } });
}

/** Starts a fluent flexible text block with optional fields or accessory. */
export function SectionBlock(): FluentBuilder<
  SectionBlockInput,
  Output<typeof createSectionBlock>
> {
  return createFluentBuilder(createSectionBlock, { collections: { fields: "flat" } });
}

/** Starts a fluent row of interactive elements. */
export function ActionsBlock(): FluentBuilder<
  FirstInput<typeof createActionsBlock>,
  Output<typeof createActionsBlock>
> {
  return createFluentBuilder(createActionsBlock, { collections: { elements: "flat" } });
}

/** Starts a fluent compact context block of text and images. */
export function ContextBlock(): FluentBuilder<
  FirstInput<typeof createContextBlock>,
  Output<typeof createContextBlock>
> {
  return createFluentBuilder(createContextBlock, { collections: { elements: "flat" } });
}

/** Starts a fluent visual divider. */
export function DividerBlock(): FluentBuilder<
  NonNullable<FirstInput<typeof createDividerBlock>>,
  Output<typeof createDividerBlock>
> {
  return createFluentBuilder((input, settings) => createDividerBlock(input, settings));
}

/** Starts a fluent Slack remote-file block. */
export function FileBlock(): FluentBuilder<
  FirstInput<typeof createFileBlock>,
  Output<typeof createFileBlock>
> {
  return createFluentBuilder(createFileBlock);
}

/** Starts a fluent prominent plain-text heading. */
export function HeaderBlock(): FluentBuilder<
  FirstInput<typeof createHeaderBlock>,
  Output<typeof createHeaderBlock>
> {
  return createFluentBuilder(createHeaderBlock);
}

/** Starts a fluent image block with optional title text. */
export function ImageBlock(): FluentBuilder<
  FirstInput<typeof createImageBlock>,
  Output<typeof createImageBlock>
> {
  return createFluentBuilder(createImageBlock);
}

/** Starts a fluent labelled form control. */
export function InputBlock(): FluentBuilder<
  FirstInput<typeof createInputBlock>,
  Output<typeof createInputBlock>
> {
  return createFluentBuilder(createInputBlock);
}

/** Starts a fluent GitHub-flavored Markdown block. */
export function MarkdownBlock(): FluentBuilder<
  FirstInput<typeof createMarkdownBlock>,
  Output<typeof createMarkdownBlock>
> {
  return createFluentBuilder(createMarkdownBlock);
}

/** Starts a fluent rich-text block. */
export function RichTextBlock(): FluentBuilder<
  FirstInput<typeof createRichTextBlock>,
  Output<typeof createRichTextBlock>
> {
  return createFluentBuilder(createRichTextBlock, { collections: { elements: "flat" } });
}

/** Starts a fluent table. Add each row with one `rows()` call. */
export function TableBlock(): FluentBuilder<
  FirstInput<typeof createTableBlock>,
  Output<typeof createTableBlock>
> {
  return createFluentBuilder(createTableBlock, {
    collections: { rows: "nested", columnSettings: "flat" },
  });
}

/** Starts a fluent embedded video block. */
export function VideoBlock(): FluentBuilder<
  FirstInput<typeof createVideoBlock>,
  Output<typeof createVideoBlock>
> {
  return createFluentBuilder(createVideoBlock);
}
