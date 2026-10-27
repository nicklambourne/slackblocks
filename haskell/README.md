# slackblocks for Haskell

Typed, immutable Slack Block Kit composition with checked construction and Aeson
encoding. Requires GHC 9.6.7 or newer within the tested compiler range.

This package is unreleased. The native API covers all 89 shared models, closed
roles and enumerations in contract 1.4.0. It is not part of the active
coordinated publisher; helpers, expanded quality checks and documentation are
being completed in the dependent PR train.

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

`fromValue` and `decode` check JSON at the ingress boundary. Typed child roles
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
`cabal check`, a source archive and a fresh archive consumer.
