module Main (main) where

import Control.Monad (unless)
import Data.Aeson (ToJSON (..), object, (.=))
import Data.Maybe (isNothing)
import Data.Text qualified as T
import Slackblocks

assert :: String -> Bool -> IO ()
assert label ok = unless ok (fail label)

main :: IO ()
main = do
  let payload = do
        label <- buildPlainText ((plainText "Open") {plainTextEmoji = Just False})
        action <- buildButtonElement ((buttonElement label) {buttonElementActionId = Just "open"})
        text <- buildMarkdownText (markdownText "*Hello*")
        section <- buildSectionBlock (sectionBlock {sectionBlockText = Just (markdownTextAsText text), sectionBlockAccessory = Just (buttonElementAsElement action)})
        buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just [sectionBlockAsBlock section]})
  message <- either (fail . show) pure payload
  assert "typed complete payload" $
    toJSON message
      == object
        [ "channel" .= ("C123" :: T.Text),
          "text" .= ("" :: T.Text),
          "mrkdwn" .= True,
          "blocks"
            .= [ object
                   [ "type" .= ("section" :: T.Text),
                     "text" .= object ["type" .= ("mrkdwn" :: T.Text), "text" .= ("*Hello*" :: T.Text)],
                     "accessory"
                       .= object
                         [ "type" .= ("button" :: T.Text),
                           "action_id" .= ("open" :: T.Text),
                           "text" .= object ["type" .= ("plain_text" :: T.Text), "text" .= ("Open" :: T.Text), "emoji" .= False]
                         ]
                   ]
               ]
        ]
  let empty = buildPlainText (plainText "")
  assert "empty text rejected" $ either (all ((== LengthExceeded) . errorCategory)) (const False) empty
  boundary <- either (fail . show) pure $ buildPlainText (plainText (T.replicate 3000 "😀"))
  assert "Unicode counts code points" $ plainTextText (inspectPlainText boundary) == T.replicate 3000 "😀"
  assert "Unicode over-limit rejected" $ either (const True) (const False) $ buildPlainText (plainText (T.replicate 3001 "😀"))
  assert "checked editing" $ either (const True) (const False) $ buildPlainText ((inspectPlainText boundary) {plainTextText = ""})
  label <- either (fail . show) pure $ buildPlainText (plainText "Ask")
  assert "display requires prompt" $
    either (all ((== MissingRequired) . errorCategory)) (const False) $
      buildButtonElement ((buttonElement label) {buttonElementAgentPromptDisplay = Just "Summary"})
  assert "empty prompt is present" $
    either (const False) (const True) $
      buildButtonElement ((buttonElement label) {buttonElementAgentPrompt = Just "", buttonElementAgentPromptDisplay = Just "Summary"})
  divider <- either (fail . show) pure $ buildDividerBlock dividerBlock
  item <- either (fail . show) pure $ buildLayoutItem ((layoutItem (dividerBlockAsLayoutChild divider)) {layoutItemWidth = Just LayoutWidthWide, layoutItemHorizontalAlignment = Just LayoutAlignmentEnd})
  row <- either (fail . show) pure $ buildLayoutRow (layoutRow [item])
  grid <- either (fail . show) pure $ buildLayoutBlock (layoutBlock "Grid" [row])
  _ <- either (fail . show) pure $ buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just (replicate 50 (layoutBlockAsBlock grid))})
  _ <- either (fail . show) pure $ buildHomeTabView (homeTabView (replicate 100 (layoutBlockAsBlock grid)))
  changed <- either (fail . show) pure $ buildLayoutBlock ((inspectLayoutBlock grid) {layoutBlockFallback = Just (replicate 51 (dividerBlockAsLayoutFallback divider)), layoutBlockIsLabelVisible = Just False})
  _ <- either (fail . show) pure $ buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just [layoutBlockAsBlock changed]})
  assert "layout omission preserved by editing" $ isNothing (layoutBlockFallback (inspectLayoutBlock grid))
  assert "layout false preserved" $ layoutBlockIsLabelVisible (inspectLayoutBlock changed) == Just False
  assert "layout typed width" $ layoutItemWidth (inspectLayoutItem item) == Just LayoutWidthWide
  assert "layout native decoding" $ (fromValue (toJSON changed) :: Result LayoutBlock) == Right changed
  assert "spec identity" $ specVersion == "1.5.0"
  putStrLn "Foundation construction, wire, Unicode, editing and prompt controls passed."
