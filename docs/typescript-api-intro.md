# TypeScript API reference

This is the complete guide to slackblocks' public TypeScript API. Use it to find the builder, setter, or validation error you need while building a message, modal, or home tab.

Most applications start with `Message()`, add blocks such as `SectionBlock()`,
and call `.build()` once the payload is complete. Singular setters replace the
previous value, collection setters append, and nested builders are materialized
automatically. The result is the plain snake_case object expected by Slack.

The compatibility factories from earlier TypeScript releases remain available
at the package root for v2 applications, but are intentionally omitted here.
New code should use the fluent PascalCase API.
