# Changelog

All notable changes to the TypeScript package are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] — 2026-10-24

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- Validate feedback-choice limits when standalone choices are built, using the same checks as the parent element.

- Reject header dividers at the accordion helper boundary with a clear field path.

- Report path-aware missing/type errors before reading required chart, option and view fields.

- Preserve the required channel in message return types for strict official SDK handoff.

- Expose confirmation style and emoji skin tone through typed native builders.
- SB-006: reject messages with more than two data visualization blocks after composition, including attachment charts. The rule does not impose a two-chart limit on views.

- SB-003: reject explicitly supplied `focus_on_load` on external multi-selects, including false. Omit this deprecated parameter; other selectors retain focus support.
- Reject negative external-query thresholds, empty supplied static options/groups and malformed date/time strings, matching the retained Slack structural probes. Date shape checks do not add a calendar-validity guarantee; empty image alt text remains accepted (SB-013).

- Reject a plain-text input whose supplied `min_length` exceeds `max_length` (`out-of-range`); equal, zero-minimum and omitted bounds remain supported (SB-013).

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).
- Enforce the existing 150-character placeholder limit for external, user, channel and conversation selectors, including multi-select variants; Unicode code points are counted.

## [2.8.0] — 2026-10-18

- Coordinate the first Elixir implementation release; existing language support remains unchanged.

## [2.7.0] — 2026-10-11

- Add the PHP implementation to the coordinated release and documentation.

## [2.6.0] — 2026-10-09

- Release alongside the new native Rust implementation; supported language versions are unchanged.
- Update `@slack/types` to 3.2.0.

## [2.5.0] — 2026-09-28

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
- A task card used as a block in a message, message response, webhook message,
  attachment, or view can no longer be `pending`; plan tasks still can, so
  `taskCardBlock()` and `TaskCardBlock()` accept `pending` for use in a plan.
- Image blocks accept a Slack-hosted file: `ImageBlock().slackFile()` and
  `imageBlock({ slackFile })` take exactly one of `imageUrl` or `slackFile`.
- The rich text input's `initialValue` is now a `rich_text` block (typed
  `SlackObject<"rich_text">`, as returned by `richTextBlock` and `RichTextBlock()`)
  instead of a rich text element, which Slack rejects.
- Tables accept any number of `column_settings` up to 20, rather than exactly one
  per column.
- `input` blocks are now allowed in messages and `data_visualization` blocks in App
  Home tabs.
- `actionId` is now optional on every interactive element except `richTextInput`
  and `workflowButton`, in both the factory input types and the fluent builders;
  when omitted, no `action_id` is emitted.
- Validation now accepts task-card URL sources of any length, empty markdown
  blocks, modals and App Home tabs with no blocks, and table rows with different
  numbers of cells (data tables still require equal rows).

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
