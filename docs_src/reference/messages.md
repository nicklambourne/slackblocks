# Messages

**Messages** are the top-level objects you send to Slack. `slackblocks` provides several message types depending on the API surface you're targeting:

| Class                | When to use it                                                                                  |
|----------------------|-------------------------------------------------------------------------------------------------|
| `Message`            | Posting a normal message via `chat.postMessage` (the most common case).                         |
| `WebhookMessage`     | Sending to an [Incoming Webhook](https://api.slack.com/messaging/webhooks) URL.                 |
| `MessageResponse`    | Replying to a slash command or an interaction `response_url`.                                   |

All three implement `__getitem__`/`keys()`, so you can unpack them with `**message` straight into the Slack SDK. See [Sending Messages](../usage/sending_messages.md) for end-to-end examples.

Slack reference: <https://api.slack.com/methods/chat.postMessage>

::: messages
    options:
        filters: ["!^BaseMessage"]
        show_bases: false
