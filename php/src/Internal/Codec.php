<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks\{ErrorCategory as C, JsonObject, RichTextStyle, TaskCardBlock, TaskStatus, ValidationError, Value};

/** @internal Translation between typed values and Slack's wire objects. */
final class Codec
{
    public static function name(Value $value): string
    {
        return substr($value::class, strlen('Slackblocks\\'));
    }

    public static function encode(Value $value, bool $planTask = false): \stdClass
    {
        $name = self::name($value);
        $schema = Schema::TYPES[$name];
        if ($value instanceof TaskCardBlock && $value->status === TaskStatus::Pending && !$planTask) {
            throw new ValidationError(C::TypeMismatch, $name . '.status', 'pending tasks are only valid inside a plan');
        }
        $wire = [];
        if ($schema['wireType'] !== '' && !$planTask) {
            $wire['type'] = $schema['wireType'];
        }
        foreach ($schema['fields'] as $field) {
            $child = $value->{$field['method']};
            if ($child === null) {
                continue;
            }
            $wire[$field['wire']] = self::encodeChild($child, $name === 'PlanBlock' && $field['wire'] === 'tasks');
        }
        foreach ($value->extensions?->toArray() ?? [] as $key => $child) {
            $wire[$key] = $child;
        }
        return (object) $wire;
    }

    private static function encodeChild(mixed $child, bool $planTask = false): mixed
    {
        if ($child instanceof Value) {
            return self::encode($child, $planTask);
        }
        if ($child instanceof \BackedEnum) {
            return $child->value;
        }
        if ($child instanceof JsonObject || $child instanceof RichTextStyle) {
            return $child->jsonSerialize();
        }
        if (is_array($child)) {
            return array_map(static fn(mixed $item): mixed => self::encodeChild($item, $planTask), $child);
        }
        return $child;
    }

    /** @param class-string<Value> $class */
    public static function decode(string $class, mixed $input, string $path): Value
    {
        $name = substr($class, strlen('Slackblocks\\'));
        if (is_array($input) && (!array_is_list($input) || $input === [])) {
            $input = (object) $input;
        }
        if (!$input instanceof \stdClass) {
            throw new ValidationError(C::TypeMismatch, $path, 'expected an object');
        }
        $schema = Schema::TYPES[$name];
        $data = (array) $input;
        if ($schema['wireType'] !== '' && ($data['type'] ?? null) !== $schema['wireType']) {
            throw new ValidationError(C::TypeMismatch, $path . '.type', 'unexpected or missing type');
        }
        if ($schema['wireType'] !== '') {
            unset($data['type']);
        }
        $args = [];
        foreach ($schema['fields'] as $field) {
            $key = $field['wire'];
            if (!array_key_exists($key, $data)) {
                if (($field['required'] ?? false) && !array_key_exists($key, $schema['defaults'])) {
                    throw new ValidationError(C::MissingRequired, $path . '.' . $key, 'required field is missing');
                }
                continue;
            }
            $value = $data[$key];
            unset($data[$key]);
            if ($value === null) {
                if ($field['required'] ?? false) {
                    throw new ValidationError(C::MissingRequired, $path . '.' . $key, 'required field is null');
                }
                $args[$field['method']] = null;
                continue;
            }
            if ($name === 'PlanBlock' && $key === 'tasks' && is_array($value)) {
                $value = array_map(static function (mixed $task): mixed {
                    if ($task instanceof \stdClass || (is_array($task) && !array_is_list($task))) {
                        $task = (array) $task;
                        $task['type'] ??= 'task_card';
                        return (object) $task;
                    }
                    return $task;
                }, $value);
            }
            $args[$field['method']] = self::field($field, $value, $path . '.' . $key);
        }
        if ($data !== []) {
            $args['extensions'] = new JsonObject($data);
        }
        try {
            return new $class(...$args);
        } catch (ValidationError $error) {
            // Keep constructor errors anchored in the incoming nested wire path.
            $suffix = str_starts_with($error->path, $name) ? substr($error->path, strlen($name)) : '.' . $error->path;
            throw new ValidationError($error->category, $path . $suffix, $error->getMessage());
        }
    }

    /** @param array<string, mixed> $field */
    private static function field(array $field, mixed $value, string $path): mixed
    {
        $kind = $field['kind'];
        $type = $field['type'] ?? null;
        $bad = static fn(): ValidationError => new ValidationError(C::TypeMismatch, $path, 'expected ' . $kind);
        switch ($kind) {
            case 'string':
                if (!is_string($value)) {
                    throw $bad();
                }
                Json::length($value, $path);
                return $value;
            case 'boolean':
                if (!is_bool($value)) {
                    throw $bad();
                }
                return $value;
            case 'int':
            case 'long':
                if (!is_int($value)) {
                    throw $bad();
                }
                return $value;
            case 'number':
            case 'double':
                if (!is_int($value) && !is_float($value)) {
                    throw $bad();
                }
                Json::validate($value, $path);
                return $value;
            case 'enum':
                $enum = 'Slackblocks\\' . $type;
                if (!is_string($value) || ($case = $enum::tryFrom($value)) === null) {
                    throw $bad();
                }
                return $case;
            case 'map':
                if (!$value instanceof \stdClass && !(is_array($value) && (!array_is_list($value) || $value === []))) {
                    throw $bad();
                }
                return new JsonObject($value);
            case 'style':
                if (!$value instanceof \stdClass && !is_array($value)) {
                    throw $bad();
                }
                $args = [];
                foreach ((array) $value as $key => $flag) {
                    if (!is_bool($flag) || !in_array($key, $field['flags'], true)) {
                        throw $bad();
                    }
                    $args[$key === 'client_highlight' ? 'clientHighlight' : $key] = $flag;
                }
                return new RichTextStyle(...$args);
            case 'text':
                if (is_string($value)) {
                    $class = $field['coerce'] === 'plain_text' ? \Slackblocks\PlainText::class : \Slackblocks\MarkdownText::class;
                    return new $class($value);
                }
                // Text objects and other role-typed fields share discriminator resolution.
                // no break
            case 'object':
                return self::child($type, $value, $path);
            case 'list':
            case 'rows':
            case 'textList':
            case 'stringList':
                if (!is_array($value) || !array_is_list($value)) {
                    throw $bad();
                }
                $result = [];
                foreach ($value as $i => $item) {
                    $inner = $field;
                    $inner['kind'] = match ($kind) {
                        'rows' => 'list', 'textList' => 'text', 'stringList' => 'string', default => 'object',
                    };
                    $result[] = self::field($inner, $item, $path . '[' . $i . ']');
                }
                return $result;
            default:
                throw new \LogicException('Unknown generated field kind: ' . $kind);
        }
    }

    private static function child(string $target, mixed $value, string $path): Value
    {
        $class = 'Slackblocks\\' . $target;
        if (isset(Schema::TYPES[$target]) && is_a($class, Value::class, true)) {
            return self::decode($class, $value, $path);
        }
        $wire = $value instanceof \stdClass ? (array) $value : $value;
        if (is_array($wire)) {
            foreach (Schema::TYPES as $name => $model) {
                $candidate = 'Slackblocks\\' . $name;
                if (($wire['type'] ?? null) === $model['wireType'] && is_a($candidate, $class, true)) {
                    return self::decode($candidate, $value, $path);
                }
            }
        }
        throw new ValidationError(C::TypeMismatch, $path, 'expected ' . $target);
    }
}
