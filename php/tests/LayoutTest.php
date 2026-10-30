<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class LayoutTest extends TestCase
{
    private function grid(): S\LayoutBlock
    {
        return new S\LayoutBlock('Grid', [new S\LayoutRow([new S\LayoutItem(new S\DividerBlock())])]);
    }
    public function testBoundariesAndReplacementFallback(): void
    {
        $g = $this->grid();
        self::assertCount(25, (new S\MessagePayload('C123', blocks: array_fill(0, 25, $g)))->blocks);
        self::assertCount(50, (new S\HomeTabView(array_fill(0, 50, $g)))->blocks);
        new S\MessagePayload('C123', blocks: [new S\LayoutBlock('Grid', $g->rows, fallback: array_fill(0, 50, new S\DividerBlock()))]);
        new S\MessagePayload('C123', blocks: array_fill(0, 50, new S\ContainerBlock(array_fill(0, 10, new S\DividerBlock()), title: 'Existing')));
    }
    public function testEnumsAndDetachedInput(): void
    {
        $items = [new S\LayoutItem(new S\DividerBlock(), S\LayoutWidth::Wide, S\LayoutAlignment::End)];
        $row = new S\LayoutRow($items);
        $items = [];
        self::assertCount(1, $row->items);
        self::assertSame('end', $row->items[0]->horizontalAlignment->value);
        self::assertSame(S\LayoutWidth::Wide, $row->items[0]->width);
    }
}
