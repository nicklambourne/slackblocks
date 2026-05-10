# Sending Messages

`slackblocks` produces the JSON payloads that the Slack APIs accept; it does **not** make HTTP calls itself. You're free to send the rendered messages with any HTTP client (`curl`, `requests`, `httpx`), but the recommended path is to combine `slackblocks` with the official [`slack-sdk`](https://pypi.org/project/slack-sdk/) (or its legacy [`slackclient`](https://pypi.org/project/slackclient/) predecessor).

The trick is the `**` operator: a `slackblocks.Message` is a mapping, so you can unpack it directly into `client.chat_postMessage(...)`.

## With the modern `slack-sdk`

=== "Python (`slackblocks`)"
    ```python
    from os import environ
    from slack_sdk import WebClient
    from slackblocks import Message, SectionBlock


    client = WebClient(token=environ["SLACK_API_TOKEN"])
    block = SectionBlock("Hello, world!")
    message = Message(channel="#general", blocks=block)

    response = client.chat_postMessage(**message)
    ```

=== "JSON Message"
    ```json
    {
        "channel": "#general",
        "mrkdwn": true,
        "blocks": [
            {
                "type": "section",
                "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hello, world!"
                }
            }
        ]
    }
    ```
    Note: the `block_id` field is a pseudorandomly generated UUID. Pass an explicit `block_id` to any block constructor if you need deterministic IDs (e.g. for testing or interaction handling).

=== "Equivalent `curl`"
    ```bash
    curl -H "Content-type: application/json" \
      --data '{"channel":"#general","blocks":[{"type":"section","block_id":"992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3","text":{"type":"mrkdwn","text":"Hello, world!"}}]}' \
      -H "Authorization: Bearer ${SLACK_API_TOKEN}" \
      -X POST https://slack.com/api/chat.postMessage
    ```

=== "Slack UI"
    ![Hello World rendered in Slack](../img/hello_world.png)

## With the legacy `slackclient`

The API is identical — only the import path changes:

```python
from os import environ
from slack import WebClient   # legacy slackclient package
from slackblocks import Message, SectionBlock


client = WebClient(token=environ["SLACK_API_TOKEN"])
message = Message(channel="#general", blocks=SectionBlock("Hello, world!"))

response = client.chat_postMessage(**message)
```

## Other delivery surfaces

`slackblocks` provides specialized message classes for each Slack delivery surface. They all unpack the same way as `Message`.

### Incoming webhooks

```python
from slack_sdk.webhook import WebhookClient
from slackblocks import WebhookMessage, SectionBlock

webhook = WebhookClient(url="https://hooks.slack.com/services/...")
message = WebhookMessage(blocks=SectionBlock("Build complete :white_check_mark:"))

webhook.send(**message)
```

`WebhookMessage` supports webhook-only options like `response_type`, `replace_original`, and `delete_original`.

### Slash command / interaction responses

When responding to a slash command or interactive payload, use `MessageResponse`:

```python
from slackblocks import MessageResponse, SectionBlock

response_body = MessageResponse(
    blocks=SectionBlock("Got it! Working on that now..."),
    ephemeral=True,        # only visible to the invoking user
).json()
```

### Modals & home tabs

For modals and home tab views, build a `Modal` (or `HomeTabView`) and pass it to `views_open` / `views_publish`:

```python
from slack_sdk import WebClient
from slackblocks import Modal, SectionBlock

client = WebClient(token=environ["SLACK_API_TOKEN"])
modal = Modal(
    title="Confirm action",
    blocks=[SectionBlock("Are you sure?")],
    submit="Yes",
    close="Cancel",
)

client.views_open(trigger_id=trigger_id, view=modal.to_dict())
```

See [Modals reference](../reference/modals.md) and [Views reference](../reference/views.md) for the full surface.

## Sending without an SDK

Because `Message` renders to a plain dict, you can also send it directly:

```python
import json
import os
import urllib.request

from slackblocks import Message, SectionBlock

message = Message(channel="#general", blocks=SectionBlock("Hello, world!"))

req = urllib.request.Request(
    "https://slack.com/api/chat.postMessage",
    data=message.json().encode("utf-8"),
    headers={
        "Content-Type": "application/json; charset=utf-8",
        "Authorization": f"Bearer {os.environ['SLACK_API_TOKEN']}",
    },
)
urllib.request.urlopen(req)
```
