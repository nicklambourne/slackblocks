module Main (main) where

import Control.Exception (IOException, try)
import Control.Monad (forM_, unless)
import Data.Aeson (Value (..), object, (.=))
import Data.Map.Strict qualified as Map
import Network.HTTP.Client (newManager)
import Network.HTTP.Client.TLS (tlsManagerSettings)
import Send
import Slackblocks
import System.Environment (getArgs)

accept :: (Show e) => Either e a -> IO a
accept = either (fail . show) pure

main :: IO ()
main = do
  args <- getArgs
  endpoint <- case args of [value] -> pure value; _ -> fail "Expected local acceptance endpoint"
  manager <- newManager tlsManagerSettings
  title <- accept (buildPlainText (plainText "Hello 😀"))
  heading <- accept (buildHeaderBlock (headerBlock title))
  let blocks = [headerBlockAsBlock heading]
      metadata = Map.singleton "opaque" (object ["type" .= String "unmodeled", "integer" .= (9007199254740993 :: Integer), "flag" .= False, "empty" .= object []])
  attachmentValue <- accept (buildAttachment ((attachment blocks) {attachmentColor = Just "#8800ff"}))
  message <- accept (buildMessagePayload ((messagePayload "C_TEST") {messagePayloadBlocks = Just blocks, messagePayloadMetadata = Just metadata, messagePayloadAttachments = Just [attachmentValue]}))
  webhook <- accept (buildWebhookMessage (webhookMessage {webhookMessageBlocks = Just blocks, webhookMessageMetadata = Just metadata}))
  response <- accept (buildMessageResponse (messageResponse {messageResponseBlocks = Just blocks, messageResponseReplaceOriginal = Just False}))
  view <- accept (buildModalView (modalView title blocks))
  home <- accept (buildHomeTabView (homeTabView blocks))
  postPayload manager SlackAPI (endpoint <> "/chat.postMessage") (Just "test-token") message
  postPayload manager Webhook (endpoint <> "/webhook") Nothing webhook
  postPayload manager Webhook (endpoint <> "/response") Nothing response
  postPayload manager SlackAPI (endpoint <> "/views.open") (Just "test-token") (object ["trigger_id" .= String "trigger-test", "view" .= view])
  postPayload manager SlackAPI (endpoint <> "/views.publish") (Just "test-token") (object ["user_id" .= String "U_TEST", "view" .= home])
  forM_ ["/api-error", "/malformed", "/rate-limited", "/server-error", "/redirect"] $ \path -> do
    result <- try (postPayload manager SlackAPI (endpoint <> path) (Just "test-token") message) :: IO (Either IOException ())
    unless (either (const True) (const False) result) (fail ("Expected failure for " <> path))
  result <- try (postPayload manager Webhook (endpoint <> "/webhook-error") Nothing webhook) :: IO (Either IOException ())
  unless (either (const True) (const False) result) (fail "Expected webhook failure")
  putStrLn "Five intact payload surfaces and six HTTP/API error responses checked."
