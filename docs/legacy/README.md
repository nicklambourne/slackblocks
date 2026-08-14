# Legacy documentation snapshots

This directory freezes the contract for the 22 historical (MkDocs-era)
documentation versions that were ported into Docusaurus.

## What lives here

- `manifest.json` — the single source of truth. One entry per published
  historical version: its git tag and commit, the tree hashes of its
  `docs_src/` and `slackblocks/` sources, its canonical route list, and the
  hash of the generated snapshot. `migration.tool_version` records the
  converter version (`TOOL_VERSION` in `docs/scripts/port_legacy_docs.py`)
  that produced the committed snapshots.
- `inventories/v*.json` — per-version inventories scraped from the old
  published gh-pages site (titles, headings, anchors, images, links, code
  block counts). The converter and checkers use them to prove the port lost
  nothing.
- `fixtures/v1.0.0.tar.gz` — a frozen copy of the v1.0.0 `docs_src/` and
  `slackblocks/` sources. `port_legacy_docs.py fixture` generates the version
  twice from this archive and byte-compares the results to prove the
  converter is deterministic without needing the git tags.

The generated output lives in `docs/versioned_docs/version-*/`,
`docs/versioned_sidebars/version-*-sidebars.json`, and
`docs/static/img/legacy/<docs-tree-hash>/`. All of it is committed; CI
re-derives it from the git tags and byte-compares.

## When the snapshot byte-compare fails in CI

CI (`check:legacy-snapshots`, then `git diff --exit-code`) fails when the
committed snapshots no longer match what the converter re-derives — for
example after a `griffe` version bump changes signature rendering, after a
converter change, or if a release tag was moved (tag moves also fail the
`tag_commit` assertions and should be treated as an incident, not
regenerated over).

To re-derive, review, and re-commit:

```bash
# 1. If the converter's OUTPUT changed (converter edit, griffe bump, etc.),
#    bump TOOL_VERSION in docs/scripts/port_legacy_docs.py so the pages'
#    provenance comments stay truthful. Skip the bump only when the inputs
#    were wrong (e.g. a corrupted checkout) and the output is unchanged.

# 2. Regenerate every snapshot (rewrites versioned_docs, sidebars, legacy
#    assets, manifest hashes, and versions.json):
pnpm --filter @slackblocks/docs generate:legacy

# 3. Audit the diff — every changed line must trace to the change that
#    triggered regeneration. Historical content is immutable; unexpected
#    churn means the converter (or environment) is wrong.
git diff --stat docs/versioned_docs docs/static/img/legacy docs/legacy

# 4. Verify determinism and reproducibility locally:
pnpm --filter @slackblocks/docs test:legacy-fixture
pnpm --filter @slackblocks/docs check:legacy-snapshots
pnpm --filter @slackblocks/docs build

# 5. Commit the regenerated trees together with the converter change.
```

## Forward compatibility

`docs/versions.json` must **end with** the registered legacy versions in
manifest order; newer, non-legacy versions created with
`docusaurus docs:version <x.y>` may precede them. Both
`port_legacy_docs.py` (which preserves any non-legacy prefix when it
rewrites `versions.json`) and `check_legacy_docs.mjs` (which asserts the
suffix rule) derive all counts from `manifest.json` — nothing is hardcoded,
so adding a new current-docs version does not require touching this
directory.
