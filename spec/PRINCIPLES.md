# slackblocks implementation principles

1. Idiomatic APIs take priority over uniform APIs. Names, casing, and construction patterns may differ by language.
2. Serialized output must be semantically identical for every shared valid fixture.
3. Validation outcomes must agree on rejection and error category; concrete error types and messages are language-specific.
4. Feature gaps must be explicit in a checked-in conformance skip list.
5. Every implementation must declare the spec version it conforms to: TypeScript exports `specVersion` and Python exports `slackblocks.SPEC_VERSION`, and both conformance harnesses assert the declared version matches `manifest.json`.

## Documented policies and intentional divergences

- **Character limits count Unicode code points** in both implementations. Python's `len` counts code points natively; TypeScript counts code points explicitly (not UTF-16 units). The corpus pins this with astral-plane emoji fixtures at and beyond the limits.
- **`block_id` defaulting diverges deliberately.** Python auto-generates a UUID `block_id` when one is not supplied (long-standing behavior that downstream users rely on), while TypeScript omits `block_id` entirely when unset. Shared fixtures therefore pin explicit `block_id` values so both implementations render identical JSON. This divergence is intentional and preserved for backwards compatibility.
