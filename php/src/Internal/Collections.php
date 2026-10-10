<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks\{ErrorCategory, MarkdownText, Text, ValidationError};

/** @internal Detach PHP array references before storing validated readonly values. */
final class Collections
{
    /**
     * Reject non-list keys before copying entries without PHP references.
     * @template T
     * @param array<array-key, T> $items
     * @return list<T>
     */
    public static function copy(array $items, string $path): array
    {
        if (!array_is_list($items)) {
            throw new ValidationError(ErrorCategory::TypeMismatch, $path, 'expected a list');
        }
        $copy = [];
        foreach ($items as $item) {
            $copy[] = $item;
        }
        return $copy;
    }

    /**
     * Copy exactly two levels. Deeper or cyclic arrays are invalid table cells.
     * @template T
     * @param array<array-key, array<array-key, T>> $rows
     * @return list<list<T>>
     */
    public static function rows(array $rows, string $path): array
    {
        $copy = [];
        foreach (self::copy($rows, $path) as $key => $row) {
            $rowPath = $path . '[' . $key . ']';
            if (!is_array($row)) {
                throw new ValidationError(ErrorCategory::TypeMismatch, $rowPath, 'expected a list');
            }
            $copy[] = self::copy($row, $rowPath);
        }
        return $copy;
    }

    /**
     * Internal array callbacks coerce scalars even with strict_types enabled.
     * Check the original element before applying the section's text shorthand.
     * @param array<array-key, mixed> $items
     * @return list<Text>
     */
    public static function text(array $items, string $path): array
    {
        if (!array_is_list($items)) {
            throw new ValidationError(ErrorCategory::TypeMismatch, $path, 'expected a list');
        }
        $copy = [];
        foreach ($items as $key => $item) {
            if (!is_string($item) && !$item instanceof Text) {
                throw new ValidationError(ErrorCategory::TypeMismatch, $path . '[' . $key . ']', 'expected Text or string');
            }
            $copy[] = is_string($item) ? new MarkdownText($item) : $item;
        }
        return $copy;
    }
}
