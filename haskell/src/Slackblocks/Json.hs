{-# LANGUAGE FlexibleInstances #-}

-- | Checked JSON ingress. Unknown modeled values are rejected; opaque metadata
-- remains ordinary Aeson 'Value' and is never traversed as Block Kit content.
module Slackblocks.Json (FromSlackJSON (..), decode, fromValue, decimalText) where

import Data.Aeson (Value (..))
import Data.Aeson.Key qualified as Key
import Data.Aeson.KeyMap qualified as KM
import Data.ByteString.Lazy qualified as B
import Data.Map.Strict (Map)
import Data.Map.Strict qualified as Map
import Data.Scientific (FPFormat (Generic), Scientific, floatingOrInteger, formatScientific)
import Data.Text (Text)
import Data.Text qualified as T
import Data.Vector qualified as V
import Slackblocks.Internal.JsonSyntax (parseJSONBytes)
import Slackblocks.Validation

-- | Validate an Aeson value as a supported modeled type. It is not an arbitrary Slack event parser.
class FromSlackJSON a where
  -- | Check nested fields and receiving context; return structured validation errors.
  fromSlackJSON :: Value -> Result a

-- | Validate an already parsed value. Use decode to retain duplicate-key and UTF-8 checks on raw bytes.
fromValue :: (FromSlackJSON a) => Value -> Result a
fromValue = fromSlackJSON

-- | Exact numeric convenience for a number-input bound. No binary floating-point
-- conversion is performed; the wire representation remains a decimal string.
decimalText :: Scientific -> Text
decimalText = T.pack . formatScientific Generic Nothing

-- | Decode UTF-8 JSON bytes, rejecting duplicate keys, trailing input, numeric
-- exponent overflow in Aeson's Scientific representation and invalid modeled values.
decode :: (FromSlackJSON a) => B.ByteString -> Result a
decode input = parseJSONBytes input >>= fromSlackJSON

instance FromSlackJSON Text where
  fromSlackJSON (String value) = Right value
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected text"

instance FromSlackJSON Bool where
  fromSlackJSON (Bool value) = Right value
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected a boolean"

instance FromSlackJSON Integer where
  fromSlackJSON (Number value) = case floatingOrInteger value :: Either Double Integer of
    Right integer -> Right integer
    Left _ -> failure TypeMismatch [] "type" "expected an integer"
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected an integer"

instance FromSlackJSON Scientific where
  fromSlackJSON (Number value) = Right value
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected a number"

instance (FromSlackJSON a) => FromSlackJSON [a] where
  fromSlackJSON (Array values) = traverse (\(index, value) -> atIndex index (fromSlackJSON value)) (zip [0 ..] (V.toList values))
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected an array"

instance FromSlackJSON (Map Text Value) where
  fromSlackJSON (Object fields) = Right (Map.fromList [(Key.toText key, value) | (key, value) <- KM.toList fields])
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected an object"
