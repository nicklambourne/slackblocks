# Changelog

All notable changes to the C# package are documented here. C# versions move in
lockstep with the Python, TypeScript, Go, and Java packages.

## [3.1.0] — 2026-10-28

- Coordinate this release with the new Haskell implementation and ten-language documentation. Existing runtime support remains unchanged.

## [3.0.0] — 2026-10-24

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- Reject header dividers at the accordion helper boundary with a clear field path.
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

- Keep application metadata outside message-wide Block Kit text totals.

- Release alongside the new native Rust implementation; supported language versions are unchanged.

## [2.5.0] — 2026-09-28

### Changed

- Validation now rejects plain-text, email, URL, number, date picker, time picker, and
  rich text input placeholders over 150 characters (only the plain-text input was
  checked before), and dispatch-action configurations whose `trigger_actions_on` list
  is empty or has more than two triggers.
- A raw `dispatch_action_config` passed through `additionalFields` without
  `trigger_actions_on` is now rejected as missing-required, matching Slack's
  `blocks.validate`.
- `VideoBlock` `altText` now accepts up to 2000 characters, including an empty
  string, matching Slack's validator (previously 1 to 200 characters).
- `DataTableBlock` now rejects `column_settings` supplied through
  `additionalFields` or raw JSON with `ErrorCategory.InvalidUsage`; Slack
  supports `column_settings` only on the plain `table` block.
- Validation follows Slack's `blocks.validate` more closely (spec 1.2.0):
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
    `Attachment` `blocks`, and `WorkflowButtonElement` `actionId`. The
    constructor parameters keep their 2.x optional shape, so existing code
    compiles and a missing value is rejected with `ErrorCategory.MissingRequired`.
    `IconButtonElement` `icon` and `FileBlock` `source` are required too but keep
    their `Trash` and `remote` defaults.
  - A task card placed as a block, rather than as a plan task, cannot be
    `Pending`.
  - Slack file IDs must match `^F[A-Z0-9]{8,}$`, and conversation filter
    `include` must be non-empty and contain only `im`, `mpim`, `private`, and
    `public`.
  - A table's `column_settings` no longer has to have one entry per column.
- `input` blocks are now accepted in messages, and `data_visualization` blocks
  in App Home.
- `ImageBlock` accepts a Slack-hosted image through a new trailing `slackFile`
  parameter and `SlackFile` property; exactly one of `imageUrl` and `slackFile`
  is required. `imageUrl` keeps its position but is now nullable, as is the
  `ImageUrl` property.
- `RichTextInputElement` `initialValue` (and the `InitialValue` property) is now
  a `RichTextBlock`, since Slack rejects a bare rich text element there. Callers
  that passed a `RichTextText` need to wrap it in a `RichTextBlock`.
- `actionId` is now optional on buttons, checkboxes, date, datetime and time
  pickers, email, URL, number, plain-text and file inputs, overflow menus, radio
  buttons, and every select and multi-select, and nothing is sent when it is
  `null`; rich text inputs and workflow buttons still require it. The
  parameter keeps its position and becomes `string?` (defaulting to `null`
  except on `CheckboxesElement`, `OverflowElement`, and `RadioButtonsElement`,
  where the required `options` follows it), and `ActionId` is now `string?`.
- Validation now accepts an empty markdown block, a modal or home tab with no
  blocks, a `url` source URL of any length, and `table` rows with different
  numbers of cells (`DataTableBlock` rows must still match).

## [2.4.0] — 2026-09-17

The first public .NET release of slackblocks, published to NuGet as
`Slackblocks` and versioned in step with the other packages. The package
supports .NET 8 and newer.

### Added

- Immutable values for Slack Block Kit composition objects, elements, blocks,
  rich text, messages, attachments, responses, modals, and App Home payloads,
  constructed with named arguments and validated in the constructor.
- Typed parameters and read-only properties: strings convert to the text
  object each field requires, nested fields accept their Slack type or role
  interface such as `IBlock` and `IElement`, and closed value sets use enums
  such as `ButtonStyle`, `AlertLevel`, `ContainerWidth`, and `TaskCardStatus`.
- `ValidationException` with a stable `ErrorCategory` and a field path rooted
  at the type being constructed, for required fields, length and range limits,
  mutually exclusive fields, supported block surfaces, and modal submission
  rules.
- Compact JSON through `ToJson()`, mutable copies through `ToJsonNode()`, and
  `System.Text.Json` serialization of values and role interfaces.
- Higher-level `Paginator`, `Accordion`, and `AccordionSection` components that
  return ordinary Slack blocks.
- An `additionalFields` constructor argument for Slack fields that have no
  named parameter yet, restricted to JSON-compatible values and validated with
  the rest of the object.
- Shared valid and invalid conformance coverage, built through the named
  constructor parameters, against the same JSON fixtures as the Python,
  TypeScript, Go, and Java implementations.
