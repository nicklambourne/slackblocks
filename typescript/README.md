# `@nicklambourne/slackblocks`

Typed Block Kit construction with eager, path-aware validation and conformance with the shared cross-language slackblocks specification.

```bash
npm install @nicklambourne/slackblocks
```

```ts
import {
  button,
  dividerBlock,
  message,
  mrkdwn,
  sectionBlock,
} from "@nicklambourne/slackblocks";

const payload = message({
  channel: "C0123456",
  blocks: [
    sectionBlock({
      text: mrkdwn("*Deploy complete* :rocket:"),
      accessory: button({
        text: "View logs",
        actionId: "logs",
        url: "https://example.com/logs",
      }),
    }),
    dividerBlock(),
  ],
});
```

Factory inputs use idiomatic camelCase and return plain Slack-shaped objects with snake_case keys. Factories validate eagerly; pass `{ validate: false }` as the final argument for Slack features that have moved ahead of this package's limits registry.
