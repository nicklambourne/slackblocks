# Spec changelog

## 1.0.1 - 2026-08-14

- Correct `option.value.max_length` from 75 to 150 to match the current Slack
  documentation ("Maximum length for this field is 150 characters",
  <https://docs.slack.dev/reference/block-kit/composition-objects/option-object>).
- Verify the overflow-menu options minimum against the live documentation:
  Slack documents only "an array of up to five option objects" with no
  minimum, so `overflow.options.min_items` stays 1.
- Register scalar limits already enforced by both implementations:
  `checkboxes.options` (1-10), `radio_buttons.options` (1-10),
  `option.url.max_length` (3000), and `url_source.url` (1-3000), each with a
  new invalid case.
- Add structural invalid cases for the table block: row count over the
  100-row limit, ragged rows, and a column-settings/column-count mismatch.
- Add at-limit valid fixtures (`since` 1.0.1): header text at exactly 150
  characters, header text of exactly 150 astral-plane emoji code points,
  button text at exactly 75 characters, and an option value at exactly 150
  characters; plus an emoji-based over-limit invalid case pinning that
  character limits count Unicode code points.
- Pin an explicit `block_id` in the `blocks/table_block` fixture so both
  implementations construct it identically.
- Enforce `coverage.json` against the packages' public exports: both
  conformance harnesses now fail when a public JSON-producing symbol lacks a
  registered capability.

## 1.0.0 - 2026-08-12

- Promote the existing Python golden fixtures into the shared valid corpus and expand it to 96 Slack-validated cases.
- Define the cross-language validation categories and seed invalid cases.
- Publish a shared scalar limits registry.
- Require fixture coverage for every supported JSON capability and invalid-case coverage for every scalar limit.
