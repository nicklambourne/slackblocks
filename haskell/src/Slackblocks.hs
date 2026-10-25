-- | Immutable Slack values. Field records are editable inputs; only checked
-- values implement 'ToJSON'. Block identifiers are omitted unless supplied.
module Slackblocks
  ( PlainText, PlainTextFields (..), plainText, buildPlainText, inspectPlainText,
    MarkdownText, MarkdownTextFields (..), markdownText, buildMarkdownText,
    TextObject, plainTextAsText, markdownTextAsText,
    ButtonElement, ButtonElementFields (..), buttonElement, buildButtonElement,
    SectionBlock, SectionBlockFields (..), sectionBlock, buildSectionBlock,
    Block, sectionBlockAsBlock,
    MessagePayload, MessagePayloadFields (..), messagePayload, buildMessagePayload,
    packageVersion, specVersion, module Slackblocks.Validation
  ) where

import Data.Aeson (ToJSON (..), object, (.=))
import qualified Data.Aeson.Key
import Data.Aeson.Types (Pair)
import Data.Text (Text)
import qualified Data.Text as T
import Slackblocks.Validation

packageVersion, specVersion :: Text
packageVersion = "3.1.0"
specVersion = "1.4.0"

optional :: ToJSON a => Text -> Maybe a -> [Pair]
optional key = maybe [] (\v -> [fromStringKey key .= v])
  where fromStringKey = Data.Aeson.Key.fromText

checkLength :: Text -> Int -> Int -> Text -> Result ()
checkLength key lo hi value
  | T.length value < lo = failure LengthExceeded [] (key <> ".min_length") "text is too short"
  | T.length value > hi = failure LengthExceeded [] (key <> ".max_length") "text is too long"
  | otherwise = Right ()

data PlainTextFields = PlainTextFields
  { plainTextText :: Text, plainTextEmoji :: Maybe Bool } deriving (Eq, Show)
newtype PlainText = CheckedPlainText PlainTextFields deriving (Eq, Show)
plainText :: Text -> PlainTextFields
plainText value = PlainTextFields value Nothing
buildPlainText :: PlainTextFields -> Result PlainText
buildPlainText fields = do
  atField "text" $ checkLength "text" 1 3000 (plainTextText fields)
  pure (CheckedPlainText fields)
inspectPlainText :: PlainText -> PlainTextFields
inspectPlainText (CheckedPlainText fields) = fields
instance ToJSON PlainText where
  toJSON (CheckedPlainText fields) = object $
    ["type" .= ("plain_text" :: Text), "text" .= plainTextText fields]
      <> optional "emoji" (plainTextEmoji fields)

data MarkdownTextFields = MarkdownTextFields
  { markdownTextText :: Text, markdownTextVerbatim :: Maybe Bool } deriving (Eq, Show)
newtype MarkdownText = CheckedMarkdownText MarkdownTextFields deriving (Eq, Show)
markdownText :: Text -> MarkdownTextFields
markdownText value = MarkdownTextFields value Nothing
buildMarkdownText :: MarkdownTextFields -> Result MarkdownText
buildMarkdownText fields = do
  atField "text" $ checkLength "text" 1 3000 (markdownTextText fields)
  pure (CheckedMarkdownText fields)
instance ToJSON MarkdownText where
  toJSON (CheckedMarkdownText fields) = object $
    ["type" .= ("mrkdwn" :: Text), "text" .= markdownTextText fields]
      <> optional "verbatim" (markdownTextVerbatim fields)

data TextObject = PlainTextValue PlainText | MarkdownTextValue MarkdownText deriving (Eq, Show)
plainTextAsText :: PlainText -> TextObject
plainTextAsText = PlainTextValue
markdownTextAsText :: MarkdownText -> TextObject
markdownTextAsText = MarkdownTextValue
instance ToJSON TextObject where
  toJSON (PlainTextValue value) = toJSON value
  toJSON (MarkdownTextValue value) = toJSON value

data ButtonElementFields = ButtonElementFields
  { buttonElementText :: PlainText,
    buttonElementActionId :: Maybe Text,
    buttonElementAgentPrompt :: Maybe Text,
    buttonElementAgentPromptDisplay :: Maybe Text
  } deriving (Eq, Show)
newtype ButtonElement = CheckedButtonElement ButtonElementFields deriving (Eq, Show)
buttonElement :: PlainText -> ButtonElementFields
buttonElement label = ButtonElementFields label Nothing Nothing Nothing
buildButtonElement :: ButtonElementFields -> Result ButtonElement
buildButtonElement fields = do
  atField "text" $ checkLength "button.text" 1 75 (plainTextText (inspectPlainText (buttonElementText fields)))
  mapM_ (atField "action_id" . checkLength "action_id" 0 255) (buttonElementActionId fields)
  mapM_ (atField "agent_prompt" . checkLength "button.agent_prompt" 0 4000) (buttonElementAgentPrompt fields)
  case (buttonElementAgentPrompt fields, buttonElementAgentPromptDisplay fields) of
    (Nothing, Just _) -> failure MissingRequired [Field "agent_prompt"] "button.agent_prompt_display.requires_agent_prompt" "agent_prompt_display requires agent_prompt"
    _ -> pure (CheckedButtonElement fields)
instance ToJSON ButtonElement where
  toJSON (CheckedButtonElement fields) = object $
    ["type" .= ("button" :: Text), "text" .= buttonElementText fields]
      <> optional "action_id" (buttonElementActionId fields)
      <> optional "agent_prompt" (buttonElementAgentPrompt fields)
      <> optional "agent_prompt_display" (buttonElementAgentPromptDisplay fields)

data SectionBlockFields = SectionBlockFields
  { sectionBlockText :: Maybe TextObject,
    sectionBlockAccessory :: Maybe ButtonElement,
    sectionBlockBlockId :: Maybe Text
  } deriving (Eq, Show)
newtype SectionBlock = CheckedSectionBlock SectionBlockFields deriving (Eq, Show)
sectionBlock :: SectionBlockFields
sectionBlock = SectionBlockFields Nothing Nothing Nothing
buildSectionBlock :: SectionBlockFields -> Result SectionBlock
buildSectionBlock fields = do
  case sectionBlockText fields of
    Nothing -> failure MissingRequired [Field "text"] "section.content" "section needs content"
    Just _ -> pure ()
  mapM_ (atField "block_id" . checkLength "block_id" 0 255) (sectionBlockBlockId fields)
  pure (CheckedSectionBlock fields)
instance ToJSON SectionBlock where
  toJSON (CheckedSectionBlock fields) = object $
    ["type" .= ("section" :: Text)]
      <> optional "text" (sectionBlockText fields)
      <> optional "accessory" (sectionBlockAccessory fields)
      <> optional "block_id" (sectionBlockBlockId fields)

newtype Block = SectionBlockValue SectionBlock deriving (Eq, Show)
sectionBlockAsBlock :: SectionBlock -> Block
sectionBlockAsBlock = SectionBlockValue
instance ToJSON Block where toJSON (SectionBlockValue value) = toJSON value

data MessagePayloadFields = MessagePayloadFields
  { messagePayloadChannel :: Text,
    messagePayloadBlocks :: Maybe [Block],
    messagePayloadText :: Maybe Text,
    messagePayloadMrkdwn :: Maybe Bool
  } deriving (Eq, Show)
newtype MessagePayload = CheckedMessagePayload MessagePayloadFields deriving (Eq, Show)
messagePayload :: Text -> MessagePayloadFields
messagePayload channel = MessagePayloadFields channel Nothing (Just "") (Just True)
buildMessagePayload :: MessagePayloadFields -> Result MessagePayload
buildMessagePayload fields = do
  atField "channel" $ checkLength "message.channel" 1 maxBound (messagePayloadChannel fields)
  case messagePayloadBlocks fields of
    Just blocks | length blocks > 50 -> failure LengthExceeded [Field "blocks"] "message.blocks.max_items" "too many blocks"
    _ -> pure (CheckedMessagePayload fields)
instance ToJSON MessagePayload where
  toJSON (CheckedMessagePayload fields) = object $
    ["channel" .= messagePayloadChannel fields]
      <> optional "blocks" (messagePayloadBlocks fields)
      <> optional "text" (messagePayloadText fields)
      <> optional "mrkdwn" (messagePayloadMrkdwn fields)
