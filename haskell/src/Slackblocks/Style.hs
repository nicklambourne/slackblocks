module Slackblocks.Style (RichTextStyle(..), richTextStyle) where

import Data.Aeson (ToJSON(..), FromJSON(..), object)
import Slackblocks.Json
import Slackblocks.Internal.Wire
import Slackblocks.Validation
import qualified Data.Map.Strict as Map

-- | Style flags are optional: explicit false is preserved. Each receiving
-- element checks its own allowed flags, including flags supplied as false.
data RichTextStyle = RichTextStyle
  { styleBold :: Maybe Bool, styleItalic :: Maybe Bool, styleStrike :: Maybe Bool,
    styleCode :: Maybe Bool, styleHighlight :: Maybe Bool,
    styleClientHighlight :: Maybe Bool, styleUnlink :: Maybe Bool
  } deriving (Eq, Show)

richTextStyle :: RichTextStyle
richTextStyle = RichTextStyle Nothing Nothing Nothing Nothing Nothing Nothing Nothing

instance ToJSON RichTextStyle where
  toJSON value = object $
    optional "bold" (styleBold value) <> optional "italic" (styleItalic value)
    <> optional "strike" (styleStrike value) <> optional "code" (styleCode value)
    <> optional "highlight" (styleHighlight value)
    <> optional "client_highlight" (styleClientHighlight value) <> optional "unlink" (styleUnlink value)

instance FromSlackJSON RichTextStyle where
  fromSlackJSON value = do
    fields <- wireObject "" value
    let unknown = remaining ["bold","italic","strike","code","highlight","client_highlight","unlink"] fields
    case Map.keys unknown of
      [] -> pure ()
      key:_ -> failure TypeMismatch [Field key] "style.flags" "unknown style flag"
    RichTextStyle <$> optionalField "bold" fields <*> optionalField "italic" fields
      <*> optionalField "strike" fields <*> optionalField "code" fields
      <*> optionalField "highlight" fields <*> optionalField "client_highlight" fields
      <*> optionalField "unlink" fields
instance FromJSON RichTextStyle where parseJSON = checkedParser
