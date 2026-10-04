# Rust implementation and integration audit

## Current baseline

- Started 4 October 2026 from `00bfaa72bbcea31063cafdac8750ccac09665c3d`.
- Six published languages at 2.5.0; shared spec 1.2.0; proposed coordinated release 2.6.0.
- Crates.io name lookup: no exact `slackblocks` crate on 4 October. This does not reserve the name.
- PR train is unmerged; publication and release activation remain disabled.
- Followed `docs/docs/contributing/adding-a-language.mdx`, including an API checkpoint before bulk generation.

## API checkpoint evidence

- 25 representative/supporting types and 8 receiving roles in the foundation.
- External-facade example compiles and runs on Rust 1.85.0, 1.97.1, and current stable 1.99.0.
- Owned fields, borrowed getters, consuming setters, iterator replacement/append semantics, safe editing, contextual text coercion and defaults are exercised.
- A pending task builds locally and serializes tagless within a plan; a message rejects it outside a plan with the complete path.
- Plain tables preserve ragged rows; data tables validate their separate structure.
- Block/Element roles use boxed concrete variants to bound enum size without requiring callers to box values. The checkpoint asserts the Block layout stays small.
- Integers above 2^53 remain exact; nonfinite values are rejected.
- Clippy and rustdoc run with warnings denied. Seven generator negative/drift tests and four focused runtime regression tests pass.
- Foundation tests are explicitly scoped; the full shared corpus belongs to PR 2.

## Lessons incorporated from previous implementations

| Earlier follow-up | Rust acceptance evidence |
| --- | --- |
| Ruby tests initially lacked independent native fixture construction (#374) | Checked-in native constructions separate from parsing and generated code; exact registry sets |
| Ruby guide examples differed from documented JSON (#375) | Compile examples extracted from real Markdown/MDX and compare shared example output |
| Ruby CI was added after the initial train (#376) | Foundation contains hosted platform/MSRV CI and an installed archive consumer |
| Root README badges/examples and less obvious guides were missed (#378) | Explicit root/package README, badges, all language panels, comments/reference, search/version/nav inventory |
| Language introduction could pollute outgoing docs history | Snapshot six-language 2.5.0 before current docs add Rust; verify historical languages |
| Release account/bootstrap assumptions needed clarification (#377) | Disabled publisher with explicit first-publish bootstrap, later OIDC, and independent account setup gate |
| Stacked PRs can close when predecessor branches are deleted | Keep branches; retarget successors before deletion during a separately authorized merge |

## Parity inventory

The registry checks will derive exact ID sets rather than freeze audit totals.

| Item | Implementation | Tests | Documentation | Status |
| --- | --- | --- | --- | --- |
| Types/fields/roles/defaults | `src/generated/`, generator naming map | API checkpoint then conformance | rustdoc/reference | in progress |
| Validation and checked ingress | `src/wire.rs`, `src/rules.rs` | invalid corpus, boundary/presence tests | errors/validation guide | in progress |
| Helpers and native editing | consuming builders and `src/components.rs` | helper, ownership, property tests | cookbook and examples | planned |
| Public API coverage | explicit facade, independent `syn` audit | mutation guard | public symbol inventory | planned |
| MSRV/platform/package | Cargo and hosted CI | MSRV/stable, archive and fresh consumers | package README | in progress |
| Docs and root README | current guides, site registries and badges | extracted snippets/site guards | all current pages | planned |
| Coordinated release | Rust publisher and seven-language guards | positive/negative dry runs | releasing and recovery guide | planned; activation disabled |

The following reviewed design remains the contract for this train. References to
29 September observations are historical; use the baseline above when executing.

---

# Rust support implementation plan

29 September 2026. Revised after a source and design audit. This is a plan; no Rust implementation exists yet. Implementation, merging, and publication are separate actions.

## 1. Objective, baseline, and acceptance criteria

Add a Rust `slackblocks` crate with a native, typed API and parity across the shared Slack contract, useful composition helpers, validation, documentation, testing, packaging, and releases. Rust ownership, enums, builders, iterators, `Result`, Serde, and rustdoc should determine its API. Existing language implementations supply behavioral evidence, not a class hierarchy to translate mechanically.

The audit used the six-language 2.5.0 source at commit `a94480c` in [the historical six-language audit checkout](the historical six-language audit checkout). That checkout includes the documentation changes in [PR #378](https://github.com/nicklambourne/slackblocks/pull/378), which was still open when audited. The main local checkout is older and must not supply the implementation baseline. Rebase implementation on current master and incorporate the final #378 result.

| Registry at spec 1.2.0 | Observed coverage |
| --- | --- |
| Value types | 89: 21 blocks, 27 elements, 35 objects, 6 payloads |
| Role interfaces / closed wire vocabularies / field kinds | 11 / 8 / 15 |
| Valid fixtures / invalid cases | 105 / 167 |
| Scalar limit leaves / registered capabilities | 142 / 85 |

These counts describe the audit, not constants to put in tests. Discover and compare complete ID sets from the current registries. Reconfirm the release, spec version, registries, and crate name when implementation begins.

The first Rust release belongs to the next coordinated 2.x release, tentatively **2.6.0** if 2.5.0 remains latest. No placeholder crate publication, separate Rust stable version, or unnecessary spec bump. A contract change requires a spec change and coordinated updates in all implementations. The initial 29 September lookup and the repeated 4 October `cargo info slackblocks` lookup found no exact crate; the name is not reserved. Recheck availability before finalizing public links and immediately before publication.

| Area | Required evidence before release |
| --- | --- |
| Contract | Every valid fixture built natively and parsed independently; every invalid case rejected with its normative category; every capability and scalar limit accounted for; empty release skip list. |
| Rust API | Typed construction, inspection, editing, composition, errors, serialization and validated parsing; downstream examples compile; no foreign-language naming or mutation requirements. |
| Helpers | Accordion, AccordionSection, Paginator, Block Kit Builder URLs, workflow shortcut, and attachment color conveniences covered by native tests. |
| Quality | MSRV and current stable verified; Linux, macOS, Windows; formatting, lints, unit/integration/doctests, compile-fail checks, generator checks, coverage and clean consumers green. |
| Documentation | Complete current guides, API reference, runnable examples, language/search/version integration, package/root READMEs, contributing and releasing instructions. Historical snapshots remain accurate. |
| Package | Extracted archive builds and its examples/doctests run without the checkout, Python, spec files, or generator. License, metadata and package contents verified. |
| Release | Six stored package versions, seven changelogs and seven tags agree; Go retains its correct major module path. First publication, recovery and later trusted publishing are explicitly covered. |

## 2. Rust API decisions

### Crate and compatibility

Use one published library crate rooted at `../rust`, Rust 2024 edition, `std`, and initial MSRV **1.85**. Verify that MSRV rather than inferring it from the edition. An increase needs a documented compatibility decision and release note. Transport clients, async runtimes, `no_std`, proc macros and custom derive syntax are outside this first release.

Use owned values with private fields; `Clone`, `Debug`, and `PartialEq` where meaningful. Add `Eq`/`Hash` only when the data supports them, and verify `Send + Sync` on representative public types. Forbid unsafe code. Avoid public lifetimes throughout the Block Kit graph and prevent mutation that bypasses validation. Do not expose internal wire structs or builder storage.

Model roles are enums with concrete payload variants and generated `From<Concrete>` conversions. Mark evolving public role enums, vocabulary enums and `ErrorCategory` **`#[non_exhaustive]` from the first release**. This permits future variants while preserving downstream source compatibility. Unknown wire values still fail validation; non-exhaustiveness does not create an arbitrary JSON variant. Adding variants to an exhaustive enum, or adding this attribute later, can break callers. [Cargo SemVer guidance](https://doc.rust-lang.org/cargo/reference/semver.html#major-adding-new-enum-variants-without-non_exhaustive)

Keep public modules small and intentional. Prefer explicit re-exports from private implementation modules; avoid wildcard exports and unnecessary public traits. Use enum composition instead of `Box<dyn Trait>`. Check the size of representative nested types and resolve any recursive-layout or large-enum problems during the prototype, before committing to public variant shapes.

### Naming and everyday use

Maintain a reviewed mapping from every model name and wire field to its Rust name. Use `UpperCamelCase` types/variants and `snake_case` methods. Explicitly map the model's `Option` to **`SelectOption`**, and `OptionGroup` to **`SelectOptionGroup`**, to keep the standard prelude usable. Use `RichTextText` for the inline text value and `ConfirmationDialogue` for the confirmation object; do not import historical Python aliases just to match spelling. Document the mappings in the cross-language guide.

Provide direct constructors for genuinely simple values and consuming builders for values with several fields or cross-field rules. Required arguments may belong in a builder constructor when that shortens common calls; do not generate typestate permutations. Fallible constructors and `build(self)` return `Result<T, ValidationError>`. No normal invalid user input may panic. Only infallible conversions implement `From`; use `TryFrom` for checked conversion.

The intended style is:

```rust
let section = SectionBlock::builder()
    .text("Hello, Rust!")
    .block_id("hello")
    .build()?;
let message = MessagePayload::builder("C0123456")
    .block(section)
    .build()?;
let json = serde_json::to_string(&message)?;
```

This is an API target, not tested implementation. Prototype `PlainText`, `SelectOption`, `ButtonElement`, `SectionBlock`, `MessagePayload`, a table, a pending plan task and nested rich text in an external consumer before bulk generation. Verify useful compiler diagnostics and readable generated rustdoc against the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html).

The prototype must settle these operations for every field shape:

| Operation | Required behavior |
| --- | --- |
| String setter | Accept `impl Into<String>`; own the result. |
| Typed child setter | Accept the relevant concrete value or `impl Into<Role>`; never raw JSON for a modeled child. |
| Collection setters | Singular method appends; plural method replaces using `IntoIterator`. Preserve order and document replacement. Heterogeneous vectors use explicit role conversion where inference requires it. |
| Getter | Borrow strings as `&str`, lists as slices and children as references; return optional views for optional fields and copied small scalars where appropriate. |
| Edit an existing value | `into_builder(self)` preserves all values and presence flags; clone first to retain the original. Rebuilding revalidates. No mutable getter or unchecked update path. |
| Defaults and omission | New builders apply model defaults; optional fields have explicit clear methods, including optional defaulted fields. Required fields cannot be cleared through typed setters. |
| Traits | Builders may implement `Default` where an empty builder makes sense. Validated values implement `Default` only if that result is valid; never fabricate missing required data. |

Text coercion is field-specific: a string becomes plain text or mrkdwn according to that field's rule. Explicit text values remain available where allowed. Do not add a blanket conversion from string to the heterogeneous `Text` role. Preserve false, zero and supplied empty collections, then validate their actual rules. Omit absent optional fields; unset `block_id` never gets an invented value. Pin defaults including message text/mrkdwn, response type/replace_original, data-table pagination, file source and icon-button icon.

### Helper parity and intentional language differences

| Existing behavior | Rust contract |
| --- | --- |
| Accordion / AccordionSection / Paginator | Typed helpers render validated `Vec<Block>` (a section renders a container). Users compose through iterators. Recheck parent surface and block-count limits after expansion. Pin page numbering, empty input, navigation IDs/labels, and first/last-page behavior against existing implementations. |
| Python `block_kit_builder_url` | Native helper accepts an explicit input enum for a block, a block slice, a complete supported payload, or an explicitly named raw JSON escape hatch. Wrap blocks correctly, support optional team ID and percent-encode compact UTF-8 JSON. Return errors rather than panic; test Unicode and URL delimiters. No HTTP dependency. |
| Python `Workflow.from_url` | `Workflow::from_url` or an equivalently short checked constructor takes the URL and an iterator of typed input parameters. Zero parameters omit the key, matching Python. |
| Attachment `Color` | Typed `AttachmentColor` constants/variants for semantic colors and common named hex colors, with checked custom hex input. Normalize six-digit input with a leading `#`. |
| Python message aliases and block `+` composition | Map to canonical payload types, vectors and iterators. Document equivalents instead of reproducing Python operators and legacy names. |
| Deprecated Python-only attachment `Field` | Explicit compatibility exclusion: it is outside the shared model/capability contract and is retained in Python for legacy callers. Do not claim identical coverage of every historical Python export. New shared legacy-field support would need a separate contract decision. |

Maintain a small parity inventory of shared capabilities, helpers, and justified legacy exclusions. The 85-capability registry alone does not discover useful helpers such as Builder URLs. Every supported helper gets executable documentation and behavioral tests.

## 3. Generation and handwritten rule ownership

Add the deterministic generator under `../rust/generator`. Python standard library is sufficient as a development tool. Read the model, limits, vocabulary, valid/invalid manifests and capability registry directly. Neither generation nor access to the monorepo may occur at Cargo build time.

Generate owned types/builders/accessors, role conversions, vocabularies, validation metadata, constants, `SPEC_VERSION`, rustdoc and site reference metadata from a single resolved Rust naming/type map. The model's `style` kind needs a handwritten `RichTextStyle` value and element-specific flag validation; the modeled 89 types are not the entire supporting API. Keep contextual rule code and helpers handwritten. Correct language-specific source descriptions, such as the model's Java SDK description of `Block`, through a reviewed Rust documentation mapping.

Handle all 15 current kinds explicitly: `boolean`, `double`, `enum`, `int`, `list`, `long`, `map`, `number`, `object`, `rows`, `string`, `stringList`, `style`, `text`, `textList`. Fail generation on an unknown kind, ambiguous membership, unresolved type, invalid default/enum, duplicate Rust identifier, keyword collision or wire collision. Role membership includes the implicit block/element package relationships; it cannot be inferred only from an `implements` list or discriminator.

Check generated output into source control. A `--check` mode compares exact generated path sets and contents, including missing, unexpected and stale files, without writing. Restrict cleanup to owned generated directories. Unit-test the generator's mapping/error behavior and prove its drift check fails for a removed output and a stale output. A clean `git diff` alone is insufficient because it misses untracked generated files.

Before implementing contextual validation, create a rule inventory with the source rule/fixture ID, Rust validator, and positive/negative test. Start from the shared contract and compare existing Python, Ruby and TypeScript rule code. If implementations disagree, resolve against the shared spec and official Slack documentation; do not copy an accidental behavior or silently change the shared contract.

The inventory must include:

- Required fields, field-specific string coercion, enum spellings, icon vocabulary, and every inclusive/exclusive scalar boundary.
- Section text/fields requirements and per-field lengths; select option/group exclusivity; image and Slack-file source exclusivity and file-ID syntax.
- Card content/icon rules; container title, collapse and header-divider combinations; conversation filters and dispatch triggers; number-input ranges.
- Tables and data tables: rectangular rows, allowed cell roles, column settings, headers, empty cell text, row/column indexing and pagination boundaries.
- Charts: axis categories, duplicates, series names, positive segment values, and each category represented exactly once per series.
- Rich-text style whitelists per element, nested role restrictions, lists and numeric flags.
- Plan task IDs and context-sensitive task cards. A locally valid pending task may be built for a plan; receiving message/attachment validation rejects it outside a plan, including inside containers. Plan serialization strips the task's block discriminator, and parsing restores the typed task in that context.
- Message/modal/home block vocabularies, modal submit requirements, message blocks/attachments, and markdown/data-table totals over all relevant nested content and attachments. Helpers cannot bypass these rules.
- Attachment color normalization and contextual restrictions, including prohibited extension keys such as data-table `column_settings`.

## 4. Wire format, numbers, parsing, and errors

### Serialization and validated ingress

Every public wire value implements Serde `Serialize`. Use exact keys/discriminators and omit absent optional values. Tagless composition objects and plan-task entries need deliberate representations. `ImageBlock` and `ImageElement` share `image` but remain distinct in their receiving roles. Generated manual serializers are acceptable for these cases and flattened extensions; do not turn the entire typed graph into dynamic maps.

Provide `TryFrom<serde_json::Value>` for concrete/role types and validated Serde `Deserialize`. Both paths use the same field normalization and validation routines as builders, with private wire input shapes. Do not derive unchecked deserialization on public validated structs. A Serde error may wrap validation through `Display`; `TryFrom<Value>` preserves the structured category and full path. Test nested errors through both APIs.

Define parsing presence semantics explicitly:

| Input | Result |
| --- | --- |
| Required modeled field absent/null | Required-field error; required fields with model defaults use the documented constructor/parser policy pinned by tests. |
| Optional field absent/null | Absent in the parsed value and omitted on serialization, including optional fields that a new builder would default. |
| Explicit false, zero or empty array/string | Preserved and validated; never mistaken for absence. |
| Tagged value with missing/wrong discriminator | A documented validation error; never guess a different role. Tagless values are accepted only in their known schema/context. |
| Unknown field | Preserved in the checked extension map if that context permits it. Unknown role discriminators and closed enum values remain errors. |

For the current required defaults (`FileBlock.source` and `IconButtonElement.icon`), missing input applies the model default; explicit null is `missing-required`. Optional parse absence is deliberately different from initializing a fresh builder, so an explicit omission survives `serialize -> parse -> serialize`. Round trips preserve canonical values and omission, not whitespace, key order or explicit null spellings. Serde JSON's value representation has already resolved duplicate object keys; do not promise duplicate-key diagnostics or lossless source-text parsing.

Extensions use a checked JSON object. Reject empty names and collisions with any modeled wire name or reserved discriminator, even when that modeled field is omitted. Keep nested unknown JSON fields, including null, but validate extension-sensitive contextual restrictions. Do not use extensions to implement a modeled conformance capability. Avoid Serde configurations that combine flattened fields with `deny_unknown_fields`, which Serde does not support. [Serde container attributes](https://serde.rs/container-attrs.html)

### Numeric and string contract

Count Unicode code points with `str::chars().count()`, not bytes or grapheme clusters. Rust strings are valid UTF-8; invalid UTF-8 or JSON syntax is a parsing failure, distinct from a well-formed value that violates a model rule.

Use signed fixed-width integers for modeled integer fields, with checked conversion and registry limits; avoid platform-dependent `usize` in the wire model. The initial mapping is `i64` for `int` and `long`, and finite `f64` for `double`. Helpers may use native collection indexes internally. Reject booleans and fractional inputs for integer fields, and never truncate or wrap an overflowing value.

For the three general `number` fields (chart segments, data points and raw numbers), use a small validated `JsonNumber` wrapper preserving the `serde_json::Number` integer/unsigned/finite-float representation. Provide infallible integer conversions and checked floating conversions. Do not funnel integers through `f64` or claim arbitrary-precision Python-integer support. Document the accepted range and reject an unsupported numeric representation rather than silently rounding it. Test values around 2^53 and integer boundaries as well as NaN/infinities. Verify behavior when downstream feature unification enables serde_json's `arbitrary_precision` or `preserve_order`; canonical fixture comparison must not depend on key order or lossy float conversion. Resolve any contract mismatch at the API checkpoint, before promising full parity.

### Errors

Expose `ValidationError` with private storage and stable accessors for category, path and human-readable message. Implement `Display` and `std::error::Error`; keep exact message prose outside the compatibility promise. `ErrorCategory` exposes exact wire spellings for `length-exceeded`, `out-of-range`, `mutually-exclusive`, `type-mismatch`, `missing-required`, and `invalid-usage`.

Paths identify the root Rust type, Slack wire keys and array indexes. When validating a parent, prepend its location to nested failures rather than returning an isolated child path. Error priority for a manifest case must match its normative category. Programming bugs are not caught and disguised as validation failures. Application JSON syntax errors and normal serialization errors retain their native error types.

## 5. Verification architecture

### Independent conformance and API coverage

Keep two independent valid-fixture paths: a checked-in manifest-ID map of public native Rust constructions, and parsed-wire reconstruction. Both compare parsed JSON against canonical fixtures. Native constructions may not pass fixture JSON into the parser or hide modeled fields in extensions. Compare exact registered, on-disk and constructed fixture ID sets.

Every invalid case maps to a public attempt. Prefer builders for invalid lengths, combinations and contexts representable in Rust. Use validated ingress for illegal runtime types, missing required constructor arguments and unsupported enum values. Assert exact category and relevant full path. Compile-time restrictions supplement these cases; they do not replace an invalid-fixture category assertion.

Add focused compile-fail tests for illegal roles, private field mutation and unsafe conversion paths, with positive companion examples. Use diagnostic expectations tied to a pinned stable compiler so a random syntax/import failure cannot count as success; run normal passing consumers at MSRV and latest stable. Rustdoc compile-fail examples supplement this gate. Avoid a nightly-only diagnostic feature in required stable tests.

For public API coverage, an inventory produced by the same generator as the implementation is only one input. Maintain an explicit public facade and a reviewed capability/helper mapping. An independent source-AST check of the public facade and public methods (for example a small dev-only `syn` tool) must find added handwritten API as well as generated API; map methods that produce an existing wire type to that capability, and compile external consumers for mapped symbols. Deny unreachable public items and forbid unchecked wildcard re-exports. Prove the guard fails for an unmapped public JSON-producing helper. Supporting types, errors and legacy exclusions must be named and justified. This enforces the shared specification's export coverage requirement without unstable rustdoc JSON.

The release skip list is empty and asserted empty. Compare all generated limit/vocabulary leaves and role memberships to the registries in both directions. Include guard checks for a missing construction mapping and changed limit so the harness itself is demonstrably effective.

### Focused behavior and coverage

Test ownership and getters, `into_builder`, clone independence, singular/plural collection behavior, defaults/clears/parse omission, typed text coercion, Unicode astral boundaries, numeric precision, nested paths, unknown-field restrictions, contextual image/plan parsing, styles, tables/charts, message totals, surfaces, and all helpers. Every handwritten rule has a positive and negative boundary test. Use bounded property tests for round trips, text, nested payloads and pagination with reproducible seeds or failure artifacts; avoid unbounded recursion.

Require **at least 90% line coverage and 90% region coverage separately**, both for the crate overall and the handwritten validators/parsers/helpers subset. Generated builder execution must not hide uncovered rule code. Record and review exclusions; exclude external code and test harnesses, not difficult validation branches. Use a pinned `cargo-llvm-cov` and inspect uncovered decisions. Its branch instrumentation currently requires nightly, so an optional nightly report supplements the mandatory cases; stable line/region percentages are not Python branch-coverage equivalence. [cargo-llvm-cov documentation](https://github.com/taiki-e/cargo-llvm-cov)

Required commands include `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets --all-features`, `cargo test --doc`, and `cargo doc --no-deps` with warnings/broken links denied, plus generator/registry checks and packaging. Invoke the separate conformance harness explicitly in CI; the library test command alone does not run it. Keep documentation linting (`missing_docs`, including builders/getters) enabled from the foundation. Do not add blanket lint suppression to generated code; use narrow documented exceptions only when necessary.

## 6. Package, dependencies, and CI

The crate manifest at `../rust/Cargo.toml` supplies edition/MSRV, description, repository/homepage/docs/readme, keywords/categories and `license = "MIT OR BSD-3-Clause"`; include both license texts. Derive `VERSION` with `env!("CARGO_PKG_VERSION")` and assert generated `SPEC_VERSION` against the shared manifest.

Start with Serde and serde_json as runtime dependencies; justify any addition with a required API behavior. Dev-only property/UI/coverage tooling never becomes a runtime dependency. Use normal compatible dependency requirements with tested lower bounds, not exact runtime version pins. Check in the development lockfile for reproducible CI. Tooling versions may be pinned independently, and the generator/toolchain setup is documented.

MSRV verification includes the library, shipped examples and doctests on 1.85 with compatible dependencies. Keep any newer maintainer-only tools outside that dependency path. Also run clean external consumers with fresh dependency resolution on MSRV and stable; a successful `--locked` monorepo build does not prove consumer compatibility. Test the declared lower dependency bounds deliberately, without making unstable minimum-version resolution a required Cargo feature. Dependency updates rerun these gates. [Cargo Rust-version policy](https://doc.rust-lang.org/cargo/reference/rust-version.html)

Set **`publish = false` until the final release-readiness PR**. Earlier stages use `cargo package --locked` and archive/consumer verification, not a claim that a disabled package passed `cargo publish --dry-run`. After enabling crates.io publication, require `cargo publish --dry-run --locked`. Foundation work cannot accidentally become a released partial implementation.

Use an explicit package inclusion policy. Include generated source, README, changelog, both licenses, and self-contained runnable examples. Exclude the generator, repository conformance harness, shared fixtures, site build and machine paths. Put shared-fixture conformance tests and the export audit in a separate development-only Cargo package under `../rust/conformance`, with `publish = false` and a path dependency on the library. Exclude that harness from the library archive and run it explicitly in repository CI. Library unit tests, doctests and shipped examples remain self-contained; never skip conformance silently because fixture files are absent. Rustdoc must not `include_str!` files outside the package. Inspect `cargo package --list`, extract the archive, run its doctests/examples and compile clean consumers on MSRV and stable. Cargo's own archive verification builds the package; our checks additionally exercise the shipped documentation and public use cases. [Cargo package verification](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

Verify local stable rustdoc before release, then check the real docs.rs build after publication. docs.rs currently uses nightly in its build service; local stable documentation success is not proof of docs.rs success. Only introduce docs.rs-specific settings if required, and test them. [docs.rs build environment](https://docs.rs/about/builds)

On this host, check `/Volumes/Repos`, `/Volumes/Cache` and `/Volumes/Scratch` before disk-heavy work. Keep worktrees on Repos, Cargo/dependency/build caches on Cache and disposable archive/consumer directories on Scratch. Stop if a required volume is missing; do not move another session's directories.

Add `../.github/workflows/rust.yml` in the foundation PR. Use **GitHub-provided runners** and repository action-pinning conventions. Stable tests run on Ubuntu, macOS and Windows; MSRV runs on Ubuntu. A stable quality/package job owns linting, generation, conformance, coverage and archive checks. Fresh-resolution consumers can run on Ubuntu; no need to multiply every tooling job by every platform. Beta/nightly are optional informational jobs, apart from any explicitly pinned diagnostic test toolchain.

Path detection must fail open and cover Rust source, spec changes, examples, docs generation, release/version inputs and workflow dependencies. Preserve stable required check names even for unrelated PRs. Verify actual runner metadata after the first workflow run. Register required checks once they exist on master, early in the train. Add Cargo dependency-update configuration and documented local commands. Changes to shared generation or docs infrastructure run affected existing-language checks too.

## 7. Documentation and version sequencing

Add the Rust package README at `../rust/README.md` with installation/MSRV, complete notification and modal examples, serialization/parsing, errors, inspection/editing, helpers, extension policy, and guide/reference links. Add crate-level rustdoc and public-item examples, including `# Errors` on fallible APIs. Use `?` with an appropriate doctest result wrapper; no untested snippets or unexplained `unwrap()` in user paths. Doctests and site snippets must demonstrate real public signatures.

Root README parity includes crate/CI badges, Cargo installation, a Rust example, changelog/reference links and contributor commands. Extend every current language-aware guide and feature: selectors, routes, syntax highlighting, sidebars, search isolation, version support, installation, quick start, using blocks, sending messages, cookbook, troubleshooting, compatibility, contributing and releasing. Explain handoff of serialized values to an HTTP client without introducing a Slack client dependency.

Put executable site snippets under `../docs/examples/rust`. Generate the site reference from the same resolved API metadata used for Rust source and the existing reference renderer; include handwritten helper/error metadata. Check symbol links and signatures against compile-tested examples. Do not parse Rust with regex or depend on unstable rustdoc JSON. The published crate's docs remain self-contained even though the site has its own snippet harness.

**Freeze outgoing documentation before adding Rust to current documentation.** The earlier plan had this order wrong. As part of the docs transition PR:

1. Start from the verified outgoing six-language documentation, incorporating #378's final changes. Reconfirm the outgoing version from package manifests.
2. Generate and snapshot that version using the established release procedure. For the current baseline this is 2.5.0, which has no frozen snapshot yet. Verify the snapshot's language list excludes Rust.
3. Advance all stored package versions and runtime constants/lock entries to the chosen next coordinated version, and add seven matching `Unreleased` changelog headings. Go is versioned by its tag and changelog; its `/v2` module path stays unchanged for 2.6.0.
4. Add Rust to the new current docs and version support map. Until publication, clearly mark the current version/Rust package as unreleased and provide a source-checkout installation path; do not imply `cargo add` for that version already works.
5. Run full generation, snippet execution, Docusaurus build, language/search/version checks and the release-snapshot guard. Verify old routes cannot select Rust and existing snapshots are unchanged apart from the newly created outgoing snapshot.

If the crate name cannot be confirmed or the next version changes, resolve links/version inputs before this PR is finalized. Release readiness later dates changelogs and switches publication status; it does **not** take a second snapshot of docs that already contain Rust. Update the contributor/release process to preserve this ordering for later language additions.

## 8. Publisher and coordinated release

Add `../.github/workflows/publish-crates.yml`. Match the repository's tag/dispatch pattern, use environment `crates-io`, and require the Rust tag `rust/vX.Y.Z`, manifest version, changelog and checked-out commit to agree. Validate a reviewed master commit; PR paths only validate/package. Credentials are obtained only after build/test/package gates, and only the publishing job receives the needed permissions. Use concurrency to prevent two writers for the same version.

Extend the coordinator's version readers, changelog loop, atomic tag creation, explicit publisher dispatch/monitoring, path filters and recovery instructions. Update every existing publisher guard that compares coordinated versions. Rust adds a sixth stored manifest version to Python, TypeScript, Java, C# and Ruby; Go supplies the seventh package/tag. Do not invent a Go minor-version manifest field. Update runtime constants, lock entries and Java's reproducible timestamp through the existing release mechanism.

At final readiness, enable the crate only for crates.io, enable the coordinator's Rust entry, and make dispatch reject any unpublished/incomplete readiness state. Keep coordinator publication inactive for the partial train. Test matching and mismatched versions, wrong branch/tag/commit, missing release date, missing artifact, existing version, and PR/no-credential execution. A registry dry run proves packaging, not authorization to publish.

### First publication and subsequent OIDC

Current crates.io documentation requires a previously published crate before configuring trusted publishing. The initial publication therefore needs an API token. This was rechecked in the [current official trusted-publishing documentation source](https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/docs/trusted-publishing/%2Bpage.svelte), rather than relying only on the 2025 announcement. Revalidate this prerequisite before activation.

For the first real coordinated release, an owner with a verified crates.io account supplies an expiring token with the minimum available new-crate publishing permissions to the protected GitHub environment. Verify that the selected scope supports creation of a new crate; an existing-crate-only token may not. Use a separately gated bootstrap mode. Environment review follows the owner's configured rules; do not require a reviewer policy that has not been configured. Never put token values in files, logs or plan documents.

After that first publish, configure the trusted publisher for `nicklambourne/slackblocks`, workflow `publish-crates.yml`, and environment `crates-io`. Verify an OIDC exchange from the intended workflow/environment, then revoke the bootstrap token, remove its secret and disable the bootstrap path. Use the [official crates.io authentication action](https://github.com/rust-lang/crates-io-auth-action) for subsequent releases. An auth-only check does not prove that a future version has published; record its scope honestly. The bootstrap mode must not become an automatic fallback when OIDC fails.

Retain a tested failure-recovery route until the first version is verified. If an upload response is uncertain, inspect registry state before any retry. Cargo packages from source when publishing: validate the same clean tagged commit, toolchain, lockfile and package file contents that were reviewed, and record archive metadata/checksums. Do not claim a prebuilt artifact was uploaded if the workflow rebuilt it.

Before creating seven tags, verify all required CI and docs gates, artifact/consumer checks, six stored versions, seven dated changelogs, Java timestamp, crate-name availability or ownership, environment configuration, and credential mode. Release dispatch remains a separate explicit user action.

After publication, install the exact registry version in an external consumer, render/compare the sample payload, and confirm crates.io metadata, docs.rs output and GitHub release. Verify all seven publishers. If Rust alone fails after other registries succeed, retry only its publisher against the existing tag after checking whether the version is already present. If published bytes need fixing, use the next coordinated patch release; never move tags or overwrite a successful version. [Cargo publishing rules](https://doc.rust-lang.org/cargo/reference/publishing.html)

## 9. Reviewable implementation train

The design checkpoint precedes bulk generation. Keep PRs stacked, rebase on current master between stages, and merge in dependency order only when separately authorized. Every PR has green tests for its delivered scope; foundation stages cannot claim full conformance or publish.

| Stage | Deliverable | Acceptance gate |
| --- | --- | --- |
| Design checkpoint | Representative downstream prototype; naming map; getters/editing/defaults; roles/numbers/contextual parsing; helper parity inventory; package-name/MSRV/dependency check | Compile-tested idiomatic calls; unresolved model/API conflicts recorded and resolved before generation. No production completeness claim. |
| PR 1 — foundation and CI | Crate with publication disabled; representative generated API; error/limit/vocabulary infrastructure; exact drift check; explicit public facade; platform/MSRV/quality CI; package smoke | Delivered types and errors work outside the repository; check names exist; drift guard catches deliberate staleness; maintainers can regenerate locally. |
| PR 2 — complete shared contract | All model types/supporting styles and role mappings; contextual validators; Serialize, TryFrom and validated Deserialize; native/parse fixture harness; invalid cases; registry/export enforcement | Every current fixture/capability/limit covered; empty skip list; no unchecked ingress or JSON substitute for typed construction. |
| PR 3 — helper and quality parity | All helpers; accessor/editing/collection ergonomics review; properties/UI tests; handwritten coverage gates; feature-unification/fresh-resolution/archive tests; complete rustdoc | All helper/API, coverage, MSRV, stable-platform and clean-consumer gates pass. Self-audit against the API checklist and parity inventory resolves porting artifacts. |
| PR 4 — snapshot, versions and docs | Snapshot outgoing six-language docs first; advance coordinated source versions with Unreleased notes; add Rust current docs/readmes/examples/reference/selector/search; docs CI | Correct outgoing snapshot, complete current language coverage, honest unreleased status, compiled snippets and full site checks green. |
| PR 5 — release plumbing | Dry-run Rust publisher, inactive coordinator integration, scoped bootstrap/OIDC configuration and recovery instructions; all applicable version guards | Positive/negative guards tested; validation runs without credentials; no registry write; partial train cannot be dispatched. |
| PR 6 — release readiness | Enable crates.io publication and seventh dispatch; date all changelogs; update Java timestamp; switch docs release status; final package/dry-run checks and concrete release checklist | Reviewed seven-tag commit is ready, account setup verified, full required checks green. No duplicate snapshot; actual release remains a separate action. |

If master advances to another release while the train is in progress, re-evaluate the outgoing snapshot and target version before PR 4. Do not retrospectively add Rust to an already-released version.

## 10. Source evidence and audit closure

The main repository sources inspected for this revision are:

- [Shared contract](../spec/SPEC.md), [model](../spec/model.json), [limits](../spec/limits.json), [vocabulary](../spec/vocabulary.json), [capability registry](../spec/coverage.json), [valid manifest](../spec/manifest.json), and [invalid manifest](../spec/fixtures/invalid/manifest.json).
- [Python public exports](../python/slackblocks/__init__.py), [Builder URL helper](../python/slackblocks/builder.py), [workflow shortcut](../python/slackblocks/objects.py), and [legacy attachment behavior](../python/slackblocks/attachments.py).
- [Ruby wire/normalization](../ruby/lib/slackblocks/value.rb), [contextual validation](../ruby/lib/slackblocks/validator.rb), and [runtime regression tests](../ruby/test/runtime_contract_test.rb).
- [Release process](../RELEASING.md), [coordinator](../.github/workflows/coordinated-release.yml), [docs version registry](../docs/versions.json), and [repository runner instructions](../AGENTS.md).

The audit corrected enum compatibility, name collisions, inspection/editing, omitted helpers, contextual rules, default/parse presence semantics, numeric precision, independent export enforcement, coverage reporting, MSRV/consumer resolution, package self-containment, premature publication, docs snapshot ordering, and seven-package release accounting. These are now implementation requirements with acceptance gates, not claims that Rust code has passed tests. Final API signatures, compatible dependency versions and account-side publication setup are verified during their named stages.

## Guide audit findings during implementation

1. Clarify Cargo artifact consumers: a path dependency on the unpacked `.crate` is
   necessary before publication and acceptable; a path/patch to checkout sources
   would invalidate the test. Record archive inspection and fresh resolution.
2. Separate train preparation from registry activation: account setup and branch
   protection cannot be reported complete merely because a workflow file exists.
   Stable required checks will be registered only after the workflow is on master.
3. Keep the scoped API checkpoint executable in the package and exercise it on
   the minimum compiler before expanding generation.
