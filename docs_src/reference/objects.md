# Objects

**Objects** are the lowest-level composable primitives in `slackblocks`. They don't render on their own — instead, they're passed into [Elements](elements.md) and [Blocks](blocks.md) to populate them with text, choices, confirmations, and so on.

The most important ones:

- [`Text`](#objects.Text) — a piece of `mrkdwn` or `plain_text` content. Most fields that accept a `str` will also accept a `Text` for fine-grained control.
- [`Option`](#objects.Option) — a single selectable choice in a select menu, checkbox, or radio group.
- [`ConfirmationDialogue`](#objects.ConfirmationDialogue) — a "Are you sure?" prompt shown before destructive actions.

Slack reference: <https://api.slack.com/reference/block-kit/composition-objects>

::: objects
    options:
        filters: ["!^CompositionObject"]
        show_bases: false
