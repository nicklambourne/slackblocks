using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Slackblocks.Blocks;
using Slackblocks.Objects;
using Slackblocks.Payloads;
namespace Slackblocks.Tests;

public sealed class LayoutTests
{
    private static LayoutBlock Grid() => new("Grid", [new LayoutRow([new LayoutItem(new DividerBlock())])]);
    [Fact]
    public void AcceptsBoundariesAndReplacementFallback()
    {
        var grid = Grid();
        Assert.Equal(50, new MessagePayload("C123", blocks: Enumerable.Repeat(grid, 50)).Blocks.Count);
        Assert.Equal(100, new HomeTabView(Enumerable.Repeat(grid, 100)).Blocks.Count);
        _ = new MessagePayload("C123", blocks: [new LayoutBlock("Grid", grid.Rows, fallback: Enumerable.Repeat(new DividerBlock(), 51))]);
        _ = new MessagePayload("C123", blocks: Enumerable.Repeat(new ContainerBlock(Enumerable.Repeat(new DividerBlock(), 10), title: "Existing"), 50));
    }
    [Fact]
    public void PreservesNativePropertiesAndSnapshots()
    {
        var item = new LayoutItem(new DividerBlock(), LayoutWidth.Wide, LayoutAlignment.End);
        var items = new List<LayoutItem> { item }; var row = new LayoutRow(items); items.Clear();
        Assert.Single(row.Items);
        Assert.Equal(LayoutWidth.Wide, item.Width);
        Assert.Equal(LayoutAlignment.End, item.HorizontalAlignment);
        var value = new LayoutBlock("Grid", [row], isLabelVisible: false, gridLines: LayoutGridLines.Rows, fallback: []);
        Assert.False(value.IsLabelVisible);
        Assert.Empty(value.Fallback);
        IBlock block = value;
        Assert.Equal(value.ToJson(), JsonSerializer.Serialize(block));
        Assert.False(typeof(ILayoutChild).IsAssignableFrom(typeof(LayoutBlock)));
        Assert.False(typeof(ILayoutChild).IsAssignableFrom(typeof(MarkdownBlock)));
        Assert.False(typeof(ILayoutFallback).IsAssignableFrom(typeof(ActionsBlock)));
    }
}
