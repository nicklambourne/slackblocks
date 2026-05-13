# Utilities

This page documents top-level helper utilities shipped with `slackblocks`.

## Block Kit Builder URL

`block_kit_builder_url(payload, team_id=None)` builds a [Slack Block Kit Builder](https://app.slack.com/block-kit-builder) URL containing the supplied payload, so you can preview a message, view, or block list in the browser.

Accepted payload shapes:

- A single [`Block`](blocks.md) — wrapped in `{"blocks": [block]}` automatically.
- A list of `Block` — also wrapped.
- Anything with a `_resolve()` method ([`Message`](messages.md), [`WebhookMessage`](messages.md), [`MessageResponse`](messages.md), [`View`](views.md), [`ModalView`](views.md), [`HomeTabView`](views.md)).
- A raw `dict` — used verbatim. This is the escape hatch for callers building payloads outside the type hierarchy.

For a `team_id`-specific Builder URL (e.g. `https://app.slack.com/block-kit-builder/T0123#…`), pass `team_id="T0123ABCD"`.

::: builder
    options:
        show_root_heading: false
        show_root_toc_entry: false

## Validation helpers

Lower-level helper functions used across `slackblocks` for validation and coercion. Most users won't need to call these directly, but they're documented here for completeness and for anyone extending `slackblocks` with custom block or element types.

::: utils
