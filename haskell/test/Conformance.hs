module Main (main) where

import Constructions (invalidCases, validCases)
import Control.Monad (forM, unless)
import Data.Aeson (FromJSON (..), Value (..), eitherDecodeFileStrict, withObject, (.:))
import Data.Aeson.Key qualified as Key
import Data.Aeson.KeyMap qualified as KM
import Data.List.NonEmpty qualified as NE
import Data.Set qualified as Set
import Data.Text (Text)
import Data.Text qualified as T
import Paths_slackblocks (getDataFileName)
import Slackblocks

newtype Fixture = Fixture String

instance FromJSON Fixture where
  parseJSON = withObject "Fixture" (fmap Fixture . (.: "id"))

newtype Manifest = Manifest [Fixture]

instance FromJSON Manifest where
  parseJSON = withObject "Manifest" (fmap Manifest . (.: "fixtures"))

data Invalid = Invalid String Text Text

instance FromJSON Invalid where
  parseJSON = withObject "Invalid case" $ \o -> Invalid <$> o .: "id" <*> o .: "category" <*> o .: "constraint"

newtype InvalidManifest = InvalidManifest [Invalid]

instance FromJSON InvalidManifest where
  parseJSON = withObject "Invalid manifest" (fmap InvalidManifest . (.: "cases"))

readData :: (FromJSON a) => FilePath -> IO a
readData name = do
  path <- getDataFileName ("test/spec/" <> name)
  result <- eitherDecodeFileStrict path
  either fail pure result

inventory :: String -> [String] -> [String] -> IO ()
inventory name expected actual = do
  unless
    ( not (null expected)
        && length actual == Set.size (Set.fromList actual)
        && Set.fromList expected == Set.fromList actual
    )
    $ fail
      ( name
          <> " inventory mismatch: missing "
          <> show (Set.fromList expected Set.\\ Set.fromList actual)
          <> "; unexpected "
          <> show (Set.fromList actual Set.\\ Set.fromList expected)
      )

checkValue :: String -> Value -> Result Value -> [String]
checkValue name expected actual = case actual of
  Left errors -> [name <> ": rejected: " <> show errors]
  Right value
    | value == expected -> []
    | otherwise -> [name <> ": wire mismatch\nexpected " <> show expected <> "\nactual " <> show value]

checkInvalid :: String -> Text -> Result () -> [String]
checkInvalid name expected actual = case actual of
  Right () -> [name <> ": accepted invalid value"]
  Left errors
    | any ((== expected) . categoryName . errorCategory) (NE.toList errors)
        && all (\e -> not (T.null (errorConstraint e)) && not (T.null (errorMessage e)) && not (T.null (renderPath (errorPath e)))) errors ->
        []
    | otherwise -> [name <> ": expected " <> T.unpack expected <> "; got " <> show errors]

main :: IO ()
main = do
  Manifest fixtures <- readData "manifest.json"
  InvalidManifest invalid <- readData "fixtures/invalid/manifest.json"
  inventory "valid" [name | Fixture name <- fixtures] [name | (name, _, _) <- validCases]
  inventory "invalid" [name | Invalid name _ _ <- invalid] [name | (name, _, _) <- invalidCases]
  limits <- readData "limits.json"
  let scalarPaths prefix (Object fields) =
        concat
          [scalarPaths (if T.null prefix then Key.toText key else prefix <> "." <> Key.toText key) value | (key, value) <- KM.toList fields]
      scalarPaths prefix (Number _) = [prefix]
      scalarPaths _ _ = error "Non-numeric shared limit"
      missing = Set.fromList (scalarPaths "" limits) Set.\\ Set.fromList [constraint | Invalid _ _ constraint <- invalid]
  unless (Set.null missing) (fail ("Scalar limits missing invalid cases: " <> show missing))
  validFailures <- forM validCases $ \(name, native, decoder) -> do
    expected <- readData ("fixtures/valid/" <> name <> ".json")
    pure (checkValue (name <> " native") expected native <> checkValue (name <> " ingress") expected (decoder expected))
  invalidFailures <- forM invalidCases $ \(name, native, ingress) ->
    case [category | Invalid identifier category _ <- invalid, identifier == name] of
      [category] -> pure (maybe [] (checkInvalid (name <> " native") category) native <> checkInvalid (name <> " ingress") category ingress)
      _ -> fail ("Invalid manifest category missing or duplicated: " <> name)
  let failures = concat (validFailures <> invalidFailures)
  mapM_ putStrLn failures
  unless (null failures) (fail (show (length failures) <> " conformance checks failed"))
  let nativeCount = length [() | (_, Just _, _) <- invalidCases]
  putStrLn
    ( show (length validCases)
        <> " valid native/ingress pairs; "
        <> show (length invalidCases)
        <> " invalid ingress cases and "
        <> show nativeCount
        <> " native invalid constructions passed. Eight unrepresentable inputs have compiler rejection controls."
    )
