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
import Data.ByteString.Lazy.Char8 qualified as C
import Data.Char (isDigit)
import Data.Text qualified as T
import Data.Vector qualified as V
import Slackblocks.Validation

syntaxFailure :: [PathPiece] -> String -> Result a
syntaxFailure path message = failure TypeMismatch path "json" (T.pack message)

parseJSONBytes :: B.ByteString -> Result Value
parseJSONBytes bytes = do
  checkExponents bytes
  (value, rest) <- valueToken [] (lbsToTokens bytes)
  if B.all (`elem` [9, 10, 13, 32]) rest
    then Right value
    else syntaxFailure [] "trailing input after JSON value"

-- Aeson's lexer narrows exponents to Int without an overflow check. Inspect
-- numeric lexemes first, including metadata, before their spelling is lost.
-- This is a representation guard, not a second JSON grammar: Aeson still
-- validates syntax/UTF-8 below. Skip quoted strings and escaped quotes intact.
checkExponents :: B.ByteString -> Result ()
checkExponents bytes = case C.uncons bytes of
  Nothing -> Right ()
  Just ('"', rest) -> quoted rest
  Just (c, _) | isDigit c || c == '-' -> do
    let (number, rest) = C.span (\x -> isDigit x || x `elem` (".eE+-" :: String)) bytes
        (mantissa, exponentPart) = C.break (\x -> x == 'e' || x == 'E') number
        fraction = C.drop 1 (snd (C.break (== '.') mantissa))
    case C.readInteger (C.drop 1 exponentPart) of
      Just (exponentValue, trailing) | B.null trailing -> do
        let adjusted = exponentValue - toInteger (B.length fraction)
        when (adjusted < toInteger (minBound :: Int) || adjusted > toInteger (maxBound :: Int)) $
          failure TypeMismatch [] "json.number.exponent" "JSON number exponent is outside Scientific's representable range"
      _ -> Right ()
    checkExponents rest
  Just (_, rest) -> checkExponents (C.dropWhile (\c -> c /= '"' && c /= '-' && not (isDigit c)) rest)
  where
    quoted input = case C.uncons (C.dropWhile (\c -> c /= '"' && c /= '\\') input) of
      Nothing -> Right () -- The JSON lexer reports the unterminated string.
      Just ('"', rest) -> checkExponents rest
      Just (_, rest) -> quoted (C.drop 1 rest)

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
