# slackblocks for Haskell

Typed, immutable Slack Block Kit composition with checked construction and Aeson
encoding. Requires GHC 9.6.7 or newer within the tested compiler range.

This package is unreleased. The native API covers all 89 shared models, closed
roles and enumerations in contract 1.4.0. It is not part of the active
coordinated publisher. Packaging integrations and complete site documentation
are being completed in the dependent PR train.

```haskell
{-# LANGUAGE OverloadedStrings #-}
import Data.Aeson (encode)
import Slackblocks

example = do
  label <- buildPlainText (plainText "Open")
  action <- buildButtonElement
    ((buttonElement label) {buttonElementActionId = Just "open"})
  body <- buildMarkdownText (markdownText "*Hello*")
  section <- buildSectionBlock
    (sectionBlock {sectionBlockText = Just (markdownTextAsText body),
                   sectionBlockAccessory = Just (buttonElementAsElement action)})
  buildMessagePayload
    ((messagePayload "C123")
      {messagePayloadBlocks = Just [sectionBlockAsBlock section]})

-- Handle Left errors or send encode payload on Right payload.
```

Field records are construction inputs, not wire values: they have no `ToJSON`
instance. Checked values have hidden constructors. Inspect a value, edit its
fields, then rebuild to validate the change. Optional `Nothing` omits a field;
explicit `Just False` and empty text remain present where the contract allows
them. Block IDs are omitted unless supplied. Validation reports a category,
structured path, constraint and explanation without throwing exceptions.

`fromValue` and `decode` check JSON at the ingress boundary. `decode` also rejects
duplicate object keys (including escaped spellings and opaque metadata), invalid
UTF-8 and trailing input. Aeson's `FromJSON` instances validate models too, but
cannot recover duplicate keys already collapsed by an external Aeson parser.
The byte decoder also rejects numeric exponent overflow before Aeson can narrow
an exponent to `Int`. Checked native values and JSON ingress reject numbers whose
normalized encoding would overflow or fail to round-trip, including numbers in
metadata and extensions. This is a
`Scientific` representation boundary, not a Slack magnitude limit; decimal bounds
supplied as strings keep their arbitrary-exponent support. Already parsed Aeson
values cannot recover an original exponent that an external parser truncated.
Integer fields with shared bounds are range-checked before expanding exponents,
so oversized inputs fail without first allocating an enormous integer.
Use `Slackblocks.decode` when accepting untrusted JSON bytes. Typed child roles
reject unsupported elements, while metadata and explicit unmodeled extension
fields preserve arbitrary JSON. Extensions cannot override modeled fields or
the `type` discriminator. Unknown enum values and receiving-element style flags
are rejected. JSON null is treated as absence for an optional field and as a
missing value for a required field.

Use `buildPlanTask (taskCardBlock id title TaskStatusPending)` for a pending task
inside a plan. `buildTaskCardBlock` checks standalone cards and rejects pending.
A `PlanTask` cannot be placed in a message's block list. Number-input bounds are
decimal strings; `decimalText` converts `Scientific` values without going
through a binary floating-point number. Exponents are compared without expanding
them into potentially enormous integers.

Run `cabal test all` from the package directory. Tests use local data and make no
Slack requests. The package version is the proposed coordinated 3.1.0 release;
the independent shared contract version is 1.4.0.

The conformance suite constructs all 113 valid fixtures through native APIs and
independently checks their wire objects. It also checks every invalid input at
JSON ingress, native invalid constructions where representable, and compiler
rejections where the type system prevents construction. `python bin/check`
adds generated-source drift, bundled-spec parity, compilation boundaries,
`cabal check`, a source archive and a fresh archive consumer. Install the source
tools separately with `cabal install ormolu-0.7.7.0 --ignore-project` and
`cabal install hlint-3.8 --ignore-project` before running the full checker.

The full checker requires deterministic formatting and generation, HLint,
independent compiled export discovery and at least **95% handwritten expression
coverage** across validation, wire handling, strict JSON, decimal comparison,
style and helpers. HPC reports branches and declarations separately; derived
`Eq`/`Show` declarations do not stand in for behavior tests. The two generated
modules remain in native/conformance and compiled API checks, but are excluded
from the handwritten percentage. All other production modules are discovered
from source and compiled coverage receipts; missing modules fail the gate.
Negative controls prove that unclassified exports and missing/zero coverage fail.
Five randomized properties each run 500 cases with a recorded reproducible seed.

`accordionSection` returns editable container fields; build each section and use
`accordion` to check and preserve their order. `paginate` uses an explicit action
ID prefix and page settings, adds previous/next controls and an optional indicator,
and leaves expanded surface limits to checked payload construction. It does not
send messages or store page state. `workflowFromURL`, named/hex/RGB colors and
`blockKitBuilderURL` are pure helpers. Builder links accept checked payloads or an
explicit `BuilderJSON` object; they do not make network requests.

The supported compiler series are GHC 9.6 through 9.14. Compiler requirements
belong to this package, not to unrelated Slack clients:

| GHC | CI patch | Platforms | Checks |
| --- | --- | --- | --- |
| 9.6 | 9.6.7 (minimum) | Linux, macOS, Windows | Native, archive, transport; Linux dependency/quality gates |
| 9.8 | 9.8.4 | Linux | Every native suite |
| 9.10 | 9.10.3 | Linux | Every native suite |
| 9.12 | 9.12.4 | Linux | Every native suite |
| 9.14 | 9.14.1 | Linux, macOS, Windows | Native, archive, transport; Linux dependency gates |

Cabal 3.16.1.0 is the pinned development tool. The package uses Cabal syntax 3.0
and GHC2021. Tested lower dependency bounds are Aeson 2.2.0.0, bytestring
0.11.5.4, text 2.0.2, containers 0.6.7, scientific 0.3.7.0 and vector 0.13.0.0.
QuickCheck is test-only. Exact lower, committed locked and latest-within-bounds
resolutions run independently; both Aeson ordered and unordered KeyMap modes
execute every native suite. The library does not impose its development lock on
downstream consumers. Resolution receipts record actual versions and flags.

`python bin/package-check` extracts the actual source distribution outside the
checkout, executes every bundled suite, builds Haddock, then builds two fresh
consumers against the tarball. Neither consumer has a workspace override or
access to the monorepo's spec. One checks the public API, encoding, validation
and editing; the other checks real HTTP request bytes against a local server.
Run `python bin/dependency-check locked`, `lower`, or `latest` to inspect a
specific dependency profile; `--unordered-keymap` selects the alternative Aeson
representation. `lower` uses GHC 9.6.7 and `locked` has endpoint compiler profiles.

The separate same-commit quality job owns source tools, export discovery and HPC.
The aggregate native status requires native, quality, dependency, intermediate
compiler and sending jobs. The default local `python bin/check` executes all
applicable profiles and archive/transport checks as well as the quality gates.

For native sending, see [the runnable HTTP integration](integrations/README.md).
HTTP and TLS packages remain outside the library's dependencies. This is an
application example, not a credential store or an automatic retry service.
