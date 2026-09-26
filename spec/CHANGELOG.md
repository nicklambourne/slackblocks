# Spec changelog

## Unreleased

- Add `vocabulary.json`, the shared registry of Slack icon names and per-surface
  block types. Java generates its validation tables from it, and the Python,
  TypeScript, and Go suites fail if their tables differ. The contents match the
  tables all four implementations already enforced.
- Add shared invalid cases for empty section fields and duplicate chart-series
  category labels. These close conformance gaps in the existing 1.1.0 contract.
- Register limits that Slack documents and that the implementations enforced
  without a `limits.json` leaf, each with invalid cases:
  - `table.rows.max_items` (100; its existing invalid case now has a leaf) and
    `table.columns.max_items` (20 cells per row);
  - `icon_button.value.max_length` (2000) and
    `icon_button.accessibility_label.max_length` (75);
  - `workflow_button.text.max_length` (75) and
    `workflow_button.accessibility_label.max_length` (75);
  - `placeholder.max_length` (150) on each input element that has one:
    `plain_text_input`, `email_input`, `url_input`, `number_input`,
    `date_picker`, `time_picker`, and `rich_text_input`, each with its own
    leaf and invalid case, as Slack documents each separately;
  - `dispatch_action_configuration.trigger_actions_on` (one or both triggers,
    1 to 2 items). The list is required whenever a configuration is present,
    as Slack's `blocks.validate` enforces although the docs mark it optional.

  Minimums that Slack documents only as "required", such as non-empty titles
  and element lists, stay as implementation checks rather than registered
  limits, as in 1.0.1. The table's 20-item `column_settings` maximum is not
  registered because it cannot be exceeded without breaching
  `table.columns.max_items` first.
- Correct `video.alt_text` from 1-200 characters to a 2000-character maximum
  with no minimum, to match Slack's validator. The video block documentation
  states no length for `alt_text`
  (<https://docs.slack.dev/reference/block-kit/blocks/video-block>), and
  `blocks.validate` accepts 2000 characters and an empty string and rejects
  2001 characters ("max_length expected 2000"). The
  `video-alt-text-empty` invalid case is removed and `video-alt-text-too-long`
  now exceeds 2000 characters.
- Align the contract with Slack's own validator. Every leaf in `limits.json`,
  every required field, every surface placement, and every shared fixture was
  probed against `blocks.validate` (1,251 probes, 2026-09-26). Where the docs
  and the validator disagree, the stricter applies; where the docs are silent,
  the validator's value does.
  - Fix four valid fixtures Slack rejected: a standalone task card with
    `pending` status, a Slack file ID shorter than `^F[A-Z0-9]{8,}$`, a rich
    text input whose `initial_value` was a text element rather than a
    `rich_text` block, and a rich text list with `border: 3`.
  - Register limits Slack enforces: rich text list `indent` 0-8, `offset` 0 or
    more, and list, quote, and preformatted `border` 0-1; image block `title`
    2000; image element `image_url` 3000 and `alt_text` 2000; video
    `thumbnail_url` and `video_url` 3000, `title_url` and `provider_icon_url`
    2000; view `external_id` 255; plain-text input `min_length` 0-3000 and
    `max_length` 1 or more; rich text input `min_lines` and `max_lines` 1-100;
    multi-select `max_selected_items` 1 or more; data table
    `row_header_column_index` 0 or more; table `column_settings` at most 20;
    container `child_blocks` at least 1; plan `tasks` at most 50 with unique
    `task_id`s; and conversation filter `include` non-empty and limited to
    `im`, `mpim`, `private`, and `public`.
  - Apply the documented message-wide totals: 12,000 characters of markdown
    block text and 20,000 characters of data table cell text.
  - Require fields Slack rejects when missing: `SlackIcon.name`, `plan.tasks`,
    `task_card.status`, `number_input.is_decimal_allowed`, `attachment.blocks`,
    `workflow_button.action_id`, and the defaulted `icon_button.icon` and
    `file.source`. A standalone task card cannot be `pending`; plan tasks can.
  - Change the rich text input's `initial_value` to a `rich_text` block, and add
    `slack_file` to the image block with exactly one of it or `image_url`.
  - Allow `input` blocks in messages and `data_visualization` in App Home, as
    both the docs and the validator do.
  - Remove the table rule that `column_settings` must have one entry per column;
    the docs allow fewer, and the validator accepts any number up to 20.
- Relax rules that neither Slack's docs nor its validator impose, found by the
  same `blocks.validate` sweep: `action_id` is optional on the 22 elements whose
  docs mark it optional (rich text inputs and workflow buttons keep it
  required, as documented); `url_source.url` has no length limits;
  `markdown.text` may be empty; views may have no blocks; and table rows may
  have different numbers of cells. New valid fixtures pin each of these:
  `elements/button_without_action_id`, `blocks/table_ragged_rows`,
  `blocks/markdown_empty`, and `views/modal_without_blocks`.

## 1.1.0 - 2026-08-28

- Enforce Slack surface compatibility for message, modal, and App Home block collections.
- Enforce the 50-block message limit, 100-attachment message limit, and non-empty message channel.
- Require submit text when a modal contains an input block.

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
