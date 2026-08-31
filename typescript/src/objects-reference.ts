/**
 * Fluent builders for reusable composition objects nested inside blocks and
 * elements. This includes text, selectable options, confirmation dialogs, files,
 * workflow metadata, rich text, table cells, and chart data.
 *
 * See: <https://docs.slack.dev/reference/block-kit/composition-objects>.
 *
 * @module objects
 */
export * from "./fluent/objects.js";
export type {
  AxisConfigInput,
  ChartSegmentInput,
  ConfirmationInput,
  DataPointInput,
  DataSeriesInput,
  MarkdownOptions,
  OptionGroupInput,
  OptionInput,
  PlainTextOptions,
  SlackIconName,
  TextLike,
  TextObject,
} from "./legacy/objects.js";
export type { RichTextStyle } from "./legacy/rich-text.js";
