# Changelog

All notable changes to the Java package are documented here. Java versions move
in lockstep with the Python, TypeScript, and Go packages.

## [2.3.0] — Unreleased

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
