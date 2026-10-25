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

-- | Normative shared categories; categoryName returns the stable wire spelling.
data ErrorCategory
  = -- | Text or collection exceeds a shared limit.
    LengthExceeded
  | -- | A number or count is outside the accepted range.
    OutOfRange
  | -- | Fields cannot be supplied together.
    MutuallyExclusive
  | -- | The value has an unsupported type or vocabulary.
    TypeMismatch
  | -- | A required value is absent.
    MissingRequired
  | -- | A combination or receiving context is unsupported.
    InvalidUsage
  deriving (Eq, Ord, Show, Enum, Bounded)

-- | A field or zero-based array index in a validation path.
data PathPiece
  = -- | Object field name.
    Field Text
  | -- | Zero-based array position.
    Index Int
  deriving (Eq, Ord, Show)

-- | One validation failure with a machine-readable category, path and constraint.
data ValidationError = ValidationError
  { -- | Normative shared category.
    errorCategory :: ErrorCategory,
    -- | Structured path from the root payload.
    errorPath :: [PathPiece],
    -- | Stable constraint identifier.
    errorConstraint :: Text,
    -- | Human-readable explanation; do not use prose as a compatibility key.
    errorMessage :: Text
  }
  deriving (Eq, Show)

-- | Checked construction result. Failure contains at least one validation error.
type Result a = Either (NonEmpty ValidationError) a

-- | Stable shared category spelling suitable for logging and matching.
categoryName :: ErrorCategory -> Text
categoryName LengthExceeded = "length-exceeded"
categoryName OutOfRange = "out-of-range"
categoryName MutuallyExclusive = "mutually-exclusive"
categoryName TypeMismatch = "type-mismatch"
categoryName MissingRequired = "missing-required"
categoryName InvalidUsage = "invalid-usage"

-- | Format a path starting at $, with dot fields and zero-based bracket indexes.
renderPath :: [PathPiece] -> Text
renderPath = ("$" <>) . T.concat . map render
  where
    render (Field name) = "." <> name
    render (Index index) = "[" <> T.pack (show index) <> "]"

-- | Create one failure without throwing an exception.
failure :: ErrorCategory -> [PathPiece] -> Text -> Text -> Result a
failure category path constraint message =
  Left (ValidationError category path constraint message :| [])

-- | Prefix every error path with a field; preserve successful values.
atField :: Text -> Result a -> Result a
atField name = either (Left . fmap prepend) Right
  where
    prepend err = err {errorPath = Field name : errorPath err}

-- | Prefix every error path with a zero-based index; preserve successful values.
atIndex :: Int -> Result a -> Result a
atIndex index = either (Left . fmap prepend) Right
  where
    prepend err = err {errorPath = Index index : errorPath err}
