-- | Stable validation categories and paths shared with the other implementations.
module Slackblocks.Validation
  ( ErrorCategory (..),
    PathPiece (..),
    ValidationError (..),
    Result,
    categoryName,
    renderPath,
    failure,
    atField,
    atIndex,
  )
where

import Data.List.NonEmpty (NonEmpty (..))
import Data.Text (Text)
import Data.Text qualified as T

data ErrorCategory
  = LengthExceeded
  | OutOfRange
  | MutuallyExclusive
  | TypeMismatch
  | MissingRequired
  | InvalidUsage
  deriving (Eq, Ord, Show, Enum, Bounded)

data PathPiece = Field Text | Index Int deriving (Eq, Ord, Show)

data ValidationError = ValidationError
  { errorCategory :: ErrorCategory,
    errorPath :: [PathPiece],
    errorConstraint :: Text,
    errorMessage :: Text
  }
  deriving (Eq, Show)

type Result a = Either (NonEmpty ValidationError) a

categoryName :: ErrorCategory -> Text
categoryName LengthExceeded = "length-exceeded"
categoryName OutOfRange = "out-of-range"
categoryName MutuallyExclusive = "mutually-exclusive"
categoryName TypeMismatch = "type-mismatch"
categoryName MissingRequired = "missing-required"
categoryName InvalidUsage = "invalid-usage"

renderPath :: [PathPiece] -> Text
renderPath = ("$" <>) . T.concat . map render
  where
    render (Field name) = "." <> name
    render (Index index) = "[" <> T.pack (show index) <> "]"

failure :: ErrorCategory -> [PathPiece] -> Text -> Text -> Result a
failure category path constraint message =
  Left (ValidationError category path constraint message :| [])

atField :: Text -> Result a -> Result a
atField name = either (Left . fmap prepend) Right
  where
    prepend err = err {errorPath = Field name : errorPath err}

atIndex :: Int -> Result a -> Result a
atIndex index = either (Left . fmap prepend) Right
  where
    prepend err = err {errorPath = Index index : errorPath err}
