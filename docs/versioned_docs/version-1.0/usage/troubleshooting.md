# Troubleshooting & FAQ

Common questions, gotchas, and error messages.

## Why is my message body coming through with literal `*asterisks*` instead of bold?

Slack supports two text types: `mrkdwn` (a Slack-flavoured markdown) and `plain_text`. `slackblocks` uses `mrkdwn` by default for `SectionBlock` text, but **`HeaderBlock`, button labels, modal titles, input labels, and tab/option labels are forced to `plain_text` by Slack** — markdown syntax in those fields is rendered literally.

If you need bold/italic styling in those places, you can't get it. If you need it elsewhere and aren't getting it, double-check you're not accidentally constructing a `Text(type_=TextType.PLAINTEXT, ...)`.

## Why does my message say `text` is required?

Slack will warn (or in some cases reject) messages that have `blocks` but no `text` fallback. The `text` field is used for:

- Desktop and mobile **push notifications**.
- Accessibility readers.
- Clients that don't render Block Kit (rare, but they exist).

Always pass a short `text=` summary alongside `blocks=`:

```python
Message(
    channel="#general",
    text="Build #482 passed",        # fallback
    blocks=[HeaderBlock("Build #482 passed :white_check_mark:"), ...],
)
```

## What is `block_id` for? Should I set it?

`block_id` is a stable identifier for a block. Slack echoes it back to you in interaction payloads (button clicks, modal submits) so you can correlate which block produced the event.

- For **non-interactive** messages, you can leave it unset — `slackblocks` generates a random UUID.
- For **interactive** messages and modals, set explicit `block_id`s so your handlers can look up state reliably:

  ```python
  ActionsBlock(
      block_id="approval:req-123",
      elements=[Button(text="Approve", action_id="approve", value="req-123")],
  )
  ```

The same applies to `action_id` for elements.

## What does `InvalidUsageError: ... exceeds limit of N characters` mean?

Slack imposes hard character limits on most fields. `slackblocks` validates these at construction time so you find out before hitting Slack. Common limits:

| Field                              | Limit (chars) |
|------------------------------------|---------------|
| `SectionBlock.text`                | 3,000         |
| `SectionBlock.fields[i]`           | 2,000         |
| `HeaderBlock.text` (`plain_text`)  | 150           |
| `Button.text`                      | 75            |
| `Button.value`                     | 2,000         |
| `Button.url`                       | 3,000         |
| `Modal.title` / `submit` / `close` | 24            |
| `action_id`                        | 255           |
| `private_metadata`                 | 3,000         |
| `Option.url`                       | 3,000         |

If a field exceeds its limit, truncate or split the content.

## How many blocks can a single message have?

- Channel messages: **50 blocks** (Slack-side limit).
- Modals and home tabs: **100 blocks**.
- `ContextBlock`: max **10 elements**.
- `ActionsBlock`: max **25 elements**.
- `SectionBlock.fields`: max **10 items**.
- `TableBlock`: max **100 rows × 20 columns**.

## My buttons / selects don't do anything when clicked.

`slackblocks` only constructs the *outgoing* payload. Handling button clicks, menu selections, and modal submissions requires:

1. A public HTTPS endpoint registered as your app's **Interactivity & Shortcuts** URL.
2. An interaction handler that parses the payload Slack POSTs to that URL.

The [`slack-bolt`](https://slack.dev/bolt-python/) framework is the easiest way to wire this up.

## Why doesn't markdown work inside a `HeaderBlock`?

By design — Slack only allows `plain_text` in headers. Use a `SectionBlock` (which supports `mrkdwn`) followed by a `DividerBlock` if you want bold or styled text as a heading.

## How do I preview a message without actually sending it?

Use Slack's [Block Kit Builder](https://app.slack.com/block-kit-builder) and paste the JSON output from `message.json()`:

```python
print(message.json())
```

## How do I send a `slackblocks.Message` with `requests`?

`Message` renders to a dict, so:

```python
import os
import requests

from slackblocks import Message, SectionBlock

message = Message(channel="#general", blocks=SectionBlock("Hi!"))

requests.post(
    "https://slack.com/api/chat.postMessage",
    json=message.to_dict(),
    headers={"Authorization": f"Bearer {os.environ['SLACK_API_TOKEN']}"},
).raise_for_status()
```

## Can I use `slackblocks` with async code?

Yes — `slackblocks` itself is synchronous and side-effect-free, so `Message(...)` construction works identically inside `async def` functions. Use the async `WebClient` from `slack_sdk.web.async_client` to actually send:

```python
from slack_sdk.web.async_client import AsyncWebClient
from slackblocks import Message, SectionBlock

async def notify():
    client = AsyncWebClient(token=...)
    message = Message(channel="#general", blocks=SectionBlock("Hi!"))
    await client.chat_postMessage(**message)
```

## How does `slackblocks` compare to the block classes in `slack-sdk`?

`slack-sdk` ships its own `Block` / `SectionBlock` / etc. classes. `slackblocks` predates them and offers:

- A more concise API (e.g. `SectionBlock("Hi!")` vs `SectionBlock(text=MarkdownTextObject(text="Hi!"))`).
- Stricter up-front validation with informative `InvalidUsageError` messages.
- Independent release cadence — usable with the legacy `slackclient` or no SDK at all.

Use whichever you find more ergonomic; they produce equivalent JSON.

## I'm getting a `pytest` `error` from a Python `DeprecationWarning`.

`slackblocks`' own test suite turns warnings into errors via `pyproject.toml`. That setting only affects this project's CI — it doesn't propagate to your application. If you're seeing it, it's because you're running the `slackblocks` test suite as part of contributing.

## Where do I report bugs / request features?

GitHub Issues: <https://github.com/nicklambourne/slackblocks/issues>
