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

Rust joins the implementation set in 2.6.0. Include `rust/`
when updating shared features. Run its generator with Rust 1.85.0 rustfmt, then
native/conformance/API, compile-contract, coverage and extracted-package checks.
Use `.github/workflows/rust.yml` on GitHub-provided runners. Rust publication
uses the documented registry environment and coordinated release gates.

PHP joins the implementation set in 2.7.0. Include `php/` when updating shared
features. Use PHP 8.2+ on 64-bit platforms, named constructors, readonly values,
and backed enums. Run generator, native/conformance/API, PHPStan contracts,
coverage, actual Composer archive, sending integrations and documentation checks.
Keep the Packagist publisher disabled until the protected environment and PHP
VCS distribution repository have been verified; see `RELEASING.md`.
