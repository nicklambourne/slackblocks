# ADR 0001: spec-as-tests, idiomatic implementations

- **Status:** accepted; partly superseded by ADR 0002 (generated public APIs, limit constants, and release versioning)
- **Date:** 2026-08-12

## Context

slackblocks now supports Python and TypeScript. Their public APIs should follow each language's conventions, while users must receive the same Slack JSON and validation outcomes.

## Decision

Keep both implementations handwritten in one monorepo. Treat the versioned JSON corpus, invalid-case categories, and limits registry in `spec/` as the shared contract. `spec/limits.json` is normative: every scalar leaf forces an invalid case that both conformance harnesses must reject with the required category, so the limits are test-enforced in both languages rather than aspirational documentation. Each language owns a conformance harness and a checked-in skip list that must be empty in released states. Releases and semantic versions remain independent and declare the spec version they implement.

Do not generate public APIs. Implementations may hardcode the shared scalar constants internally — the invalid-case corpus pins agreement. Reconsider generated internal constants only if at least three maintained languages make scalar drift costly.

## Consequences

Feature changes land atomically across the spec and implementations, or as an explicit skip. API names and construction patterns may differ. The monorepo keeps the docs build and corpus updates local, at the cost of ecosystem-specific tooling in one repository.

# ADR 0002: shared model, generated APIs, and lockstep releases

- **Status:** accepted
- **Date:** 2026-09-24

## Context

ADR 0001 was written for two hand-written implementations. Since then Go, Java, and C# have joined, and three of its decisions no longer match the repository:

- **Public APIs are generated in two languages.** Java's first model was derived by regex-parsing the Go sources. Fixing that introduced a checked-in model describing every value type's fields, required fields, validation rules, descriptions, and Slack links, now `spec/model.json`. Java and C# generate their public value types from it, and Go generates its builder documentation from it.
- **Limits are sourced inconsistently.** TypeScript imports `limits.json` at runtime. Python, Go, Java, and C# hardcoded their limits; Java and C# used `limits.json` only for generated documentation. ADR 0001 said to reconsider generated constants once three maintained languages made scalar drift costly; there are now five.
- **Releases are lockstep.** Since 2 September 2026 one coordinated workflow tags and publishes every package at the same version, not independent versions.

## Decision

1. **`spec/model.json` is part of the shared contract**, alongside the fixtures, invalid cases, `limits.json`, and `vocabulary.json`. A model change regenerates every generated implementation in the same pull request and must keep its conformance suite passing.
2. **Public APIs may be generated from the model when the generator emits that language's idiomatic API.** Principle 1, idiomatic over uniform, applies equally to generated and hand-written code. Java and C# are generated because their per-type code (builders or constructors, accessors, validation calls, equality, and doc comments) is voluminous and mechanical once the idiom is chosen, and because they arrived after the model existed. Python, TypeScript, and Go stay hand-written because their established public APIs predate the model and regenerating them would churn users for no change in output. A new implementation applies the same test.
3. **Every implementation takes its limit values from `limits.json`:** by direct import where the package can bundle the file (TypeScript), otherwise as checked-in generated constants that CI regenerates and diffs (Python, Go, Java, C#). This supersedes ADR 0001's allowance to hardcode them. Python and Go are fully migrated; Java and C# generate the constants but their validators still hardcode most limits, which is follow-up work.
4. **Packages release in lockstep.** Every package carries the same version and is published together by `.github/workflows/coordinated-release.yml`. Each still declares the spec version it implements, and the harnesses assert it. This supersedes ADR 0001's independent versions.

## Consequences

Generated sources are checked in, and CI regenerates them and fails on any difference. The shared corpus remains the arbiter of cross-language agreement for generated and hand-written implementations alike. Hand-written implementations keep their idiomatic APIs and documented divergences, and implement each new capability by hand in the same pull request as its fixtures, or record it in their skip list. A limit changes by editing `limits.json` and regenerating, not by editing each language.
