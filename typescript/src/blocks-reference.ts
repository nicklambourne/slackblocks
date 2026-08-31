/**
 * Fluent builders for the Block Kit containers that make up messages, modals,
 * and App Home tabs. Each PascalCase function returns a chainable builder; set
 * its content and options, then call `.build()` for validated Slack wire data.
 *
 * See: <https://docs.slack.dev/reference/block-kit/blocks>.
 *
 * @module blocks
 */
export * from "./fluent/blocks.js";
export type {
  AlertLevel,
  CardBlockInput,
  ContainerWidth,
  SectionBlockInput,
  TaskStatus,
} from "./legacy/blocks.js";
