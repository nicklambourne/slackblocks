# slackblocks for TypeScript

Typed Block Kit construction with eager, path-aware validation and conformance with slackblocks spec 1.0.0.

```ts
import { button, divider, message, mrkdwn, section } from "slackblocks";

const payload = message({
  channel: "C0123456",
  blocks: [
    section({
      text: mrkdwn("*Deploy complete* :rocket:"),
      accessory: button({
        text: "View logs",
        actionId: "logs",
        url: "https://example.com/logs",
      }),
    }),
    divider(),
  ],
});
```

Factory inputs use idiomatic camelCase and return plain Slack-shaped objects with snake_case keys. Factories validate eagerly; pass `{ validate: false }` as the final argument for Slack features that have moved ahead of this package's limits registry.
