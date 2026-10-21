# Changelog

## Unreleased

- Reject explicit `focus_on_load` on external multi-selects, including false (SB-003). The compatibility option remains available; omit it or use nil. Other selectors retain focus support.

- Correct number-input bounds to preserved decimal strings, retaining numeric constructor/change conveniences and exact ordering (SB-001, spec 1.3.0). Stored fields now contain strings; package version unchanged.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).

## [2.8.0] — 2026-10-18

- Add all 89 modeled Block Kit values, 360 fields, shared validation and checked JSON ingress/egress as native immutable Elixir structs.
- Add tagged and raising constructors/changes, Jason encoding, components, colors, workflows and Block Kit Builder URLs.
- Support Elixir 1.16.3+ and compatible Erlang/OTP 26.2+ runtime pairs on 64-bit systems, including a tested Rebar3 consumer and standalone OTP release.
- Provide executable community-client, direct Req and webhook sending examples, plus an interaction-response payload example.
- Include native documentation, conformance, independent API auditing, property tests and standalone Hex artifact checks.
- Keep publication disabled pending completion of the train, account setup and maintainer approval.
