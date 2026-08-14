# Changelog

All notable changes to the TypeScript package are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
