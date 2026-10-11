<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks\{ErrorCategory as C, JsonObject, RichTextStyle, TaskCardBlock, ValidationError, Value};

/** @internal Validates native fields before applying cross-field and surface rules. */
final class Validator
{
    public static function value(Value $value): void
    {
        $name = Codec::name($value);
        $model = Schema::TYPES[$name];
        foreach ($model['fields'] as $field) {
            $item = $value->{$field['method']};
            $path = $name . '.' . $field['wire'];
            if ($item === null) {
                if ($field['required'] ?? false) {
                    self::fail(C::MissingRequired, $path, 'required field is missing');
                }
                continue;
            }
            self::field($field, $item, $path);
            if (isset($field['limits'])) {
                self::limits($item, $field['limits'], $path);
            }
        }
        $reserved = array_column($model['fields'], 'wire');
        foreach ($value->extensions?->toArray() ?? [] as $key => $unused) {
            if ($key === '' || $key === 'type' || in_array($key, $reserved, true)) {
                self::fail(C::InvalidUsage, $name . '.' . $key, 'extension field is reserved');
            }
        }
        if ($value instanceof \Slackblocks\PlainText || $value instanceof \Slackblocks\MarkdownText) {
            self::limits($value->text, 'text', $name . '.text');
        }
        Rules::validate($value, Codec::encode($value, $value instanceof TaskCardBlock));
    }

    /** @param array<string, mixed> $field */
    private static function field(array $field, mixed $item, string $path): void
    {
        $kind = $field['kind'];
        if (in_array($kind, ['list', 'rows', 'textList', 'stringList'], true)) {
            if (!is_array($item) || !array_is_list($item)) {
                self::fail(C::TypeMismatch, $path, 'expected a list');
            }
            foreach ($item as $index => $child) {
                $inner = $field;
                $inner['kind'] = match ($kind) {
                    'rows' => 'list', 'textList' => 'text', 'stringList' => 'string', default => 'object',
                };
                self::field($inner, $child, $path . '[' . $index . ']');
            }
        } elseif ($kind === 'string') {
            if (!is_string($item)) {
                self::fail(C::TypeMismatch, $path, 'expected string');
            }
            Json::length($item, $path);
        } elseif ($kind === 'style') {
            if (!$item instanceof RichTextStyle) {
                self::fail(C::TypeMismatch, $path, 'expected RichTextStyle');
            }
            foreach ((array) $item->jsonSerialize() as $flag => $unused) {
                if (!in_array($flag, $field['flags'], true)) {
                    self::fail(C::TypeMismatch, $path . '.' . $flag, 'unsupported style flag');
                }
            }
        } elseif (in_array($kind, ['object', 'text', 'enum'], true)) {
            $class = 'Slackblocks\\' . $field['type'];
            if (!$item instanceof $class || ($kind !== 'enum' && !$item instanceof Value)) {
                self::fail(C::TypeMismatch, $path, 'expected ' . $field['type']);
            }
        } elseif ($kind === 'map') {
            if (!$item instanceof JsonObject) {
                self::fail(C::TypeMismatch, $path, 'expected JsonObject');
            }
        } elseif (in_array($kind, ['double', 'number'], true)) {
            if (!is_int($item) && !is_float($item)) {
                self::fail(C::TypeMismatch, $path, 'expected number');
            }
            Json::validate($item, $path);
        }
    }

    public static function limits(mixed $value, string $prefix, string $path): void
    {
        $length = is_array($value) ? count($value) : (is_string($value) ? Json::length($value, $path) : null);
        if ($value instanceof \Slackblocks\PlainText || $value instanceof \Slackblocks\MarkdownText) {
            $length = Json::length($value->text, $path);
        }
        $suffixes = is_array($value) ? ['min_items', 'max_items'] : ['min_length', 'max_length'];
        foreach ($suffixes as $suffix) {
            $bound = Schema::LIMITS[$prefix . '.' . $suffix] ?? null;
            if ($length !== null && $bound !== null && (str_starts_with($suffix, 'min') ? $length < $bound : $length > $bound)) {
                self::fail(C::LengthExceeded, $path, 'length outside ' . $prefix . ' bounds');
            }
        }
        if (is_int($value) || is_float($value)) {
            foreach (['min', 'max', 'exclusive_min', 'exclusive_max'] as $suffix) {
                $bound = Schema::LIMITS[$prefix . '.' . $suffix] ?? null;
                if ($bound !== null && match ($suffix) {
                    'min' => $value < $bound, 'max' => $value > $bound, 'exclusive_min' => $value <= $bound, 'exclusive_max' => $value >= $bound,
                }) {
                    self::fail(C::OutOfRange, $path, 'number outside ' . $prefix . ' bounds');
                }
            }
        }
    }

    public static function fail(C $category, string $path, string $message): never
    {
        throw new ValidationError($category, $path, $message);
    }
}
