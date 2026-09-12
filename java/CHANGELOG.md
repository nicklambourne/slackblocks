# Changelog

All notable changes to the Java package are documented here. Java versions move
in lockstep with the Python, TypeScript, and Go packages.

## [2.3.0] — Unreleased

The first public Java release of slackblocks, versioned in step with the Python,
TypeScript, and Go packages. The package supports Java 17 and newer.

### Added

- Immutable, fluent concrete builders for Slack Block Kit composition objects,
  elements, blocks, messages, attachments, responses, modals, and App Home
  payloads.
- Typed, path-aware validation for required fields, string and collection
  limits, supported block surfaces, and modal submission rules.
- Direct Slack Java SDK interoperability: top-level block values implement
  `LayoutBlock` and can be supplied directly to the official Slack client.
- Shared valid and invalid conformance coverage against the same JSON fixtures
  used by the Python, TypeScript, and Go implementations.
- Higher-level `Accordion` and `Paginator` components that expand into ordinary
  Slack blocks.
- Complete language-specific guides and generated API reference documentation.
- Java 17 and latest-stable CI coverage, reproducible source generation, signed
  Maven Central publishing, and coordinated release automation.
