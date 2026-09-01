# slackblocks <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/static/img/sb.png" align="right" width="250px"/>

![Licence: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Licence: BSD-3-Clause](https://img.shields.io/badge/License-BSD_3_Clause-green.svg)
![Python Versions](https://img.shields.io/pypi/pyversions/slackblocks)
[![PyPI](https://img.shields.io/pypi/v/slackblocks?color=yellow&label=PyPI&logo=python&logoColor=white)](https://pypi.org/project/slackblocks/#history)
[![npm](https://img.shields.io/npm/v/%40nicklambourne%2Fslackblocks?color=CB3837&label=npm&logo=npm)](https://www.npmjs.com/package/@nicklambourne/slackblocks)
[![Downloads](https://static.pepy.tech/badge/slackblocks)](https://pepy.tech/project/slackblocks)
[![Python CI](https://github.com/nicklambourne/slackblocks/actions/workflows/unit-tests.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)
[![TypeScript CI](https://github.com/nicklambourne/slackblocks/actions/workflows/typescript.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)
[![Go CI](https://github.com/nicklambourne/slackblocks/actions/workflows/go.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)
[![Docs](https://img.shields.io/badge/Docs-8A2BE2.svg)](https://nicklambourne.github.io/slackblocks)

> **Build Slack messages in Python, TypeScript, or Go — without writing JSON by hand.**

Anyone who has built a non-trivial Slack message knows the drill: a wall of nested
[Block Kit](https://docs.slack.dev/block-kit/) JSON, five levels deep, where a typo'd
field name or an over-long string sails silently through your code and only blows up
when Slack rejects the API call. `slackblocks` replaces that JSON with typed objects
that assemble it for you — and that complain at construction time, in your editor and
your tests, rather than in production.

## Why `slackblocks`?

- **Concise** — `SectionBlock("Hello, *world*!")` / `SectionBlock().text("Hello, *world*!").build()`
  instead of a ten-line JSON object.
- **Validated up front** — character limits, required fields, mutually-exclusive options,
  and element-type restrictions are enforced when you construct the block, so you find
  out *before* hitting Slack's API.
- **Typed** — full type hints and `py.typed` in Python, strict types in TypeScript,
  and compile-checked fluent methods in Go.
- **Plays well with established Slack clients** — unpack a `Message` straight into
  `client.chat_postMessage(**message)` with [`slack-sdk`](https://pypi.org/project/slack-sdk/),
  pass a payload directly to [`@slack/web-api`](https://www.npmjs.com/package/@slack/web-api),
  or pass Go block builders directly to [`slack-go/slack`](https://github.com/slack-go/slack).
- **One library, three languages** — the same blocks, validation rules, and version
  numbers in Python, TypeScript, and Go. A shared conformance corpus keeps all three
  implementations emitting the same Slack JSON.
- **Everything Block Kit ships today** — all current blocks and elements, rich text,
  modals and Home tabs, and the 2025 block families (tables, cards, carousels, charts).
- **Light** — zero runtime dependencies in Python, a self-contained ESM module on npm,
  and one direct Go dependency: `slack-go/slack`, used for delivery.

## Installation

Python (3.10+ — earlier Pythons should pin the `1.x` line, see
[Compatibility](https://nicklambourne.github.io/slackblocks/usage/compatibility)):

```bash
pip install slackblocks
```

TypeScript / JavaScript (Node 20.19+ or 22.12+, ESM):

```bash
npm install @nicklambourne/slackblocks
```

Go (1.22+):

```bash
go get github.com/nicklambourne/slackblocks/go/v2
```

## Quickstart

A CI notification, in Python:

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

Send it in one line with the official Slack SDK — the `**` operator unpacks
`Message` objects directly into the call, no `to_dict()` boilerplate:

```python
import os
from slack_sdk import WebClient

client = WebClient(token=os.environ["SLACK_API_TOKEN"])
client.chat_postMessage(**message)
```

The same message in TypeScript:

```ts
import {
  ActionsBlock,
  Button,
  DividerBlock,
  HeaderBlock,
  Message,
  SectionBlock,
} from "@nicklambourne/slackblocks";

const payload = Message()
  .channel("#general")
  .text("Build #482 passed") // plain-text fallback for notifications
  .blocks(
    HeaderBlock().text("Build #482 passed :white_check_mark:"),
    SectionBlock().fields(
      "*Branch*\n`main`",
      "*Author*\n@nick",
      "*Duration*\n3m 12s",
      "*Tests*\n1,247 passed",
    ),
    DividerBlock(),
    ActionsBlock().elements(
      Button().text("View build").actionId("view").url("https://ci.example.com/482"),
      Button().text("Re-run").actionId("rerun").value("482").style("primary"),
    ),
  )
  .build();
```

```ts
import { WebClient } from "@slack/web-api";

const client = new WebClient(process.env.SLACK_API_TOKEN);
await client.chat.postMessage(payload);
```

And in Go:

```go
client := slack.New(os.Getenv("SLACK_API_TOKEN"))
_, _, err := client.PostMessageContext(
    context.Background(),
    "C0123456",
    slack.MsgOptionText("Build #482 passed", false),
    slack.MsgOptionBlocks(
        slackblocks.NewHeaderBlock().Text("Build #482 passed :white_check_mark:"),
        slackblocks.NewSectionBlock().Fields(
            "*Branch*\n`main`",
            "*Tests*\n1,247 passed",
        ),
        slackblocks.NewDividerBlock(),
    ),
)
```

<p align="center">
  <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/static/img/usage/build_notification.png" alt="The build notification rendered in Slack" width="600px" />
</p>

## Documentation

- **Full docs:** <https://nicklambourne.github.io/slackblocks/>
- [Installation](https://nicklambourne.github.io/slackblocks/usage/installation)
- [Using Blocks](https://nicklambourne.github.io/slackblocks/usage/using_blocks) — every
  block type with code in all three languages, the JSON it produces, and screenshots.
- [Sending Messages](https://nicklambourne.github.io/slackblocks/usage/sending_messages)
- [Recipe Book](https://nicklambourne.github.io/slackblocks/usage/cookbook) — end-to-end
  recipes for build notifications, approval requests, modals, and more.
- [API Reference](https://nicklambourne.github.io/slackblocks/reference) —
  [Python](https://nicklambourne.github.io/slackblocks/reference/python) and
  [TypeScript](https://nicklambourne.github.io/slackblocks/reference/typescript), and
  [Go](https://nicklambourne.github.io/slackblocks/reference/go).
- [Migrating from 1.x](https://nicklambourne.github.io/slackblocks/usage/migration) ·
  [Troubleshooting & FAQ](https://nicklambourne.github.io/slackblocks/usage/troubleshooting)
- Changelogs: [Python](python/CHANGELOG.md) · [TypeScript](typescript/CHANGELOG.md) · [Go](go/CHANGELOG.md)
- [Roadmap](ROADMAP.md) — including the TypeScript legacy API removal planned for v3.0.

## Repository layout

- [`python/`](python/) — the established Python package (`slackblocks` on PyPI).
- [`typescript/`](typescript/) — the TypeScript package (`@nicklambourne/slackblocks` on npm).
- [`go/`](go/) — the Go v2 module (`github.com/nicklambourne/slackblocks/go/v2`).
- [`spec/`](spec/) — the shared conformance contract: fixtures, invalid cases, limits,
  and capability coverage that all three implementations are tested against.
- [`docs/`](docs/) — the Docusaurus documentation site.

## Licensing

`slackblocks` is dual-licensed under [MIT](LICENSE) and
[BSD-3-Clause](LICENSE.BSD-3-Clause). Use whichever fits your project — this makes it
safe to vendor into projects under either license.

## Contributing

Contributions are welcome. Python development uses [uv](https://docs.astral.sh/uv/) from
`python/`; TypeScript and the docs site use [pnpm](https://pnpm.io/) from the repository
root:

```bash
git clone https://github.com/nicklambourne/slackblocks.git

cd slackblocks/python
uv sync --group dev
uv run pytest test/unit test/conformance test/docs

cd ..
pnpm install
pnpm --filter @nicklambourne/slackblocks test

cd go
go test -race -cover ./...
```

For the full development guide — testing conventions, the conformance-fixture workflow,
docstring style, and the release process — see the
[Contributing page](https://nicklambourne.github.io/slackblocks/contributing).

Bug reports and feature requests: <https://github.com/nicklambourne/slackblocks/issues>.
