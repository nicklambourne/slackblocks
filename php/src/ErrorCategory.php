<?php

declare(strict_types=1);

namespace Slackblocks;

/** Stable validation categories shared by every Slackblocks implementation. */
enum ErrorCategory: string
{
    /** Text length or collection size is outside its allowed bounds. */
    case LengthExceeded = 'length-exceeded';
    /** A numeric value or page index is outside its allowed range. */
    case OutOfRange = 'out-of-range';
    /** Fields violate an exclusive-choice requirement. */
    case MutuallyExclusive = 'mutually-exclusive';
    /** A value has the wrong type, shape, vocabulary or serialization context. */
    case TypeMismatch = 'type-mismatch';
    /** A required field or required content is absent. */
    case MissingRequired = 'missing-required';
    /** Individually typed values violate a dependency or composition rule. */
    case InvalidUsage = 'invalid-usage';
}
