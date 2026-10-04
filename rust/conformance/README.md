# Rust conformance and public API audit

This development-only workspace package is excluded from the published crate.
Its tests can read the shared specification; distributable tests cannot.

- `src/valid.rs` contains checked-in, ordinary Rust constructions. Tests never
  construct these values by parsing the expected fixture or interpreting the model.
- `src/invalid.rs` contains native attempts and separate checked-ingress inputs.
  The two wrong-role cases use ingress because the native API rejects their types
  at compile time; compile-fail companions cover that distinction.
- Both registries must match the shared manifests exactly, including duplicate,
  missing, stale and on-disk fixture checks. The skiplist must remain empty.
- `capabilities.json` maps concrete Rust values to every shared capability.
- `api-inventory.json` records the actual facade and public method signatures,
  parsed independently using Syn. `api-coverage.json` records coverage decisions
  for every export, including handwritten helpers and supporting APIs.
- Scalar and vocabulary tests compare exact source tables with the shared files.

When an intentional public API change occurs, review the output of
`cargo run --manifest-path rust/Cargo.toml -p slackblocks-conformance --bin api_inventory`
against the old inventory, add tests and a coverage decision, then update the
checked-in inventory. The library generator never edits these audit registries.
Counts are useful reporting evidence; exact ID and key sets are the test oracle.
