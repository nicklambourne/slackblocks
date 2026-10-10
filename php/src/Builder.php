<?php

declare(strict_types=1);

namespace Slackblocks;

/** Slack Block Kit Builder previews, without making network requests. */
final class Builder
{
    /**
     * Blocks are wrapped in a blocks object; payloads and explicit JsonObject input are used as-is.
     * @param Block|MessagePayload|MessageResponse|WebhookMessage|ModalView|HomeTabView|JsonObject|list<Block> $payload Preview content.
     * @param string|null $teamId Optional workspace ID, encoded as one URL path segment.
     */
    public static function url(Block|MessagePayload|MessageResponse|WebhookMessage|ModalView|HomeTabView|JsonObject|array $payload, ?string $teamId = null): string
    {
        if ($payload instanceof Block) {
            $payload = [$payload];
        }
        if (is_array($payload)) {
            if (!array_is_list($payload)) {
                throw new ValidationError(ErrorCategory::TypeMismatch, 'Builder.blocks', 'expected a list');
            }
            foreach ($payload as $block) {
                if (!$block instanceof Block || !$block instanceof Value) {
                    throw new ValidationError(ErrorCategory::TypeMismatch, 'Builder.blocks', 'expected validated blocks');
                }
            }
            $payload = (object) ['blocks' => $payload];
        }
        Internal\Json::length($teamId ?? '', 'Builder.team_id');
        return 'https://app.slack.com/block-kit-builder/' . rawurlencode($teamId ?? '') . '#' . rawurlencode(Internal\Json::encode($payload));
    }
}
