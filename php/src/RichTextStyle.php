<?php

declare(strict_types=1);

namespace Slackblocks;

/** Inline text styling. Each rich text owner restricts which flags it accepts. */
final readonly class RichTextStyle implements \JsonSerializable
{
    /** Sets optional flags; null omits a flag and explicit false is preserved.
     * The owning rich-text node rejects flags outside its supported set.
     * @param bool|null $bold Uses bold emphasis on text, links or mentions.
     * @param bool|null $italic Uses italic emphasis on text, links or mentions.
     * @param bool|null $strike Uses strikethrough on text, links or mentions.
     * @param bool|null $code Uses inline code styling; accepted by text and link nodes.
     * @param bool|null $highlight Highlights a channel, user or user-group mention.
     * @param bool|null $clientHighlight Applies client highlight styling to a channel, user or user-group mention.
     * @param bool|null $unlink Displays a channel, user or user-group mention without a link.
     */
    public function __construct(
        /** Uses bold emphasis on text, links or mentions. */
        public ?bool $bold = null,
        /** Uses italic emphasis on text, links or mentions. */
        public ?bool $italic = null,
        /** Uses strikethrough on text, links or mentions. */
        public ?bool $strike = null,
        /** Uses inline code styling; accepted by text and link nodes. */
        public ?bool $code = null,
        /** Highlights a channel, user or user-group mention. */
        public ?bool $highlight = null,
        /** Applies client highlight styling to a channel, user or user-group mention. */
        public ?bool $clientHighlight = null,
        /** Displays a channel, user or user-group mention without a link. */
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
