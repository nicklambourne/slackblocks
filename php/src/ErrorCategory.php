<?php

declare(strict_types=1);

namespace Slackblocks;

/** Stable validation categories shared by every Slackblocks implementation. */
enum ErrorCategory: string
{
    case LengthExceeded = 'length-exceeded';
    case OutOfRange = 'out-of-range';
    case MutuallyExclusive = 'mutually-exclusive';
    case TypeMismatch = 'type-mismatch';
    case MissingRequired = 'missing-required';
    case InvalidUsage = 'invalid-usage';
}
