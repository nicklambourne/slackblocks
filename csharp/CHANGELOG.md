# Changelog

All notable changes to the C# package are documented here. C# versions move in
lockstep with the Python, TypeScript, Go, and Java packages.

## [Unreleased]

### Changed

- A raw `dispatch_action_config` passed through `additionalFields` without
  `trigger_actions_on` is now rejected as missing-required, matching Slack's
  `blocks.validate`.

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
