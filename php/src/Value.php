<?php

declare(strict_types=1);

namespace Slackblocks;

use Slackblocks\Internal\{Codec, Json, Validator};

/**
 * A validated immutable Slack value. Use concrete classes and named constructors.
 * Optional null fields are omitted. Arrays are detached; nested values are immutable.
 */
abstract readonly class Value implements \JsonSerializable
{
    protected function __construct(
        /** Additional opaque JSON fields; modeled names and type are reserved. */
        public ?JsonObject $extensions = null,
    ) {
        Validator::value($this);
    }

    /** Validates a copy with named constructor changes; the original is unchanged.
     * @param mixed $changes Named constructor arguments to replace; positional changes are rejected.
     * @return static A new validated value of the same concrete class.
     * @throws \InvalidArgumentException A field is unknown or a change is positional.
     * @throws ValidationError Replacement content violates a field or contextual rule.
     * @throws \TypeError A replacement is incompatible with its constructor parameter.
     */
    final public function with(mixed ...$changes): static
    {
        $values = get_object_vars($this);
        foreach ($changes as $name => $value) {
            if (!is_string($name) || !array_key_exists($name, $values)) {
                throw new \InvalidArgumentException('Unknown field: ' . $name);
            }
            $values[$name] = $value;
        }
        $class = $this::class;
        return new $class(...$values);
    }

    /** Exports Slack wire fields with detached collections and opaque object values.
     * @return array<string, mixed> Slack wire fields, preserving opaque object values.
     * @throws ValidationError A pending task is serialized outside a plan.
     */
    final public function toArray(): array
    {
        return (array) $this->jsonSerialize();
    }

    /** Serializes for json_encode(), rejecting pending tasks outside plans.
     * @return \stdClass Slack wire object with absent optional fields omitted.
     * @throws ValidationError A pending task is serialized outside a plan.
     */
    final public function jsonSerialize(): \stdClass
    {
        return Codec::encode($this);
    }

    /** Encodes Slack JSON, throwing ValidationError for invalid serialization context.
     * @return string Complete JSON text, preserving integer and opaque-object semantics.
     * @throws ValidationError A pending task is serialized outside a plan.
     */
    final public function toJson(): string
    {
        return Json::encode($this);
    }

    /** Reconstructs the concrete value from checked Slack wire fields.
     * @param array<string, mixed>|\stdClass $fields Untrusted Slack wire fields.
     * @return static Validated concrete value with normalized child objects.
     * @throws ValidationError A wire type, field, extension or contextual rule is invalid.
     */
    final public static function fromArray(array|\stdClass $fields): static
    {
        Json::validate((object) $fields, (new \ReflectionClass(static::class))->getShortName());
        $value = Codec::decode(static::class, (object) $fields, (new \ReflectionClass(static::class))->getShortName());
        assert($value instanceof static);
        return $value;
    }

    /** Parses untrusted Slack JSON with categorized errors and checked number ranges.
     * @param string $json Complete Slack JSON object text for this concrete type.
     * @return static Validated concrete value with normalized child objects.
     * @throws ValidationError JSON syntax, numeric range, fields or context are invalid.
     */
    final public static function fromJson(string $json): static
    {
        $value = Codec::decode(static::class, Json::decode($json), (new \ReflectionClass(static::class))->getShortName());
        assert($value instanceof static);
        return $value;
    }
}
