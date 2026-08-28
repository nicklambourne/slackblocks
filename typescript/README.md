# `@nicklambourne/slackblocks` <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/static/img/sb.png" align="right" width="250px"/>

![Licence: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Licence: BSD-3-Clause](https://img.shields.io/badge/License-BSD_3_Clause-green.svg)
![Node Versions](https://img.shields.io/node/v/%40nicklambourne%2Fslackblocks)
[![npm](https://img.shields.io/npm/v/%40nicklambourne%2Fslackblocks?color=CB3837&label=npm&logo=npm)](https://www.npmjs.com/package/@nicklambourne/slackblocks)
[![Downloads](https://img.shields.io/npm/dm/%40nicklambourne%2Fslackblocks)](https://www.npmjs.com/package/@nicklambourne/slackblocks)
[![Build Status](https://github.com/nicklambourne/slackblocks/actions/workflows/typescript.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)
[![Docs](https://img.shields.io/badge/Docs-8A2BE2.svg)](https://nicklambourne.github.io/slackblocks)

> **Build Slack messages in TypeScript — without writing JSON by hand.**

`slackblocks` is a typed, validating TypeScript wrapper around the Slack
[Block Kit API](https://docs.slack.dev/block-kit/). It exists because Block Kit JSON is
verbose, easy to get subtly wrong, and unpleasant to maintain in source control.

This release conforms to the shared cross-language slackblocks specification.

## Why `slackblocks`?

- **Concise** — `SectionBlock().text("Hello, *world*!").build()` instead of a 10-line JSON object.
- **Validated** — character limits, required fields, mutually-exclusive options, and
  element-type restrictions are enforced at construction time, with a typed error
  hierarchy (`LengthError`, `MissingRequiredError`, …), so you find out *before* hitting
  Slack's API.
- **Typed** — chainable setters reject typo'd properties and invalid values at compile time;
  `.build()` returns the plain Slack-shaped object, ready for the wire.
- **Drop-in compatible** with the official [`@slack/web-api`](https://www.npmjs.com/package/@slack/web-api)
  and [Bolt](https://www.npmjs.com/package/@slack/bolt) — pass a `Message().build()` payload
  straight to `client.chat.postMessage(payload)`.
- **The same library in two languages** — the Python
  [`slackblocks`](https://pypi.org/project/slackblocks/) shares the blocks, the
  validation rules, and the version number; a shared conformance corpus keeps both
  emitting the same Slack JSON.
- **Self-contained** — a single ESM module with no runtime imports.

## Installation

```bash
npm install @nicklambourne/slackblocks
```

Requires Node `20.19+` or `22.12+`. The package ships as ESM; CommonJS consumers
can load it with dynamic `import()`.

## Quickstart

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

`payload` can be sent in one line with the official Slack SDK:

```ts
import { WebClient } from "@slack/web-api";

const client = new WebClient(process.env.SLACK_API_TOKEN);
await client.chat.postMessage(payload);
```

<p align="center">
  <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/static/img/usage/build_notification.png" alt="The build notification rendered in Slack" width="600px" />
</p>

Builders use idiomatic camelCase setters and return plain Slack-shaped objects with
snake_case keys from `.build()`. Validation runs when the complete object is built;
pass `{ validate: false }` to `.build()` only when Slack has moved ahead of this
package's limits registry.

## Documentation

- **Full docs:** <https://nicklambourne.github.io/slackblocks/>
- [Installation](https://nicklambourne.github.io/slackblocks/usage/installation)
- [Using Blocks](https://nicklambourne.github.io/slackblocks/usage/using_blocks) — every
  block type with code in both languages, the JSON it produces, and screenshots.
- [Sending Messages](https://nicklambourne.github.io/slackblocks/usage/sending_messages)
- [Recipe Book](https://nicklambourne.github.io/slackblocks/usage/cookbook) — end-to-end
  recipes for build notifications, approval requests, modals, and more.
- [TypeScript API Reference](https://nicklambourne.github.io/slackblocks/reference/typescript)
- [Troubleshooting & FAQ](https://nicklambourne.github.io/slackblocks/usage/troubleshooting)
- [Changelog](https://github.com/nicklambourne/slackblocks/blob/master/typescript/CHANGELOG.md)
- [Roadmap](https://github.com/nicklambourne/slackblocks/blob/master/ROADMAP.md)

## Licensing

`slackblocks` is dual-licensed under
[MIT](https://github.com/nicklambourne/slackblocks/blob/master/LICENSE) and
[BSD-3-Clause](https://github.com/nicklambourne/slackblocks/blob/master/LICENSE.BSD-3-Clause).
Use whichever fits your project — this makes it safe to vendor into projects under
either license.

## Contributing

Contributions are welcome. The package lives in the `typescript/` directory of the
[slackblocks monorepo](https://github.com/nicklambourne/slackblocks) and uses
[pnpm](https://pnpm.io/):

```bash
git clone https://github.com/nicklambourne/slackblocks.git
cd slackblocks
pnpm install
pnpm --filter @nicklambourne/slackblocks test
```

For the full development guide — testing conventions, the conformance-fixture workflow,
and the release process — see the
[Contributing page](https://nicklambourne.github.io/slackblocks/contributing).

Bug reports and feature requests: <https://github.com/nicklambourne/slackblocks/issues>.
