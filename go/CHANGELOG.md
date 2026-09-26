# Changelog

All notable changes to the Go module are documented here. Go releases use the
same version number as the Python, TypeScript, and Java packages.

## [Unreleased]

### Changed

- Validation now rejects email, URL, number, date picker, time picker, and rich text
  input placeholders over 150 characters, and dispatch-action configurations whose
  `trigger_actions_on` list is empty or has more than two triggers. Plain-text input
  placeholders were already limited to 150 characters.
- A dispatch-action configuration without `trigger_actions_on` is now rejected with
  `MissingRequired`, matching Slack's `blocks.validate`.
- Video block `alt_text` now accepts up to 2000 characters, including an empty
  string, matching Slack's validator (previously 1 to 200 characters).
- `Validate` now rejects `column_settings` on a `data_table` block with
  `InvalidUsage`; Slack supports `column_settings` only on the plain `table`
  block.
- Validation now follows Slack's `blocks.validate` more closely. Newly rejected:
  - Rich text list `indent` outside 0-8, negative `offset`, and list, quote, and
    preformatted `border` outside 0-1; image block `title` over 2000 characters;
    video `thumbnail_url` or `video_url` over 3000 and `title_url` or
    `provider_icon_url` over 2000 characters; view `external_id` over 255
    characters; plain-text input `min_length` outside 0-3000 or `max_length`
    below 1; rich text input `min_lines` or `max_lines` outside 1-100;
    multi-select `max_selected_items` below 1; negative data table
    `row_header_column_index`; more than 20 table `column_settings`; containers
    with no child blocks; and plans with more than 50 tasks or duplicate
    `task_id`s.
  - Conversation filters with an empty `include` list or a type other than
    `im`, `mpim`, `private`, or `public`, including filters on raw
    conversation selects passed to `Validate`.
  - Slack file IDs that do not match `^F[A-Z0-9]{8,}$`, in `NewSlackFile` and in
    image `slack_file` objects passed to `Validate`.
  - Messages, message responses, and webhook messages whose markdown blocks
    total more than 12,000 characters or whose data table cells total more
    than 20,000 characters.
  - A standalone task card with `pending` status (`TypeMismatch`). Plan tasks
    can still be `pending`.
  - Missing required fields (`MissingRequired`), checked at `Build`: a Slack
    icon's `Name`, a plan's `Tasks`, a task card's `Status` (also for plan
    tasks), a number input's `IsDecimalAllowed`, an attachment's `Blocks`, and
    a workflow button's `ActionID`. Icon buttons and file blocks still default
    `icon` and `source`.
- Newly accepted: `input` blocks in messages, `data_visualization` blocks in
  App Home, and tables with fewer or more `column_settings` than columns (up
  to 20).
- `ActionID` is now optional on every interactive element except rich text
  inputs and workflow buttons; when it is not set, no `action_id` is sent. The
  255-character limit still applies when it is set.
- Also newly accepted: task card source URLs of any length (including empty),
  empty markdown block text, modals and App Home tabs with no blocks, and
  `table` rows with different numbers of cells (`data_table` rows must still
  match).
- `ImageBlockBuilder` gains `SlackFile`; an image block needs exactly one of
  `ImageURL` or `SlackFile`.
- `RichTextInputBuilder.InitialValue` now takes a `*RichTextBlockBuilder`
  instead of a `*RichTextBuilder`, since Slack requires a `rich_text` block.

## [2.4.0] — 2026-09-17

This coordinated release introduces the C# package, published to NuGet as
`Slackblocks`, and moves all five packages onto one shared release number. The
Go public API and wire format are unchanged.

### Changed

- Coordinated release validation now requires the Python, TypeScript, Java, and
  C# package manifests and all five language changelogs to agree before any
  tags are created.
- Project documentation now includes the C# implementation alongside the
  existing Python, TypeScript, Go, and Java variants.

## [2.3.0] — 2026-09-16

This coordinated release introduces the Java package and replaces the `any`
parameters on Go builder methods with typed parameters. The wire format is
unchanged.

### Changed (breaking for Go callers)

- Builder methods take typed parameters instead of `any`. Nested fields accept
  role interfaces (`Block`, `Element`, `InputElement`, `ContextElement`,
  `ContextActionsElement`, `TableCell`, `DataTableCell`, `Chart`,
  `RichTextBlockElement`, `RichTextSectionElement`, and `TextObject`) or a
  concrete builder such as `*ConfirmationBuilder`. Only slackblocks builders
  implement the interfaces, so a value of the wrong kind no longer compiles.
- Text fields take a `string`. Pass a text object with the new companion
  method, such as `TitleObject` or `FieldObjects`.
- Enumerated fields take named string types with constants, such as
  `ButtonStyle` and `ButtonStylePrimary`. Untyped string constants still
  compile.
- Rich text styles take `RichTextStyle`, table rows take typed slices such as
  `[]TableCell`, numeric chart and cell values take `float64`, and message
  metadata takes `Object`.
- `NewAccordion().Sections` takes `*AccordionSectionBuilder` values and
  `NewPaginator().Blocks` takes `Block` values.
- Raw `Object` values and maps are no longer accepted by typed fields; use
  `Set(field, value)` for wire-format values.
- Builder documentation now states each field's Slack limit, whether it is
  required, and links to Slack's reference, and a test runs every Go snippet
  in the Using Blocks guide.

### Changed

- Coordinated release validation now requires the Python, TypeScript, and Java
  package manifests and all four language changelogs to agree before any tags
  are created.
- Project documentation now includes the Java implementation alongside the
  existing Python, TypeScript, and Go variants.

## [2.2.0] — 2026-09-02

The first public Go release of slackblocks, versioned in step with the Python
and TypeScript packages. The module supports Go 1.22 and newer.

### Added

- Fluent concrete builders for Slack Block Kit composition objects, elements,
  blocks, messages, attachments, responses, modals, and App Home payloads.
- Typed validation errors and complete payload validation for required fields,
  length and collection limits, supported block surfaces, and modal submission
  rules.
- Direct `slack-go/slack` interoperability: top-level block builders implement
  `slack.Block` and can be passed directly to `slack.MsgOptionBlocks`.
- Shared conformance coverage against the same valid and invalid JSON fixtures
  used by the Python and TypeScript implementations.
- Higher-level `Accordion` and `Paginator` components that expand into ordinary
  Slack blocks.
- Dedicated documentation, examples, generated-builder verification, and a CI
  matrix covering Go 1.22 and the latest stable Go release.
