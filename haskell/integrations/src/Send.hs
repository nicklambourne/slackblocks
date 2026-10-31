-- | Example application transport. The core slackblocks package has no HTTP
-- dependency; applications own credentials, endpoints, retries and routing.
module Send (ResponseKind (..), postPayload) where

import Control.Monad (unless)
import Data.Aeson (ToJSON, Value (..), eitherDecode, encode)
import Data.Aeson.KeyMap qualified as KM
import Data.ByteString qualified as B
import Network.HTTP.Client
import Network.HTTP.Types.Status (statusCode)

data ResponseKind = SlackAPI | Webhook

-- | Send an intact JSON object. API calls use a bearer token; webhook/response
-- URLs use Nothing. Do not log either credentials or credential-bearing URLs.
postPayload :: (ToJSON a) => Manager -> ResponseKind -> String -> Maybe B.ByteString -> a -> IO ()
postPayload manager kind endpoint token payload = do
  baseRequest <- parseRequest endpoint
  let request =
        baseRequest
          { method = "POST",
            requestHeaders =
              [("Content-Type", "application/json; charset=utf-8")]
                <> maybe [] (\value -> [("Authorization", "Bearer " <> value)]) token,
            requestBody = RequestBodyLBS (encode payload),
            redirectCount = 0,
            checkResponse = \_ _ -> pure ()
          }
  response <- httpLbs request manager
  let code = statusCode (responseStatus response)
  unless (code >= 200 && code < 300) (fail ("Slack HTTP status " <> show code))
  case kind of
    Webhook -> unless (responseBody response == "ok") (fail "Unexpected webhook response")
    SlackAPI -> case eitherDecode (responseBody response) of
      Right (Object result) | KM.lookup "ok" result == Just (Bool True) -> pure ()
      _ -> fail "Slack API did not confirm success"
