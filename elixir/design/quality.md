# Helpers and quality evidence (PR 3)

The helper layer is native immutable Elixir: keyword options, tagged/bang results,
ordinary lists, explicit expansion and no supervising process or transport client.
It includes Accordion, AccordionSection, Paginator, RichText.Style, Workflow URL
construction, named/semantic/hex/RGB colors and Block Kit Builder URLs.

## Independent verification

- `public_api.json` classifies every compiled production module and exported
  function. Model producers map to all 85 shared capabilities, and the audit
  verifies those capability fixtures actually construct the corresponding model.
- The API contract suite examines all 89 models / 360 fields: struct keys,
  requiredness, defaults, types, text coercion, native edits, wrong field kinds,
  default clearing, and all receiving-role memberships.
- Each of the 30 declared contextual rules has paired native positive/negative
  evidence in `rules_test.exs`. The normative invalid corpus adds scalar and
  aggregate-limit cases. String/text field maxima are tested at and beyond the
  boundary using ASCII and astral code points.
- StreamData checks bounded recursive blocks and JSON metadata, Unicode text,
  concurrency-safe immutable output, and arbitrary-size integer preservation.
- The minimum-runtime full check passes 297 tests, four properties and four
  doctests. Dialyzer reports zero warnings; strict Credo and pinned formatting pass.
- Handwritten executable-line coverage is 398/423 (94.09%). All production
  coverage is 871/1013 (85.98%); generated wrappers are reported separately and
  cannot raise the handwritten score. This is line coverage, not branch coverage.
- Scratch mutation checks prove unclassified modules/functions fail, missing
  production coverage fails, and an untouched classified production module counts
  as zero and causes the 90% gate to fail. They leave the checkout unchanged.
- Actual Hex artifact consumers execute native and helper tests without the
  shared spec or generator. Decimal 2.4.0, 3.0.0 and current 3.2.0 are supported.
- Credential-free `mix hex.publish docs --dry-run` succeeds in an isolated Hex
  home. No account, registry reservation or publication has been performed.

## CI and support

CI uses GitHub-provided runners. The native matrix covers Elixir 1.16.3/OTP26,
1.17.3/OTP26–27, 1.18.5/OTP26–27, 1.19.6/OTP26–28 and 1.20.4/OTP27–29.
macOS and Windows run minimum/current endpoints. Formatter output remains pinned
to Elixir 1.16.3. All matrix legs consume the actual package; Erlang release lanes
add minimum/current OTP and both supported Decimal majors.

The full local checker also runs Python tooling checks (Ruff 0.15.12), compiled
API/coverage guards, docs generation, Dialyzer, archive checks and Rebar3 release
checks. Set `SLACKBLOCKS_PLT_PATH` for an external durable runtime-specific PLT
cache on the maintainer host. Source packages have no dependency on those paths.

Remaining work at the PR 3 checkpoint (now completed in later draft PRs; see
[checkpoint.md](checkpoint.md)): executable sending examples and dependency co-installation,
complete current-site/root-README documentation, outgoing documentation freeze,
disabled release wiring and monitoring preparation. Do not merge or publish this
train without maintainer approval.
