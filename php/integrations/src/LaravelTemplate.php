<?php

declare(strict_types=1);

namespace Slackblocks\Examples;

use Illuminate\Notifications\Slack\SlackMessage;
use Slackblocks\{Block, MessagePayload};

/** A blocks-only bridge for Laravel's native notification message. */
final class LaravelTemplate
{
    /**
     * Channel and fallback text are explicit: usingBlockKitTemplate imports only blocks.
     * Laravel's associative JSON decoding cannot preserve empty object-shaped extensions.
     * Reject that case rather than silently changing a payload; use Sending for such content.
     * @param list<Block> $blocks
     */
    public static function message(string $channel, string $fallback, array $blocks): SlackMessage
    {
        $payload = new MessagePayload(channel: $channel, blocks: $blocks, text: $fallback);
        $message = (new SlackMessage())->to($channel)->text($fallback)->usingBlockKitTemplate($payload->toJson());
        $actual = json_encode($message->toArray()['blocks'] ?? [], JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION);
        $expected = json_encode($payload->toArray()['blocks'], JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION);
        if ($actual !== $expected) {
            throw new \LogicException('Laravel template decoding changes this JSON shape; send the complete payload with JoliCode or cURL');
        }
        return $message;
    }
}
