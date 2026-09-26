# Changelog

All notable changes to the TypeScript package are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Validation now rejects email, URL, number, date picker, time picker, and rich text
  input placeholders over 150 characters, and dispatch-action configurations whose
  `trigger_actions_on` list is empty or has more than two triggers.
- A dispatch-action configuration without `trigger_actions_on` is now rejected with
  `MissingRequiredError`, matching Slack's `blocks.validate`.
- Video block `alt_text` now accepts up to 2000 characters, including an empty
  string, matching Slack's validator (previously 1 to 200 characters).
- Raw JSON validation now rejects `column_settings` on a `data_table` block with
  `InvalidUsageError`; Slack supports `column_settings` only on the plain `table`
  block.
- Validation now rejects more inputs that Slack's `blocks.validate` rejects: rich
  text list `indent` outside 0-8, negative `offset`, and list, quote, and code
  block `border` outside 0-1; image block titles over 2000 characters; video
  `thumbnail_url` and `video_url` over 3000 characters and `title_url` and
  `provider_icon_url` over 2000; view `external_id` over 255 characters; plain
  text input `min_length` outside 0-3000 and `max_length` below 1; rich text input
  `min_lines` and `max_lines` outside 1-100; multi-select `max_selected_items`
  below 1; data table `row_header_column_index` below 0; more than 20 table
  `column_settings`; containers with no child blocks; plans with more than 50
  tasks or repeated task IDs; and conversation filters whose `include` list is
  empty or names a type other than `im`, `mpim`, `private`, or `public`.
- Slack file IDs must now match `^F[A-Z0-9]{8,}$` (for example `F0123ABC456`).
- Message payloads, attachments, and raw payloads without a `type` are now limited
  to 12,000 characters of markdown block text and 20,000 characters of data table
  cell text across the whole payload.
- These fields are now required, and their input types say so: `status` on
  `taskCardBlock` (and on plan tasks), `tasks` on `planBlock`, `isDecimalAllowed`
  on `numberInput`, and `actionId` on `workflowButton`. `slackIcon` without a name
  and an attachment without blocks now throw `MissingRequiredError`; an
  attachment with an empty `blocks` list, previously emitted as `{}`, is rejected
  too. Raw JSON also requires `icon_button.icon` and `file.source`, which the
  factories already default.
- A standalone task card can no longer be `pending`; only plan tasks can. Add
  `TaskCardBlock()` builders to `PlanBlock().tasks()`, which validates them as
  plan tasks, or build legacy plan tasks with `{ validate: false }` and let
  `planBlock` validate them.
- Image blocks accept a Slack-hosted file: `ImageBlock().slackFile()` and
  `imageBlock({ slackFile })` take exactly one of `imageUrl` or `slackFile`.
- The rich text input's `initialValue` is now a `rich_text` block (typed
  `SlackObject<"rich_text">`, as returned by `richTextBlock` and `RichTextBlock()`)
  instead of a rich text element, which Slack rejects.
- Tables accept any number of `column_settings` up to 20, rather than exactly one
  per column.
- `input` blocks are now allowed in messages and `data_visualization` blocks in App
  Home tabs.

## [2.4.0] — 2026-09-17

This coordinated release introduces the C# package, published to NuGet as
`Slackblocks`, and moves all five packages onto one shared release number. The
TypeScript and JavaScript public API and wire format are unchanged.

### Changed

- Coordinated release validation now requires the Python, TypeScript, Java, and
  C# package manifests and all five language changelogs to agree before any
  tags are created.
- Project documentation now includes the C# implementation alongside the
  existing Python, TypeScript, Go, and Java variants.

## [2.3.0] — 2026-09-16

This coordinated release introduces the Java package while moving Python,
TypeScript, Go, and Java onto one shared release number. The TypeScript and
JavaScript public API and wire format are unchanged.

### Changed

- Coordinated release validation now requires the Python, TypeScript, and Java
  package manifests and all four language changelogs to agree before any tags
  are created.
- Project documentation now includes the Java implementation alongside the
  existing Python, TypeScript, and Go variants.

## [2.2.0] — 2026-08-29

### Added

- Fluent PascalCase builders for every composition object, element, block, message,
  attachment, response, modal, and App Home payload.
- Higher-level `Paginator` and Slack-native `Accordion` components that expand inside
  fluent collection setters.
- Complete payload-level validation for message block and attachment limits, required
  channels, supported block surfaces, and modal submission rules.

### Changed

- Fluent builders are now the documented and recommended TypeScript API.
- The generated API reference describes every chainable setter and its build-time
  validation behavior.
- Lowercase compatibility factories now live in the internal `src/legacy` area while
  retaining their v2 root-package exports.

### Deprecated

- Lowercase object-input factories remain available for v2 compatibility but are no
  longer documented. They are scheduled for removal in v3.0; see the project roadmap.

## [2.1.1] — 2026-08-16

A documentation-only patch; no library behaviour changes.

### Changed

- The npm page README now matches the PyPI treatment: project logo, badges,
  a why-section, the CI-notification quickstart with one-line sending via
  `@slack/web-api`, its rendered message, and full documentation links.

## [2.1.0] — 2026-08-14

The first public TypeScript/JavaScript release of
`@nicklambourne/slackblocks`, versioned in step with the Python package.

### Added

- Typed, eagerly validated factories for Block Kit blocks, elements,
  composition objects, rich text, messages, and views.
- Support for the current block families: alert, card, carousel, container,
  context actions, data table, data visualization, task card, and plan.
- Cross-language conformance tests covering the shared valid and invalid JSON
  corpus with no skipped capabilities.
- ESM package exports and generated TypeScript declarations, validated by
  Publint and Are the Types Wrong.

### Changed

- Factory names use representative `*Block` suffixes, such as
  `sectionBlock`, `videoBlock`, and `dataTableBlock`.
- Package releases use the same version number as the Python implementation.
