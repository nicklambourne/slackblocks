# slackblocks for Rust

[![Rust MSRV](https://img.shields.io/badge/Rust-1.85%2B-b7410e?logo=rust)](https://nicklambourne.github.io/slackblocks/usage/compatibility?language=rust)
[![crates.io](https://img.shields.io/badge/crates.io-2.6.0%20unreleased-b7410e?logo=rust)](#use-from-a-checkout)
[![Rust CI](https://github.com/nicklambourne/slackblocks/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions/workflows/rust.yml)
[![Docs](https://img.shields.io/badge/Docs-8A2BE2.svg)](https://nicklambourne.github.io/slackblocks/reference/rust)

An unreleased Rust implementation of Slack Block Kit, targeting coordinated
version 2.6.0 and shared specification 1.2.0. Registry publication is disabled.

Rust 2024; minimum supported Rust version (MSRV) 1.85. Values own their data,
builders consume themselves, getters borrow, and invalid input returns
`ValidationError`. The core depends only on Serde and serde_json.

All shared model types are implemented. Independent native examples and checked
JSON ingress cover every shared valid fixture and invalid case.
See [the implementation plan](https://github.com/nicklambourne/slackblocks/blob/master/rust/IMPLEMENTATION.md) for the train and acceptance evidence.

## Use from a checkout

The Rust crate is not yet published. In an application beside this repository:

```sh
cargo add slackblocks --path ../slackblocks/rust
cargo add serde_json
```

```rust
use slackblocks::{ButtonElement, MessagePayload, SectionBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let button = ButtonElement::builder().text("Open build").action_id("open_build").build()?;
    let section = SectionBlock::builder().text("Build passed").accessory(button).build()?;
    let message = MessagePayload::builder("C01234567")
        .text("Build passed")
        .block(section)
        .build()?;
    let edited = message.clone().into_builder().text("Build passed on main").build()?;
    assert_eq!(message.text(), Some("Build passed"));
    println!("{}", serde_json::to_string(&edited)?);
    Ok(())
}
```

## Native API conventions

- Builders consume themselves; `build()` returns `Result<T, ValidationError>`.
  Built values have private fields, own their data, and expose borrowed getters.
- Clone a value before `into_builder()` when both versions are needed.
  Plural collection setters replace; singular setters append. Iterators are accepted.
- Concrete children convert into role enums with `From`; unsupported roles fail
  at compile time. Vocabulary and role enums are `#[non_exhaustive]`.
- `SelectOption`, `SelectOptionGroup`, and `ConfirmationDialogue` avoid ambiguous
  Rust names. No SDK class hierarchy is copied into the crate.
- Strings coerce according to each text field. Use `PlainText` or `MarkdownText`
  explicitly when you need flags or a particular allowed text kind.
- Fresh builders use documented model defaults. Checked parsing and editing
  preserve omitted fields; `clear_*()` removes optional defaults. Allowed false,
  zero and empty values remain present. Block IDs are omitted unless supplied.
- Every modeled value supports Serde serialization and checked deserialization,
  plus `TryFrom<serde_json::Value>` for structured `ValidationError` access.
  This is outbound Block Kit parsing, not an arbitrary Slack event decoder.
- Extensions preserve unknown JSON but reject empty keys, modeled fields and
  `type` collisions. `BuilderPayload::Raw` is an explicit unvalidated preview input.
- `JsonNumber` preserves `i64`/`u64` integers and finite floating-point values;
  unsupported precision and nonfinite inputs are rejected. Text lengths count
  Unicode code points.

## Helpers and transport

`AccordionSection` and `Accordion` compose collapsible containers. `Paginator`
uses one-based pages and `.previous`/`.next` action IDs; applications handle the
interaction and render the requested page. Both expand into ordinary `Block`
values, so the receiving message/view validates aggregate constraints.

`Workflow::from_url` accepts ordered input parameters and omits an empty parameter
list. `AttachmentColor` supports named constants and checked hexadecimal parsing.
`block_kit_builder_url` takes borrowed blocks/payloads and creates an encoded URL.

Serialize a `MessagePayload`, `WebhookMessage` or `MessageResponse` through Serde
and hand it to an application-owned HTTP client. `ModalView` and `HomeTabView`
belong inside the appropriate Slack request envelope. The crate makes no network
requests and imposes no async runtime. Check Slack's JSON `ok`/`error` response as
well as HTTP status. Python's deprecated legacy attachment `Field` is outside the
shared contract and is deliberately not reproduced.

## Support and reference

MSRV is 1.85.0; raising it requires an explicit compatibility decision and release
notes. CI tests minimum Rust and stable, plus stable on Linux, macOS and Windows.
Serde/serde_json use compatible version requirements. Fresh consumers test the
declared dependency lower bounds and serde_json feature unification independently
of the development lockfile.

Rust support starts at coordinated 2.6.0; no earlier Rust API migration is needed.
Read the [guides](https://nicklambourne.github.io/slackblocks/quick-start?language=rust)
and [API reference](https://nicklambourne.github.io/slackblocks/reference/rust),
or run `cargo doc --manifest-path rust/Cargo.toml --no-deps --open` for native
rustdoc and trait implementations.

## Development

Install stable Rust and Rust 1.85.0 with rustfmt and Clippy, plus Python 3.11+ for
the development-only generator. The package itself does not need Python.
Formatting is pinned to Rust 1.85.0; current stable owns Clippy and rustdoc checks.

```sh
export RUSTFMT="$(rustup which --toolchain 1.85.0 rustfmt)"
python3 rust/generator/generate.py
rust/bin/check
python3 rust/bin/check-package.py --toolchain 1.85.0
python3 rust/bin/check-package.py --toolchain stable
python3 rust/bin/check-package.py --toolchain 1.85.0 --lower-bounds
python3 rust/bin/check-package.py --toolchain 1.85.0 --json-features
```

CI runs stable on Linux, macOS, and Windows and the minimum toolchain on Linux.
Package checks extract the actual Cargo archive and compile fresh consumers
against that extracted artifact, without access to the checkout's Rust sources.

Quality checks include independent API and contract audits, property tests, and
negative compile checks with a passing companion. CI enforces at least 90% line
and 90% region coverage independently for the whole crate and handwritten code:

```sh
rustup component add llvm-tools-preview --toolchain stable
cargo +stable install cargo-llvm-cov --version 0.9.1 --locked
python3 rust/bin/check-coverage.py
```

Only external code, test/example harnesses, and files containing no executable
regions are excluded. Coverage is measured on stable Rust; it is not a branch
coverage claim. The report also exercises consumer-enabled serde_json features.
