<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks\{ErrorCategory as C, ValidationError};

/** @internal Checked JSON primitives, independent of Block Kit's modeled graph. */
final class Json
{
    public static function encode(mixed $value): string
    {
        try {
            return json_encode($value, JSON_THROW_ON_ERROR | JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES | JSON_PRESERVE_ZERO_FRACTION);
        } catch (\JsonException $error) {
            throw new ValidationError(C::TypeMismatch, '$', $error->getMessage());
        }
    }

    public static function decode(string $json): mixed
    {
        try {
            $value = json_decode($json, flags: JSON_THROW_ON_ERROR);
        } catch (\JsonException $error) {
            throw new ValidationError(C::TypeMismatch, '$', $error->getMessage());
        }
        // Skip strings before inspecting number tokens. PHP otherwise rounds oversized
        // integers and underflows tiny finite literals silently during json_decode.
        preg_match_all('/"(?:[^"\\\\]|\\\\.)*"|(-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?)/s', $json, $tokens, PREG_SET_ORDER);
        foreach ($tokens as $token) {
            if (!isset($token[1])) {
                continue;
            }
            $number = $token[1];
            if (!strpbrk($number, '.eE')) {
                $negative = str_starts_with($number, '-');
                $digits = ltrim($number, '-');
                $bound = $negative ? '9223372036854775808' : '9223372036854775807';
                if (strlen($digits) > strlen($bound) || (strlen($digits) === strlen($bound) && strcmp($digits, $bound) > 0)) {
                    throw new ValidationError(C::OutOfRange, '$', 'JSON integer exceeds signed 64-bit range');
                }
            } else {
                $float = (float) $number;
                $mantissa = explode('e', strtolower($number), 2)[0];
                if (!is_finite($float) || ($float == 0.0 && preg_match('/[1-9]/', $mantissa))) {
                    throw new ValidationError(C::OutOfRange, '$', 'JSON number exceeds finite double range');
                }
            }
        }
        self::validate($value, '$');
        return $value;
    }

    public static function validate(mixed $value, string $path, int $depth = 0): void
    {
        if ($depth > 128) {
            throw new ValidationError(C::TypeMismatch, $path, 'JSON nesting exceeds 128 levels or contains a cycle');
        }
        if (is_string($value)) {
            self::length($value, $path);
        } elseif (is_float($value) && !is_finite($value)) {
            throw new ValidationError(C::OutOfRange, $path, 'number must be finite');
        } elseif (is_array($value) || $value instanceof \stdClass) {
            foreach ((array) $value as $key => $child) {
                self::length((string) $key, $path);
                self::validate($child, $path . '.' . $key, $depth + 1);
            }
        } elseif ($value !== null && !is_int($value) && !is_float($value) && !is_bool($value)) {
            throw new ValidationError(C::TypeMismatch, $path, 'expected JSON primitives, arrays or stdClass');
        }
    }

    public static function length(string $text, string $path): int
    {
        $length = preg_match_all('/./us', $text);
        if ($length === false) {
            throw new ValidationError(C::TypeMismatch, $path, 'expected valid UTF-8');
        }
        return $length;
    }
}
