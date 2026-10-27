{-# LANGUAGE OverloadedStrings #-}

-- Native construction only. Emit JSONL for the independent structural checker.
import Control.Monad (forM)
import Data.Aeson (Value, encode, object, toJSON, (.=))
import qualified Data.ByteString.Lazy.Char8 as LBS
import Data.Text (Text)
import Slackblocks

payloads :: Result [Value]
payloads = do
  optionText <- buildPlainText (plainText "One")
  option <- buildSelectOption (selectOption (plainTextAsText optionText) "one")
  static <-
    buildStaticMultiSelectElement
      ( staticMultiSelectElement
          { staticMultiSelectElementActionId = Just "choose",
            staticMultiSelectElementOptions = Just [option]
          }
      )
  external <-
    buildExternalMultiSelectElement
      ( externalMultiSelectElement
          { externalMultiSelectElementActionId = Just "choose",
            externalMultiSelectElementMinQueryLength = Just 0
          }
      )
  users <- buildUserMultiSelectElement (userMultiSelectElement {userMultiSelectElementActionId = Just "choose"})
  conversations <- buildConversationMultiSelectElement (conversationMultiSelectElement {conversationMultiSelectElementActionId = Just "choose"})
  channels <- buildChannelMultiSelectElement (channelMultiSelectElement {channelMultiSelectElementActionId = Just "choose"})
  label <- buildPlainText (plainText "Choose values")
  body <- buildMarkdownText (markdownText "Choose values")
  title <- buildPlainText (plainText "Multi-select probe")
  submit <- buildPlainText (plainText "Save")
  let families :: [(Text, Element, InputElement)]
      families =
        [ ("static", staticMultiSelectElementAsElement static, staticMultiSelectElementAsInputElement static),
          ("external", externalMultiSelectElementAsElement external, externalMultiSelectElementAsInputElement external),
          ("users", userMultiSelectElementAsElement users, userMultiSelectElementAsInputElement users),
          ("conversations", conversationMultiSelectElementAsElement conversations, conversationMultiSelectElementAsInputElement conversations),
          ("channels", channelMultiSelectElementAsElement channels, channelMultiSelectElementAsInputElement channels)
        ]
  fmap concat $ forM families $ \(family, element, inputElement) -> do
    actions <- buildActionsBlock ((actionsBlock [element]) {actionsBlockBlockId = Just "choices"})
    section <-
      buildSectionBlock
        ( sectionBlock
            { sectionBlockText = Just (markdownTextAsText body),
              sectionBlockAccessory = Just element,
              sectionBlockBlockId = Just "choices"
            }
        )
    input <- buildInputBlock ((inputBlock label inputElement) {inputBlockBlockId = Just "choices"})
    forM [("actions", actionsBlockAsBlock actions), ("section", sectionBlockAsBlock section), ("input", inputBlockAsBlock input)] $ \(parent, block) -> do
      payload <-
        if parent == ("input" :: Text)
          then toJSON <$> buildModalView ((modalView title [block]) {modalViewSubmit = Just submit})
          else toJSON <$> buildMessagePayload ((messagePayload "C0123456") {messagePayloadText = Just "Choose values", messagePayloadBlocks = Just [block]})
      pure
        ( object
            [ "family" .= family,
              "parent" .= parent,
              "surface" .= (if parent == "input" then "modal" else "message" :: Text),
              "payload" .= payload
            ]
        )

main :: IO ()
main = either (fail . show) (mapM_ (LBS.putStrLn . encode)) payloads
