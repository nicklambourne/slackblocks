# Changelog

## [Unreleased]

## [2.6.0] — Unreleased

- Keep application metadata outside message-wide Block Kit text totals.
- Reject lossy JSON number literals, validate limit references during generation,
  simplify typed validation/plan serialization, and accept borrowed string arguments.

- Prepare the native Rust implementation for the coordinated 2.6.0 release.
- Publication remains disabled while implementation and registry setup are reviewed.

- Native convenience APIs for pagination, accordion sections, workflow triggers,
  attachment colors and Block Kit Builder preview URLs.
- Full shared conformance, independent public API auditing, property tests,
  diagnostic-specific compile-fail checks and separate production coverage gates.
- Runnable Slack Morphism and reqwest sending examples, tested against local HTTP
  servers on minimum and stable Rust; transport dependencies remain application-owned.
