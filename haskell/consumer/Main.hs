{-# LANGUAGE OverloadedStrings #-}

module Main (main) where

import Control.Monad (unless)
import Data.Aeson (Value (..), encode, object, toJSON, (.=))
import qualified Data.Map.Strict as Map
import Slackblocks
import qualified Slackblocks.Helpers as Helpers
import qualified Slackblocks.Json as Json
import qualified Slackblocks.Models as Models
import qualified Slackblocks.Style as Style
import qualified Slackblocks.Validation as Validation

accept :: (Show e) => Either e a -> IO a
accept = either (fail . show) pure

main :: IO ()
main = do
  label <- accept (Models.buildPlainText (Models.plainText "Archive consumer 😀"))
  heading <- accept (buildHeaderBlock (headerBlock label))
  let fields =
        (messagePayload "C_ARCHIVE")
          { messagePayloadBlocks = Just [headerBlockAsBlock heading],
            messagePayloadMetadata = Just (Map.singleton "opaque" (object ["integer" .= (9007199254740993 :: Integer), "false" .= False, "type" .= String "future"]))
          }
  payload <- accept (buildMessagePayload fields)
  decoded <- accept (Json.decode (encode payload) :: Validation.Result MessagePayload)
  unless (payload == decoded) (fail "Archive consumer lost JSON values")
  edited <- accept (buildMessagePayload ((inspectMessagePayload payload) {messagePayloadText = Just "edited"}))
  unless (messagePayloadText (inspectMessagePayload edited) == Just "edited") (fail "Checked editing failed")
  case buildPlainText (plainText "") of
    Left _ -> pure ()
    Right _ -> fail "Invalid construction escaped validation"
  case Json.decode "{\"type\":\"plain_text\",\"text\":\"a\",\"text\":\"b\"}" :: Validation.Result PlainText of
    Left _ -> pure ()
    Right _ -> fail "Duplicate keys escaped strict ingress"
  let style = Style.richTextStyle {Style.styleBold = Just False}
  unless (toJSON style == object ["bold" .= False]) (fail "Explicit style false disappeared")
  color <- accept (Helpers.hexColor "#8800ff")
  unless (Helpers.colorText color == "#8800ff") (fail "Native color helper failed")
  putStrLn "External archive consumer: all public modules, typed payloads, exact opaque values, checked editing and invalid ingress passed."
