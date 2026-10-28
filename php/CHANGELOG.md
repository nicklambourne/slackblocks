# Changelog

## [3.1.0] — 2026-10-28

- Coordinate this release with the new Haskell implementation and ten-language documentation. Existing runtime support remains unchanged.

## [3.0.0] — 2026-10-24

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- Enforce the aggregate two-chart message limit across blocks, attachments and container children; views and opaque metadata remain unaffected (SB-006).

- Reject explicit `focusOnLoad` on external multi-selects, including false (SB-003). Omit the compatibility parameter; other selectors retain focus support.

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).
- Reject inverted plain-text input bounds, negative external-query thresholds, malformed date/time values and empty supplied static-select options/groups. Retain zero thresholds, equal bounds and the documented structural calendar boundary (SB-013).

## [2.8.0] — 2026-10-18

- Coordinate the first Elixir implementation release; existing language support remains unchanged.

## [2.7.0] — 2026-10-11

- Introduce native PHP 8.2+ Slack Block Kit values, typed roles and enums, immutable edits and checked JSON serialization.
- Cover all shared fixtures, invalid cases, limits and capabilities, plus independent public API, PHPStan contracts and actual installed archive checks.
- Add accordion, pagination, workflow, color and preview helpers; preserve opaque JSON objects and exact 64-bit integers.
- Test JoliCode, native cURL and Laravel integration examples with bounded transport behavior.
- Add complete native reference, block guides, recipes, README examples and language-aware search/navigation.
