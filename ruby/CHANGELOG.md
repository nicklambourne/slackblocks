# Changelog

## [Unreleased]

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

## [3.0.0] — 2026-10-22

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- SB-006: reject messages with more than two data visualization blocks after composition, including attachment charts. The rule does not impose a two-chart limit on views.

- SB-003: reject explicitly supplied `focus_on_load` on external multi-selects, including false. Omit this deprecated parameter; other selectors retain focus support.
- Reject negative external-query thresholds, empty supplied static options/groups and malformed date/time strings, matching the retained Slack structural probes. Date shape checks do not add a calendar-validity guarantee; empty image alt text remains accepted (SB-013).

- Reject a plain-text input whose supplied `min_length` exceeds `max_length` (`out-of-range`); equal, zero-minimum and omitted bounds remain supported (SB-013).

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).

## [2.8.0] — 2026-10-18

- Coordinate the first Elixir implementation release; existing language support remains unchanged.

## [2.7.0] — 2026-10-11

- Add the PHP implementation to the coordinated release and documentation.

## [2.6.0] — 2026-10-09

- Keep application metadata outside message-wide Block Kit text totals.

- Release alongside the new native Rust implementation; supported language versions are unchanged.

## [2.5.0] — 2026-09-28

- Initial Ruby implementation of the shared Block Kit model.
