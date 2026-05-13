# Changelog

All notable changes to `slackblocks` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] — Unreleased

The 2.0 development series.

`slackblocks 2.x` requires **Python 3.10 or newer**. Users on Python 3.8 or 3.9
should pin to the `1.x` release line; see the
[Compatibility](https://nicklambourne.github.io/slackblocks/latest/usage/compatibility/)
docs page for details.

### Added

- **`MarkdownBlock`** for Slack's 2024 GitHub-flavored Markdown block type
  (1-12000 character `text` field). Used for AI / agentic app outputs.
  ([#198], [#199])
- **`VideoBlock`** for embedding video content with `alt_text`,
  `thumbnail_url`, `title`, `video_url`, plus six optional fields.
  ([#200], [#201])
- **`PlainText`** and **`Markdown`** thin subclasses of `Text` that remove
  the boilerplate of constructing plain-text or `mrkdwn`-flavored text:
  `PlainText("Hi", emoji=True)` instead of
  `Text("Hi", type_=TextType.PLAINTEXT, emoji=True)`. ([#202], [#203])
- **`block_kit_builder_url(payload, team_id=None)`** utility that turns any
  block, list of blocks, message, view, or raw dict into a
  [Block Kit Builder](https://app.slack.com/block-kit-builder) URL for
  browser-based preview. ([#204], [#205])
- **`Workflow.from_url(url, **params)`** classmethod factory that collapses
  the four-deep nested workflow construction (`Workflow` -> `Trigger` ->
  `customizable_input_parameters` -> `InputParameter`) into a single call.
  ([#206], [#207])
- **`from_dict()`** parsers for round-tripping Slack JSON back into
  `slackblocks` objects. Supported now: every composition object
  (`Text`, `Option`, `OptionGroup`, `Confirm`, `ConversationFilter`,
  `DispatchActionConfiguration`, `InputParameter`, `SlackFile`, `Trigger`,
  `Workflow`) and the basic blocks (`DividerBlock`, `FileBlock`,
  `HeaderBlock`, `MarkdownBlock`, `ImageBlock`, `SectionBlock`,
  `ContextBlock`, `VideoBlock`). Element-level and rich-text round-tripping
  is deferred to follow-up phases. ([#208], [#209])
- Top-level **`Block.from_dict(data)`** dispatcher that reads `data["type"]`
  and routes to the right subclass. `Block` is now exported from the
  package root. ([#208], [#209])
- Five new **typed exception subclasses** of `InvalidUsageError` so
  consumer code can `except` for specific failure categories instead of
  string-matching the message: `LengthError`, `RangeError`,
  `TypeMismatchError`, `MutualExclusivityError`, `MissingRequiredError`.
  Existing `except InvalidUsageError` blocks continue to catch every
  subclass unchanged. ([#196], [#197])
- **`PEP 561` `py.typed` marker file** so downstream type checkers honour
  `slackblocks`' inline annotations. ([#164], [#165])
- **Compatibility docs page** ([`usage/compatibility.md`](usage/compatibility.md))
  documenting which Python versions each release line supports. ([#166], [#167])
- **`PEP 585` / `PEP 604` annotation syntax** across the codebase
  (`list[X]`, `X | Y`) plus `from __future__ import annotations` in every
  module. ([#170], [#171])
- **`PEP 613` `TypeAlias`** declarations on the public type aliases:
  `TextLike`, `ButtonStyleLike`, `ButtonStyleName`, `ColumnAlignment`,
  `ConversationType`. ([#194], [#195])
- **`Literal` type narrowing** for string-valued enums:
  `Button.style: ButtonStyle | Literal["primary", "danger"] | None`,
  `ColumnSettings.align: Literal["left", "center", "right"] | None`,
  `ConversationFilter.include` items typed as
  `Literal["im", "mpim", "private", "public"]`. mypy / pyright now reject
  misspellings at type-check time. ([#190], [#191])
- **`@overload` on `Text.to_text`** so the return type narrows to `Text`
  when `allow_none=False` (the default), and to `Text | None` when
  `allow_none=True`. ([#192], [#193])
- Internal `slackblocks/_core.py` module providing a shared `Resolvable`
  protocol, recursive `resolve()` walker, and `omit_none()` helper. The
  walker structurally eliminates the entire class of bugs that produced
  eight Phase 1 P0 fixes (forgotten `._resolve()` calls on nested
  objects). ([#174], [#175])

### Changed

- **Minimum Python version raised to 3.10** (was 3.8.1). Bump version to
  `2.0.0.dev0`. ([#168], [#169])
- **Tooling migrated from `black` + `flake8` + `flake8-pyproject` to
  [`ruff`](https://docs.astral.sh/ruff/)** for both linting and
  formatting. Workflow files renamed (`formatting.yml` -> `ruff-format.yml`,
  `linting.yml` -> `ruff-lint.yml`); CI job names updated to
  `Ruff Format` and `Ruff Lint`. Branch protection required-status-check
  list updated correspondingly. ([#162], [#163])
- **Additional ruff rule families enabled**: `B` (bugbear), `I` (isort),
  `SIM` (simplify), `TC` (type-checking), `UP` (pyupgrade) on top of the
  baseline `E`, `F`, `W`. ([#172], [#173])
- **`_resolve()` methods refactored** across the entire library to use the
  central `resolve()` / `omit_none()` helpers from `_core.py`. The diff is
  internal — public API and JSON output are byte-identical — but the
  per-class boilerplate is dramatically reduced. ([#176]-[#185])
- **Shared `RenderableMixin`** factored out of the five abstract base
  classes (`Block`, `Element`, `CompositionObject`, `RichTextElement`,
  `RichTextObject`) which previously each defined their own copy of
  `__repr__`. ([#188], [#189])
- **`_MessagePayloadMixin`** factored out of `BaseMessage` and
  `WebhookMessage` to dedupe the `to_dict` / `json` / `__repr__` /
  `__getitem__` / `keys` payload helpers. ([#186], [#187])
- **Rich-text style emission** deduplicated via a private `_style_dict()`
  helper; the same 8-line block previously appeared in 5 classes. ([#182], [#183])
- Renamed `text_basic_file_block` test (typo, never collected by pytest)
  to `test_basic_file_block`. ([#156], [#157])

### Fixed

19 P0 correctness bugs found in the audit before 2.0 development began,
each in its own focused PR. The most impactful:

- **`Confirm.__init__` was broken** — `super(*args, **kwargs)` (missing
  parens) raised `TypeError` on every invocation. ([#126], [#127])
- **Eight `_resolve()` methods forgot to recurse into nested objects**,
  causing `json.dumps` to raise `TypeError` for any caller exercising the
  affected fields: `Image.slack_file` ([#130], [#131]),
  `URLInput.placeholder` ([#132], [#133]), `TimePicker.confirm` ([#134],
  [#137]), `DatePicker` (confirm + missing `initial_date`) ([#135],
  [#138]), `DateTimePicker` (confirm + missing `initial_datetime`)
  ([#136], [#139]), `ConversationSelectMenu.filter` ([#140], [#141]),
  `RichTextInput.dispatch_action_config` and `placeholder` ([#142], [#143]).
- **`ConversationMultiSelectMenu` typo'd JSON key** `"intial_conversations"`
  (Slack silently ignored it). ([#144], [#145])
- **`NumberInput` dropped `min_value=0` / `max_value=0`** via truthy
  checks, plus a typo'd error message that referenced `min_value` twice.
  ([#146], [#147])
- **`StaticSelectMenu` raised `UnboundLocalError`** when neither `options`
  nor `option_groups` was provided. ([#150], [#151])
- **`FileBlock` required an explicit `block_id`** instead of synthesising
  one. ([#152], [#153])
- **`FileInput` and `SlackFile` were missing from the top-level
  package exports**. ([#154], [#155])
- **`validate_int` max-value error message said "less than the minimum"**
  (copy-paste bug from the min-value branch). ([#120], [#121])
- **`validate_string_nonnull` truthy `min_length` / `max_length` checks**
  silently dropped explicit `0` values. ([#122], [#123])
- **`validate_string` and `validate_string_nonnull` had inconsistent
  positional argument order**. ([#124], [#125])
- **`DispatchActionConfiguration.__init__` did not call `super().__init__`**,
  so `self.type` was unset. ([#128], [#129])
- **`option_groups` flattening used quadratic `sum([list], [])`** instead
  of `itertools.chain.from_iterable`. ([#148], [#149])
- **mypy was failing on master** because `DatePicker.initial_date` was
  inferred as `str` (from the `if` branch) and conflicted with the `None`
  in the `else` branch. ([#158], [#160])
- **black was failing on master** in `test_elements.py` after a
  multi-line `Option(...)` invocation. ([#159], [#161])

### Removed

- Removed `black`, `flake8`, `flake8-pyproject` dev dependencies (replaced
  by `ruff`). ([#162], [#163])
- Removed Python 3.8 and 3.9 from the supported versions and the CI
  matrix. ([#168], [#169])
- Removed Slack-API-URL `# noqa: E501` comments inside docstrings (ruff
  correctly does not honour them since they are within string literals,
  not real code comments). ([#162], [#163])

## [1.2.5] — 2026-05-10

Last release of the `1.x` line. Patch release; no functional changes.

## Earlier releases

For 1.x and 0.x release history, see the
[Git tag history](https://github.com/nicklambourne/slackblocks/tags).

[#120]: https://github.com/nicklambourne/slackblocks/pull/121
[#121]: https://github.com/nicklambourne/slackblocks/pull/121
[#122]: https://github.com/nicklambourne/slackblocks/pull/123
[#123]: https://github.com/nicklambourne/slackblocks/pull/123
[#124]: https://github.com/nicklambourne/slackblocks/pull/125
[#125]: https://github.com/nicklambourne/slackblocks/pull/125
[#126]: https://github.com/nicklambourne/slackblocks/pull/127
[#127]: https://github.com/nicklambourne/slackblocks/pull/127
[#128]: https://github.com/nicklambourne/slackblocks/pull/129
[#129]: https://github.com/nicklambourne/slackblocks/pull/129
[#130]: https://github.com/nicklambourne/slackblocks/pull/131
[#131]: https://github.com/nicklambourne/slackblocks/pull/131
[#132]: https://github.com/nicklambourne/slackblocks/pull/133
[#133]: https://github.com/nicklambourne/slackblocks/pull/133
[#134]: https://github.com/nicklambourne/slackblocks/pull/137
[#135]: https://github.com/nicklambourne/slackblocks/pull/138
[#136]: https://github.com/nicklambourne/slackblocks/pull/139
[#137]: https://github.com/nicklambourne/slackblocks/pull/137
[#138]: https://github.com/nicklambourne/slackblocks/pull/138
[#139]: https://github.com/nicklambourne/slackblocks/pull/139
[#140]: https://github.com/nicklambourne/slackblocks/pull/141
[#141]: https://github.com/nicklambourne/slackblocks/pull/141
[#142]: https://github.com/nicklambourne/slackblocks/pull/143
[#143]: https://github.com/nicklambourne/slackblocks/pull/143
[#144]: https://github.com/nicklambourne/slackblocks/pull/145
[#145]: https://github.com/nicklambourne/slackblocks/pull/145
[#146]: https://github.com/nicklambourne/slackblocks/pull/147
[#147]: https://github.com/nicklambourne/slackblocks/pull/147
[#148]: https://github.com/nicklambourne/slackblocks/pull/149
[#149]: https://github.com/nicklambourne/slackblocks/pull/149
[#150]: https://github.com/nicklambourne/slackblocks/pull/151
[#151]: https://github.com/nicklambourne/slackblocks/pull/151
[#152]: https://github.com/nicklambourne/slackblocks/pull/153
[#153]: https://github.com/nicklambourne/slackblocks/pull/153
[#154]: https://github.com/nicklambourne/slackblocks/pull/155
[#155]: https://github.com/nicklambourne/slackblocks/pull/155
[#156]: https://github.com/nicklambourne/slackblocks/pull/157
[#157]: https://github.com/nicklambourne/slackblocks/pull/157
[#158]: https://github.com/nicklambourne/slackblocks/pull/160
[#159]: https://github.com/nicklambourne/slackblocks/pull/161
[#160]: https://github.com/nicklambourne/slackblocks/pull/160
[#161]: https://github.com/nicklambourne/slackblocks/pull/161
[#162]: https://github.com/nicklambourne/slackblocks/pull/163
[#163]: https://github.com/nicklambourne/slackblocks/pull/163
[#164]: https://github.com/nicklambourne/slackblocks/pull/165
[#165]: https://github.com/nicklambourne/slackblocks/pull/165
[#166]: https://github.com/nicklambourne/slackblocks/pull/167
[#167]: https://github.com/nicklambourne/slackblocks/pull/167
[#168]: https://github.com/nicklambourne/slackblocks/pull/169
[#169]: https://github.com/nicklambourne/slackblocks/pull/169
[#170]: https://github.com/nicklambourne/slackblocks/pull/171
[#171]: https://github.com/nicklambourne/slackblocks/pull/171
[#172]: https://github.com/nicklambourne/slackblocks/pull/173
[#173]: https://github.com/nicklambourne/slackblocks/pull/173
[#174]: https://github.com/nicklambourne/slackblocks/pull/175
[#175]: https://github.com/nicklambourne/slackblocks/pull/175
[#176]: https://github.com/nicklambourne/slackblocks/pull/177
[#177]: https://github.com/nicklambourne/slackblocks/pull/177
[#178]: https://github.com/nicklambourne/slackblocks/pull/179
[#179]: https://github.com/nicklambourne/slackblocks/pull/179
[#180]: https://github.com/nicklambourne/slackblocks/pull/181
[#181]: https://github.com/nicklambourne/slackblocks/pull/181
[#182]: https://github.com/nicklambourne/slackblocks/pull/183
[#183]: https://github.com/nicklambourne/slackblocks/pull/183
[#184]: https://github.com/nicklambourne/slackblocks/pull/185
[#185]: https://github.com/nicklambourne/slackblocks/pull/185
[#186]: https://github.com/nicklambourne/slackblocks/pull/187
[#187]: https://github.com/nicklambourne/slackblocks/pull/187
[#188]: https://github.com/nicklambourne/slackblocks/pull/189
[#189]: https://github.com/nicklambourne/slackblocks/pull/189
[#190]: https://github.com/nicklambourne/slackblocks/pull/191
[#191]: https://github.com/nicklambourne/slackblocks/pull/191
[#192]: https://github.com/nicklambourne/slackblocks/pull/193
[#193]: https://github.com/nicklambourne/slackblocks/pull/193
[#194]: https://github.com/nicklambourne/slackblocks/pull/195
[#195]: https://github.com/nicklambourne/slackblocks/pull/195
[#196]: https://github.com/nicklambourne/slackblocks/pull/197
[#197]: https://github.com/nicklambourne/slackblocks/pull/197
[#198]: https://github.com/nicklambourne/slackblocks/pull/199
[#199]: https://github.com/nicklambourne/slackblocks/pull/199
[#200]: https://github.com/nicklambourne/slackblocks/pull/201
[#201]: https://github.com/nicklambourne/slackblocks/pull/201
[#202]: https://github.com/nicklambourne/slackblocks/pull/203
[#203]: https://github.com/nicklambourne/slackblocks/pull/203
[#204]: https://github.com/nicklambourne/slackblocks/pull/205
[#205]: https://github.com/nicklambourne/slackblocks/pull/205
[#206]: https://github.com/nicklambourne/slackblocks/pull/207
[#207]: https://github.com/nicklambourne/slackblocks/pull/207
[#208]: https://github.com/nicklambourne/slackblocks/pull/209
[#209]: https://github.com/nicklambourne/slackblocks/pull/209
