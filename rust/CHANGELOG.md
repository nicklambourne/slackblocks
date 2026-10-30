# Changelog

## [3.2.0] — 2026-10-30

- Add native layout blocks, rows and items with typed widths/alignment, closed child and fallback roles, checked nesting and receiving-surface validation (spec 1.5.0).
- Preserve confirmed top-level message/view block counting, supported runtime minimums, legacy container-only behavior, and existing chart/text limits. No unconfirmed recursive block budget is imposed.
- Add a runnable ten-language layout recipe and rendered preview; preserve outgoing 3.1.0 documentation.

## [3.1.0] — 2026-10-28

- Coordinate this release with the new Haskell implementation and ten-language documentation. Existing runtime support remains unchanged.

## [3.0.0] — 2026-10-24

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- SB-006: reject messages with more than two data visualization blocks after composition, including attachment charts. The rule does not impose a two-chart limit on views.

- SB-003: reject explicitly supplied `focus_on_load` on external multi-selects, including false. Omit this deprecated parameter; other selectors retain focus support.
- Reject negative external-query thresholds, empty supplied static options/groups and malformed date/time strings, matching the retained Slack structural probes. Date shape checks do not add a calendar-validity guarantee; empty image alt text remains accepted (SB-013).

- Reject a plain-text input whose supplied `min_length` exceeds `max_length` (`out-of-range`); equal, zero-minimum and omitted bounds remain supported (SB-013).

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).
- Document native response URL visibility, replacement and delete-only payloads with executable examples.

## [2.8.0] — 2026-10-18

- Coordinate the first Elixir implementation release; existing language support remains unchanged.

## [2.7.0] — 2026-10-11

- Add the PHP implementation to the coordinated release and documentation.

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
