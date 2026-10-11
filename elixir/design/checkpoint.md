# Native API checkpoint

Implementation started from master `70ffab58` on 11 October 2026. This is PR 1
of the reviewed train. The generator is deliberately restricted to the prototype
and its concrete dependencies (23 models). It does not claim full conformance.

## Decisions exercised

- Immutable structs with documented fields and types; `new/1`, `new!/1`,
  `change/2`, `change!/2`; zero-argument constructors only for valid defaults.
- Binary text conveniences apply to keyword construction/editing. Output checks
  struct shape and stored child types, including fabricated/edited values.
- Tagged validation errors carry stable categories, wire keys and list indexes.
- Message defaults and explicit clearing differ from wire-ingress omissions.
- Pending tasks require a plan at complete-output boundaries. Plan task encoding
  omits child discriminators without mutation.
- Jason integration validates the complete modeled subtree. Checked decode retains
  decimal tokens until precision/range checks and rejects duplicate JSON keys.
- Opaque metadata remains JSON data, including maps that resemble blocks.
- Unicode limits count code points; arbitrary-size integers remain integers.
- Ragged plain tables, rectangular data tables, rich text and accordion expansion
  demonstrate receiving-context behavior.

## Executed evidence

| Gate | Result |
| --- | --- |
| Elixir 1.16.3 / OTP 26.2.5.21 | Native tests and warnings-as-errors compilation pass |
| Elixir 1.20.4 / OTP 29.1.1 | Native tests and actual archive consumer pass |
| Decimal 2.4.0 and 3.2.0 | Independent archive consumers pass |
| Deterministic generation | Complete output set checked; missing, stale and obsolete mutation probes reject |
| Native style | Pinned formatter and strict Credo pass |
| ExDoc | HTML and EPUB generation pass with warnings treated as errors |
| Actual Hex archive | File allowlist, licenses/dependencies, isolated test and production compilation pass |
| Erlang | Public calls, EUnit, complete message round trip, edits/errors and plan context pass |
| Relx release | Runs after build tree removal, with compiler/Mix/Rebar3 absent from PATH; consolidated Jason protocol verified |

The native suite contains ten checkpoint tests; these are not the shared full
conformance suite. No Slack workspace or Hex publishing account was contacted.
Hosted checks add minimum/current Linux, macOS and Windows Elixir lanes, plus
Linux Erlang release lanes; their results must be inspected on the PR.

## Erlang integration findings

1. Rebar3 3.27.1 cannot discover the name of a Mix-only `_checkouts` dependency
   before compilation. The test uses the plugin's Hex fetch resource, serving the
   exact built tarball on localhost. No source checkout override is involved.
2. Released `rebar_mix` 0.5.1 predates current Mix lock tuple shapes. The example
   explicitly declares Jason and Decimal in Rebar3. Both major Decimal lines are
   checked rather than depending on incomplete transitive discovery.
3. The upstream Rebar3 guide's `${REL_VSN}` example no longer matches Relx's
   exported variables. Use `${RELEASE_VSN}` in `vm.args.src`. The test asserts the
   loaded protocol is consolidated, catching this even when serialization works.

Sources: [Rebar3 plugins](https://www.rebar3.org/docs/configuration/plugins/),
[released plugin source](https://repo.hex.pm/tarballs/rebar_mix-0.5.1.tar),
[Relx launcher](https://github.com/erlware/relx/blob/main/priv/templates/extended_bin).

## Remaining train gates

The full model/fixture/invalid registries, independent API inventory, broader
property and coverage checks, Dialyzer, sending integrations, complete site/README
integration and disabled coordinated-release wiring remain assigned to later PRs.
Publication is disabled in `release.json`; no coordinator or active package
versions have changed. Merge and release require explicit maintainer approval.

## Complete modeled contract (PR 2)

Generation now includes all 89 models. Independently maintained native
construction registries cover all 105 valid fixtures and 167 invalid cases, with
exact error categories, duplicate/obsolete ID checks and no exclusions. Valid
cases also round-trip through both checked map and JSON ingress. The registry
checks limits, vocabulary, capability references and scalar constraint coverage.

Full-corpus testing corrected standalone pending-task errors to `type_mismatch`
and added the chart segment's positive-value rule. The suite has 283 passing
tests. Public-producer discovery, helper parity, property testing, coverage and
Dialyzer remain PR 3 gates; corpus completeness alone does not establish those.

## Sending and ecosystem integration (PR 4)

The separate integration project executes the community client's Req request,
direct Req, incoming webhooks and a pure map suitable for a Phoenix response.
Eleven tests inspect actual localhost HTTP bytes, authentication, complete nested
values, error responses, bounded 429 retry counts/delays, redirects and timeouts.
Both displayed Req README snippets are extracted and executed by those tests.

Consumers resolve independently against the actual Hex archive: committed lock,
explicit direct lower bounds, and clean latest-compatible resolution. Ecto 3.14
and Decimal 3 coexist. A production Jason-only consumer is compiled first, then
adds slackblocks without cleaning; checked decimal decoding and consolidated
protocols succeed. Transport errors are returned unchanged across Req/Finch
versions. No HTTP client or framework dependency was added to the core package.
