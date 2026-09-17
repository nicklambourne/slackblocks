using System.Linq;
using System.Text.Json.Nodes;
using Slackblocks.Blocks;
using Slackblocks.Components;
using Slackblocks.Elements;

namespace Slackblocks.Tests;

public sealed class ComponentTests
{
    [Fact]
    public void AccordionBuildsSlackNativeContainerBlocks()
    {
        var blocks = Accordion.Create([
            AccordionSection.Create("Deployment details", [Section("*Build:* 482")], expanded: true, width: ContainerWidth.Wide),
        ]);

        var container = Wire(blocks[0]);
        Assert.Equal("container", container["type"]!.GetValue<string>());
        Assert.True(container["is_collapsible"]!.GetValue<bool>());
        Assert.False(container["default_collapsed"]!.GetValue<bool>());
        Assert.Equal("wide", container["width"]!.GetValue<string>());
    }

    [Fact]
    public void AccordionSectionsCarryOptionalHeaderSettings()
    {
        var icon = new ImageElement("Build", imageUrl: "https://example.com/icon.png");
        var section = AccordionSection.Create(
            "Checks",
            [new DividerBlock()],
            subtitle: "*3* passed",
            icon: icon,
            hasHeaderDivider: false,
            blockId: "checks").ToJsonNode();

        Assert.Equal("""{"type":"mrkdwn","text":"*3* passed"}""", section["subtitle"]!.ToJsonString());
        Assert.Equal(icon.ToJson(), section["icon"]!.ToJsonString());
        Assert.False(section["has_header_divider"]!.GetValue<bool>());
        Assert.Equal("checks", section["block_id"]!.GetValue<string>());
        Assert.True(section["default_collapsed"]!.GetValue<bool>());
    }

    [Fact]
    public void PaginatorBuildsOnePageAndNavigationControls()
    {
        IBlock[] source = [Section("one"), Section("two"), Section("three"), Section("four"), Section("five")];

        var page = Paginator.Create("results", source, page: 2, pageSize: 2, blockId: "results.controls");

        Assert.Equal(["section", "section", "context", "actions"], page.Select(block => Wire(block)["type"]!.GetValue<string>()));
        var actions = Wire(page[3]);
        Assert.Equal("results.controls", actions["block_id"]!.GetValue<string>());
        var buttons = actions["elements"]!.AsArray();
        Assert.Equal("1", buttons[0]!["value"]!.GetValue<string>());
        Assert.Equal("results.previous", buttons[0]!["action_id"]!.GetValue<string>());
        Assert.Equal("3", buttons[1]!["value"]!.GetValue<string>());
        Assert.Equal("Page 2 of 3", Wire(page[2])["elements"]![0]!["text"]!.GetValue<string>());
    }

    [Fact]
    public void PaginatorLabelsAndIndicatorAreConfigurable()
    {
        var page = Paginator.Create(
            "results",
            [Section("one"), Section("two")],
            pageSize: 1,
            previousText: "Back",
            nextText: "More",
            showPageIndicator: false);

        Assert.Equal(["section", "actions"], page.Select(block => Wire(block)["type"]!.GetValue<string>()));
        Assert.Equal("""{"type":"plain_text","text":"More"}""", Wire(page[1])["elements"]![0]!["text"]!.ToJsonString());

        var last = Paginator.Create("results", [Section("one"), Section("two")], page: 2, pageSize: 1, previousText: "Back");
        Assert.Equal("""{"type":"plain_text","text":"Back"}""", Wire(last[2])["elements"]![0]!["text"]!.ToJsonString());
    }

    [Fact]
    public void ASinglePageHasNoControlsAndIgnoresTheBlockId()
    {
        var page = Paginator.Create("results", [Section("one")], blockId: "unused");

        Assert.Equal("section", Wire(Assert.Single(page))["type"]!.GetValue<string>());
    }

    [Fact]
    public void ComponentsReportStructuredValidationFailures()
    {
        Assert.Equal(ErrorCategory.MissingRequired, Assert.Throws<ValidationException>(() => Accordion.Create([])).Category);
        Assert.Equal(
            ErrorCategory.OutOfRange,
            Assert.Throws<ValidationException>(() => Paginator.Create("results", [Section("one")], page: 2)).Category);
        Assert.Equal(
            ErrorCategory.MissingRequired,
            Assert.Throws<ValidationException>(() => Paginator.Create(string.Empty, [Section("one")])).Category);
        Assert.Equal(
            ErrorCategory.TypeMismatch,
            Assert.Throws<ValidationException>(() => Accordion.Create([new ContainerBlock([new DividerBlock()], title: "Plain")])).Category);
    }

    [Fact]
    public void ComponentOutputSpreadsIntoPayloadCollections()
    {
        var message = new Payloads.MessagePayload(
            "C123",
            blocks: [new HeaderBlock("Results"), .. Paginator.Create("results", [Section("one"), Section("two")], pageSize: 1)]);

        Assert.Equal(4, message.Blocks.Count);
    }

    private static SectionBlock Section(string text) => new(text: text);

    private static JsonObject Wire(IBlock block) => block.ToJsonNode();
}
