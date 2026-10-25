-- | Preserve Aeson's exact numbers and UTF-8 checks while rejecting duplicate
-- object keys before they can be discarded by a KeyMap, including in metadata.
module Slackblocks.Internal.JsonSyntax (parseJSONBytes) where

import Control.Monad (when)
import Data.Aeson (Value (..))
import Data.Aeson.Decoding.ByteString.Lazy (lbsToTokens)
import Data.Aeson.Decoding.Tokens qualified as Token
import Data.Aeson.Key qualified as Key
import Data.Aeson.KeyMap qualified as KM
import Data.ByteString.Lazy qualified as B
import Data.Text qualified as T
import Data.Vector qualified as V
import Slackblocks.Validation

syntaxFailure :: [PathPiece] -> String -> Result a
syntaxFailure path message = failure TypeMismatch path "json" (T.pack message)

parseJSONBytes :: B.ByteString -> Result Value
parseJSONBytes bytes = do
  (value, rest) <- valueToken [] (lbsToTokens bytes)
  if B.all (`elem` [9, 10, 13, 32]) rest
    then Right value
    else syntaxFailure [] "trailing input after JSON value"

valueToken :: [PathPiece] -> Token.Tokens k String -> Result (Value, k)
valueToken path tokens = case tokens of
  Token.TkLit literal rest -> Right (case literal of Token.LitNull -> Null; Token.LitTrue -> Bool True; Token.LitFalse -> Bool False, rest)
  Token.TkText value rest -> Right (String value, rest)
  Token.TkNumber value rest -> Right (Number (case value of Token.NumInteger n -> fromInteger n; Token.NumDecimal n -> n; Token.NumScientific n -> n), rest)
  Token.TkArrayOpen items -> arrayTokens path 0 [] items
  Token.TkRecordOpen fields -> objectTokens path KM.empty fields
  Token.TkErr message -> syntaxFailure path message

arrayTokens :: [PathPiece] -> Int -> [Value] -> Token.TkArray k String -> Result (Value, k)
arrayTokens path index values tokens = case tokens of
  Token.TkItem item -> do
    (value, rest) <- valueToken (path <> [Index index]) item
    arrayTokens path (index + 1) (value : values) rest
  Token.TkArrayEnd rest -> Right (Array (V.fromList (reverse values)), rest)
  Token.TkArrayErr message -> syntaxFailure path message

objectTokens :: [PathPiece] -> KM.KeyMap Value -> Token.TkRecord k String -> Result (Value, k)
objectTokens path fields tokens = case tokens of
  Token.TkPair key item -> do
    let childPath = path <> [Field (Key.toText key)]
    when (KM.member key fields) $ failure TypeMismatch childPath "json.duplicate_key" "duplicate object key"
    (value, rest) <- valueToken childPath item
    objectTokens path (KM.insert key value fields) rest
  Token.TkRecordEnd rest -> Right (Object fields, rest)
  Token.TkRecordErr message -> syntaxFailure path message
