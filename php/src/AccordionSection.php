<?php

declare(strict_types=1);

namespace Slackblocks;

/** Creates one validated collapsible container for an accordion. */
final class AccordionSection
{
    /** Creates a collapsible container with the requested initial expansion state.
     * @return ContainerBlock Validated section for a modal or Home tab.
     * @throws ValidationError A heading, child block or container option is invalid.
     * @param PlainText|string $title Section heading.
     * @param list<Block> $blocks Ordered child blocks.
     * @param PlainText|string|null $subtitle Secondary heading.
     * @param ImageElement|null $icon Optional image beside the heading.
     * @param bool $expanded Whether the section initially displays its child blocks.
     * @param ContainerWidth $width Layout width, standard by default.
     * @param string|null $blockId Stable identifier returned in interaction payloads.
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
