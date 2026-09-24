# Changelog

All notable changes to the Go module are documented here. Go releases use the
same version number as the Python, TypeScript, and Java packages.

## [Unreleased]

### Changed

- Video block `alt_text` now accepts up to 2000 characters, including an empty
  string, matching Slack's validator (previously 1 to 200 characters).
- `Validate` now rejects `column_settings` on a `data_table` block with
  `InvalidUsage`; Slack supports `column_settings` only on the plain `table`
  block.

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
