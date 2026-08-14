# slackblocks conformance specification

Version 1.0.1 defines the language-neutral contract for slackblocks implementations.

## Valid fixtures

`fixtures/valid/` contains canonical Slack Block Kit JSON. Every entry in `manifest.json` must be constructed through an implementation's public API and compared as parsed JSON. Both harnesses enforce this generatively: each maps every manifest fixture ID to a construction through its public API and fails for unregistered IDs and for constructions whose rendered JSON differs from the fixture. Object key order and whitespace are not significant; array order and values are significant.

## Invalid cases

`fixtures/invalid/manifest.json` names invalid constructions and their required error category. Each language owns the code that attempts the construction. Implementations may use language-native error types and messages.

The normative categories are `length-exceeded`, `out-of-range`, `mutually-exclusive`, `type-mismatch`, `missing-required`, and `invalid-usage`.

## Coverage

`coverage.json` lists every JSON-producing capability in the shared public API and the valid fixtures that exercise it. Every referenced fixture must be registered in `manifest.json` and link to the official Slack documentation used to validate its shape. The registry is enforced against the packages' exports: each harness enumerates its language's public JSON-producing symbols and fails unless every one maps to a registered capability or appears in an explicit, commented exclusions list (errors, enums, abstract bases, and utility helpers). Adding a shared capability therefore requires adding it to this registry and giving it a fixture.

## Skip lists

Each implementation keeps a `conformance/skiplist.txt`. In any released state both files must be empty, and both harnesses hard-assert emptiness. Entries are permitted only as a temporary escape hatch while a spec change and its implementation updates land across multiple pull requests, and must be removed before release.

## Limits

`limits.json` is the normative registry for stable scalar constraints. Changes to limits and fixtures must land atomically with implementation updates (or a temporary skip-list entry during a multi-PR transition).

Every scalar leaf in `limits.json` must have a corresponding invalid case whose `constraint` is the leaf's dotted path, so both implementations demonstrably reject values beyond each limit. Structural rules that are not scalar limits may have additional invalid cases. Implementations may hardcode limit values internally; the shared invalid-case corpus is what pins cross-language agreement.

Character limits count Unicode code points in both implementations; the corpus includes astral-plane fixtures that pin this.
