module Slackblocks.Internal.Wire
  ( optional,
    extensionPairs,
    checkExtensions,
    checkedParser,
    wireObject,
    wireTag,
    required,
    optionalField,
    remaining,
    coerceField,
    stripTag,
    restoreTask,
    restoreTasks,
  )
where

import Data.Aeson (ToJSON (..), Value (..), object, (.=))
import Data.Aeson.Key qualified as Key
import Data.Aeson.KeyMap qualified as KM
import Data.Aeson.Types (Pair, Parser)
import Data.Bifunctor (first)
import Data.Map.Strict (Map)
import Data.Map.Strict qualified as Map
import Data.Text (Text)
import Slackblocks.Json
import Slackblocks.Validation

optional :: (ToJSON a) => Text -> Maybe a -> [Pair]
optional key = maybe [] (\value -> [Key.fromText key .= value])

extensionPairs :: Map Text Value -> [Pair]
extensionPairs = map (first Key.fromText) . Map.toList

checkExtensions :: [Text] -> Map Text Value -> Result ()
checkExtensions fields = mapM_ check . Map.keys
  where
    check key
      | key == "" || key == "type" || key `elem` fields =
          failure InvalidUsage [Field key] "extensions.reserved" "extension collides with a reserved field"
      | otherwise = Right ()

checkedParser :: (FromSlackJSON a) => Value -> Parser a
checkedParser = either (fail . show) pure . fromSlackJSON

wireObject :: Text -> Value -> Result (KM.KeyMap Value)
wireObject tag (Object fields)
  | tag == "" = Right fields
  | KM.lookup "type" fields == Just (String tag) = Right (KM.delete "type" fields)
  | otherwise = failure TypeMismatch [Field "type"] "type" "unexpected wire discriminator"
wireObject _ _ = failure TypeMismatch [] "type" "expected an object"

wireTag :: Value -> Result Text
wireTag (Object fields) = required "type" fields
wireTag _ = failure TypeMismatch [] "type" "expected an object with a discriminator"

required :: (FromSlackJSON a) => Text -> KM.KeyMap Value -> Result a
required key fields = case KM.lookup (Key.fromText key) fields of
  Nothing -> absent
  Just Null -> absent
  Just value -> atField key (fromSlackJSON value)
  where
    absent = failure MissingRequired [Field key] "required" "required field is missing"

optionalField :: (FromSlackJSON a) => Text -> KM.KeyMap Value -> Result (Maybe a)
optionalField key fields = case KM.lookup (Key.fromText key) fields of
  Nothing -> Right Nothing
  Just Null -> Right Nothing
  Just value -> Just <$> atField key (fromSlackJSON value)

remaining :: [Text] -> KM.KeyMap Value -> Map Text Value
remaining names =
  Map.fromList
    . map (first Key.toText)
    . KM.toList
    . KM.filterWithKey (\key _ -> Key.toText key `notElem` names)

coerceField :: Text -> Text -> KM.KeyMap Value -> KM.KeyMap Value
coerceField field tag = KM.mapWithKey (\key value -> if Key.toText key == field then coerce value else value)
  where
    coerce (Number value) | tag == "decimal_string" = String (decimalText value)
    coerce value | tag == "decimal_string" = value
    coerce (String value) = object ["type" .= tag, "text" .= value]
    coerce (Array values) = Array (fmap coerce values)
    coerce value = value

stripTag :: Value -> Value
stripTag (Object fields) = Object (KM.delete "type" fields)
stripTag value = value

restoreTasks :: KM.KeyMap Value -> KM.KeyMap Value
restoreTasks = KM.mapWithKey (\key value -> if key == "tasks" then restore value else value)
  where
    restore (Array tasks) = Array (fmap restoreTask tasks)
    restore value = value

restoreTask :: Value -> Value
restoreTask (Object fields) = Object (if KM.member "type" fields then fields else KM.insert "type" (String "task_card") fields)
restoreTask value = value
