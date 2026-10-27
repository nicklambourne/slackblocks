# Repository instructions

This open source repository uses GitHub-provided runners for GitHub Actions. The repository owner has authorized standard hosted runner labels for its workflows. Follow the existing workflow conventions, keep matrix checks stable on unrelated pull requests, and verify the actual runner metadata after a new workflow runs.

## Block Kit changes

When adding or changing a Block Kit type, field, vocabulary, validation rule, or
composition helper, follow [Adding New Block Kit Features](docs/docs/contributing/maintaining-block-kit.mdx).
Start with official Slack evidence and the shared contract; update every
maintained implementation using its native API, regenerate owned outputs, and
verify conformance, exports, documentation and package checks. Do not treat raw
JSON escape hatches or nonempty release skip lists as completed support. Extend
the guide's language and verification lists whenever a new implementation ships.

## New language implementations

Follow [Adding a language implementation](docs/docs/contributing/adding-a-language.mdx).
Review a native API prototype and parity inventory before bulk generation. Cover
the shared contract, useful helpers, independent public API coverage, language
quality gates, standalone packaging, documentation and coordinated releases.
Keep publication disabled until the complete implementation and registry setup
are ready; preserve outgoing documentation before adding new-language content.
Follow the guide's README badge palette, collapsed installation/quickstart
accordions and shared download-total activation checks whenever adding an
implementation.

Rust joins the implementation set in 2.6.0. Include `rust/`
when updating shared features. Run its generator with Rust 1.85.0 rustfmt, then
native/conformance/API, compile-contract, coverage and extracted-package checks.
Use `.github/workflows/rust.yml` on GitHub-provided runners. Rust publication
uses the documented registry environment and coordinated release gates.

PHP joins the implementation set in 2.7.0. Include `php/` when updating shared
features. Use PHP 8.2+ on 64-bit platforms, named constructors, readonly values,
and backed enums. Run generator, native/conformance/API, PHPStan contracts,
coverage, actual Composer archive, sending integrations and documentation checks.
PHP publication uses the monorepo's Packagist registration, GitHub update hook
and protected `packagist` environment; see `RELEASING.md`. Generate
the root Composer descriptor with `php/bin/package_manifest.py` after changing
PHP package metadata. Verify trimmed archives and native source installs.

Elixir joins the implementation set in 2.8.0. Include `elixir/`
when updating shared features. Use immutable structs, keyword constructors,
native typespecs and atom enums. Elixir 1.16.3 is the pinned formatter; the full
`mix run tools/check.exs` gate covers conformance, independent API inventory,
coverage, Dialyzer, actual Hex archives, Erlang releases and isolated HTTP tests.
Current docs execute literal examples and render compiled native metadata.
Keep Hex publication disabled until the protected environment, owner and scoped
credential are verified and the maintainer approves activation; see `RELEASING.md`.

Haskell is prepared for the coordinated 3.1.0 release. Include `haskell/` in
shared changes. Use checked immutable values, native field records and closed
roles; regenerate with Ormolu 0.7.7.0. `python bin/check` covers native fixtures,
compiled public API inventory, compile-fail controls, 95% handwritten HPC
coverage, independent dependency graphs, actual Cabal archives and fresh HTTP
consumers. Docs use GHC signatures, compiled Haddock and literal executable
snippets. Keep Hackage publication and dashboard download activation disabled
until registry ownership, credentials and the complete release are verified.
