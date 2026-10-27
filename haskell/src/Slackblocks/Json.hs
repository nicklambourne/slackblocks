{-# LANGUAGE FlexibleInstances #-}
-- | Checked JSON ingress. Unknown modeled values are rejected; opaque metadata
-- remains ordinary Aeson 'Value' and is never traversed as Block Kit content.
module Slackblocks.Json (FromSlackJSON (..), decode, fromValue, decimalText) where

import Data.Aeson (Value(..), eitherDecode)
import qualified Data.Aeson.Key as Key
import qualified Data.Aeson.KeyMap as KM
import Data.Map.Strict (Map)
import qualified Data.Map.Strict as Map
import Data.Scientific (Scientific, floatingOrInteger, formatScientific, FPFormat(Generic))
import Data.Text (Text)
import qualified Data.Vector as V
import qualified Data.ByteString.Lazy as B
import qualified Data.Text as T
import Slackblocks.Validation

class FromSlackJSON a where
  fromSlackJSON :: Value -> Result a

fromValue :: FromSlackJSON a => Value -> Result a
fromValue = fromSlackJSON

-- | Exact numeric convenience for a number-input bound. No binary floating-point
-- conversion is performed; the wire representation remains a decimal string.
decimalText :: Scientific -> Text
decimalText = T.pack . formatScientific Generic Nothing

decode :: FromSlackJSON a => B.ByteString -> Result a
decode input = case eitherDecode input of
  Left message -> failure TypeMismatch [] "json" (T.pack message)
  Right value -> fromSlackJSON value

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
instance FromSlackJSON a => FromSlackJSON [a] where
  fromSlackJSON (Array values) = traverse (\(index,value) -> atIndex index (fromSlackJSON value)) (zip [0..] (V.toList values))
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected an array"
instance FromSlackJSON (Map Text Value) where
  fromSlackJSON (Object fields) = Right (Map.fromList [(Key.toText key,value) | (key,value) <- KM.toList fields])
  fromSlackJSON _ = failure TypeMismatch [] "type" "expected an object"
