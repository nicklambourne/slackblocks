# Rust sending examples

This unpublished application project demonstrates delivery with Slack Morphism
2.29 and reqwest 0.13. Both send validated `slackblocks::MessagePayload` values
through Serde. The core package remains independent of clients and async runtimes.

## Run

Set `SLACK_BOT_TOKEN` (a bot token with `chat:write`) and `SLACK_CHANNEL_ID` to a
channel your bot can post to. Slack Morphism also needs `SLACK_TEAM_ID` for its
workspace rate-control state. Do not put credentials in source files.

From the repository root, **either command sends one real message**:

```sh
cargo run --manifest-path rust/integrations/Cargo.toml --locked --example slack_morphism
cargo run --manifest-path rust/integrations/Cargo.toml --locked --example reqwest
```

Both examples have a 60-second operation deadline and at most two retries for
explicit rate-limit rejection. The reqwest example requires a numeric
`Retry-After`; the SDK owns its own retry policy. A timeout/server failure may
follow an accepted message, so these examples do not retry ambiguous failures.
Reuse clients between calls, and coordinate per-channel sending across processes.

Slack Morphism's generic `http_post` accepts our payload directly and retains its
authentication and configured rate control. Its typed `chat_post_message` expects
SDK block types; converting through them could reject newer blocks or discard
unknown fields. These examples read only the response receipt, so echoed blocks
also do not depend on the SDK model's coverage.

## Compatibility and tests

Rust 1.85 is supported by this project's locked dependencies. Cargo resolver 3
and the manifest's `rust-version` select compatible transitive releases; do not
copy a newer application's lockfile and assume its dependencies support 1.85.
wiremock is pinned to 0.6.2 because newer releases use Rust 1.88 let chains without
declaring that compiler requirement. Dependabot updates must pass the MSRV gate.

When both HTTP clients are linked, their defaults enable different rustls crypto
providers. The SDK example chooses ring unless the application already installed
a provider. A test initializes both production clients to catch this conflict.

```sh
cargo +1.85.0 test --manifest-path rust/integrations/Cargo.toml --all-targets --locked
cargo +stable test --manifest-path rust/integrations/Cargo.toml --all-targets --locked
cargo +1.85.0 fmt --manifest-path rust/integrations/Cargo.toml -- --check
cargo +stable clippy --manifest-path rust/integrations/Cargo.toml --all-targets --locked -- -D warnings
```

Tests import the actual example source and use real clients against local HTTP
mock servers. They check request method/path, bearer auth, content type, exact
JSON including extensions/Unicode/large integers, response parsing, API and HTTP
errors, rate-limit delay/replay/exhaustion, deadlines, and redirects. No test
contacts Slack or requires credentials. Workspace authorization and real delivery
must be verified separately by running an example with your own credentials.

The documentation checker requires exact copies of these tested examples in the
[sending guide](https://nicklambourne.github.io/slackblocks/usage/sending_messages?language=rust).
The examples, HTTP dependencies and test framework are outside the published
core archive and have their own lockfile.
