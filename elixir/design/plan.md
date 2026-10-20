# Elixir implementation plan for slackblocks

**Prepared and audited:** 11 October 2026 (Australia/Sydney). Revision 4; implementation evidence is recorded in [checkpoint.md](checkpoint.md); [audit findings and resolutions](audit.md).
**Status:** implementation prepared on 11–12 October 2026 in the stacked draft PR train. Do not merge or publish until the maintainer approves. The foundation PR carries this design and a portable parity inventory.

## 1. Outcome and baseline

Deliver a native Elixir library with the same Slack wire contract, useful helpers, documentation, package quality and release assurance as the maintained implementations. Elixir's structs, pattern matching, keyword options, immutable values, pipes, tagged results, typespecs, ExUnit, Mix and ExDoc determine the API.

The controlling guidance is [Adding a language implementation](../../docs/docs/contributing/adding-a-language.mdx), together with [AGENTS.md](../../AGENTS.md), [SPEC.md](../../spec/SPEC.md), [PRINCIPLES.md](../../spec/PRINCIPLES.md) and [ADR 0002](../../spec/ADR.md). This plan follows all ten stages of that guide.

### Verified starting point

| Item | Observation |
| --- | --- |
| Audited master | `250a5fe4e5cdacf51823ef0371c5d3a8dc4a3bc3`; original draft used `2cb409b04264b23360487423504124d0eaf76078` |
| Shared spec | `1.2.0`; adding another implementation does not itself change this |
| Source release version | `2.7.0`, with eight maintained implementations |
| Published state | Python, TypeScript, Go, Java, Ruby, Rust and PHP 2.7.0 verified; NuGet still lists C# 2.6.0 as its latest version |
| Existing 2.7.0 tags | All eight language tags exist at the release commit; their existence is not proof that every upload succeeded |
| Required checks | 51 checks with strict up-to-date protection |
| Proposed Elixir release | **2.8.0**, subject to a fresh baseline check before implementation and activation |
| Hex package name | `slackblocks` returned HTTP 404 from the public Hex API; available at the time of lookup, not reserved |
| Local Elixir | Neither `elixir` nor `mix` was found on PATH; no native prototype was executed for this plan |

The audit fetched current master and archived its guidance/README in the scratch audit reference directory. The shared contract files are unchanged from the original reference checkout; their hashes are recorded in the inventory. Master now explicitly requires collapsed README examples, the badge palette and staged shared-download activation; sections 10 and 12 include those rules. Registry observations remain point-in-time evidence: the audit reconfirmed NuGet 2.6.0, the unused Hex package name and the unavailable Hex identity guide.

Resolve the outstanding C# 2.7.0 publication before activating the next coordinated release. It does not block design or implementation. Recheck tags, registry versions and master when starting the train; use the next appropriate coordinated version if the baseline advances.

### Computed contract inventory

| Registry | Current count |
| --- | ---: |
| Models | 89: 21 blocks, 27 elements, 35 objects, 6 payloads |
| Modeled fields | 360 |
| Receiving roles | 11 |
| Model enums / field kinds | 8 / 15 |
| Valid fixture IDs | 105 |
| Invalid case IDs | 167 |
| Scalar limit leaves / additional model rule descriptions | 142 / 30 |
| Registered capabilities | 85 |
| Slack icon names | 54 |
| Surface block vocabularies | Home 15; message 20; modal 11 |

These are audit observations, never hardcoded test expectations. [The parity inventory](parity.json) records individual types, fields, roles, enums, fixtures, invalid cases, limits, model rules, capabilities, Python root-export dispositions and helper work with proposed implementation, test and documentation locations. Implementation must compare exact sets against the current registries.

## 2. Package, support and dependencies

### Proposed defaults

- Hex package and OTP application: **`:slackblocks`**; public namespace **`Slackblocks`**; directory **`../../elixir`**. Publish from this subdirectory of the existing monorepo. No distribution repository, umbrella application or root Mix project is needed.
- Pure Elixir library; no NIF, application supervisor, GenServer, ETS table, network connection or background process in the core.
- Proposed support floor: **Elixir 1.16.3 / Erlang OTP 26.2**, on 64-bit Linux, macOS and Windows. Declare Elixir `>= 1.16.3 and < 2.0.0`. The API checkpoint must validate this proposal, including dependency resolution; these versions are not claimed tested yet.
- Test every supported Elixir minor from 1.16 through current stable 1.20. Current official documentation identifies stable 1.20.4. Use only valid Elixir/OTP pairs, as described in section 9. Older OTP releases are outside this initial promise even where Elixir itself can run on them.
- The floor gives a reasonably broad supported Elixir range without relying on the newer built-in JSON module or experimental typing features. Raising the floor requires an explicit compatibility decision and release note. Preserve all existing languages' support floors.
- Package version follows the coordinated release. Expose `Slackblocks.version/0` and `Slackblocks.spec_version/0`; keep spec and package versions distinct.
- Preserve the repository's **MIT OR BSD-3-Clause** choice. Ship both `../../elixir/LICENSE` and `../../elixir/LICENSE.BSD-3-Clause`, copied from the corresponding root files. Declare both SPDX identifiers in Hex `licenses: ["MIT", "BSD-3-Clause"]`, and state the OR choice in the package README. Copy required notices for reused assets/code; do not narrow this to MIT merely because an older package manifest does.

The runtime floor is a proposal rather than a discovered technical necessity. Confirm it at the prototype gate; broaden support if inexpensive, but do not advertise an untested matrix. Elixir's own compatibility table currently pairs 1.16 with OTP 24–26, 1.17/1.18 with 25–27, 1.19 with 26–28, and 1.20 with 27–29. [Official compatibility policy](https://elixir.hexdocs.pm/compatibility-and-deprecations.html)

### Dependency policy

| Scope | Proposed tools | Reason and verification |
| --- | --- | --- |
| Runtime | Jason `>= 1.4.5 and < 2.0.0`, proposed lower bound 1.4.5 | Established JSON encoder/protocol; verify minimum and latest stable in consumers |
| Runtime | Decimal `~> 2.4 or ~> 3.0`, subject to tests on both majors | Preserve decimal tokens without a handwritten JSON scanner; allow current Ecto to resolve Decimal 3.x |
| Dev/test | ExUnit and doctest; StreamData; Credo; Dialyxir; ExDoc | Native testing, meaningful properties, lint, success typing and reference generation |
| Coverage | OTP/Mix line coverage with a handwritten-module gate | Smallest tool set that measures the code requiring human judgment; add ExCoveralls only if required for publishing coverage reports |
| Separate integration Mix project | `slack_elixir`, Req, and a local HTTP test server such as Bypass | Exercise actual send paths without making transport or web frameworks core dependencies |

The original 2.x-only Decimal proposal would conflict with Ecto 3.14, whose published dependency requires Decimal 3.x. Test proposed floors 2.4.0 and 3.0.0 and current stable 2.4.1/3.2.0, including a fresh consumer that also depends on Ecto 3.14. Do not claim a Phoenix-specific adapter from this dependency-resolution check. Avoid Decimal arithmetic that rounds through a caller-modified process context when validating lexical numbers; the conversion policy must be independent of that context. [Ecto dependency metadata](https://hex.pm/api/packages/ecto/releases/3.14.0), [Decimal 2.4.1](https://hex.pm/api/packages/decimal/releases/2.4.1), [Decimal 3.2.0](https://hex.pm/api/packages/decimal/releases/3.2.0)

Pin compatible development versions and commit the root development `mix.lock`. Commit the separate integration lock as well. Hex consumers resolve their own dependency graph; a green repository lock is insufficient. Mix does not put a path dependency or the root project itself into `mix.lock`; validate the integration's declared own-package requirement and the loaded application version instead of requiring a fictitious lock entry. Apply checksum/version guards to a Hex lock entry only when one actually exists. Test locked, clean latest-compatible and declared-lower-bound resolutions. When building a lower-bound consumer, pin direct dependencies explicitly in that temporary project; do not invent a nonexistent Mix equivalent of Composer's `--prefer-lowest`.

Hex metadata verified for `slack_elixir` 1.2.2, Req 0.7.5 and Jason 1.4.5. Req's and Jason's `latest_version` API fields include prereleases, so select the highest **stable** release explicitly. Audit transitive requirements on the actual minimum compiler; upstream master and published package metadata can differ. [Slack client release metadata](https://hex.pm/api/packages/slack_elixir/releases/1.2.2), [Req metadata](https://hex.pm/api/packages/req/releases/0.7.5), [Jason metadata](https://hex.pm/api/packages/jason/releases/1.4.5)

### Erlang consumption

Erlang compatibility is an explicit, tested consumer contract. Keep the core API native to Elixir: Erlang calls quoted `Elixir.*` module atoms, passes UTF-8 binaries and keyword tuples, and handles `{ok, Value}` / `{error, Reason}` results. Do not introduce a second implementation or wrapper namespace.

Deliver `elixir/erlang/` with a pinned Rebar3/rebar_mix example, EUnit checks, release configuration and usage documentation. Prove construction, checked editing, invalid-input categories/paths, complete-message serialization, JSON reconstruction, pending tasks in plans, and opaque metadata. Use the same Hex package, with Elixir runtime, Jason and Decimal included as OTP dependencies. Document Elixir/Mix build prerequisites and supported OTP pairs.

The consumer must build against the actual unpacked Hex artifact, consolidate Jason protocols, build a self-contained OTP release, and run it outside the repository with no build-time tools on PATH. Test minimum and current OTP with compatible Elixir compilers in hosted CI; test both supported Decimal majors. Verify `rebar_mix` against those real versions before claiming support. If the published plugin needs configuration or a compatibility fix, record the supported path and evidence rather than silently reducing the support matrix. Add an independent Erlang doctest/example extraction check and stable Erlang CI gate. Erlang is consumption of the Elixir package, so it does not create another coordinated package/tag/version or download counter.

Sources: https://www.rebar3.org/docs/configuration/plugins/#elixir-dependencies and https://hex.pm/packages/rebar_mix. Release validation is required because successful direct calls alone do not establish Rebar3 packaging support.

## 3. Native public API

### Values and construction

Use small immutable structs with public documented fields and `@type t`. Required and optional constructor arguments use snake_case keyword options. Provide `new/1` returning `{:ok, value} | {:error, %Slackblocks.ValidationError{}}` and `new!/1` raising that exception. Provide `new/0` and `new!/0` only for types that are valid with their declared defaults; all other constructors require the keyword argument. The independent export inventory must include default-argument arities. Required fields without defaults fail categorically; required fields with defaults are handled explicitly below.

Use semantic namespaces rather than inherited object hierarchies:

- `Slackblocks.Blocks.Section`, `Slackblocks.Blocks.Image`, `Slackblocks.Blocks.Table`, etc.
- `Slackblocks.Elements.Button`, `Slackblocks.Elements.Image`, etc.
- `Slackblocks.Objects.PlainText`, `Slackblocks.Objects.MarkdownText`, `Slackblocks.Objects.Option`, etc.
- `Slackblocks.RichText.Section`, `Slackblocks.RichText.Text`, etc.
- `Slackblocks.Message`, `Slackblocks.MessageResponse`, `Slackblocks.WebhookMessage`, `Slackblocks.Attachment`, `Slackblocks.Views.Modal`, `Slackblocks.Views.Home`.

The complete proposed mapping is in the inventory. Approve it before generating all models. Role unions and closed atom vocabularies belong in `Slackblocks.Types`; model enums use atoms such as `:primary`, `:pending` and `:in_channel`, converted to their exact wire strings. Type names and roles must distinguish image blocks from image elements.

Each modeled module also exposes `change/2` and `change!/2`, with the value first for piping. Changes are keyword replacements applied atomically and then validated. Omitted keys retain their prior values; an explicit `nil` clears an optional field. Duplicate or unknown keyword keys fail clearly. Use normal lists and `Enum` operations for collection composition rather than generating hundreds of append/getter/setter methods.

Struct field access, pattern matching and `%{value | field: replacement}` remain available. **Struct syntax, `@enforce_keys`, typespecs and Dialyzer do not enforce runtime invariants.** Validate at constructors/changes and revalidate the complete typed graph at every public map/JSON output boundary, including the Jason protocol. Directly fabricated or edited structs must not bypass rules. Check the exact known struct shape, including missing/extra keys and forged `__struct__` maps; `is_struct/1` alone is insufficient. Stored child fields contain normalized typed values: binary-to-text conveniences belong to `new`/`change`, while direct struct edits must use those declared field types. Output validation neither applies constructor defaults nor silently coerces invalid edited children. Do not claim private or unforgeable values.

### Proposed consumer example — design sketch, not executed

```elixir
alias Slackblocks.{Message, ValidationError}
alias Slackblocks.Blocks.Section
alias Slackblocks.Elements.Button

button = Button.new!(text: "View build", action_id: "view_build", url: "https://example.com/build/42")

section =
  Section.new!(text: "*Build complete*")
  |> Section.change!(accessory: button)

with {:ok, message} <- Message.new(channel: "C123", text: "Build complete", blocks: [section]),
     {:ok, wire} <- Slackblocks.to_map(message) do
  # A validated map with Slack's string keys, ready for a JSON HTTP client.
  {:ok, wire}
else
  {:error, %ValidationError{category: category, path: path}} ->
    {:error, {category, path}}
end
```

### Errors and checked boundaries

`%Slackblocks.ValidationError{category: atom, path: list, message: binary}` is a native `defexception`. Categories are `:length_exceeded`, `:out_of_range`, `:mutually_exclusive`, `:type_mismatch`, `:missing_required` and `:invalid_usage`; the conformance adapter maps them to normative hyphenated strings. Paths contain wire keys and zero-based indexes. Traverse modeled fields in schema order and arrays in order; use sorted keys for deterministic opaque-map diagnostics.

| Public boundary | Success | Expected failure |
| --- | --- | --- |
| `Model.new/1`, `Model.change/2` | `{:ok, normalized_struct}` | `{:error, ValidationError}`; bang forms unwrap/raise |
| `Slackblocks.validate/1` | `:ok` | `{:error, ValidationError}`; full standalone-context validation |
| `Slackblocks.to_map/1` | `{:ok, string_keyed_map}` | `{:error, ValidationError}`; `to_map!/1` unwraps/raises |
| `Slackblocks.encode/1` | `{:ok, json_binary}` | `{:error, ValidationError}` or `{:error, Jason.EncodeError}`; `encode!/1` unwraps/raises |
| `Slackblocks.from_map/2` | `{:ok, normalized_struct}` | `{:error, ValidationError}`; bang companion |
| `Slackblocks.decode/2` | `{:ok, normalized_struct}` | `{:error, Jason.DecodeError}` or `{:error, ValidationError}`; bang companion |

Bad dynamic argument shapes are `:type_mismatch`; duplicate/unknown constructor keys, unknown modeled wire fields and unsupported root selectors are `:invalid_usage`. Missing/null required values have their documented category. JSON syntax/invalid UTF-8 syntax failures retain `Jason.DecodeError`; syntactically valid out-of-policy numbers and wire values use categorized validation errors. Do not rescue programming bugs into normal user-input failures.

`from_map(module_or_role, map)` and `decode(module_or_role, binary)` accept only known public model modules and a fixed documented set of role atoms such as `:block`, `:element` and `:text`. Use a static lookup, never arbitrary dynamic `apply` or module/atom creation. Untagged payloads require a concrete root module. Dispatch children using the receiving field's role plus the wire discriminator: an image block and image element share `"image"` but are not interchangeable. Reconstruct tagless task entries only in a plan's task field; reject missing/mismatched discriminators elsewhere. Wire ingress accepts actual wire shapes, not constructor text conveniences.

Opaque metadata uses proper JSON arrays, string-keyed maps, binaries, numbers, booleans and nil. Reject arbitrary atoms, keyword objects, dates and custom structs even if Jason could stringify them. Do not inspect metadata's `type`, `text`, `rows`, `tasks` or `blocks` as Block Kit. Full Slack event parsing and raw extensions are outside the initial contract; callers editing a final wire map leave the checked boundary.

**Jason protocol distinction:** implement encoding through checked wire conversion and `Jason.Encode.map/2`. On invalid structs, raise the library's `ValidationError`; both `Jason.encode/1` and `Jason.encode!/1` can propagate it. Jason 1.4.5 does not catch arbitrary exceptions from a custom encoder. The library's own `Slackblocks.encode/1` validates first and supplies the promised tagged failure. Document and test this difference; do not depend on Jason's internal throw convention to fake its native errors. [Jason encoder implementation](https://github.com/michalmuskala/jason/blob/v1.4.5/lib/encode.ex)

### Presence and defaults

| Operation/input | Contract |
| --- | --- |
| `new`: absent field with a model default | Apply the default, whether required or optional |
| `new`: absent required field without a default | `:missing_required` |
| `new` / `change`: explicit nil | Clear an optional field; `:missing_required` for a required field, even one with a default |
| `change`: absent keyword | Keep its previous value; never restore a cleared default |
| `from_map` / `decode`: absent or null optional field | Store nil and omit on output, including fields defaulted by `new` |
| `from_map` / `decode`: absent required field with a default | Apply the required default (`File.source = "remote"`, `IconButton.icon = :trash`) |
| `from_map` / `decode`: absent nondefaulted or explicitly null required field | `:missing_required` |
| `false`, integer `0`, float `0.0` | Preserve; do not filter with truthiness |
| Empty binary/list/map | Preserve its kind and apply field-specific rules; `[]` and `%{}` stay distinct |
| nil inside metadata | Preserve JSON null |
| Unspecified `block_id` | Omit; no random IDs |

Declare model defaults in `defstruct` as well as `new`, and use `@enforce_keys` only for required fields without defaults as a developer aid. Parsing must explicitly populate every optional field, overriding struct defaults with nil for absent values. All runtime boundaries still validate. Pin every current defaulted model, not only messages: File, IconButton, DataTable and the message/response defaults recorded in the inventory. The inventory determines the exact current set; do not hardcode that count in tests.

Message defaults are `text: ""` and `mrkdwn: true`; MessageResponse additionally defaults to `response_type: :in_channel`, `replace_original: false`. Explain visibility and explicit overrides. The round-trip promise is **canonical wire values and omission**, not original key order, null spellings or construction history. Once two inputs produce the same JSON, a parser cannot recover whether a default was explicitly cleared. Test `new -> clear -> encode -> decode -> encode` and require unchanged wire output.

### Future compatibility

Adding optional fields, supported role members or enum atoms should be an additive release. Keep existing native names, arities, defaults, coercions and error categories stable; unknown current inputs still fail. Document catch-all handling when callers match evolving enum atoms or error categories. Public struct patterns should select relevant fields, not depend on an exact map-key set. Adding a required keyword, renaming fields, changing a default or narrowing a dependency/runtime range needs explicit compatibility review. Store canonical wire JSON for persistence; cross-version BEAM term serialization and hot-upgrade migration of old struct shapes are outside the initial support promise.

### API checkpoint before bulk generation

Implement and execute a small prototype through an external Mix consumer: plain and markdown text, a section with a button, a complete message, a ragged table, a rectangular data table, nested rich text, a pending task inside a plan, and one accordion/paginator operation. Demonstrate success/error handling, field inspection, keyword edits, direct/forged struct edits, Jason exception behavior, omitted-default parsing, receiving-role dispatch, both Decimal majors and checked reconstruction. Preserve examples as runnable consumer tests after the checkpoint.

Run minimum/current Elixir and OTP, compile typespecs/docs, and prove an invalid directly edited struct is rejected when serialized. Decide the runtime floor, naming, keyword behavior, JSON precision policy and any dependency lower bounds using this evidence. Review the checkpoint before expanding the generator. No claim of an executed prototype is made by this plan.

## 4. Serialization, validation and Elixir-specific risks

### Unicode and native terms

`String.length/1` counts graphemes; the shared contract counts Unicode code points. Check `String.valid?/1`, then count code points with a correct UTF-8 traversal. Never silently normalize input or confuse byte size with either count. Test combining marks, astral characters, emoji sequences and malformed UTF-8. [Elixir String documentation](https://elixir.hexdocs.pm/1.16.3/String.html)

Accept binaries for strings, not arbitrary `String.Chars` values or charlists. A charlist is an ordinary integer list in Elixir: reject it for a text field, but preserve `[97, 98]` as an array in metadata. Require proper lists for JSON arrays and model collections; reject improper lists, PIDs, references, functions and unsupported structs with clear paths. Elixir's immutable terms remove PHP's referenced-array aliasing issue, but direct struct construction/editing still needs explicit negative tests.

Use only compile-time known atoms for native field/vocabulary identifiers. JSON keys stay binaries; resolve modeled names through a static lookup table. Never call `String.to_atom/1` on user input. Reject conflicting atom/string representations. Decode JSON with `keys: :strings`, `strings: :copy`, `objects: :ordered_objects` and `floats: :decimals`; check duplicate keys in every object before converting it to a map, including opaque metadata. This prevents duplicate evidence from being erased and avoids retaining a large source binary through tiny sub-binaries. `from_map` cannot recover duplicates already discarded by a caller. These Jason options must remain fixed for the checked boundary, not exposed as unsafe caller overrides. [Jason decode options](https://github.com/michalmuskala/jason/blob/v1.4.5/lib/jason.ex)

### Numeric and JSON policy

- Preserve native integers exactly, including values above `2^53`; enforce each field's actual limits rather than imposing another language's integer width. Record any Slack-specific range in the rule inventory.
- Native floats retain BEAM float semantics. A nonfinite/invalid JSON number is rejected where representable or encountered in textual input.
- Checked `decode` uses Jason's `floats: :decimals` to inspect the original decimal value. Conversion to a float must reject overflow, nonzero underflow and loss of the declared decimal value under the chosen round-trip policy. The proposed rule is numerical equality between the input Decimal and the Decimal reconstructed from the candidate float's shortest decimal rendering. Thus `0.1` and `1.2300` are permitted while excess significant digits must not silently disappear. Prove this on minimum/current OTP and both Decimal majors before adoption; never compare only two already-rounded floats.
- Keep integer tokens as native arbitrary-size integers. For modeled `int`/`long`, accept only integer tokens/native integers; do not coerce `1.0`, `1e0`, booleans or numeric strings. For `number`/`double`, preserve native integers without first converting them to floats; decimal/exponent tokens use the checked float path. Reject out-of-policy numeric values as `:out_of_range`, and wrong numeric kinds as `:type_mismatch`. State negative-zero and decimal-scale normalization explicitly. Avoid expanding huge powers of ten during validation; check exponent/conversion bounds before allocation. Resource limits, if needed, must be measured, documented and compatible with maximum valid payloads rather than invented as shared Slack limits.
- Decimal is initially an internal decoding aid. Do not expose arbitrary precision decimals as a promised model feature or accidentally encode numbers as JSON strings through a generic Decimal encoder.
- Use the same checks for opaque metadata numeric values. Test high precision literals, extreme exponents, negative zero, long escaped strings before a number, deeply nested input and duplicate object keys. Preserve `{}` versus `[]` and JSON null.
- `from_map` sees already parsed native numbers and cannot recover precision a caller's earlier parser lost; document this boundary honestly.

Jason documents decimal-preserving decoding and the risks of atomizing JSON keys. Its encoder protocol is the interoperability point. [Jason decoding options](https://jason.hexdocs.pm/1.4.5/Jason.html), [Jason.Encoder](https://jason.hexdocs.pm/Jason.Encoder.html)

### Contextual rules and output

Keep handwritten receiving-context validators that walk typed graph edges. Cover all surface vocabularies, message/modal/home/attachment limits, aggregate markdown and data-table character totals, modal submit labels, exclusive fields, chart dimensions, header-cell restrictions and task uniqueness.

Use one private context-aware graph walker with local construction mode and complete output mode; share rule predicates rather than implementing independent validators in every boundary. Constructors/changes enforce all locally knowable rules but defer requirements that need a receiving ancestor. Public `validate`, map/JSON output and checked wire ingress enforce the complete root context. Carry role, surface and error path through typed edges; accumulate surface/text totals without repeatedly walking the same subtree.

A pending task can be locally constructed for use in a plan. Serializing it alone or under an invalid parent fails. Plan encoding emits its child tasks without a `type` field, using the receiving context without mutating the task. Avoid recursively invoking a context-free task encoder from inside the plan.

`Jason.Encoder` implementations call the same checked wire conversion used by the library; no blind `@derive Jason.Encoder` over structs. Produce the complete plain wire tree before generic JSON encoding, so internal keys, modeled nil omissions, context and discriminator transformations cannot leak. Verify direct Jason use both standalone and inside an ordinary application map/list. An external raw map is not a validated Slack payload: its parent totals cannot be inferred from a nested model's protocol callback. Use `Slackblocks.Message`/view wrappers for whole-payload guarantees.

## 5. Helpers and behavioral parity

| Public behavior | Proposed Elixir API/location | Required tests/docs |
| --- | --- | --- |
| Accordion and AccordionSection | Immutable values in `Slackblocks.Components`; explicit `to_blocks/1` expansion | Empty/open/closed sections, heading/body order, action IDs, labels, expansion limits; cookbook |
| Paginator | `Slackblocks.Components.Paginator`; pure page selection and block construction | Empty input, first/last/out-of-range pages, buttons, stable caller-supplied IDs; no server process |
| RichTextStyle | `Slackblocks.RichText.Style` with boolean keyword fields | Every style flag, false/omission, JSON protocol, nested rich text |
| Text conveniences | Field-specific binary-to-PlainText/MarkdownText conversion | Only normalize valid binaries in eligible positions; validate collection item types first |
| Workflow URL shortcut | `Slackblocks.Objects.Workflow.from_url/2` plus bang variant | URL and input-parameter encoding, native keyword/map API, validation |
| Attachment colors | `Slackblocks.Color` functions/validated atoms and hex conversion | Semantic colors, named colors, hex/rgb boundaries; exact existing meanings |
| Block Kit Builder URL | `Slackblocks.block_kit_builder_url/1` plus needed options settled at checkpoint | JSON serialization, percent encoding, Unicode, ordering, IDs, no unintended double encoding |
| Inspection and changes | Struct pattern matching; checked `change/2`; normal immutable lists | Originals unchanged; valid edits must succeed; invalid edits and final serialization must fail correctly |
| Existing Python names and historical aliases | One documented native canonical name | `Confirm`/`ConfirmationDialogue` → `Objects.Confirmation`; `Markdown` → `Objects.MarkdownText`; menu classes → corresponding `Elements.*Select`; `Modal` → `Views.Modal`; `RichText` → `RichText.Text`. Full root-export mapping is in the inventory; do not add compatibility aliases for a new package |
| Legacy Python attachment Field | Explicitly excluded as an old attachment-only API outside the current model | State the disposition in parity inventory and migration/reference notes; preserve modeled attachment support |

The Python package's root exports are insufficient as the entire helper inventory. Review its public subpackages and current Ruby/Rust/PHP components as well. Every intentional difference gets an explanation and reviewer disposition, not a silently empty cell.

Helpers expand explicitly into normal typed blocks before payload validation. They must not be accepted through an unvalidated custom protocol that evades parent role or post-expansion count checks. Do not claim the helpers implement Slack interaction routing, persistence or a bot framework.

## 6. Package layout and generation

Proposed layout beneath `../../elixir`:

```text
mix.exs, mix.lock, .formatter.exs, .credo.exs
README.md, CHANGELOG.md, LICENSE, LICENSE.BSD-3-Clause, IMPLEMENTATION.md
release.json
lib/slackblocks.ex
lib/slackblocks/{blocks,elements,objects,rich_text,views}/
lib/slackblocks/{types,validation_error,components,color}.ex
lib/slackblocks/internal/{wire,rules,json,limits,vocabulary}.ex
lib/slackblocks/generated/                  # owned generated metadata if needed
generator/generate.py
conformance/skiplist.txt
test/{native,conformance,helpers,property}/
test/support/{valid_constructions,invalid_cases,api_inventory}.ex
tools/{check,package_smoke,export_reference,check_api}.exs
integrations/{mix.exs,mix.lock,lib,test,README.md}
```

Generate repetitive concrete structs, constructors/changes, typespecs, docs and schema/role/limit tables directly from the shared JSON registries. Keep semantic rules, JSON normalization and helper algorithms handwritten. Generated sources are checked in; installing the package requires only Mix and its declared Hex dependencies.

Use one resolved naming/type/default/coercion map for code and reference metadata. Fail on unsupported field kinds, unknown roles/enums, unresolved limit paths, collisions and malformed model references. Retain readable generated modules and explicit module ownership. No new macro DSL or runtime dynamic module creation is required.

Use Python 3.11+ tooling if it reduces maintenance alongside the existing generators; execute one pinned Elixir 1.16.3 formatter over generated output. Runtime lanes must not regenerate or compare formatting with whichever compiler they happen to test; newer formatter output can differ even for equivalent code. The generator and formatter are maintainer tools, never consumer runtime requirements. Include Python tooling formatting/tests in quality checks.

Generate struct modules without compile-time dependency cycles: reference neighboring types through remote typespecs and dispatch metadata, not mutually expanding `%OtherStruct{}` literals or macros. Prove a cold parallel compile and protocol consolidation from the archive. Test string/default escaping, reserved words, case-insensitive filename collisions and added/missing/stale generated files; do not rely only on Linux file semantics.

Generation `--check` must compare the complete expected file set and contents, detecting extra, missing, stale and obsolete outputs. Test these failures deliberately. Separate generated API consistency from independent public API coverage: they answer different questions.

Use **`mix run tools/check.exs`**, from `../../elixir`, as the complete local checker on its documented pinned maintainer toolchain. It runs generation/format checks, tests, snippets, static checks, package consumers and the separately resolved integration tests. Add `--profile runtime` for native/conformance/protocol/artifact/integration checks on another supported compiler; do not multiply formatter, coverage and full site builds across every runtime. Explicit resolution profiles exercise locked, lower-bound and clean latest dependencies. Each profile reports executed suites; missing required prerequisites fail with an actionable error. The CI aggregates combine all required profiles.

On the local host, probe `/Volumes/Repos`, `/Volumes/Cache` and `/Volumes/Scratch`; keep toolchains/build/dependency/PLT caches under a project-specific directory in `/Volumes/Cache` and extracted consumers under `/Volumes/Scratch`. Document prerequisites and cache variables for contributors; do not assume those mount points on GitHub-hosted runners.

The original scratch inventory is an expanded audit snapshot. Before checking it in, use portable repository-root-relative identifiers and shared location templates, retaining source hashes and explicit API/helper decisions. Do not commit host paths or duplicate whole model definitions as a competing source of truth. Recompute derived rows from current registries, and independently verify the resulting API after compilation.

## 7. Conformance and independent API coverage

1. Build a handwritten native construction registry for every valid fixture. Read expected JSON separately and compare decoded data, preserving list order and exact numeric/object/list semantics. Use a structural comparator with an explicit numeric policy, never blanket Elixir `==` or float-cast normalization: `==` can equate integer and float representations and hide precision loss. Integer-field runtime checks must remain strict even where a general JSON-number comparison permits mathematically equal values. Do not construct values by feeding expected fixtures into `decode` or `from_map`.
2. Assert exact equality of fixture ID sets, including duplicate and obsolete registration failures.
3. Implement every invalid-case public attempt. Assert its exact normative category and representative complete path. Since Elixir is dynamically typed, do not substitute Dialyzer warnings for required runtime invalid-case assertions.
4. Discover every scalar leaf of limits and check matching invalid-case `constraint` coverage. Add positive boundary cases and require equality of generated limits/vocabulary to the source files.
5. Independently discover application modules and public exports through the compiled application's module list, `__info__/1`, native docs/typespec metadata and a maintained classification file. Include structs/fields, constructors/defaults, changes, helpers and protocol implementations. Do not simply trust the generator's exported list.
6. Every public producer maps to a shared capability or a named, justified supporting/helper exclusion with its own tests. Newly added handwritten public producers must trigger the guard. Prove it with a deliberate scratch mutation that adds an unclassified producer.
7. Compare all 360 field names, requiredness, defaults, role memberships and coercions through an independent API contract test. Test actual behavior as well as exported names.
8. Assert `Slackblocks.spec_version()` equals the live manifest, all published skip lists are empty, and missing input manifests fail instead of silently skipping conformance.

Native tests add direct struct construction/edits, keyword errors, protocol dispatch, concurrency-safe immutable use, defaults, role ambiguity and safe context-sensitive output. Use StreamData for bounded recursive trees, Unicode, metadata, numeric edges and serialization/reconstruction properties. Shared fixtures remain an independent oracle; a parser/encoder round trip alone is insufficient.

Quality target: at least **90% handwritten executable-line coverage**, reported separately from generated modules; also publish total coverage. Read all instrumentable handwritten production modules from the compiled application, including modules never loaded by tests, so unexercised modules count as zero. Compare the coverage module set to the classified production set; removing a module from the report must fail. Exclude generated/test/external modules only by a reviewed explicit rule, and test the guard with an untouched handwritten module. Pair that with an explicit positive/negative contextual-rule matrix and deliberate failure probes. Do not call line coverage branch coverage or allow generated accessors to inflate the handwritten score. Ratchet a higher threshold after the complete implementation establishes a defensible baseline.

Compiler warnings, `mix format --check-formatted`, `mix credo --strict`, Dialyzer and doctests are required. Typespecs are documentation and analysis aids, not runtime enforcement. Any negative static-analysis fixture must assert the intended diagnostic and have a passing companion case. Do not force a Rust-like compile-fail contract onto ordinary Elixir inputs.

## 8. Real sending integrations

### Ecosystem audit and choice

Slack lists official SDKs for Java, JavaScript, Python and Deno, not Elixir. `slack_elixir` is a community option with current 1.2.2 publication. The older `slack` package targets RTM and was last updated in 2020; `slack_web` was last published in 2022. Use the maintained community client as an integration example and **Req as the straightforward direct HTTP path**. This is a suitability recommendation, not a claim that the community client has Slack's support guarantees. [Slack tools](https://docs.slack.dev/tools/), [community client](https://github.com/ryanwinchester/slack_elixir), [older slack package](https://hex.pm/packages/slack), [slack_web](https://hex.pm/packages/slack_web)

The audit inspected the actual published `slack_elixir` 1.2.2 archive, not just repository master. That client has `Slack.API.client(token)` returning a configured Req request, while `Slack.API.post/3` uses `form: args`. A nested slackblocks wire map cannot simply be passed to that form wrapper as though it were JSON. Keep that version/source evidence with the integration lock; re-audit on upgrades. [Client API](https://slack-elixir.hexdocs.pm/Slack.API.html), [inspected release source](https://github.com/ryanwinchester/slack_elixir/blob/v1.2.2/lib/slack/api.ex)

### Delivered examples

1. **Community client:** use `Slack.API.client(token)` and its native Req request API to send `json: Slackblocks.to_map!(message)`. This preserves the complete nested payload. Explain the form-wrapper limitation. No SDK adapter belongs in core unless the prototype demonstrates a real need.
2. **Direct Req:** construct a request with Slack's API base URL, bearer token and JSON body. Check HTTP status and Slack's `ok` field. Req stays in the integration/example project.
3. **Incoming webhook:** demonstrate a `Slackblocks.WebhookMessage` sent as JSON to a supplied webhook URL, with its distinct response semantics.
4. **Phoenix usage:** show the validated plain map or JSON binary returned from an interaction handler, and document that an immediate acknowledgement and later response are application responsibilities. Keep a framework dependency out of core; execute the actual displayed pure payload code, and add a small separate framework smoke consumer only if claiming framework-specific APIs.

Use a local HTTP server, not just a stubbed serializer, to observe request bytes, bearer authentication, content type and complete data preservation. Test `{}`, `[]`, null, false, zero, Unicode, attachments, metadata, tasks and recent block types. Cover HTTP errors, Slack `ok: false`, malformed responses, timeout, 429 and Retry-After.

Retries must be explicitly bounded in attempts and delay. Avoid retrying ambiguous non-idempotent POST failures by default; a request accepted before a timeout could otherwise be posted twice. Test retry exhaustion and exactly how many requests were sent. Explicitly disable Req's built-in retry middleware when the example owns 429 handling, so retries cannot multiply across two layers. Match exact request counts and bounded elapsed waits. Use an injectable base URL/sleep seam in examples/tests; no global production configuration mutation in asynchronous tests. If a dependency requires global test state, scope those tests synchronously.

Keep send code as executable modules/examples called by the HTTP tests and extracted docs; test the code users copy, not a different test-only adapter. Verify dependency co-installation with current Ecto and a previously compiled Jason-only consumer that later adds slackblocks, including decimal decode/protocol consolidation. Jason selects Decimal support at compile time; document and test its native `mix deps.compile jason --force` prerequisite for an existing application whose Jason predates Decimal. Force that real stale-beam state in the consumer test, require an actionable tagged error, then execute the displayed rebuild commands. The full local checker runs the actual separately resolved integration suite on its selected compiler and dependency profile. CI repeats it on minimum/current compilers and locked/lower-bound/latest supported dependencies. Record mock-server evidence separately from any later optional live-workspace test. No real Slack message is needed for routine CI or this planning request.

## 9. CI and package assurance

### Runtime matrix proposal

Use `erlef/setup-beam` pinned to a reviewed released action commit, and GitHub-provided runners per repository policy. Do not use an unqualified `latest` toolchain value that can resolve to a prerelease. Verify actual toolchain output and runner metadata. [Setup action](https://github.com/erlef/setup-beam)

| Required native/conformance lane | Runtime pairs |
| --- | --- |
| Linux (`ubuntu-24.04`) | 1.16.3/OTP 26.2; latest 1.16/26; 1.17/26 and 27; 1.18/26 and 27; 1.19/26, 27 and 28; 1.20/27, 28 and 29 |
| macOS | Minimum 1.16.3/26.2 and current stable 1.20/29 |
| Windows (`windows-2025`) | Minimum 1.16.3/26.2 and current stable 1.20/29 |
| Quality/package | Pinned formatter/tooling lane plus minimum/current artifact consumers |
| Sending | Minimum/current; locked and clean resolutions, with lower-bound checks on minimum |

This explicitly limits the initial OTP floor to 26.2. Recheck action support for each exact patch/OS before committing workflow pins. Keep check names tied to semantic lanes or major/minor pairs so routine patch updates do not invalidate branch protection. Cache keys include OS, architecture, Elixir, OTP, dependency lock and PLT inputs; never reuse BEAM/PLT outputs across incompatible toolchains.

Use `../../.github/workflows/elixir.yml` with an unfiltered `pull_request` event. Follow Ruby's step-level relevance handling; include Elixir, shared spec, examples/docs, root README/instructions, release guards, manifests/locks and the workflow itself. A changed-file lookup failure runs the checks. Put required release-preflight checks in this always-created workflow rather than requiring a path-filtered publisher job that might never appear.

Create stable aggregate checks such as **Elixir native checks**, **Elixir quality and package**, **Elixir sending**, and **Elixir release checks**. Their `always()` logic must reject failures/cancellation/missing relevant jobs and permit only explicitly unrelated changes. Either require each stable matrix check or these validated aggregates; record the exact adopted list.

Add the agreed checks to branch protection/rulesets after they exist on master, retaining the existing 51 checks and strictness. Prove relevant and unrelated PR behavior using actual job steps and runner identities. There is no need to wait until the release to add protection for completed language CI.

Add `/elixir` and `/elixir/integrations` to Dependabot using the documented `mix` ecosystem. Apply development grouping and explicit compatibility constraints; never merge a tooling upgrade that silently raises the supported floor. [GitHub supported ecosystems](https://docs.github.com/en/code-security/reference/supply-chain-security/supported-ecosystems-and-repositories)

### Actual artifact checks

- Build the real Hex tarball with `mix hex.build`; inspect it, its metadata and its nested source archive. `mix hex.build --unpack` is useful for inspection. Allowlist production sources, manifest, formatter config where useful, README, both licenses and changelog; exclude caches, tests requiring the monorepo, PLTs, credentials and build products.
- Run a fresh external Mix consumer against the **unpacked tarball**, never a path back into the checkout. This is archive verification, not registry verification. Its directory lives under `/Volumes/Scratch` locally or the hosted runner's temporary directory in CI.
- Execute a realistic payload, errors, direct Jason encoding, editing, contextual validation and reconstruction on the supported minimum/current runtime. Inspect packaged types/docs and absence of repository-only file reads.
- Resolve dependencies independently in locked-equivalent, clean latest and lower-bound modes. Include protocol consolidation and production compilation, since test-only behavior can differ.
- Verify public package metadata, runtime requirements, both license texts and source links. Keep versions and spec constants self-contained: consumer compilation must not run Git or read the shared spec. Test production compilation with development dependencies absent; compiler `@external_resource` paths and docs configuration must not require excluded repository files. The package must build without Python, the generator, `../spec`, another language or a root umbrella.
- PR checks run `mix hex.build`, `mix docs`, archive inspection and `mix hex.publish docs --dry-run` with the pinned compatible Hex CLI, no credentials and an isolated Hex config. **Do not require credential-free `mix hex.publish --dry-run`: Hex 2.5.1 calls account discovery even for this dry run, before publishing is skipped.** Exercise the complete publisher dry run later in the protected `hex` environment; do not add a fake key or PR secret to appease account discovery. Confirm the chosen client's actual behavior when pinning it. [Hex 2.5.1 publisher source](https://github.com/hexpm/hex/blob/v2.5.1/lib/mix/tasks/hex.publish.ex)
- If publish rebuilds from source, reproduce from the same immutable commit with a pinned toolchain and compare the resulting package identity/contents/checksum to the tested artifact. Do not claim that `mix hex.publish` uploads an arbitrary previously built tarball unless the actual command supports it.

[Hex build/publish command documentation](https://hex.hexdocs.pm/Mix.Tasks.Hex.Publish.html)

## 10. Documentation, README and discoverability

### Freeze the outgoing release first

Before adding Elixir to current language panels or advancing package versions, generate and freeze the **2.7.0 eight-language documentation** from verified outgoing sources. The 2.6.0 snapshot already exists; do not recreate it. Record the outgoing source commit, language set and snapshot digest. Keep all older snapshots free of Elixir claims.

Document the temporary discrepancy between source 2.7.0 and the blocked C# package while establishing the release baseline; resolve publication before the next release activation. If the next release target changes, adjust the outgoing snapshot plan to the actual previous release.

### User-facing parity checklist

- Root README: Elixir description, both registry and CI badge rows, API reference/changelog/layout links, dependency/support prose and development command. Add one collapsed `<details>` section under Installation and one under Quickstart, after PHP, with plain-text `<summary>` and no `open` attribute. Keep native examples/commands inside, shared illustrations outside and blank lines around fences. Verify GitHub-rendered output and all nine language pairs without losing existing examples.
- Badge specifics: registry label `Hex`, `style=flat`, `labelColor=555`, white registry logo and Elixir highlight `8847B9`; CI label `Elixir CI`, status-driven colour, `branch=master&event=push`. Keep both rows and examples in the same language order. Add Elixir's colour to the contributor guide palette; preserve project Docs/Licence colours and the download badge last in the project row.
- Package README: requirements, dependency declaration, quick start, normal/bang errors, fields/changes, JSON interoperability, spec version, helpers, send paths, development command and support policy.
- Current docs: installation, quick start, each block/element/composition page, message/view/attachment payloads, validation, migration, cookbook, components and sending. No empty Elixir panels or other-language code under the Elixir choice.
- Public comments: module purpose, constructor/change keywords, types, requiredness, defaults, validation errors/paths, role restrictions, limits and official Slack links. Include handwritten helpers.
- Generate **ExDoc** for HexDocs. Export native compiled doc/signature/typespec metadata for the Docusaurus adapter and cross-check the independent public inventory. Do not parse formatted comments with fragile regular expressions or reuse Java-shaped signatures. Verify native helper docs and defaults render correctly. [ExDoc](https://ex-doc.hexdocs.pm/readme.html)
- Execute actual README/MDX examples and `iex>` doctests. Extract source examples, not handwritten duplicate strings. Compare shared examples against their documented JSON, including the section-hello fixture. Add a guard that a snippet extraction count cannot drop silently.
- Use the official Elixir SVG variants linked by the [Elixir brand policy](https://elixir-lang.org/trademarks/) for light/dark backgrounds, preserve their geometry and attribution, and use the documented single-colour purple `#8847B9` for the badge highlight. Verify selector sizing, existing solid chevrons and all historical versions' shared chrome. No older snapshot gains Elixir content merely because shared chrome changes.

### Concrete site touchpoints

Review and extend the actual files under `../../docs`:

- `src/components/LanguageContext`, `LanguageContent`, `LanguageSelector`, `VersionSelector`.
- `src/theme/DocVersionBadge`, `DocSidebar`, `DocBreadcrumbs`, `DocItem/Paginator`, `TOCItems`, `SearchBar`, `SearchPage`.
- `plugins/language-search-indexes.cjs`, `docusaurus.config.ts`, `package.json`, `scripts/reference_pages.mjs`, `docs/reference/index.mdx`.
- New Elixir reference exporter, snippet checker and rendered-reference checker; extend language-content, search-language, version-selector and release-snapshot checks.
- Prism/Elixir highlighting, current API groups, generated search corpus and docs CI toolchain/source filters.

Update the root README, repository instructions, release guide, contributing pages, new-language guide, Block Kit maintenance guide, SPEC and PRINCIPLES language inventories. The audit already found older lists in SPEC/PRINCIPLES/model commentary that omit PHP or describe Rust as unreleased; reconcile these descriptions with the verified baseline while adding Elixir. Do not alter historical ADR decisions or frozen user docs merely to rewrite history.

Run the complete docs build/typecheck/snapshot guards and **all existing language snippet suites**, then inspect rendered signatures, links and helpers. In a browser test desktop/mobile, keyboard use, stored `elixir` preferences, direct historical URLs with/without trailing slashes, language changes, version changes, search and navigation after hydration. Versions before the introduction must route to supported current docs without relabeling an old page.

Before publication use truthful unreleased instructions and neutral preparation badges. After publication activate the real Hex version and HexDocs links. Keep the project download-count wording as “downloads” wherever the separate approved website change establishes that wording.

## 11. Publishing and coordinated release design

### Ownership, bootstrap and later authentication

Use protected GitHub environment **`hex`**, publisher **`.github/workflows/publish-hex.yml`**, repository **`nicklambourne/slackblocks`**, package **`slackblocks`**, and tag scheme **`elixir/vX.Y.Z`**. Registry identity and the proposed environment are not yet configured or verified. Recheck package availability before committing public identifiers and immediately before first publication.

The maintainer must create/verify a Hex account, satisfy the registry's account security requirements and prepare a narrowly controlled publishing key for the first package publication. Store it only as environment secret **`HEX_API_KEY`**, with a short lifetime and permissions that permit creating the new package. A package-scoped key for a nonexistent package must not be assumed to work. Confirm ownership after publication and add any intended additional owners explicitly.

Hex's official publishing page currently recommends **Workload Identity / trusted publishing** for GitHub Actions but says the package must already exist: it cannot create the first release. Bootstrap the real approved coordinated version with the key, not a placeholder package or independent `0.1.0`. [Hex publishing guide](https://hex.pm/docs/publish)

**Verification gap:** that page links to `https://hex.pm/docs/workload-identity`, which returned HTTP 404 during this audit. Do not invent an action, token-exchange endpoint, audience or pending-publisher behavior. At the release-integration gate, retrieve the working official setup instructions and pin the supported client/action. Configure the exact owner/repo/workflow/environment identity they require, test a real exchange and publish/docs permissions, then revoke the bootstrap key after a verified OIDC release. Until that is proven, keep the explicit token mode documented for first and subsequent releases. This affects authentication activation, not the core implementation plan.

Use explicit release configuration with `enabled: false` throughout the implementation train and an explicit auth mode (`api-key` initially; `oidc` only after verification). No automatic secret-presence fallback. PR checks have no publishing credentials. Grant `id-token: write` only to the OIDC publishing job when that mode is actually implemented.

### Workflow responsibilities

1. Build/package/docs validation runs on PRs without publishing.
2. Publisher is tag-bound, supports manual recovery at the same tag, and checks tag format, commit, master ancestry, version equality, complete coordinated tags and activation before authentication.
3. Protected publication has concurrency control and minimal write permissions. Register package and documentation as separately recoverable results.
4. Extend `../../.github/scripts/verify-release.py`, its tests and the historical Ruby wrapper integration. Add Mix version/spec/activation checks with a stable parseable version source; guard it against the actual Mix manifest and runtime version.
5. Extend `../../.github/workflows/coordinated-release.yml` and **every** coordinator component: relevance filters, stored-version comparison, changelog dates, readiness, nine-tag atomic creation, existing-tag identity, explicit dispatch matrix and failure monitoring. Do not rely solely on multi-tag push triggers.
6. Update all nine package versions/changelogs, exported constants, Java reproducible-build timestamp, docs version guard, root Composer generation and separately resolved own-package declarations/actual lock entries. For Mix path consumers, check the declared version requirement and loaded application version; no own-package lock record is expected. There will be nine languages/tags/changelogs and eight stored package versions because Go derives its version from its tag/module path. Count these separately.
7. Test wrong prefix/version, branch dispatch, dirty/unmerged commit, disabled or partial train, partial/moved tags, missing setup, registry outages and already-published versions using isolated refs and fake responses.
8. Version advancement, nine-language guard registration and the strict `enabled: false` gate must land together. Before that commit the eight-language 2.7 train remains coherent; after it no partial nine-language release can create tags. Do not bump all existing packages in an earlier docs PR while old guards can still publish only eight of them. Test both states.
9. Perform a protected Hex readiness check before coordinated tag creation: account verification/ownership, appropriate key permissions, package-name state and authenticated dry run. OIDC is not a prerequisite for the key-based first release; it is a later verified auth-mode transition. Activation remains a separate reviewed step after the whole train and account setup pass. Actual merge and release authorization remain separate maintainer actions.

### Release evidence and recovery

At the authorized release commit, create the coordinated tags and monitor **all nine publishers**. Verify the exact Hex version, ownership, dependencies, runtime requirement, public package checksum, HexDocs pages/source links and the GitHub Release. Install `{:slackblocks, "== X.Y.Z"}` in a fresh consumer from **Hex itself**, without Git/path overrides, and execute it on minimum/current runtimes. Capture evidence rather than treating authentication or a successful upload exit code as completion.

If a publication outcome is uncertain, query Hex and compare identity before retrying. If the package exists, complete missing docs/GitHub Release/verification steps without rebuilding a different package over it. Use docs-only publication where appropriate. Even if Hex permits short replacement/revert windows, this repository should treat a successful public version as immutable: never use `--replace`, move tags or overwrite a shipped artifact to fix a defect. Release a coordinated patch instead.

Keep retries bounded for package indexing, documentation visibility and GitHub release listings. The PHP first-release draft-list race should have a corresponding regression here. If a registry-side version already exists with a different identity, fail rather than guessing it is safe to resume. [Hex publish and recovery behavior](https://hex.hexdocs.pm/Mix.Tasks.Hex.Publish.html)

## 12. Dashboard and project-site companion

Prepare a separate Noodle PR after the package identity is settled, following that repository's instructions. Current collector/test paths are `noodle:state/ci-cluster/monitoring/slackblocks_exporter.py` and `noodle:state/ci-cluster/monitoring/test_slackblocks_exporter.py`; metric definitions live in `noodle:state/ci-cluster/README.md`. It is part of release-completion visibility, not a dependency for implementing the Elixir core.

- Extend the Slackblocks exporter with Hex package/version metadata and its authoritative all-time downloads field. Preserve its actual counting definition and cache/freshness behavior.
- Add the ninth language version and Hex download/trend panels to the dashboard. Keep the removed Java release cards removed.
- Make Hex the **seventh** required all-time download source only after a real stable package is published and collection works. Before that, disabled/unpublished is not a measured zero. Preserve fail-closed completeness/freshness behavior and the existing private dashboard boundary.
- Continue exposing only the public aggregate badge metric. Recalculate and verify the sum of the seven provider counters; update README coverage prose after deployment.
- Review ndl.au's project registry links/language list for Hex/Elixir. Its total already consumes the shared aggregate, so avoid adding Hex again on the website. The project's visible count should remain “downloads”.
- Tests cover unavailable/retired/prerelease-only metadata, actual zero, malformed counters, a single provider failure and exact aggregate completeness. Confirm actual production metrics and public/private routes after deployment and after the project consumer's five-minute cache expiry. Verify badge alt text and the README Download statistics source definition alongside the public aggregate; do not just add a dashboard panel.

The public Hex API includes release and download fields; verify their semantics and update cadence against its current docs before adding them to the maintained metric definition. [Hex API](https://hex.pm/docs/api)

## 13. Reviewable PR train

Implementation is authorized. Prepare these PRs in order and leave them open for review; merging and release remain separate approval gates:

| Stage | Scope | Exit gate |
| --- | --- | --- |
| **PR 1 — native API checkpoint and foundation** | Reviewed plan/inventory, manifest, minimal representative structs, errors/JSON, initial generator, local checker, hosted CI and actual tarball consumer; release disabled | Consumer on minimum/current; construction/editing/context/JSON decisions reviewed; prototype docs render |
| **PR 2 — complete modeled contract** | All 89 types, all fields/roles/vocabularies, scalar/contextual rules, checked ingress, complete valid/invalid construction registries | Exact fixture/capability/limit sets; empty skip list; Unicode/numeric/opaque-data regressions |
| **PR 3 — helper and quality parity** | Components, colors/workflow/builder URL, native editing, independent exports/defaults/roles audit, property tests, handwritten coverage and artifact modes | Every helper disposition resolved; full native gates and mutation probes pass |
| **PR 4 — sending integrations** | Erlang/Rebar3 consumer and packaged release, community-client JSON path, direct Req, webhook and documented Phoenix boundary; isolated locks and local HTTP tests | Full payload preserved; bounded errors/retries; minimum/current and resolution modes execute |
| **PR 5 — disabled release integration and version preparation** | Freeze outgoing 2.7.0 first; atomically prepare all nine 2.8.0 versions, docs version guard, shared release registry/activation gate and Hex workflow; credential-free artifact/docs checks plus protected dry-run procedure | Eight-package old-version state and nine-package disabled new-version state both tested; partial trains rejected before tags; native package/ExDoc ready; publication remains disabled |
| **PR 6 — full documentation and final parity** | Root/package README accordions/badges, official logo, complete ExDoc/Docusaurus/examples, all language maps, dependency maintenance and required-check instructions | Full docs/all existing snippet suites; browser/history/search/GitHub README review; final inventory, dependency/API/coverage guards pass; publication remains disabled |
| **Companion PR — Noodle visibility** | Hex provider/dashboard/project registry entry, initially disabled | Provider/aggregate tests; no premature zero or double counting |
| **Separate release-readiness change** | Verified account/auth setup, required checks, final dated notes/versions, activation and clean master rebase | Complete original-inventory audit and explicit release approval; then registry installs, docs and monitoring activation |

PRs are stacked in dependency order, each describing its actual delivered scope. Retarget descendants before deleting predecessor branches. Rebase and rerun affected gates when master or the shared model changes. Do not merge a partial train into an active release set. Readiness is demonstrated by evidence; no day estimate is assigned before the API/JSON/integration checkpoints.

## 14. Audit findings applied to this plan

| Earlier problem or Elixir-specific risk | Planned prevention |
| --- | --- |
| Ruby fixtures effectively testing one implementation against itself | Handwritten native fixture registry, independent expected JSON, exact ID sets |
| Missing README badges, examples, reference/changelog/layout entries | Explicit README inventory and PR 6 review gate |
| Missing handwritten comments/default values in generated references | Native doc/typespec extraction plus independent API/default inventory |
| Ruby/Go/Rust nonstandard logo/sizing fixes after release | Official logo and attribution, aspect-ratio and shared selector checks |
| Historical language routing and selectors inconsistent after changes | Outgoing snapshot before edits, stored-preference/direct-URL/hydrated browser tests |
| Rust tests conditionally skipping failed valid edits | Explicit success assertions plus deliberate broken-edit/producer probes |
| Rust/PHP metadata incorrectly traversed as Block Kit | Typed-edge validators; adversarial metadata at every public boundary |
| PHP reference/coercion bypasses | Elixir immutability is native; direct edited/fabricated struct, charlist, atom and collection checks cover the corresponding boundaries |
| JSON number scanning or float conversion losing information | Decimal-preserving lexical ingress; targeted precision/underflow/long-string cases; no regex tokenizer |
| SDK adapter silently dropping/coercing nested data | Actual HTTP observation through the selected SDK/Req send path |
| Library lockfile hiding unsupported downstream dependencies | Fresh artifact consumers and separate integration resolution matrices |
| Missing new required checks in branch protection | Explicit stable checks, current 51 preserved, relevant/unrelated live job evidence |
| First-publish account assumptions and registry bootstrap differences | Hex name check, explicit key bootstrap, documented OIDC verification gap, activation held until ready |
| PHP release-list eventual consistency | Bounded visibility waits with idempotent identity checks and regression tests |
| Monitoring lag or double counting | Stable-version activation gate, completeness test, seven-provider exact sum, one aggregate consumed by the site |

## 15. Acceptance and remaining decisions

The implementation is ready for review only when the complete inventory has statuses and evidence, rather than just a large generated diff. Release completion additionally requires actual publication and registry consumers.

- [ ] Native checkpoint reviewed; minimum/runtime/dependency and naming decisions confirmed.
- [x] Every live model/field/role, valid/invalid ID, capability, limit and vocabulary is covered.
- [x] Helpers, aliases and exclusions have explicit dispositions; release skip list empty.
- [x] Local versus complete validation, required/optional parse defaults, duplicate keys, role dispatch and forged structs have explicit passing/failing tests.
- [x] Tagged APIs and direct Jason exception behavior are documented/tested; both Decimal majors and current Ecto resolve; numeric policy is proven on minimum/current OTP.
- [x] Independent public API/default coverage, docs/comments and meaningful handwritten coverage pass.
- [x] Actual Hex archive and fresh consumers pass minimum/current, clean and lower-bound modes.
- [x] Erlang example, Rebar3 artifact consumer, consolidated protocols and standalone OTP release pass minimum/current runtimes.
- [x] Real local HTTP sending tests execute from displayed source; transport remains optional to core.
- [x] Both license texts/metadata ship; production consumers need no dev tools, Git or monorepo files.
- [x] Pre-publication docs, collapsed README accordions, both badge rows/palette, official logo, rendered ExDoc source links, actual snippets and historical browser checks pass.
- [ ] Stable hosted checks and branch protection are validated on relevant and unrelated changes.
- [x] Nine-language versions and disabled strict guards advance atomically; real Mix declarations/lock semantics and partial-failure tests pass.
- [x] PR artifact/docs checks require no credentials and verify committed package inputs and rendered source links.
- [ ] Protected full dry run and registry readiness are verified before coordinated tags.
- [ ] Hex ownership/environment/bootstrap credentials are verified; OIDC configuration is independently proven before claiming tokenless operation.
- [ ] Maintainer authorizes merge/release; all publishers and exact registry installs succeed.
- [ ] Post-release badges/HexDocs and dashboard/project entry are activated and verified.

**Decisions exercised by the implemented checkpoint (maintainer review pending):** validate the minimum OTP policy and proposed API ergonomics; prove checked decimal-to-float/resource behavior on both Decimal majors; confirm dependency bounds; demonstrate independent export discovery and complete handwritten-module coverage accounting. The proposed defaults above give the implementation a concrete starting point.

**Account action needed later:** Hex ownership and the protected publishing environment/key. No account setup, credentials or release approval is needed to review this plan.

**Evidence limits:** The original planning audit was read-only. Implementation now compiles and tests the native API, actual archives, minimum/current consumers, consolidated Erlang releases, local HTTP integrations and docs; see [checkpoint.md](checkpoint.md). It has not authenticated to Hex, reserved a name, published, sent a real Slack message or changed branch protection. Protected readiness, OIDC activation (if adopted), production monitoring and registry installs remain explicit post-approval gates.
