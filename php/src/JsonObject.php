<?php

declare(strict_types=1);

namespace Slackblocks;

use Slackblocks\Internal\Json;

/**
 * Immutable, opaque application JSON. An empty instance encodes as {}, not [].
 * Nested objects are copied on input and export; modeled validation never traverses this content.
 */
final readonly class JsonObject implements \JsonSerializable
{
    private string $json;

    /** @param array<string|int, mixed>|\stdClass $fields Object members, including explicit nulls. */
    public function __construct(array|\stdClass $fields = [])
    {
        $object = (object) $fields;
        Json::validate($object, 'JsonObject');
        $this->json = Json::encode($object);
    }

    /** Parses object-shaped JSON, rejecting unsupported numeric ranges and malformed UTF-8. */
    public static function fromJson(string $json): self
    {
        $value = Json::decode($json);
        if (!$value instanceof \stdClass) {
            throw new ValidationError(ErrorCategory::TypeMismatch, 'JsonObject', 'expected a JSON object');
        }
        return new self($value);
    }

    /** Returns a detached object, preserving nested {} versus [] and explicit nulls. */
    public function jsonSerialize(): \stdClass
    {
        /** @var \stdClass */
        return json_decode($this->json, flags: JSON_THROW_ON_ERROR);
    }

    /** @return array<string|int, mixed> Detached members; nested objects remain stdClass. */
    public function toArray(): array
    {
        return (array) $this->jsonSerialize();
    }

    /** Encodes without exposing the internal storage. */
    public function toJson(): string
    {
        return $this->json;
    }
}
