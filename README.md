# slackblocks <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs_src/img/sb.png" align="right" width="250px"/>

![Licence: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Licence: BSD-3-Clause](https://img.shields.io/badge/License-BSD_3_Clause-green.svg)
![Python Versions](https://img.shields.io/pypi/pyversions/slackblocks)
[![PyPI](https://img.shields.io/pypi/v/slackblocks?color=yellow&label=PyPI&logo=python&logoColor=white)](https://pypi.org/project/slackblocks/#history)
[![Downloads](https://static.pepy.tech/badge/slackblocks)](https://pepy.tech/project/slackblocks)
[![Build Status](https://github.com/nicklambourne/slackblocks/actions/workflows/unit-tests.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)
[![Docs](https://img.shields.io/badge/Docs-8A2BE2.svg)](https://nicklambourne.github.io/slackblocks)

> **Build Slack messages in Python — without writing JSON by hand.**

`slackblocks` is a typed, validating Python wrapper around the Slack [Block Kit API](https://api.slack.com/block-kit). It exists because Block Kit JSON is verbose, easy to get subtly wrong, and unpleasant to maintain in source control.

## Why `slackblocks`?

- **Concise** — `SectionBlock("Hello, *world*!")` instead of a 10-line JSON object.
- **Validated** — character limits, required fields, mutually-exclusive options, and element-type restrictions are enforced at construction time, so you find out *before* hitting Slack's API.
- **Drop-in compatible** with both the official [`slack-sdk`](https://pypi.org/project/slack-sdk/) and the legacy [`slackclient`](https://pypi.org/project/slackclient/) — unpack a `Message` directly into `client.chat_postMessage(**message)`.
- **Typed** — full type hints; ships `py.typed`.
- **Zero runtime dependencies.**

## Installation

```bash
pip install slackblocks
```

Requires Python 3.8.1 or newer.

## Quickstart

```python
from slackblocks import (
    ActionsBlock,
    Button,
    DividerBlock,
    HeaderBlock,
    Message,
    SectionBlock,
)

message = Message(
    channel="#general",
    text="Build #482 passed",  # plain-text fallback for notifications
    blocks=[
        HeaderBlock("Build #482 passed :white_check_mark:"),
        SectionBlock(
            fields=[
                "*Branch*\n`main`",
                "*Author*\n@nick",
                "*Duration*\n3m 12s",
                "*Tests*\n1,247 passed",
            ],
        ),
        DividerBlock(),
        ActionsBlock(
            elements=[
                Button(text="View build", action_id="view", url="https://ci.example.com/482"),
                Button(text="Re-run", action_id="rerun", value="482", style="primary"),
            ],
        ),
    ],
)
```

`message` can be sent in one line with the official Slack SDK:

```python
import os
from slack_sdk import WebClient

client = WebClient(token=os.environ["SLACK_API_TOKEN"])
client.chat_postMessage(**message)
```

The `**` operator unpacks `slackblocks` `Message` objects directly into the SDK call — no `to_dict()` boilerplate required.

<p align="center">
  <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs_src/img/hello_world.png" alt="A simple Slack message rendered in Slack" width="600px" />
</p>

## What's supported

| Surface             | Status |
|---------------------|:------:|
| Blocks              | ✅ All current block types (Section, Header, Divider, Image, Context, Actions, Input, RichText, File, Table) |
| Elements            | ✅ Buttons, all select menus, date/time pickers, checkboxes, radio groups, all input types, overflow menus, workflow buttons |
| Composition Objects | ✅ Text, Option, Confirm, Conversation/Dispatch filters, Workflow, Trigger |
| Rich Text           | ✅ Sections, lists, quotes, code blocks, inline links/users/channels/emoji |
| Modals & Home Tabs  | ✅ Full views API |
| Messages            | ✅ `chat.postMessage`, webhook messages, slash-command/interaction responses, threaded replies, ephemeral messages |
| Attachments         | ⚠️ Supported but [deprecated by Slack](https://api.slack.com/reference/messaging/attachments) |

## Documentation

- **Full docs:** <https://nicklambourne.github.io/slackblocks/>
- [Installation](https://nicklambourne.github.io/slackblocks/latest/usage/installation/)
- [Using Blocks](https://nicklambourne.github.io/slackblocks/latest/usage/using_blocks/) — every block type with code, JSON, and screenshots.
- [Sending Messages](https://nicklambourne.github.io/slackblocks/latest/usage/sending_messages/)
- [Cookbook](https://nicklambourne.github.io/slackblocks/latest/usage/cookbook/) — end-to-end recipes for build notifications, approval requests, modals, and more.
- [Troubleshooting & FAQ](https://nicklambourne.github.io/slackblocks/latest/usage/troubleshooting/)

## Comparison with `slack-sdk` block classes

The official `slack-sdk` ships its own block classes. `slackblocks` predates them and offers a more concise API, stricter up-front validation, and independent versioning. They produce equivalent JSON; pick whichever you find more ergonomic.

```python
# slackblocks
SectionBlock("Hello, *world*!")

# slack-sdk equivalent
from slack_sdk.models.blocks import SectionBlock as SDKSectionBlock
from slack_sdk.models.blocks.basic_components import MarkdownTextObject
SDKSectionBlock(text=MarkdownTextObject(text="Hello, *world*!"))
```

## Licensing

`slackblocks` is dual-licensed under [MIT](https://github.com/nicklambourne/slackblocks/blob/master/LICENSE) and [BSD-3-Clause](https://github.com/nicklambourne/slackblocks/blob/master/LICENSE.BSD-3-Clause). Use whichever fits your project — this makes it safe to vendor into projects under either license.

## Contributing

Contributions are welcome. Quick start (the project uses [uv](https://docs.astral.sh/uv/) for dependency management):

```bash
git clone https://github.com/nicklambourne/slackblocks.git
cd slackblocks
uv sync --all-groups

uv run pytest test/unit
uv run black . --check
uv run flake8 slackblocks
uv run mypy slackblocks
```

Preview the documentation locally with `uv run mkdocs serve`.

For the full development guide — testing conventions, validation patterns, docstring style, release process, and a PR checklist — see the [Contributing page](https://nicklambourne.github.io/slackblocks/latest/contributing/).

Bug reports and feature requests: <https://github.com/nicklambourne/slackblocks/issues>.
