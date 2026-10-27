# slackblocks for Haskell

Typed, immutable Slack Block Kit composition with checked construction and Aeson
encoding. Requires GHC 9.6.7 or newer within the tested compiler range.

This foundation is unreleased and implements text, buttons, sections and message
payloads. It is not yet a complete Block Kit implementation and is not part of
the active coordinated publisher.

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
                   sectionBlockAccessory = Just action})
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

Run `cabal test all` from the package directory. Tests use local data and make no
Slack requests. The package version is the proposed coordinated 3.1.0 release;
the independent shared contract version is 1.4.0.
