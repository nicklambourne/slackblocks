# slackblocks conformance specification

Version 1.1.0 defines the language-neutral contract for slackblocks implementations.

## Valid fixtures

`fixtures/valid/` contains canonical Slack Block Kit JSON. Every entry in `manifest.json` must be constructed through an implementation's public API and compared as parsed JSON. Every harness enforces this generatively: each maps every manifest fixture ID to a construction through its public API and fails for unregistered IDs and for constructions whose rendered JSON differs from the fixture. Object key order and whitespace are not significant; array order and values are significant.

## Invalid cases

`fixtures/invalid/manifest.json` names invalid constructions and their required error category. Each language owns the code that attempts the construction. Implementations may use language-native error types and messages.

Payload-level cases also enforce the official 50-block message limit, the
100-attachment message limit, block compatibility for message, modal, and App
Home surfaces, and the requirement for submit text when a modal contains an
input block.

The normative categories are `length-exceeded`, `out-of-range`, `mutually-exclusive`, `type-mismatch`, `missing-required`, and `invalid-usage`.

## Coverage

`coverage.json` lists every JSON-producing capability in the shared public API and the valid fixtures that exercise it. Every referenced fixture must be registered in `manifest.json` and link to the official Slack documentation used to validate its shape. The registry is enforced against the packages' exports: each harness enumerates its language's public JSON-producing symbols and fails unless every one maps to a registered capability or appears in an explicit, commented exclusions list (errors, enums, abstract bases, and utility helpers). Adding a shared capability therefore requires adding it to this registry and giving it a fixture.

## Skip lists

Each implementation keeps a `conformance/skiplist.txt`. In any released state every skip list must be empty, and every harness hard-asserts emptiness. Entries are permitted only as a temporary escape hatch while a spec change and its implementation updates land across multiple pull requests, and must be removed before release.

## Model

`model.json` describes every value type the generated implementations expose: its Slack wire type, fields and their kinds, required fields, validation rules, descriptions, and Slack documentation links, with limits referenced by their dotted path in `limits.json`. Java and C# generate their value types from it, and Go generates the documentation on its builder methods; `java/generator/generate_models.py --check-go` fails if the Go builder registry and the model disagree. Each generator's output is checked in and regenerated in CI, so a model change lands in every generated implementation in the same pull request.

## Limits

`vocabulary.json` is the normative registry for Slack vocabularies that validation checks by name: the Slack-provided icon names accepted by icon objects, and the block types each surface (`message`, `modal`, `home`) accepts. Java and C# generate their tables from this file. Python, TypeScript, and Go keep native tables and each has a test that fails when its tables differ from this file, so a vocabulary change lands in every implementation at once.

`limits.json` is the normative registry for stable scalar constraints. Changes to limits and fixtures must land atomically with implementation updates (or a temporary skip-list entry during a multi-PR transition).

Every scalar leaf in `limits.json` must have a corresponding invalid case whose `constraint` is the leaf's dotted path, so every implementation demonstrably rejects values beyond each limit. Structural rules that are not scalar limits may have additional invalid cases. Implementations may hardcode limit values internally; the shared invalid-case corpus is what pins cross-language agreement.

Character limits count Unicode code points in every implementation; the corpus includes astral-plane fixtures that pin this.
