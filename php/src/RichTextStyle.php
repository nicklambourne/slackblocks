<?php

declare(strict_types=1);

namespace Slackblocks;

/** Inline text styling. Each rich text owner restricts which flags it accepts. */
final readonly class RichTextStyle implements \JsonSerializable
{
    public function __construct(
        public ?bool $bold = null,
        public ?bool $italic = null,
        public ?bool $strike = null,
        public ?bool $code = null,
        public ?bool $highlight = null,
        public ?bool $clientHighlight = null,
        public ?bool $unlink = null,
    ) {}

    /** Returns only supplied flags, preserving explicit false. */
    public function jsonSerialize(): \stdClass
    {
        return (object) array_filter([
            'bold' => $this->bold, 'italic' => $this->italic, 'strike' => $this->strike,
            'code' => $this->code, 'highlight' => $this->highlight,
            'client_highlight' => $this->clientHighlight, 'unlink' => $this->unlink,
        ], static fn(?bool $value): bool => $value !== null);
    }
}
