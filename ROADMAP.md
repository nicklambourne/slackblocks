# Roadmap

This roadmap records planned compatibility boundaries rather than promising release
dates. Priorities may move, but removals listed here will not happen before their named
major release.

## 2.2

- Make the fluent PascalCase TypeScript API the default for examples, guides, and API
  reference material.
- Add higher-level components that compose ordinary Block Kit blocks, beginning with
  pagination and Slack-native accordions.
- Keep the lowercase object-input TypeScript factories available only as a deprecated
  compatibility layer under `src/legacy`.

## 3.0

- Remove every deprecated lowercase TypeScript factory and its root-package re-export.
- Remove the `src/legacy` compatibility layer and its compatibility-only tests.
- Retain the fluent PascalCase builders as the sole TypeScript construction API.
- Publish a focused migration guide before the first v3 prerelease. The mechanical
  migration is from one object argument to chained camelCase setters followed by
  `.build()` at the payload boundary.

The legacy API will remain functional throughout the 2.x line, but it will receive only
compatibility fixes needed to avoid regressions. New blocks, components, examples, and
documentation target the fluent API only.
