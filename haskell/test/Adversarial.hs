module Main (main) where

import Control.Exception (evaluate)
import Control.Monad (forM_, unless)
import Data.Aeson qualified as A
import Data.ByteString.Lazy qualified as L
import Data.List.NonEmpty qualified as NE
import Data.Map.Strict qualified as Map
import Data.Text qualified as T
import Slackblocks

assert :: String -> Bool -> IO ()
assert label condition = unless condition (fail label)

accept :: (Show e) => Either e a -> IO a
accept = either (fail . show) pure

reject :: (FromSlackJSON a) => String -> ErrorCategory -> L.ByteString -> (a -> ()) -> IO ()
reject label category bytes witness = case decode bytes of
  Right value -> witness value `seq` fail (label <> " accepted")
  Left errors -> do
    diagnosticLength <- evaluate (length (show errors))
    assert (label <> ": diagnostics") (diagnosticLength > 0)
    assert (label <> ": category " <> show errors) (any ((== category) . errorCategory) (NE.toList errors))

main :: IO ()
main = do
  reject "untagged object cannot silently discard type" InvalidUsage "{\"channel\":\"C123\",\"type\":\"future\"}" (const () :: MessagePayload -> ())
  reject "empty section fields with text" MissingRequired "{\"type\":\"section\",\"text\":\"ok\",\"fields\":[]}" (const () :: SectionBlock -> ())
  reject "empty card" MissingRequired "{\"type\":\"card\"}" (const () :: CardBlock -> ())
  reject "two card icon sources" MutuallyExclusive "{\"type\":\"card\",\"body\":\"body\",\"slack_icon\":{\"type\":\"icon\",\"name\":\"check\"},\"icon\":{\"type\":\"image\",\"image_url\":\"x\",\"alt_text\":\"x\"}}" (const () :: CardBlock -> ())
  reject "container missing title" MissingRequired "{\"type\":\"container\",\"child_blocks\":[{\"type\":\"divider\"}]}" (const () :: ContainerBlock -> ())
  reject "collapsed noncollapsible container" InvalidUsage "{\"type\":\"container\",\"title\":\"Title\",\"child_blocks\":[{\"type\":\"divider\"}],\"default_collapsed\":true}" (const () :: ContainerBlock -> ())
  reject "collapsible header divider" InvalidUsage "{\"type\":\"container\",\"title\":\"Title\",\"child_blocks\":[{\"type\":\"divider\"}],\"is_collapsible\":true,\"has_header_divider\":true}" (const () :: ContainerBlock -> ())
  reject "empty Slack file" MutuallyExclusive "{}" (const () :: SlackFile -> ())
  reject "empty conversation filter" MissingRequired "{}" (const () :: ConversationFilter -> ())
  reject "duplicate axis categories" InvalidUsage "{\"categories\":[\"a\",\"a\"]}" (const () :: AxisConfig -> ())
  reject "bad attachment color" TypeMismatch "{\"color\":\"#GG0000\",\"blocks\":[{\"type\":\"divider\"}]}" (const () :: Attachment -> ())
  reject "attachment surface" TypeMismatch "{\"blocks\":[{\"type\":\"alert\",\"text\":\"Alert\"}]}" (const () :: Attachment -> ())
  reject "unequal table row widths" InvalidUsage "{\"type\":\"data_table\",\"caption\":\"Table\",\"rows\":[[{\"type\":\"raw_text\",\"text\":\"A\"}],[{\"type\":\"raw_text\",\"text\":\"B\"},{\"type\":\"raw_text\",\"text\":\"C\"}]]}" (const () :: DataTableBlock -> ())
  reject "rich text table header" TypeMismatch "{\"type\":\"data_table\",\"caption\":\"Table\",\"rows\":[[{\"type\":\"rich_text\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"header\"}]}]}],[{\"type\":\"raw_text\",\"text\":\"Body\"}]]}" (const () :: DataTableBlock -> ())
  reject "table column settings extension" InvalidUsage "{\"type\":\"data_table\",\"caption\":\"Table\",\"column_settings\":[],\"rows\":[[{\"type\":\"raw_text\",\"text\":\"A\"}],[{\"type\":\"raw_text\",\"text\":\"Body\"}]]}" (const () :: DataTableBlock -> ())
  reject "duplicate chart series names" InvalidUsage "{\"type\":\"line\",\"axis_config\":{\"categories\":[\"a\"]},\"series\":[{\"name\":\"s\",\"data\":[{\"label\":\"a\",\"value\":1}]},{\"name\":\"s\",\"data\":[{\"label\":\"a\",\"value\":2}]}]}" (const () :: LineChart -> ())
  forM_ ["24:00", "23:60", "23:5x", "x3:59", "23-59"] $ \value ->
    reject "invalid time" TypeMismatch (A.encode (A.object ["type" A..= A.String "timepicker", "initial_time" A..= (value :: T.Text)])) (const () :: TimePickerElement -> ())
  forM_ [("-1", "1"), ("-1", "-1.0"), ("-2", "-1"), ("1e+2", "100"), ("0", "-0")] $ \(lo, hi) -> do
    number <- accept $ buildNumberInputElement ((numberInputElement True) {numberInputElementMinValue = Just lo, numberInputElementMaxValue = Just hi})
    assert "exact decimal bounds preserved" (numberInputElementMinValue (inspectNumberInputElement number) == Just lo)
  reject "repeated exponent" TypeMismatch "{\"type\":\"number_input\",\"is_decimal_allowed\":true,\"min_value\":\"1e2e3\"}" (const () :: NumberInputElement -> ())
  -- A nested rich-text table exercises content accounting through lists, links
  -- and non-text items. Metadata that resembles content stays opaque.
  let table = "{\"type\":\"data_table\",\"caption\":\"Table\",\"rows\":[[{\"type\":\"raw_text\",\"text\":\"Header\"}],[{\"type\":\"rich_text\",\"elements\":[{\"type\":\"rich_text_list\",\"style\":\"bullet\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Body\"},{\"type\":\"link\",\"url\":\"https://example.com\",\"text\":\"Link\"},{\"type\":\"emoji\",\"name\":\"wave\"}]}]}]}]]}"
  checked <- accept (decode table :: Result DataTableBlock)
  title <- accept (buildPlainText (plainText "Nested"))
  inner <- accept (buildContainerBlock (accordionSection title [dataTableBlockAsBlock checked]))
  outer <- accept (buildContainerBlock (accordionSection title [containerBlockAsBlock inner]))
  message <- accept (buildMessagePayload ((messagePayload "C123") {messagePayloadBlocks = Just [containerBlockAsBlock outer]}))
  assert "nested message roundtrip" (decode (A.encode message) == Right message)
  richTitle <- accept (decode "{\"type\":\"rich_text\",\"elements\":[{\"type\":\"rich_text_section\",\"elements\":[{\"type\":\"text\",\"text\":\"Title\"}]}]}" :: Result RichTextBlock)
  divider <- accept (buildDividerBlock dividerBlock)
  _ <- accept (buildContainerBlock ((containerBlock [dividerBlockAsBlock divider]) {containerBlockRichTextTitle = Just richTitle}))
  forM_ ["{\"exclude_external_shared_channels\":false}", "{\"exclude_bot_users\":false}"] $ \bytes -> do
    value <- accept (decode bytes :: Result ConversationFilter)
    assert "explicit false is content" (decode (A.encode value) == Right value)
  -- Verify Aeson's standard instance adapters in both directions, separately
  -- from the stricter duplicate-aware byte decoder.
  extended <- accept (buildPlainText ((plainText "text") {plainTextExtensions = Map.singleton "future" (A.Bool False)}))
  assert "extension roundtrip through Aeson" (A.eitherDecode (A.encode extended) == Right extended)
  assert "Aeson checked failure" (case A.eitherDecode "{\"type\":\"plain_text\",\"text\":\"\"}" :: Either String PlainText of Left errorText -> not (null errorText); Right _ -> False)
  putStrLn "Adversarial relationships, nested accounting, exact bounds and Aeson adapters passed."
