# Changelog

## [Unreleased]

## [2.6.0] — 2026-10-09

- Keep application metadata outside message-wide Block Kit text totals.
- Reject lossy JSON number literals, validate limit references during generation,
  simplify typed validation/plan serialization, and accept borrowed string arguments.

- Introduce the native Rust implementation in coordinated release 2.6.0.
- Add crates.io publishing with explicit API-token or trusted authentication,
  verified artifacts and coordinated release guards.

- Native convenience APIs for pagination, accordion sections, workflow triggers,
  attachment colors and Block Kit Builder preview URLs.
- Full shared conformance, independent public API auditing, property tests,
  diagnostic-specific compile-fail checks and separate production coverage gates.
- Require explicit API editing outcomes and execute sending tests in `rust/bin/check`.
- Runnable Slack Morphism and reqwest sending examples, tested against local HTTP
  servers on minimum and stable Rust; transport dependencies remain application-owned.
