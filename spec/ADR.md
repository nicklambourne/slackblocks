# ADR 0001: spec-as-tests, idiomatic implementations

- **Status:** accepted
- **Date:** 2026-08-12

## Context

slackblocks now supports Python and TypeScript. Their public APIs should follow each language's conventions, while users must receive the same Slack JSON and validation outcomes.

## Decision

Keep both implementations handwritten in one monorepo. Treat the versioned JSON corpus, invalid-case categories, and limits registry in `spec/` as the shared contract. `spec/limits.json` is normative: every scalar leaf forces an invalid case that both conformance harnesses must reject with the required category, so the limits are test-enforced in both languages rather than aspirational documentation. Each language owns a conformance harness and a checked-in skip list that must be empty in released states. Releases and semantic versions remain independent and declare the spec version they implement.

Do not generate public APIs. Implementations may hardcode the shared scalar constants internally — the invalid-case corpus pins agreement. Reconsider generated internal constants only if at least three maintained languages make scalar drift costly.

## Consequences

Feature changes land atomically across the spec and implementations, or as an explicit skip. API names and construction patterns may differ. The monorepo keeps the docs build and corpus updates local, at the cost of ecosystem-specific tooling in one repository.
