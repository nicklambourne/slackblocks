# Changelog

All notable changes to the TypeScript package are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
