module Main (main) where

import Control.Monad (unless)
import Data.Aeson (ToJSON (..), object, (.=))
import qualified Data.Text as T
import Slackblocks

assert :: String -> Bool -> IO ()
assert label ok = unless ok (fail label)

main :: IO ()
main = do
  let payload = do
        label <- buildPlainText ((plainText "Open") {plainTextEmoji = Just False})
        action <- buildButtonElement ((buttonElement label) {buttonElementActionId = Just "open"})
        text <- buildMarkdownText (markdownText "*Hello*")
        section <- buildSectionBlock (sectionBlock {sectionBlockText = Just (markdownTextAsText text), sectionBlockAccessory = Just action})
        buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just [sectionBlockAsBlock section]})
  message <- either (fail . show) pure payload
  assert "typed complete payload" $ toJSON message == object
    ["channel" .= ("C123" :: T.Text), "text" .= ("" :: T.Text), "mrkdwn" .= True,
     "blocks" .= [object ["type" .= ("section" :: T.Text), "text" .= object ["type" .= ("mrkdwn" :: T.Text), "text" .= ("*Hello*" :: T.Text)],
       "accessory" .= object ["type" .= ("button" :: T.Text), "action_id" .= ("open" :: T.Text),
         "text" .= object ["type" .= ("plain_text" :: T.Text), "text" .= ("Open" :: T.Text), "emoji" .= False]]]]]
  empty <- pure $ buildPlainText (plainText "")
  assert "empty text rejected" $ either (all ((== LengthExceeded) . errorCategory)) (const False) empty
  boundary <- either (fail . show) pure $ buildPlainText (plainText (T.replicate 3000 "😀"))
  assert "Unicode counts code points" $ plainTextText (inspectPlainText boundary) == T.replicate 3000 "😀"
  assert "Unicode over-limit rejected" $ either (const True) (const False) $ buildPlainText (plainText (T.replicate 3001 "😀"))
  assert "checked editing" $ either (const True) (const False) $ buildPlainText ((inspectPlainText boundary) {plainTextText = ""})
  label <- either (fail . show) pure $ buildPlainText (plainText "Ask")
  assert "display requires prompt" $ either (all ((== MissingRequired) . errorCategory)) (const False) $
    buildButtonElement ((buttonElement label) {buttonElementAgentPromptDisplay = Just "Summary"})
  assert "empty prompt is present" $ either (const False) (const True) $
    buildButtonElement ((buttonElement label) {buttonElementAgentPrompt = Just "", buttonElementAgentPromptDisplay = Just "Summary"})
  assert "spec identity" $ specVersion == "1.4.0"
  putStrLn "Foundation construction, wire, Unicode, editing and prompt controls passed."
