<?php

declare(strict_types=1);

namespace Slackblocks;

/** Creates one validated collapsible container for an accordion. */
final class AccordionSection
{
    /**
     * @param PlainText|string $title Section heading.
     * @param list<Block> $blocks Ordered child blocks.
     * @param PlainText|string|null $subtitle Secondary heading.
     */
    public static function create(
        PlainText|string $title,
        array $blocks,
        PlainText|string|null $subtitle = null,
        ?ImageElement $icon = null,
        bool $expanded = false,
        ContainerWidth $width = ContainerWidth::Standard,
        ?string $blockId = null,
    ): ContainerBlock {
        return new ContainerBlock(
            childBlocks: $blocks,
            title: $title,
            subtitle: $subtitle,
            icon: $icon,
            width: $width,
            isCollapsible: true,
            defaultCollapsed: !$expanded,
            hasHeaderDivider: false,
            blockId: $blockId,
        );
    }
}
