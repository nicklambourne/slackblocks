# Elixir implementation checkpoints

Implementation started from master `70ffab58` on 11 October 2026. The opening checkpoint below records PR 1
of the review train; subsequent sections record completed implementation phases. The generator is deliberately restricted to the prototype
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

## Gates remaining at the PR 1 checkpoint

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

### Existing Jason build caches

Hosted CI exposed an intermittent optional-dependency invalidation in Jason 1.4.5:
its Decimal decoder is selected when Jason compiles. Adding Decimal later does
not reliably replace the existing BEAM module. Preserve native Mix dependency
management: document `mix deps.compile jason --force` in each build environment
and rebuilding the release. Checked decode returns an actionable tagged error
for this specific state; it never falls back to precision-losing parsing. The
consumer restores a genuine Jason-without-Decimal beam to make the regression
deterministic, then executes the documented repair and complete JSON/protocol
checks. No runtime compilation, dependency patching or alternate parser is added.


## Quality and complete API (PR 3)

The final native suite has 297 tests, four properties and four doctests. Compiled
module/export discovery is checked against the independent inventory, including
default arities and native required/defaulted field behavior across compilers.
Mutation probes reject an unclassified module, missing producer coverage and an
unexercised handwritten module. The full pinned checker passes strict Credo,
Dialyzer, formatter/generator checks and the 90% handwritten line coverage gate
(400 of 427 executable lines, 93.68%, in the final preparation run).
Generated coverage is reported separately and does not inflate that gate.

## Disabled release and coordinated versions (PR 5)

All eight outgoing references were generated before freezing 2.7.0. The snapshot
manifest records the exact file set and hashes. All nine packages now prepare
2.8.0 together, with unreleased changelog entries. Strict guards reject disabled
Hex publication before coordinated tags. The protected environment, verified
owner and `HEX_API_KEY` remain maintainer setup; no account calls or publication
were performed. Sixty-one release/recovery tests pass, along with actual Hex
archive preparation and byte-for-byte reconstruction from a clean commit.

The first hosted run caught PHP's independent public version constant still at
2.7.0. Updating only that expected constant restored all 392 PHP tests and the
release PR's hosted checks. This is now included in the new-language guidance.

## Documentation and repository guidance (PR 6)

- Compiled Elixir docs/typespecs generate 97 public modules and all 360 modeled
  fields. Independent export/default-arity and rendered-anchor checks pass.
- Literal README and guide evaluation runs 46 scripts; every one of the 21 block
  examples matches its displayed JSON. Sending snippets match the source tested
  against a local HTTP server; the Erlang guide embeds exact EUnit/release sources.
- Every language-switched section has Elixir content. Installation, quick start,
  all blocks, recipes, migration, compatibility, troubleshooting, sending and
  Erlang consumption are documented. Existing Jason rebuild requirements are
  explicit. The core still has no HTTP or framework dependency.
- Collapsed README sections, an honest pending Hex badge, Elixir CI badge and
  official light/dark logos preserve the established language order. The public
  download total remains the existing six sources until verified Hex activation.
- Chromium checks verify light/dark logo variants, current-only Elixir versions,
  reference switching, page back/forward history, stored-language fallback from
  2.7.0 and untouched historical panels, with no page errors.
- The complete nine-language site renders; all reference-rendering, search,
  selector, legacy fixture and historical snapshot checks pass. ExDoc passes with
  warnings as errors; publication supplies immutable commit-based source links.
- Contributor, Block Kit and language-addition guides include the implemented
  Elixir commands, native metadata/compiler lessons, installed-dependency tests,
  archive/ignore-rule checks and required branch-protection additions.

## Monitoring companion and remaining approval gates

[Noodle #1316](https://github.com/nicklambourne/noodle/pull/1316) prepares the
Hex provider, ninth version/card/trend queries and ndl.au registry metadata.
Collection and website visibility are disabled. Thirty-six exporter/HTTP/dashboard
checks and seven project metadata tests pass. Actual publication, activation,
deployment and production cache/privacy checks remain future approved work.

All train PRs remain drafts. Nothing is merged or published. The maintainer must
review the native API and complete train before authorizing merge; branch
protection is updated only after stable check names exist on the default branch.
Protected Hex readiness, release approval and verified registry consumers are
separate from these credential-free preparation checks.
