# slackblocks for Elixir

Native immutable values for constructing validated Slack Block Kit payloads.
The library builds messages; your application chooses the HTTP client that sends them.

**Unreleased implementation:** all 89 shared models and the complete conformance
corpus are implemented. Site documentation, Hex packaging, Erlang consumers and sending integrations are
prepared in this train; release activation remains a separate approval gate. See
[the implementation plan](https://github.com/nicklambourne/slackblocks/blob/master/elixir/design/plan.md) and [API reference](https://nicklambourne.github.io/slackblocks/reference/elixir). No Hex release is available.

## Requirements

Elixir 1.16.3 or newer (below 2.0), with a compatible Erlang/OTP version starting
at 26.2, on 64-bit systems. CI checks Elixir 1.16–1.20 on their compatible OTP versions, with Linux, macOS and Windows endpoint lanes.
Runtime dependencies are Jason and Decimal; HTTP clients and frameworks are optional.

For review, add a path dependency to this directory:

```elixir
{:slackblocks, path: "../slackblocks/elixir"}
```

If this application already compiled Jason before adding slackblocks/Decimal,
refresh Jason's optional Decimal integration in each build environment:

```sh
mix deps.get
mix deps.compile jason --force
mix compile
# Repeat with MIX_ENV=prod before building a production release.
```

Checked decoding requires this integration to preserve decimal tokens. An old
Jason build returns an actionable `ArgumentError` through `decode/2` (`decode!/2`
raises); it never silently switches to lossy float parsing. Fresh builds resolve
both runtime dependencies normally. CI tests an actual pre-Decimal Jason beam,
the diagnostic, these rebuild commands and the resulting consolidated release.

## Quick start

```elixir
alias Slackblocks.Message
alias Slackblocks.Blocks.Section
alias Slackblocks.Elements.Button

button = Button.new!(text: "View build", action_id: "view_build", url: "https://example.com/build/42")
section = Section.new!(text: "*Build complete*", accessory: button)
message = Message.new!(channel: "C123", text: "Build complete", blocks: [section])
wire = Slackblocks.to_map!(message)
# Send `wire` as JSON using your chosen Slack client.
```

`new/1` and `change/2` return `{:ok, value}` or `{:error, ValidationError}`;
the corresponding bang functions raise. Use keyword options and ordinary lists.
Fields are public for inspection and pattern matching. `change/2` validates a
complete replacement atomically, and values remain immutable:

```elixir
updated = Section.change!(section, text: "*Build deployed*", block_id: "build")
```

A text field may accept a binary in its constructor, converting it to the field's
plain or markdown text type. Direct struct edits must use the stored typed value.
Every map/JSON output revalidates the graph, including directly edited structs.
Error `category` is an atom; `path` contains wire keys and zero-based indexes.

## JSON and defaults

- `Slackblocks.to_map/1` produces a string-keyed Slack map. `encode/1` produces JSON.
- Models implement `Jason.Encoder`. Direct `Jason.encode/1` and `encode!/1` may
  raise `Slackblocks.ValidationError`; use `Slackblocks.encode/1` for tagged errors.
- `from_map/2` and `decode/2` accept a model module or a documented receiving role
  such as `:block`, `:element` or `:text`. They reconstruct modeled payloads;
  they are not parsers for arbitrary Slack events.
- Checked decoding rejects duplicate keys, unknown modeled fields, and decimal
  values that cannot survive the shortest-decimal float round trip. Arbitrary-size
  integer tokens are preserved. Integer fields reject decimal/exponent tokens.
  Negative zero is a float; decimal scale is not preserved.
- Map ingress cannot recover precision or duplicate keys lost by a previous parser.
- Constructors apply declared defaults. Parsing preserves absent optional fields
  as `nil`; `change/2` retains omitted keys, while explicit `nil` clears an optional
  field. False, zero, empty JSON objects, arrays and metadata nulls are preserved.
- `Message` defaults to `text: ""` and `mrkdwn: true`. No block IDs are generated.
- Limits count Unicode code points, not graphemes or UTF-8 bytes.
- Pending task cards can be constructed for use inside a plan. Standalone output
  rejects them; a plan serializes its tasks without their block discriminator.
- Whole-message rules require a `Message` or view wrapper. A model inside an
  arbitrary application map cannot validate the surrounding map as a Slack payload.

Adding optional fields or enum members is additive. Match the fields you need and
include catch-all branches for evolving enums. Persist wire JSON rather than BEAM
struct terms across package upgrades.

## Composition helpers

```elixir
alias Slackblocks.Components.Paginator
alias Slackblocks.{Attachment, Color}

page = Paginator.to_blocks!([section], action_id_prefix: "builds", page: 1, page_size: 5)
attachment = Attachment.new!(blocks: page, color: Color.named!(:purple))
preview = Slackblocks.block_kit_builder_url!(message, team_id: "T123")
```

`AccordionSection.new/1` returns a collapsible container; `Accordion.to_blocks/1`
checks and expands the section list. `Paginator.to_blocks/2` returns ordinary
blocks with explicit navigation IDs. Put expanded blocks into the final message
or view so its total block limit is checked. These helpers do not manage state or
route interactions.

`Objects.Workflow.from_url/2` builds a trigger from its URL and an optional list of
`Objects.InputParameter` values. `RichText.Style` holds optional boolean flags;
receiving fields enforce their allowed flags. `Color` supports semantic/named
colors, six-digit hex strings and RGB channels. Each helper has tagged and bang
variants; see ExDoc for accepted options and defaults.

## Erlang

The same BEAM modules are callable from Erlang:

```erlang
{ok, Section} = 'Elixir.Slackblocks.Blocks.Section':new([{text, <<"Hello from Erlang!">>}]),
{ok, Wire} = 'Elixir.Slackblocks':to_map(Section).
```

Use UTF-8 binaries and keyword tuples. The Erlang consumer requires Elixir runtime
libraries and the declared package dependencies. The [Erlang guide](https://nicklambourne.github.io/slackblocks/usage/erlang) covers
the tested Rebar3/Mix configuration and standalone OTP release; successful direct calls alone
are not a promise that every Erlang build configuration works.

## Development

From this directory, with Elixir 1.16.3, OTP 26.2+, Python 3.11+, Ruff 0.15.12,
Hex 2.5.1 and Rebar3 3.27.1 installed:

```sh
mix deps.get
mix run tools/check.exs
```

The checker verifies deterministic generation, formatting, compilation,
native and full conformance tests, lint, ExDoc and an actual Hex archive installed into an independent
consumer. `--profile runtime` checks native and archive behavior on another
compiler without changing the pinned formatter. The full profile includes Dialyzer, doctests, properties, a separate 90% handwritten
coverage gate, API/coverage mutation probes, credential-free HexDocs packaging and
a standalone Erlang release. It also runs the separately resolved sending examples in locked, lower-bound and
latest-compatible profiles. `--profile runtime` includes locked sending checks;
`--profile native` is the narrower native/archive lane used by CI alongside its
separate sending jobs.

On the maintainer's Mac, keep durable build/dependency caches under `/Volumes/Cache`
and disposable consumers under `/Volumes/Scratch`. The package does not need these
paths. The consumer checker detects the external volumes; CI uses its temporary
directory. Override `MIX_BUILD_ROOT`, `MIX_DEPS_PATH`, `MIX_HOME` and `HEX_HOME` as
appropriate for your host. `SLACKBLOCKS_PLT_PATH` selects a runtime-specific durable
Dialyzer cache, and `SLACKBLOCKS_RUFF` can select an installed Ruff executable.

Generated modules and schema live under `lib/slackblocks`; their first comment
identifies their owner. Edit `spec/model.json` or `generator/generate.py`, then run
`python3 generator/generate.py`. `--check` detects changed, missing and obsolete
owned files. Contextual rules, codec and helpers are handwritten.

## License

MIT **OR** BSD-3-Clause, at your option. Both license texts are included.
