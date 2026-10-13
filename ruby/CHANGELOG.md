# Changelog

## [2.7.0] — 2026-10-11

- Add the PHP implementation to the coordinated release and documentation.

## [Unreleased]

- SB-006: reject messages with more than two data visualization blocks after composition, including attachment charts. The rule does not impose a two-chart limit on views.

- SB-003: reject explicitly supplied `focus_on_load` on external multi-selects, including false. Omit this deprecated parameter; other selectors retain focus support.

- Correct number-input bounds to string wire values, retain native numeric convenience arguments, and reject nonfinite/inverted bounds (SB-001, spec 1.3.0). Bound getters now return strings; reflection/signature consumers must migrate. Package version unchanged.

- Clarify that `submit_disabled`, including explicit false, is configuration-modal-only; ordinary modal examples omit it (SB-015).

## [2.6.0] — 2026-10-09

- Keep application metadata outside message-wide Block Kit text totals.

- Release alongside the new native Rust implementation; supported language versions are unchanged.

## [2.5.0] — 2026-09-28

- Initial Ruby implementation of the shared Block Kit model.
