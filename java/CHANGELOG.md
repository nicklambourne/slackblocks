# Changelog

All notable changes to the Java package are documented here. Java versions move
in lockstep with the Python, TypeScript, and Go packages.

## [Unreleased]

### Changed

- Validation now rejects plain-text, email, URL, number, date picker, time picker, and
  rich text input placeholders over 150 characters (only the plain-text input was
  checked before), and dispatch-action configurations whose `trigger_actions_on` list
  is empty or has more than two triggers.
- A raw `dispatch_action_config` set through `wireField` without
  `trigger_actions_on` is now rejected as missing-required, matching Slack's
  `blocks.validate`.
- `VideoBlock` `alt_text` now accepts up to 2000 characters, including an empty
  string, matching Slack's validator (previously 1 to 200 characters).
- `DataTableBlock` now rejects `column_settings` supplied through `wireField` or
  raw JSON with `INVALID_USAGE`; Slack supports `column_settings` only on the
  plain `table` block.
- Validation follows Slack's `blocks.validate` more closely (spec 1.1.0):
  - New limits: rich text list `indent` 0-8, `offset` 0 or more, and list, quote,
    and preformatted `border` 0-1; image block `title` 2000 characters; image
    element `image_url` 3000 and `alt_text` 2000; video `thumbnail_url` and
    `video_url` 3000, `title_url` and `provider_icon_url` 2000; view
    `external_id` 255; plain-text input `min_length` 0-3000 and `max_length` 1
    or more; rich text input `min_lines` and `max_lines` 1-100; multi-select
    `max_selected_items` 1 or more; data table `row_header_column_index` 0 or
    more; table `column_settings` at most 20; container `child_blocks` at least
    one; plan `tasks` at most 50 with unique `task_id`s.
  - Message payloads, webhook messages, and message responses reject more than
    12,000 characters of markdown block text or 20,000 characters of data table
    cell text across the whole message.
  - Newly required: `SlackIcon` `name`, `PlanBlock` `tasks`, `TaskCardBlock`
    `status` (including plan tasks), `NumberInputElement` `isDecimalAllowed`,
    `Attachment` `blocks`, and `WorkflowButtonElement` `actionId`.
    `IconButtonElement` `icon` and `FileBlock` `source` are required too but keep
    their `trash` and `remote` defaults. Getters keep their 2.x return types.
  - A task card placed as a block, rather than as a plan task, cannot be
    `pending`.
  - Slack file IDs must match `^F[A-Z0-9]{8,}$`, and conversation filter
    `include` must be non-empty and contain only `im`, `mpim`, `private`, and
    `public`.
  - A table's `column_settings` no longer has to have one entry per column.
- `input` blocks are now accepted in messages, and `data_visualization` blocks
  in App Home.
- `ImageBlock` accepts a Slack-hosted image through `slackFile(SlackFile)`, read
  back with `getSlackFile()`; exactly one of `imageUrl` and `slackFile` is
  required. `getImageUrl()` keeps its `String` return type and throws
  `IllegalStateException` when the block uses a Slack file.
- `RichTextInputElement.initialValue` now takes a `RichTextBlock` (and
  `getInitialValue()` returns one), since Slack rejects a bare rich text
  element there. This is a source-incompatible change for callers that passed a
  `RichTextText`.

## [2.4.0] — 2026-09-17

This coordinated release introduces the C# package, published to NuGet as
`Slackblocks`, and moves all five packages onto one shared release number. The
Java public API and wire format are unchanged.

### Changed

- Update the Gson runtime dependency to 2.14.0 and JSpecify to 1.0.1. Both
  still target Java 8 bytecode, so the Java 17 minimum is unchanged.
- Maven Central releases publish automatically once Central validates the
  bundle.
- Coordinated release validation now requires the Python, TypeScript, Java, and
  C# package manifests and all five language changelogs to agree before any
  tags are created.
- The C# package is generated from the same `java/generator/model.json` as the
  Java builders, so field documentation and limits stay consistent across both.

## [2.3.0] — 2026-09-16

The first public Java release of slackblocks, versioned in step with the Python,
TypeScript, and Go packages. The package supports Java 17 and newer.

### Added

- Immutable values with concrete fluent builders for Slack Block Kit
  composition objects, elements, blocks, messages, attachments, responses,
  modals, and App Home payloads.
- Typed builder methods: text fields accept a string or a `PlainText`/`Text`
  object, nested fields accept their Slack type, and closed value sets use enums
  such as `ButtonStyle`, `AlertLevel`, `ContainerWidth`, and `TaskStatus`.
- Rich text style methods (`bold`, `italic`, `strike`, `code`, and mention
  highlighting) and a reusable `RichTextStyle`.
- Validation at `build()` for required fields, string and collection limits,
  mutually exclusive fields, supported block surfaces, and modal submission
  rules. `ValidationException` reports a stable category and a path rooted at
  the Java type being built.
- Direct Slack Java SDK interoperability: blocks implement `LayoutBlock`,
  elements extend `BlockElement`, and text objects extend `TextObject`.
- Higher-level `Accordion` and `Paginator` components that expand into ordinary
  Slack blocks.
- A `wireField` escape hatch for Slack fields that do not have a named method
  yet, restricted to JSON-compatible values.
- Shared valid and invalid conformance coverage, built through the named
  builder methods, against the same JSON fixtures as the Python, TypeScript,
  and Go implementations.
- Language-specific guides and a generated API reference that documents limits,
  required fields, and exceptions; every Java documentation snippet is compiled
  and checked in CI.
- Java 17, 21, 25, and latest-JDK CI with google-java-format, Error Prone, and
  NullAway, reproducible source generation, signed Maven Central publishing,
  and coordinated release automation.
