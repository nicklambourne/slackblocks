-- | Optional rich-text flags checked by their receiving modeled element.
module Slackblocks.Style (RichTextStyle (..), richTextStyle) where

import Data.Aeson (FromJSON (..), ToJSON (..), object)
import Data.Map.Strict qualified as Map
import Slackblocks.Internal.Wire
import Slackblocks.Json
import Slackblocks.Validation

-- | Style flags are optional: explicit false is preserved. Each receiving
-- element checks its own allowed flags, including flags supplied as false.
data RichTextStyle = RichTextStyle
  { -- | Bold text. Nothing omits the flag.
    styleBold :: Maybe Bool,
    -- | Italic text. Nothing omits the flag.
    styleItalic :: Maybe Bool,
    -- | Strikethrough text. Nothing omits the flag.
    styleStrike :: Maybe Bool,
    -- | Inline code, where accepted by the receiving element. Nothing omits the flag.
    styleCode :: Maybe Bool,
    -- | Highlight a supported mention. Nothing omits the flag.
    styleHighlight :: Maybe Bool,
    -- | Client-specific highlighting on supported mentions. Nothing omits the flag.
    styleClientHighlight :: Maybe Bool,
    -- | Disable linking on a supported mention. Nothing omits the flag.
    styleUnlink :: Maybe Bool
  }
  deriving (Eq, Show)

-- | Empty style with every flag absent. Explicit Just False remains present; the receiving element validates allowed flags.
richTextStyle :: RichTextStyle
richTextStyle = RichTextStyle Nothing Nothing Nothing Nothing Nothing Nothing Nothing

instance ToJSON RichTextStyle where
  toJSON value =
    object $
      optional "bold" (styleBold value)
        <> optional "italic" (styleItalic value)
        <> optional "strike" (styleStrike value)
        <> optional "code" (styleCode value)
        <> optional "highlight" (styleHighlight value)
        <> optional "client_highlight" (styleClientHighlight value)
        <> optional "unlink" (styleUnlink value)

instance FromSlackJSON RichTextStyle where
  fromSlackJSON value = do
    fields <- wireObject "" value
    let unknown = remaining ["bold", "italic", "strike", "code", "highlight", "client_highlight", "unlink"] fields
    case Map.keys unknown of
      [] -> pure ()
      key : _ -> failure TypeMismatch [Field key] "style.flags" "unknown style flag"
    RichTextStyle
      <$> optionalField "bold" fields
      <*> optionalField "italic" fields
      <*> optionalField "strike" fields
      <*> optionalField "code" fields
      <*> optionalField "highlight" fields
      <*> optionalField "client_highlight" fields
      <*> optionalField "unlink" fields

instance FromJSON RichTextStyle where parseJSON = checkedParser
