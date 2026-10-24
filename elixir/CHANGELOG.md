# Changelog

## [3.0.0] — 2026-10-24

- Add native button `agent_prompt` and `agent_prompt_display` fields, Unicode prompt-length validation and the display/prompt dependency (spec 1.4.0). Document interaction acknowledgement regardless of Slackbot AI availability. Existing button calls remain compatible.

- Release the coordinated 3.0.0 API across all nine implementations; supported language/runtime minimums remain unchanged.
- Breaking: number-input bounds are strings in stored values and getters; numeric construction conveniences remain. See the migration guide before upgrading.
- Freeze the 2.8.0 documentation and fix shared language/version selector artwork and arrows across documentation versions.

- Validate ordered plain-text bounds, nonnegative external-query thresholds, nonempty supplied static lists, ASCII date shape and 24-hour times through native constructors, changes and JSON ingress/egress (SB-013). Preserve omitted fields, zero thresholds, empty image alt text and application-owned calendar rules.

- Enforce the aggregate two-chart message limit across native message blocks, attachments and container children; leave views and opaque metadata unaffected (SB-006).

- Reject explicit `focus_on_load` on external multi-selects, including false (SB-003). The compatibility option remains available; omit it or use nil. Other selectors retain focus support.

- Correct number-input bounds to preserved decimal strings, retaining numeric constructor/change conveniences and exact ordering (SB-001, spec 1.3.0). Stored fields now contain strings.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).

## [2.8.0] — 2026-10-18

- Add all 89 modeled Block Kit values, 360 fields, shared validation and checked JSON ingress/egress as native immutable Elixir structs.
- Add tagged and raising constructors/changes, Jason encoding, components, colors, workflows and Block Kit Builder URLs.
- Support Elixir 1.16.3+ and compatible Erlang/OTP 26.2+ runtime pairs on 64-bit systems, including a tested Rebar3 consumer and standalone OTP release.
- Provide executable community-client, direct Req and webhook sending examples, plus an interaction-response payload example.
- Include native documentation, conformance, independent API auditing, property tests and standalone Hex artifact checks.
- Keep publication disabled pending completion of the train, account setup and maintainer approval.
