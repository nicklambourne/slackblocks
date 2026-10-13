# Changelog

## Unreleased

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate. Package version unchanged.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).
- Reject inverted plain-text input bounds, negative external-query thresholds, malformed date/time values and empty supplied static-select options/groups. Retain zero thresholds, equal bounds and the documented structural calendar boundary (SB-013).

## [2.7.0] — 2026-10-11

- Introduce native PHP 8.2+ Slack Block Kit values, typed roles and enums, immutable edits and checked JSON serialization.
- Cover all shared fixtures, invalid cases, limits and capabilities, plus independent public API, PHPStan contracts and actual installed archive checks.
- Add accordion, pagination, workflow, color and preview helpers; preserve opaque JSON objects and exact 64-bit integers.
- Test JoliCode, native cURL and Laravel integration examples with bounded transport behavior.
- Add complete native reference, block guides, recipes, README examples and language-aware search/navigation.
