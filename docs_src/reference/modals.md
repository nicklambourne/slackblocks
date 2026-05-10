# Modals

A **Modal** is a pop-up dialog used to collect input from a user. Modals are opened in response to an interaction (button click, slash command) using a `trigger_id` returned by Slack.

`Modal` is kept as a thin compatibility wrapper around [`ModalView`](views.md#views.ModalView) — new code can use either. See the [Confirmation modal cookbook recipe](../usage/cookbook.md#confirmation-modal) for a complete example.

Slack reference: <https://api.slack.com/surfaces/modals>

::: modals
    options:
        show_bases: false
