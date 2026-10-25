-- | Pure composition conveniences. These produce ordinary checked Block Kit
-- values; sending, interaction routing and pagination state remain application-owned.
module Slackblocks.Helpers
  ( accordionSection,
    accordion,
    PaginatorOptions (..),
    paginator,
    paginate,
    workflowFromURL,
    NamedColor (..),
    Color,
    namedColor,
    hexColor,
    rgbColor,
    colorText,
    BuilderPayload (..),
    blockKitBuilderURL,
  )
where

import Control.Monad (unless, when)
import Data.Aeson (ToJSON (..), Value, encode, object, (.=))
import Data.ByteString qualified as B
import Data.ByteString.Lazy qualified as L
import Data.Char (isAscii, isHexDigit, toUpper)
import Data.Map.Strict (Map)
import Data.Maybe (fromMaybe)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Text.Encoding qualified as TE
import Numeric (showHex)
import Slackblocks.Models
import Slackblocks.Validation

-- | Construction input for a collapsible container, initially collapsed. Edit
-- ordinary container fields, then call 'buildContainerBlock'. No ID is invented.
accordionSection :: PlainText -> [Block] -> ContainerBlockFields
accordionSection title blocks =
  (containerBlock blocks)
    { containerBlockTitle = Just title,
      containerBlockWidth = Just ContainerWidthStandard,
      containerBlockIsCollapsible = Just True,
      containerBlockDefaultCollapsed = Just True,
      containerBlockHasHeaderDivider = Just False
    }

-- | Expand checked collapsible sections, preserving their order.
accordion :: [ContainerBlock] -> Result [Block]
accordion sections = do
  when (null sections) $ failure MissingRequired [Field "sections"] "accordion.sections" "expected at least one section"
  mapM_
    ( \(index, section) ->
        unless (containerBlockIsCollapsible (inspectContainerBlock section) == Just True) $
          failure TypeMismatch [Field "sections", Index index] "accordion.section" "expected a collapsible container"
    )
    (zip [0 ..] sections)
  pure (map containerBlockAsBlock sections)

-- | Page numbers start at one. Navigation adds up to two blocks; the receiving
-- message or view checks total counts after expansion. No fields encode directly.
data PaginatorOptions = PaginatorOptions
  { paginatorActionIdPrefix :: Text,
    paginatorPage :: Integer,
    paginatorPageSize :: Integer,
    paginatorPreviousText :: Text,
    paginatorNextText :: Text,
    paginatorShowPageIndicator :: Bool,
    paginatorBlockId :: Maybe Text
  }
  deriving (Eq, Show)

-- | Defaults: first page, five content blocks, Previous/Next and page indicator.
paginator :: Text -> PaginatorOptions
paginator prefix = PaginatorOptions prefix 1 5 "Previous" "Next" True Nothing

paginate :: PaginatorOptions -> [Block] -> Result [Block]
paginate options blocks = do
  when (null blocks) $ failure MissingRequired [Field "blocks"] "paginator.blocks" "expected content"
  when (T.null prefix) $ failure MissingRequired [Field "action_id_prefix"] "paginator.prefix" "expected a nonempty prefix"
  when (page < 1) $ failure OutOfRange [Field "page"] "paginator.page" "expected positive page"
  when (size < 1) $ failure OutOfRange [Field "page_size"] "paginator.page_size" "expected positive page size"
  let total = fromIntegral (length blocks)
      pages = (total - 1) `div` size + 1
      content = take (fromInteger (min size total)) $ drop (fromInteger (min ((page - 1) * size) total)) blocks
  when (page > pages) $ failure OutOfRange [Field "page"] "paginator.page" "page exceeds page count"
  if pages == 1
    then pure content
    else do
      previous <- if page > 1 then (: []) <$> button (paginatorPreviousText options) ".previous" (page - 1) else pure []
      next <- if page < pages then (: []) <$> button (paginatorNextText options) ".next" (page + 1) else pure []
      controls <- buildActionsBlock ((actionsBlock (previous <> next)) {actionsBlockBlockId = paginatorBlockId options})
      indicator <-
        if paginatorShowPageIndicator options
          then do
            label <- buildMarkdownText (markdownText ("Page " <> decimal page <> " of " <> decimal pages))
            context <- buildContextBlock (contextBlock [markdownTextAsContextElement label])
            pure [contextBlockAsBlock context]
          else pure []
      pure (content <> indicator <> [actionsBlockAsBlock controls])
  where
    prefix = paginatorActionIdPrefix options
    page = paginatorPage options
    size = paginatorPageSize options
    decimal = T.pack . show
    button title suffix destination = do
      label <- buildPlainText (plainText title)
      action <-
        buildButtonElement
          ( (buttonElement label)
              { buttonElementActionId = Just (prefix <> suffix),
                buttonElementValue = Just (decimal destination)
              }
          )
      pure (buttonElementAsElement action)

-- | Build a workflow's trigger and optional customizable parameters. This does
-- not call Slack or add restrictions beyond the shared trigger contract.
workflowFromURL :: Text -> [InputParameter] -> Result Workflow
workflowFromURL url parameters = do
  link <- buildTrigger ((trigger url) {triggerCustomizableInputParameters = if null parameters then Nothing else Just parameters})
  buildWorkflow (workflow link)

data NamedColor = Good | Warning | Danger | Red | Blue | Yellow | Green | Orange | Purple | Black
  deriving (Eq, Ord, Show, Enum, Bounded)

-- | Valid attachment color. Use 'colorText' with an attachment's color field.
newtype Color = Color Text deriving (Eq, Ord, Show)

colorText :: Color -> Text
colorText (Color value) = value

namedColor :: NamedColor -> Color
namedColor color = Color $ case color of
  Good -> "good"
  Warning -> "warning"
  Danger -> "danger"
  Red -> "#ff0000"
  Blue -> "#0000ff"
  Yellow -> "#ffff00"
  Green -> "#00ff00"
  Orange -> "#ff8800"
  Purple -> "#8800ff"
  Black -> "#000000"

-- | Six ASCII hexadecimal digits, with optional #. Letter case is preserved.
hexColor :: Text -> Result Color
hexColor input =
  let value = fromMaybe input (T.stripPrefix "#" input)
   in if T.length value == 6 && T.all (\c -> isAscii c && isHexDigit c) value
        then Right (Color ("#" <> value))
        else failure TypeMismatch [Field "color"] "color.hex" "expected six hexadecimal digits"

rgbColor :: Integer -> Integer -> Integer -> Result Color
rgbColor red green blue = do
  mapM_
    ( \(index, value) ->
        when (value < 0 || value > 255) $
          failure OutOfRange [Field "color", Index index] "color.rgb" "channel must be in 0..255"
    )
    (zip [0 ..] channels)
  pure (Color ("#" <> T.concat [T.justifyRight 2 '0' (T.pack (showHex channel "")) | channel <- channels]))
  where
    channels = [red, green, blue]

-- | Explicit Builder input shapes. 'BuilderJSON' is a deliberate opaque-object
-- escape hatch; it is not validation or typed support for a Block Kit feature.
data BuilderPayload
  = BuilderBlocks [Block]
  | BuilderMessage MessagePayload
  | BuilderWebhook WebhookMessage
  | BuilderResponse MessageResponse
  | BuilderModal ModalView
  | BuilderHome HomeTabView
  | BuilderJSON (Map Text Value)
  deriving (Eq, Show)

instance ToJSON BuilderPayload where
  toJSON (BuilderBlocks blocks) = object ["blocks" .= blocks]
  toJSON (BuilderMessage value) = toJSON value
  toJSON (BuilderWebhook value) = toJSON value
  toJSON (BuilderResponse value) = toJSON value
  toJSON (BuilderModal value) = toJSON value
  toJSON (BuilderHome value) = toJSON value
  toJSON (BuilderJSON value) = toJSON value

-- | Percent-encode UTF-8 JSON and an optional team as a single path segment.
-- Makes no network request; rendering in Slack is a separate acceptance step.
blockKitBuilderURL :: BuilderPayload -> Maybe Text -> Text
blockKitBuilderURL payload team =
  "https://app.slack.com/block-kit-builder/"
    <> maybe "" (escape . TE.encodeUtf8) team
    <> "#"
    <> escape (L.toStrict (encode payload))
  where
    escape = T.concat . map byte . B.unpack
    byte value
      | value >= 65 && value <= 90 || value >= 97 && value <= 122 || value >= 48 && value <= 57 || value `elem` [45, 46, 95, 126] = T.singleton (toEnum (fromIntegral value))
      | otherwise = "%" <> T.pack (map toUpper (pad (showHex value "")))
    pad [one] = ['0', one]
    pad pair = pair
