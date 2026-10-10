# Changelog

## [2.7.0] — Unreleased

- Add the PHP implementation to the coordinated release and documentation.

## [Unreleased]

## [2.6.0] — 2026-10-09

- Introduce the native Rust 2024 implementation of shared Block Kit specification
  1.2.0, with a minimum supported Rust version of 1.85.
- Provide consuming builders, owned values, borrowed getters, typed role enums,
  and structured validation errors with categories and field paths.
- Support checked Serde serialization and deserialization, lossless supported
  JSON numbers, extension fields, and validation of complete message/view payloads.
- Keep application metadata outside message-wide Block Kit text totals.
- Include helpers for pagination, accordion sections, workflow triggers,
  attachment colors and Block Kit Builder preview URLs.
- Provide executable guides, a native API reference, and runnable Slack Morphism
  and reqwest sending examples. HTTP clients and async runtimes remain owned by
  the application.
- Verify shared conformance, public API coverage, invalid compile-time usage,
  property tests and standalone consumers on minimum and stable Rust.
- Add crates.io publishing with explicit API-token or trusted authentication,
  verified artifacts and coordinated release guards.
