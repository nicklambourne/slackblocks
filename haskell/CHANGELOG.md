# Changelog

## [4.0.0] — 2026-11-14

- Coordinate the ten-language 4.0.0 release with shared contract 1.6.0. Supported runtime floors are unchanged. See the 4.0.0 migration guide for Go imports, sparse-setting types and stricter validation.

- Require non-empty numeric display labels in both plain and data tables, matching the isolated Slack validation probe.

- Preserve skipped columns in sparse table column settings through native APIs and supported parsers. Null entries count toward the existing 20-column-settings limit.
- Breaking: change `tableBlockColumnSettings` entries to `Maybe ColumnSettings`: wrap existing settings in `Just` and use `Nothing` to skip a column.

- Add numeric plain-table cells and enforce Slack’s documented 10,000-character per-table and message-wide limits (spec 1.6.0). Count displayed cell text as Unicode code points, including composed blocks and attachments; ragged rows and partial column settings remain valid. Previously accepted over-limit tables now fail validation. Supported runtime floors are unchanged.

## [3.2.0] — 2026-10-31

- Add native layout blocks, rows and items with typed widths/alignment, closed child and fallback roles, checked nesting and receiving-surface validation (spec 1.5.0).
- Preserve confirmed top-level message/view block counting, supported runtime minimums, legacy container-only behavior, and existing chart/text limits. No unconfirmed recursive block budget is imposed.
- Add a runnable ten-language layout recipe and rendered preview; preserve outgoing 3.1.0 documentation.

## [3.1.0] — 2026-10-28

- Add the complete shared Block Kit contract with checked immutable values, native field records, closed roles and Aeson encoding.
- Include strict JSON ingress, exact decimal bounds, composition/editing helpers and separate tested HTTP consumers. Reject unrepresentable numeric encodings and range-check bounded integers before exponent expansion.
- Support GHC 9.6.7 through the tested 9.14 series, independent locked/lower/latest dependency graphs, compiled public API checks, source archives and native Haddock documentation.
- Join the ten-language coordinated release with verified source/documentation artifacts and fresh Hackage consumers.
