module Main (main) where

import Control.Monad (unless, forM_)
import Data.Aeson (Value(..), ToJSON(..), object, (.=))
import qualified Data.List.NonEmpty as NE
import qualified Data.Map.Strict as Map
import qualified Data.Text as T
import Slackblocks

assert :: String -> Bool -> IO ()
assert label condition = unless condition (fail label)

accept :: Show e => Either e a -> IO a
accept = either (fail . show) pure

reject :: String -> ErrorCategory -> [PathPiece] -> Result a -> IO ()
reject label category path value = case value of
  Right _ -> fail (label <> ": unexpectedly accepted")
  Left errors -> assert (label <> ": wrong category/path: " <> show errors) $
    any (\e -> errorCategory e == category && errorPath e == path) (NE.toList errors)

main :: IO ()
main = do
  let pending = taskCardBlock "task" "Waiting" TaskStatusPending
      pendingWire = object ["type" .= String "task_card", "task_id" .= String "task", "title" .= String "Waiting", "status" .= String "pending"]
  reject "standalone pending construction" TypeMismatch [Field "status"] (buildTaskCardBlock pending)
  reject "standalone pending ingress" TypeMismatch [Field "status"] (fromValue pendingWire :: Result TaskCardBlock)
  reject "standalone role ingress" TypeMismatch [Field "status"] (fromValue pendingWire :: Result Block)
  task <- accept (buildPlanTask pending)
  plan <- accept (buildPlanBlock (planBlock "Plan" [task]))
  assert "pending task only has plan wire form" $ toJSON task == object
    ["task_id" .= String "task", "title" .= String "Waiting", "status" .= String "pending"]
  accept (fromValue (toJSON plan) :: Result PlanBlock) >>= assert "plan roundtrip" . (== plan)
  reject "duplicate plan task IDs" InvalidUsage [Field "tasks"] (buildPlanBlock (planBlock "Plan" [task,task]))
  reject "reserved extension" InvalidUsage [Field "text"] $
    buildPlainText ((plainText "safe") {plainTextExtensions = Map.singleton "text" (String "unchecked")})
  reject "unknown style" TypeMismatch [Field "mystery"] $
    (fromValue (object ["mystery" .= False]) :: Result RichTextStyle)
  reject "style false still unsupported" TypeMismatch [Field "style",Field "highlight"] $
    buildRichTextText ((richTextText "Text") {richTextTextStyle = Just (richTextStyle {styleHighlight = Just False})})
  forM_ ["NaN","Infinity"," 1","1 ","1e","--1","١","1.2.3"] $ \bound ->
    reject ("bad decimal " <> T.unpack bound) TypeMismatch [Field "min_value"] $
      buildNumberInputElement ((numberInputElement True) {numberInputElementMinValue = Just bound})
  forM_ [("0.100000000000000000000000000001","0.100000000000000000000000000002"),
         ("-1e99999999999999999999999","-1e99999999999999999999998"),
         ("-0","0.0"),("+1.0e0","1"),(".5","1.")] $ \(lo,hi) -> do
    value <- accept $ buildNumberInputElement ((numberInputElement True)
      {numberInputElementMinValue = Just lo, numberInputElementMaxValue = Just hi})
    accept (fromValue (toJSON value) :: Result NumberInputElement) >>= assert "exact decimal roundtrip" . (== value)
  reject "nearby exact inverted decimals" OutOfRange [] $
    buildNumberInputElement ((numberInputElement True)
      {numberInputElementMinValue = Just "0.100000000000000000000000000002", numberInputElementMaxValue = Just "0.100000000000000000000000000001"})
  let opaque = object ["type" .= String "data_visualization", "blocks" .= [pendingWire], "integer" .= (9007199254740993 :: Integer)]
  webhook <- accept $ buildWebhookMessage (webhookMessage {webhookMessageMetadata = Just (Map.singleton "event_payload" opaque)})
  accept (fromValue (toJSON webhook) :: Result WebhookMessage) >>= assert "metadata remains opaque and exact" . (== webhook)
  reject "nested path" LengthExceeded [Field "blocks",Index 0,Field "text",Field "text"] $
    (fromValue (object ["channel" .= String "C123", "blocks" .= [object ["type" .= String "header", "text" .= object ["type" .= String "plain_text", "text" .= String ""]]]]) :: Result MessagePayload)
  putStrLn "Native/ingress context, exact-decimal, metadata, extension, style and error-path boundaries passed."
