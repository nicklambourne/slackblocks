<?php

declare(strict_types=1);

namespace Slackblocks;

/** Expands one page into blocks, optional page text and navigation buttons. */
final class Paginator
{
    /**
     * @param string $actionIdPrefix Nonempty prefix for .previous and .next action IDs.
     * @param list<Block> $blocks Full nonempty sequence of content blocks.
     * @param int $page One-based page number.
     * @param int $pageSize Positive number of content blocks per page; controls are additional.
     * @param PlainText|string $previousText Previous-button label.
     * @param PlainText|string $nextText Next-button label.
     * @return list<Block> Spread into a payload to enforce that surface's total block limit.
     */
    public static function create(
        string $actionIdPrefix,
        array $blocks,
        int $page = 1,
        int $pageSize = 5,
        PlainText|string $previousText = 'Previous',
        PlainText|string $nextText = 'Next',
        bool $showPageIndicator = true,
        ?string $blockId = null,
    ): array {
        if (!array_is_list($blocks)) {
            throw new ValidationError(ErrorCategory::TypeMismatch, 'Paginator.blocks', 'expected a list');
        }
        foreach ($blocks as $i => $block) {
            if (!$block instanceof Block || !$block instanceof Value) {
                throw new ValidationError(ErrorCategory::TypeMismatch, 'Paginator.blocks[' . $i . ']', 'expected a validated block');
            }
        }
        if ($blocks === []) {
            throw new ValidationError(ErrorCategory::MissingRequired, 'Paginator.blocks', 'expected at least one block');
        }
        if ($actionIdPrefix === '') {
            throw new ValidationError(ErrorCategory::MissingRequired, 'Paginator.action_id_prefix', 'expected a nonempty prefix');
        }
        Internal\Json::length($actionIdPrefix, 'Paginator.action_id_prefix');
        if ($page < 1 || $pageSize < 1) {
            throw new ValidationError(ErrorCategory::OutOfRange, 'Paginator', 'page and page_size must be positive');
        }
        $pageCount = intdiv(count($blocks) - 1, $pageSize) + 1;
        if ($page > $pageCount) {
            throw new ValidationError(ErrorCategory::OutOfRange, 'Paginator.page', 'page exceeds page count');
        }
        $result = array_slice($blocks, ($page - 1) * $pageSize, $pageSize);
        if ($pageCount === 1) {
            return $result;
        }
        $controls = [];
        if ($page > 1) {
            $controls[] = new ButtonElement(text: $previousText, actionId: $actionIdPrefix . '.previous', value: (string) ($page - 1));
        }
        if ($page < $pageCount) {
            $controls[] = new ButtonElement(text: $nextText, actionId: $actionIdPrefix . '.next', value: (string) ($page + 1));
        }
        if ($showPageIndicator) {
            $result[] = new ContextBlock(elements: [new MarkdownText('Page ' . $page . ' of ' . $pageCount)]);
        }
        $result[] = new ActionsBlock(elements: $controls, blockId: $blockId);
        return $result;
    }
}
