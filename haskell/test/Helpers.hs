{-# LANGUAGE LambdaCase #-}

module Main (main) where

import Control.Exception (evaluate)
import Control.Monad (unless)
import Data.Aeson (ToJSON (..), Value (..), encode, object, (.=))
import Data.Aeson.KeyMap qualified as KM
import Data.ByteString.Lazy qualified as L
import Data.Map.Strict qualified as Map
import Data.Maybe (isNothing)
import Data.Text qualified as T
import Slackblocks
import Test.QuickCheck hiding (Result, label)
import Test.QuickCheck.Random (mkQCGen)

assert :: String -> Bool -> IO ()
assert label condition = unless condition (fail label)

accept :: (Show e) => Either e a -> IO a
accept = either (fail . show) pure

reject :: String -> Result a -> IO ()
reject label = either report (const (fail (label <> " accepted")))
  where
    report errors = do
      diagnosticLength <- evaluate (length (show errors))
      assert (label <> ": empty diagnostics") (diagnosticLength > 0)

textValue :: Gen T.Text
textValue = T.pack <$> listOf1 (elements ['a', 'Z', '0', 'é', '中', '😀', '\x301', '\n', '"', '\\'])

propertyRun :: (Testable prop) => String -> prop -> IO ()
propertyRun name test = do
  putStrLn name
  result <- quickCheckWithResult stdArgs {maxSuccess = 500, replay = Just (mkQCGen 20261025, 0)} test
  unless (isSuccess result) (fail ("Property failed: " <> name))

main :: IO ()
main = do
  divider <- accept (buildDividerBlock dividerBlock)
  let blocks = replicate 12 (dividerBlockAsBlock divider)
  title <- accept (buildPlainText (plainText "Section"))
  section <- accept (buildContainerBlock (accordionSection title [dividerBlockAsBlock divider]))
  accept (accordion [section, section]) >>= assert "accordion preserves order" . (== replicate 2 (containerBlockAsBlock section))
  reject "empty accordion" (accordion [])
  regular <- accept $ buildContainerBlock ((containerBlock [dividerBlockAsBlock divider]) {containerBlockTitle = Just title})
  reject "noncollapsible accordion" (accordion [regular])
  first <- accept (paginate (paginator "list") blocks)
  middle <- accept (paginate ((paginator "list") {paginatorPage = 2}) blocks)
  lastPage <- accept (paginate ((paginator "list") {paginatorPage = 3, paginatorShowPageIndicator = False}) blocks)
  assert "page sizes include controls" (length first == 7 && length middle == 7 && length lastPage == 3)
  let actionValues page = case matchBlockActionsBlock (last page) of
        Nothing -> []
        Just actions ->
          [ (buttonElementActionId fields, buttonElementValue fields)
          | element <- actionsBlockElements (inspectActionsBlock actions),
            Just button <- [matchElementButtonElement element],
            let fields = inspectButtonElement button
          ]
  assert "first navigation" (actionValues first == [(Just "list.next", Just "2")])
  assert "middle navigation" (actionValues middle == [(Just "list.previous", Just "1"), (Just "list.next", Just "3")])
  assert "last navigation" (actionValues lastPage == [(Just "list.previous", Just "2")])
  reject "empty pagination" (paginate (paginator "list") [])
  reject "empty prefix" (paginate (paginator "") blocks)
  reject "page zero" (paginate ((paginator "list") {paginatorPage = 0}) blocks)
  reject "size zero" (paginate ((paginator "list") {paginatorPageSize = 0}) blocks)
  reject "past end" (paginate ((paginator "list") {paginatorPage = 4}) blocks)
  reject "oversized navigation ID" (paginate (paginator (T.replicate 255 "x")) blocks)
  accept (paginate ((paginator "list") {paginatorPageSize = 10 ^ (100 :: Int)}) blocks) >>= assert "huge size has no Int overflow" . (== blocks)
  reject "expanded message count" $ do
    page <- paginate ((paginator "list") {paginatorPageSize = 50}) (replicate 51 (dividerBlockAsBlock divider))
    buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just page})
  parameter <- accept (buildInputParameter (inputParameter "ticket" "42"))
  flow <- accept (workflowFromURL "https://example.com/trigger" [parameter])
  assert "workflow has typed parameters" (triggerCustomizableInputParameters (inspectTrigger (workflowTrigger (inspectWorkflow flow))) == Just [parameter])
  flowEmpty <- accept (workflowFromURL "https://example.com/trigger" [])
  assert "empty workflow parameters omitted" (isNothing (triggerCustomizableInputParameters (inspectTrigger (workflowTrigger (inspectWorkflow flowEmpty)))))
  accept (hexColor "A0B1c2") >>= assert "hex keeps case" . (== either (error . show) id (hexColor "#A0B1c2"))
  assert "named palette" (map (colorText . namedColor) [minBound .. maxBound] == ["good", "warning", "danger", "#ff0000", "#0000ff", "#ffff00", "#00ff00", "#ff8800", "#8800ff", "#000000"])
  reject "bad hex" (hexColor "#gg0000")
  reject "short hex" (hexColor "fff")
  reject "negative RGB" (rgbColor (-1) 0 0)
  reject "large RGB" (rgbColor 0 0 256)
  assert "Builder blocks wrapped and escaped" $
    blockKitBuilderURL (BuilderBlocks [dividerBlockAsBlock divider]) (Just "T/a#")
      == "https://app.slack.com/block-kit-builder/T%2Fa%23#%7B%22blocks%22%3A%5B%7B%22type%22%3A%22divider%22%7D%5D%7D"
  assert "Builder Unicode encoded as UTF8" ("%F0%9F%98%80" `T.isInfixOf` blockKitBuilderURL (BuilderJSON (Map.singleton "emoji" (String "😀"))) Nothing)
  assert "Builder unreserved segment" ("T-a._~#" `T.isInfixOf` blockKitBuilderURL (BuilderJSON Map.empty) (Just "T-a._~"))
  msg <- accept (buildMessagePayload (messagePayload "C123"))
  web <- accept (buildWebhookMessage webhookMessage)
  response <- accept (buildMessageResponse messageResponse)
  modalTitle <- accept (buildPlainText (plainText "View"))
  modal <- accept (buildModalView (modalView modalTitle []))
  home <- accept (buildHomeTabView (homeTabView [dividerBlockAsBlock divider]))
  assert "Builder message" (toJSON (BuilderMessage msg) == toJSON msg)
  assert "Builder webhook" (toJSON (BuilderWebhook web) == toJSON web)
  assert "Builder response" (toJSON (BuilderResponse response) == toJSON response)
  assert "Builder modal" (toJSON (BuilderModal modal) == toJSON modal)
  assert "Builder home" (toJSON (BuilderHome home) == toJSON home)
  let duplicate = "{\"type\":\"plain_text\",\"text\":\"one\",\"te\\u0078t\":\"two\"}"
  reject "escaped duplicate key" (decode duplicate :: Result PlainText)
  reject "nested duplicate metadata" (decode "{\"metadata\":{\"key\":1,\"key\":2}}" :: Result WebhookMessage)
  reject "trailing JSON" (decode "{\"type\":\"divider\"} false" :: Result DividerBlock)
  reject "invalid UTF8" (decode (L.pack [34, 255, 34]) :: Result PlainText)
  reject "invalid JSON object" (decode "{\"type\":" :: Result DividerBlock)
  reject "invalid JSON array" (decode "[1," :: Result [Integer])
  reject "wrong boolean" (fromValue (String "false") :: Result Bool)
  reject "wrong text" (fromValue (Bool False) :: Result T.Text)
  reject "wrong integer" (fromValue (String "1") :: Result Integer)
  reject "fractional integer" (decode "1.5" :: Result Integer)
  reject "wrong list" (fromValue (Bool True) :: Result [Integer])
  reject "wrong map" (fromValue (Bool False) :: Result (Map.Map T.Text Value))
  reject "wrong model shape" (fromValue (Bool False) :: Result PlainText)
  reject "wrong child role shape" (fromValue (Bool False) :: Result Block)
  reject "wrong discriminator" (fromValue (object ["type" .= String "unknown"]) :: Result PlainText)
  reject "unknown child role" (fromValue (object ["type" .= String "unknown"]) :: Result Block)
  reject "required null" (fromValue (object ["type" .= String "plain_text", "text" .= Null]) :: Result PlainText)
  optionalNull <- accept (fromValue (object ["type" .= String "plain_text", "text" .= String "text", "emoji" .= Null]) :: Result PlainText)
  assert "optional null omitted" (isNothing (plainTextEmoji (inspectPlainText optionalNull)))
  reject "bad plan task collection" (fromValue (object ["type" .= String "plan", "title" .= String "Plan", "tasks" .= Bool True]) :: Result PlanBlock)
  reject "bad plan task" (fromValue (object ["type" .= String "plan", "title" .= String "Plan", "tasks" .= [Bool True]]) :: Result PlanBlock)
  reject "bad plan task discriminator" (fromValue (object ["type" .= String "plan", "title" .= String "Plan", "tasks" .= [object ["type" .= String "wrong"]]]) :: Result PlanBlock)
  reject "malformed surrogate" (decode "\"\\uD800\"" :: Result T.Text)
  assert "syntax boolean false" ((decode "false\n" :: Result Bool) == Right False)
  assert "syntax decimal exact" ((decode "{\"x\":0.125,\"y\":1.25e2}" :: Result (Map.Map T.Text Value)) == Right (Map.fromList [("x", Number 0.125), ("y", Number 125)]))
  let semantic = object ["bool" .= True, "null" .= Null, "array" .= [Number 1, String "é"], "exact" .= (9007199254740993 :: Integer)]
  assert "all syntax token variants" ((decode (encode semantic) :: Result (Map.Map T.Text Value)) == Right (Map.fromList [("bool", Bool True), ("null", Null), ("array", toJSON [Number 1, String "é"]), ("exact", Number 9007199254740993)]))
  propertyRun "Unicode checked construction / exact JSON ingress" $ forAll textValue $ \value ->
    let result = buildPlainText (plainText value)
     in case result of Left _ -> T.length value > 3000; Right checked -> decode (encode checked) == Right checked
  propertyRun "RGB produces valid six-digit colors" $ forAll (vectorOf 3 (chooseInteger (0, 255))) $ \case
    [r, g, b] -> case rgbColor r g b of
      Right color -> hexColor (colorText color) == Right color
      _ -> False
    _ -> False
  propertyRun "Pagination preserves the selected content" $ forAll (chooseInt (1, 100)) $ \count ->
    forAll (chooseInteger (1, 20)) $ \size ->
      let items = take count (cycle blocks); pages = (fromIntegral count - 1) `div` size + 1
       in forAll (chooseInteger (1, pages)) $ \page -> case paginate ((paginator "prop") {paginatorPage = page, paginatorPageSize = size}) items of
            Left _ -> False
            Right output -> take (min (fromInteger size) (count - fromInteger ((page - 1) * size))) output == take (fromInteger size) (drop (fromInteger ((page - 1) * size)) items)
  propertyRun "Record edits must cross validation" $ forAll textValue $ \value ->
    case buildPlainText (plainText value) of
      Left _ -> False
      Right checked -> either (const True) (const False) (buildPlainText ((inspectPlainText checked) {plainTextText = ""}))
  propertyRun "Opaque metadata preserves exact integer values" $ \(integer :: Integer) ->
    let payload = buildWebhookMessage (webhookMessage {webhookMessageMetadata = Just (Map.singleton "integer" (toJSON integer))})
     in case payload of Left _ -> False; Right checked -> decode (encode checked) == Right checked
  assert "no duplicate keys in helper output" $ case toJSON (BuilderJSON (Map.singleton "x" Null)) of Object fields -> KM.size fields == 1; _ -> False
  putStrLn "Helpers, strict JSON boundaries and 2,500 property cases passed."
