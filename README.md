# slackblocks

Build validated Slack Block Kit payloads in Python or TypeScript from one cross-language conformance contract.

| Implementation | Package | Version |
|---|---|---:|
| Python | [`slackblocks` on PyPI](https://pypi.org/project/slackblocks/) | 2.0.0 |
| TypeScript / JavaScript | `slackblocks` on npm | 2.0.0 (unpublished) |

Both implementations are handwritten and idiomatic. The shared corpus under [`spec/`](spec/) guarantees semantically identical Slack JSON and validation categories.

## Quickstart

Python:

```python
from slackblocks import Message, SectionBlock

payload = Message(channel="#general", blocks=SectionBlock("Hello, world!"))
```

TypeScript:

```ts
import { message, sectionBlock } from "slackblocks";

const payload = message({
  channel: "C0123456",
  blocks: [sectionBlock({ text: "Hello, world!" })],
});
```

## Repository layout

- `python/` — the established Python package.
- `typescript/` — the ESM TypeScript package.
- `spec/` — 96 valid fixtures, shared invalid cases, capability coverage, and limits.
- `docs/` — the Docusaurus documentation site.

See the [documentation site](https://nicklambourne.github.io/slackblocks/) for installation and usage.

## Development

Run Python checks from `python/` with uv. Run TypeScript and docs checks from the repository root with pnpm. The full commands and fixture workflow are in the [contributing guide](https://nicklambourne.github.io/slackblocks/contributing).
