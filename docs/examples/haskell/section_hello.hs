{-# LANGUAGE OverloadedStrings #-}
import Data.Aeson (encode)
import qualified Data.ByteString.Lazy.Char8 as LBS
import Slackblocks

main :: IO ()
main = either (fail . show) (LBS.putStrLn . encode) $ do
  text <- buildMarkdownText (markdownText "Hello from slackblocks!")
  section <- buildSectionBlock (sectionBlock {sectionBlockText = Just (markdownTextAsText text), sectionBlockBlockId = Just "hello"})
  buildMessagePayload ((messagePayload "C0123456") {messagePayloadBlocks = Just [sectionBlockAsBlock section]})
