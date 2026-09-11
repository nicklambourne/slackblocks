# Changelog

All notable changes to the Go module are documented here. Go releases use the
same version number as the Python and TypeScript packages.

## Unreleased

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
