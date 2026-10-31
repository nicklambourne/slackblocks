-- | Handwritten relationships and receiving-context validation. Only modeled
-- child positions are traversed; arbitrary JSON metadata remains opaque.
module Slackblocks.Internal.Rules (validateFields, checkIntegerBounds, standaloneTask) where

import Control.Monad (unless, when)
import Data.Aeson (Value (..))
import Data.Aeson.Key qualified as K
import Data.Aeson.KeyMap qualified as KM
import Data.Char (isAscii, isAsciiUpper, isDigit, isHexDigit)
import Data.List (nub)
import Data.Maybe (isJust)
import Data.Scientific (isInteger)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector qualified as V
import Slackblocks.Internal.Decimal
import Slackblocks.Internal.JsonSyntax (checkJSONNumbers)
import Slackblocks.Internal.Schema
import Slackblocks.Validation

at :: Text -> Value -> Value
at key (Object fields) = KM.lookup (K.fromText key) fields `orValue` Null
  where
    orValue (Just value) _ = value; orValue Nothing fallback = fallback
at _ _ = Null

has :: Text -> Value -> Bool
has key (Object fields) = KM.member (K.fromText key) fields
has _ _ = False

array :: Value -> [Value]
array (Array values) = V.toList values
array _ = []

text :: Value -> Text
text (String value) = value
text _ = ""

count :: Value -> Integer
count = fromIntegral . T.length . text

limit :: Text -> Integer
limit key = case lookup key limits of
  Just value -> value
  Nothing -> error ("internal generated limit missing: " <> T.unpack key)

check :: ErrorCategory -> [PathPiece] -> Text -> Bool -> Result ()
check category path constraint ok = unless ok (failure category path constraint "constraint violated")

-- Reject bounded integer fields while they are still compact Scientific values.
-- Expanding 1e1000000000 into Integer just to compare it with 3000 can exhaust
-- memory. Fractions still reach the ordinary integer type check below.
checkIntegerBounds :: Text -> Value -> Result ()
checkIntegerBounds owner value = mapM_ field (fieldConstraints owner)
  where
    field (key, kind, prefix, bounds, _) = when (kind `elem` ["int", "long"]) $ case at key value of
      number@(Number x) -> do
        atField key (checkJSONNumbers number)
        when (isInteger x) $ mapM_ (bound key prefix x) bounds
      _ -> pure ()
    bound key prefix x (suffix, n) = case suffix of
      "min" -> check OutOfRange [Field key] (prefix <> ".min") (x >= fromInteger n)
      "max" -> check OutOfRange [Field key] (prefix <> ".max") (x <= fromInteger n)
      _ -> pure ()

validateFields :: Text -> Value -> Result ()
validateFields owner value = do
  checkJSONNumbers value
  mapM_ field (fieldConstraints owner)
  rules owner value
  where
    field (key, kind, prefix, bounds, flags) = when (has key value && at key value /= Null) $ do
      let v = at key value
          measure = case kind of
            "string" -> count v
            "text" -> count (at "text" v)
            _ -> fromIntegral (length (array v))
          path = [Field key]
          bound (suffix, n) =
            let constraint = prefix <> "." <> suffix
                lengthCheck predicate = check LengthExceeded path constraint (predicate measure n)
                numberCheck predicate = case v of
                  Number x -> check OutOfRange path constraint (predicate x (fromInteger n))
                  _ -> failure TypeMismatch path constraint "expected a number"
             in case suffix of
                  "min_length" -> lengthCheck (>=)
                  "max_length" -> lengthCheck (<=)
                  "min_items" -> lengthCheck (>=)
                  "max_items" -> lengthCheck (<=)
                  "min" -> numberCheck (>=)
                  "max" -> numberCheck (<=)
                  "exclusive_min" -> numberCheck (>)
                  "exclusive_max" -> numberCheck (<)
                  "item_max_length" -> mapM_ (\(i, item) -> check LengthExceeded (path <> [Index i]) constraint ((if kind == "textList" then count (at "text" item) else count item) <= n)) (zip [0 ..] (array v))
                  _ -> failure InvalidUsage path constraint "unknown internal constraint"
      mapM_ bound bounds
      unless (null flags) $ case v of
        Object style -> mapM_ (\key' -> check TypeMismatch (path <> [Field (K.toText key')]) "style.flags" (K.toText key' `elem` flags)) (KM.keys style)
        _ -> failure TypeMismatch path "style" "expected a style object"

rules :: Text -> Value -> Result ()
rules owner value = case owner of
  "ButtonElement" -> check MissingRequired [Field "agent_prompt"] "button.agent_prompt_display.requires_agent_prompt" (not (present "agent_prompt_display") || present "agent_prompt")
  "SlackIcon" -> check TypeMismatch [Field "name"] "slack_icon_names" (text (get "name") `elem` icons)
  "ExternalMultiSelectElement" -> check InvalidUsage [Field "focus_on_load"] "external_multi_select.focus_on_load" (not (present "focus_on_load"))
  "SectionBlock" -> do
    check MissingRequired [] "section.content" (present "text" || not (null (array (get "fields"))))
    check MissingRequired [Field "fields"] "section.fields" (not (present "fields") || not (null (array (get "fields"))))
  "StaticSelectElement" -> options
  "StaticMultiSelectElement" -> options
  "CardBlock" -> do
    check MissingRequired [] "card.content" (any present ["hero_image", "title", "actions", "body"])
    check MutuallyExclusive [] "card.icon" (not (present "icon" && present "slack_icon"))
  "LayoutBlock" -> compositeDepth value 0 False
  "LayoutItem" -> allowedKeys value ["block", "width", "horizontal_alignment"]
  "LayoutRow" -> do
    allowedKeys value ["items"]
    let items = array (get "items")
        wide = filter ((`elem` [String "carousel", String "data_table", String "data_visualization"]) . at "type" . at "block") items
        total = sum (map layoutWeight items)
    check InvalidUsage [] "layout.row.wide_blocks" (length wide <= 1)
    mapM_ (\item -> when (at "type" (at "block" item) == String "data_visualization") (check InvalidUsage [] "layout.row.chart_width" (layoutWeight item * 2 > total))) items
  "ContainerBlock" -> do
    compositeDepth value 0 False
    check MissingRequired [] "container.title" (present "title" || present "rich_text_title")
    check InvalidUsage [Field "default_collapsed"] "container.default_collapsed" (get "default_collapsed" /= Bool True || get "is_collapsible" == Bool True)
    check InvalidUsage [Field "has_header_divider"] "container.has_header_divider" (get "has_header_divider" /= Bool True || get "is_collapsible" /= Bool True)
  "ImageBlock" -> image
  "ImageElement" -> image
  "SlackFile" -> do
    check MutuallyExclusive [] "slack_file.source" (present "id" /= present "url")
    when (present "id") $
      let identifier = text (get "id")
       in check TypeMismatch [Field "id"] "slack_file.id" (T.length identifier >= 9 && T.isPrefixOf "F" identifier && T.all (\c -> isAsciiUpper c || (isAscii c && isDigit c)) (T.drop 1 identifier))
  "ConversationFilter" -> do
    check MissingRequired [] "conversation_filter.content" (any present ["include", "exclude_external_shared_channels", "exclude_bot_users"])
    indexed "include" (\item -> check TypeMismatch [] "conversation_filter.include" (text item `elem` ["im", "mpim", "private", "public"]))
  "DispatchActionConfiguration" -> check MissingRequired [Field "trigger_actions_on"] "dispatch_action_configuration.trigger_actions_on" (present "trigger_actions_on")
  "DatePickerElement" -> when (present "initial_date") $ check TypeMismatch [Field "initial_date"] "date_picker.initial_date" (dateShape (text (get "initial_date")))
  "TimePickerElement" -> when (present "initial_time") $ check TypeMismatch [Field "initial_time"] "time_picker.initial_time" (timeShape (text (get "initial_time")))
  "PlainTextInputElement" -> case (get "min_length", get "max_length") of
    (Number lo, Number hi) -> check OutOfRange [Field "min_length"] "plain_text_input.bounds" (lo <= hi)
    _ -> Right ()
  "NumberInputElement" -> do
    mapM_ (\key -> when (present key) (check TypeMismatch [Field key] "number_input.decimal" (isJust (decimalParts (text (get key)))))) ["min_value", "max_value"]
    when (present "min_value" && present "max_value") $ check OutOfRange [] "number_input.bounds" (compareDecimal (text (get "min_value")) (text (get "max_value")) /= Just GT)
  "TableBlock" -> indexed "rows" (\row -> check LengthExceeded [] "table.columns.max_items" (fromIntegral (length (array row)) <= limit "table.columns.max_items"))
  "DataTableBlock" -> dataTable value
  "AxisConfig" -> do
    indexed "categories" (\label -> check LengthExceeded [] "data_visualization.category_label.max_length" (count label <= limit "data_visualization.category_label.max_length"))
    check InvalidUsage [Field "categories"] "data_visualization.categories.unique" (unique (array (get "categories")))
  "ChartSegment" -> check OutOfRange [Field "value"] "data_visualization.segment.value.exclusive_min" (case get "value" of Number number -> number > fromInteger (limit "data_visualization.segment.value.exclusive_min"); _ -> False)
  "LineChart" -> chart value
  "BarChart" -> chart value
  "AreaChart" -> chart value
  "PlanBlock" -> check InvalidUsage [Field "tasks"] "plan.tasks.unique" (unique (map (at "task_id") (array (get "tasks"))))
  "Attachment" -> do
    when (present "color") $
      let color = text (get "color")
       in check TypeMismatch [Field "color"] "attachment.color" (color `elem` ["good", "warning", "danger"] || (T.length color == 7 && T.isPrefixOf "#" color && T.all (\c -> isAscii c && isHexDigit c) (T.drop 1 color)))
    surface "message" [Field "blocks"] (array (get "blocks"))
  "MessagePayload" -> message value
  "MessageResponse" -> message value
  "WebhookMessage" -> message value
  "ModalView" -> do
    surface "modal" [Field "blocks"] (array (get "blocks"))
    check MissingRequired [Field "submit"] "view.submit.required" (present "submit" || not (any (any ((== String "input") . at "type") . descendants) (array (get "blocks"))))
  "HomeTabView" -> surface "home" [Field "blocks"] (array (get "blocks"))
  _ -> Right ()
  where
    get key = at key value
    present key = has key value
    options = check MutuallyExclusive [] "select.options_or_groups" (not (present "options" && present "option_groups"))
    image = do
      check MissingRequired [] "image.source" (present "image_url" || present "slack_file")
      check MutuallyExclusive [] "image.source" (not (present "image_url" && present "slack_file"))
    indexed key f = mapM_ (\(i, item) -> atField key (atIndex i (f item))) (zip [0 ..] (array (get key)))

unique :: (Eq a) => [a] -> Bool
unique values = length (nub values) == length values

dateShape :: Text -> Bool
dateShape value = T.length value == 10 && and [if i `elem` [4, 7] then c == '-' else isAscii c && isDigit c | (i, c) <- zip [0 :: Int ..] (T.unpack value)]

timeShape :: Text -> Bool
timeShape value = case T.unpack value of
  [a, b, ':', c, d] -> all (\x -> isAscii x && isDigit x) [a, b, c, d] && [a, b] <= "23" && [c, d] <= "59"
  _ -> False

standaloneTask :: Value -> Result ()
standaloneTask value
  | at "type" value == String "task_card" = check TypeMismatch [Field "status"] "task_card.pending" (at "status" value /= String "pending")
  | at "type" value == String "container" = mapM_ (\(i, child) -> atField "child_blocks" (atIndex i (standaloneTask child))) (zip [0 ..] (array (at "child_blocks" value)))
  | otherwise = Right ()

surface :: Text -> [PathPiece] -> [Value] -> Result ()
surface name path blocks = do
  mapM_
    ( \(i, block) -> do
        surface name (path <> [Index i, Field "children"]) (compositeChildren False block)
        when (at "type" block == String "layout") (surface name (path <> [Index i, Field "fallback"]) (array (at "fallback" block)))
        check TypeMismatch (path <> [Index i, Field "type"]) "surface_block_types" (text (at "type" block) `elem` surfaceBlocks name)
        either (Left . fmap (\e -> e {errorPath = path <> [Index i] <> errorPath e})) Right (standaloneTask block)
    )
    (zip [0 ..] blocks)

-- Traverse only modeled nested blocks. Metadata values with a "type" key do
-- not count as blocks, charts, markdown or tables.
descendants :: Value -> [Value]
descendants block = block : concatMap descendants (compositeChildren True block)

layoutWeight :: Value -> Int
layoutWeight item = case at "width" item of String "narrow" -> 1; String "wide" -> 3; _ -> 2

allowedKeys :: Value -> [Text] -> Result ()
allowedKeys (Object fields) keys = mapM_ (\key -> check TypeMismatch [Field (K.toText key)] "layout.properties" (K.toText key `elem` keys)) (KM.keys fields)
allowedKeys _ _ = Right ()

compositeChildren :: Bool -> Value -> [Value]
compositeChildren includeFallback block = case at "type" block of
  String "container" -> array (at "child_blocks" block)
  String "layout" -> [at "block" item | row <- array (at "rows" block), item <- array (at "items" row)] <> if includeFallback then array (at "fallback" block) else []
  _ -> []

compositeDepth :: Value -> Int -> Bool -> Result ()
compositeDepth block depth seen = do
  let hasLayout = seen || at "type" block == String "layout"
      next = depth + if at "type" block `elem` [String "layout", String "container"] then 1 else 0
  check InvalidUsage [] "layout.nesting" (not hasLayout || next <= 2)
  mapM_ (\child -> compositeDepth child next hasLayout) (compositeChildren False block)

message :: Value -> Result ()
message value = do
  let blocks = array (at "blocks" value)
      attached = concatMap (array . at "blocks") (array (at "attachments" value))
      nested = concatMap descendants (blocks <> attached)
      charts = filter ((== String "data_visualization") . at "type") nested
      markdown = sum [count (at "text" b) | b <- nested, at "type" b == String "markdown"]
      tables = sum [tableCharacters b | b <- nested, at "type" b == String "data_table"]
  surface "message" [Field "blocks"] blocks
  check LengthExceeded [] "message.data_visualizations.max_items" (fromIntegral (length charts) <= limit "message.data_visualizations.max_items")
  check LengthExceeded [] "markdown.total_text.max_length" (markdown <= limit "markdown.total_text.max_length")
  check LengthExceeded [] "data_table.total_content.max_length" (tables <= limit "data_table.total_content.max_length")

tableCharacters :: Value -> Integer
tableCharacters value = sum (map cellCharacters (concatMap array (array (at "rows" value))))
  where
    cellCharacters cell
      | at "type" cell `elem` [String "raw_text", String "raw_number"] = count (at "text" cell)
      | at "type" cell == String "rich_text" = sum (map richCharacters (array (at "elements" cell)))
      | otherwise = 0
    richCharacters element
      | at "type" element == String "rich_text_list" = sum (map richCharacters (array (at "elements" element)))
      | otherwise = sum [count (at "text" item) | item <- array (at "elements" element), at "type" item `elem` [String "text", String "link"]]

dataTable :: Value -> Result ()
dataTable value = do
  let rows = map array (array (at "rows" value))
      width = case rows of [] -> 0; first : _ -> length first
  mapM_
    ( \(i, row) -> do
        check LengthExceeded [Field "rows", Index i] "data_table.columns.max_items" (not (null row) && fromIntegral (length row) <= limit "data_table.columns.max_items")
        check InvalidUsage [Field "rows", Index i] "data_table.rectangular" (length row == width)
        mapM_
          ( \(j, cell) -> do
              check TypeMismatch [Field "rows", Index i, Index j] "data_table.header" (i /= 0 || at "type" cell /= String "rich_text")
              when (at "type" cell `elem` [String "raw_text", String "raw_number"]) $ check LengthExceeded [Field "rows", Index i, Index j, Field "text"] "data_table.cell.min_length" (count (at "text" cell) > 0)
          )
          (zip [0 ..] row)
    )
    (zip [0 ..] rows)
  check InvalidUsage [Field "column_settings"] "data_table.column_settings" (not (has "column_settings" value))
  check LengthExceeded [Field "rows"] "data_table.content.max_length" (tableCharacters value <= limit "data_table.content.max_length")

chart :: Value -> Result ()
chart value = do
  let categories = array (at "categories" (at "axis_config" value))
      series = array (at "series" value)
  mapM_
    ( \(i, entry) ->
        let points = array (at "data" entry)
         in check InvalidUsage [Field "series", Index i, Field "data"] "data_visualization.series.categories" (length points == length categories && all (\label -> length (filter ((== label) . at "label") points) == 1) categories)
    )
    (zip [0 ..] series)
  check InvalidUsage [Field "series"] "data_visualization.series.unique" (unique (map (at "name") series))
