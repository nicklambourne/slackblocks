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
    protected function __construct(public ?JsonObject $extensions = null)
    {
        Validator::value($this);
    }

    /** Validates a copy with named constructor changes; the original is unchanged. */
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

    /** @return array<string, mixed> Slack wire fields, preserving opaque object values. */
    final public function toArray(): array
    {
        return (array) $this->jsonSerialize();
    }

    /** Serializes for json_encode(), rejecting pending tasks outside plans. */
    final public function jsonSerialize(): \stdClass
    {
        return Codec::encode($this);
    }

    /** Encodes Slack JSON, throwing ValidationError for invalid serialization context. */
    final public function toJson(): string
    {
        return Json::encode($this);
    }

    /** @param array<string, mixed>|\stdClass $fields Untrusted Slack wire fields. */
    final public static function fromArray(array|\stdClass $fields): static
    {
        Json::validate((object) $fields, (new \ReflectionClass(static::class))->getShortName());
        $value = Codec::decode(static::class, (object) $fields, (new \ReflectionClass(static::class))->getShortName());
        assert($value instanceof static);
        return $value;
    }

    /** Parses untrusted Slack JSON with categorized errors and checked number ranges. */
    final public static function fromJson(string $json): static
    {
        $value = Codec::decode(static::class, Json::decode($json), (new \ReflectionClass(static::class))->getShortName());
        assert($value instanceof static);
        return $value;
    }
}
