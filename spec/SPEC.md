# slackblocks conformance specification

Version 1.0.0 defines the language-neutral contract for slackblocks implementations.

## Valid fixtures

`fixtures/valid/` contains canonical Slack Block Kit JSON. Every entry in `manifest.json` must be constructed through an implementation's public API and compared as parsed JSON. Object key order and whitespace are not significant; array order and values are significant.

## Invalid cases

`fixtures/invalid/manifest.json` names invalid constructions and their required error category. Each language owns the code that attempts the construction. Implementations may use language-native error types and messages.

The normative categories are `length-exceeded`, `out-of-range`, `mutually-exclusive`, `type-mismatch`, `missing-required`, and `invalid-usage`.

## Coverage and skips

Each implementation must either exercise every valid fixture and invalid case or list its ID in `conformance/skiplist.txt` with a reason. Conformance fails for unknown entries, unlisted failures, and stale skips that now pass.

`coverage.json` lists every JSON-producing capability in the shared public API and the valid fixtures that exercise it. Every referenced fixture must be registered in `manifest.json` and link to the official Slack documentation used to validate its shape. Adding a shared capability requires adding it to this registry.

## Limits

`limits.json` is the shared registry for stable scalar constraints. Changes to limits and fixtures must land atomically with implementation updates or explicit skip-list entries.

Every scalar leaf in `limits.json` must have a corresponding invalid case whose `constraint` is the leaf's dotted path. Structural rules that are not scalar limits may have additional invalid cases.
