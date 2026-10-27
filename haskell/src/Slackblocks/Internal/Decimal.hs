-- | Compare finite decimal strings without expanding their exponents.
module Slackblocks.Internal.Decimal (decimalParts, compareDecimal) where

import Data.Char (isAscii, isDigit)
import Data.Text (Text)
import Data.Text qualified as T
import Text.Read (readMaybe)

asciiDigits :: Text -> Bool
asciiDigits text = not (T.null text) && T.all (\c -> isAscii c && isDigit c) text

unsigned :: Text -> Text
unsigned value = case T.uncons value of
  Just (c, rest) | c == '+' || c == '-' -> rest
  _ -> value

signedInteger :: Text -> Maybe Integer
signedInteger text
  | asciiDigits (unsigned text) = readMaybe (T.unpack (if T.isPrefixOf "+" text then T.drop 1 text else text))
  | otherwise = Nothing

decimalParts :: Text -> Maybe (Int, Text, Integer)
decimalParts value = do
  (mantissa, exponentValue) <- case T.split (\c -> c == 'e' || c == 'E') value of
    [m] -> Just (m, 0)
    [m, e] -> (m,) <$> signedInteger e
    _ -> Nothing
  let pieces = T.splitOn "." (unsigned mantissa)
  fraction <- case pieces of
    [_] -> Just 0
    [_, f] -> Just (T.length f)
    _ -> Nothing
  let combined = T.concat pieces
  if not (asciiDigits combined)
    then Nothing
    else do
      let digits = T.dropWhile (== '0') combined
          sign
            | T.null digits = 0
            | T.isPrefixOf "-" mantissa = -1
            | otherwise = 1
          rank = exponentValue + fromIntegral (T.length digits) - fromIntegral fraction
      pure (sign, digits, rank)

compareDecimal :: Text -> Text -> Maybe Ordering
compareDecimal left right = do
  (a, x, rankX) <- decimalParts left
  (b, y, rankY) <- decimalParts right
  let width = max (T.length x) (T.length y)
      magnitude = compare rankX rankY <> compare (T.justifyLeft width '0' x) (T.justifyLeft width '0' y)
  pure $ if a /= b then compare a b else if a == 0 then EQ else if a < 0 then invert magnitude else magnitude
  where
    invert LT = GT; invert GT = LT; invert EQ = EQ
