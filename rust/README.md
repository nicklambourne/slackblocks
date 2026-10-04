# slackblocks for Rust

An unreleased Rust implementation of Slack Block Kit, targeting coordinated
version 2.6.0 and shared specification 1.2.0. Registry publication is disabled.

Rust 2024; minimum supported Rust version (MSRV) 1.85. Values own their data,
builders consume themselves, getters borrow, and invalid input returns
`ValidationError`. The core depends only on Serde and serde_json.

All shared model types are implemented. Independent native examples and checked
JSON ingress cover every shared valid fixture and invalid case.
See [the implementation plan](https://github.com/nicklambourne/slackblocks/blob/master/rust/IMPLEMENTATION.md) for the train and acceptance evidence.

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
