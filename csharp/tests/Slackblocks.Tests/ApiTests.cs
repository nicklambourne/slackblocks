using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Text.Json.Nodes;
using Slackblocks.Blocks;
using Slackblocks.Elements;
using Slackblocks.Objects;
using Slackblocks.Payloads;

namespace Slackblocks.Tests;

public sealed class ApiTests
{
    [Fact]
    public void VersionMatchesThePackageVersion()
    {
        var project = System.Xml.Linq.XDocument.Load(Repository.PathTo("csharp", "src", "Slackblocks", "Slackblocks.csproj"));
        Assert.Equal(SlackblocksInfo.Version, project.Descendants("Version").Single().Value);
    }

    [Fact]
    public void StringsBecomeMrkdwnForTextFields()
    {
        var block = new SectionBlock(text: "*Deploy* ready", fields: ["*Env*", "prod"]);

        Assert.Equal(
            """{"type":"section","text":{"type":"mrkdwn","text":"*Deploy* ready"},"fields":[{"type":"mrkdwn","text":"*Env*"},{"type":"mrkdwn","text":"prod"}]}""",
            block.ToJson());
        Assert.IsType<MarkdownText>(block.Text);
        Assert.All(block.Fields, field => Assert.IsType<MarkdownText>(field));
    }

    [Fact]
    public void StringsBecomePlainTextWhereSlackRequiresIt()
    {
        var option = new Option("Production", "prod", description: "Live traffic");
        var button = new ButtonElement("Deploy", "deploy");

        Assert.Equal("plain_text", option.ToJsonNode()["text"]!["type"]!.GetValue<string>());
        Assert.Equal("plain_text", option.ToJsonNode()["description"]!["type"]!.GetValue<string>());
        Assert.IsType<PlainText>(option.Text);
        Assert.Equal("Deploy", button.Text.Text);
    }

    [Fact]
    public void ExplicitTextObjectsAreKept()
    {
        var block = new SectionBlock(text: new PlainText("Plain", emoji: true));

        Assert.Equal("""{"type":"plain_text","text":"Plain","emoji":true}""", block.ToJsonNode()["text"]!.ToJsonString());
    }

    [Fact]
    public void PropertiesExposeTheConstructorArguments()
    {
        var button = new ButtonElement("Approve", "approve", style: ButtonStyle.Primary, value: "42");
        var actions = new ActionsBlock([button], blockId: "approval");

        Assert.Equal("approve", button.ActionId);
        Assert.Equal(ButtonStyle.Primary, button.Style);
        Assert.Null(button.Url);
        Assert.Same(button, Assert.Single(actions.Elements));
        Assert.Equal("approval", actions.BlockId);
        Assert.Equal("primary", ButtonStyle.Primary.ToWireValue());
    }

    [Fact]
    public void ValuesAreSnapshotsOfTheirInputs()
    {
        var fields = new List<Text> { "one" };
        var block = new SectionBlock(fields: fields);
        fields.Add("two");

        Assert.Single(block.Fields);
        Assert.Single(block.ToJsonNode()["fields"]!.AsArray());
    }

    [Fact]
    public void JsonNodesAreIndependentCopies()
    {
        var block = new HeaderBlock("Status");
        var node = block.ToJsonNode();
        node["type"] = "changed";

        Assert.Equal("header", block.ToJsonNode()["type"]!.GetValue<string>());
    }

    [Fact]
    public void ValuesCompareByTheirSlackJson()
    {
        var first = new SectionBlock(text: "Hi", blockId: "a");
        var same = new SectionBlock(blockId: "a", text: "Hi");
        var different = new SectionBlock(text: "Hi", blockId: "b");

        Assert.Equal(first, same);
        Assert.Equal(first.GetHashCode(), same.GetHashCode());
        Assert.NotEqual(first, different);
        Assert.Equal(first.ToJson(), first.ToString());
    }

    [Fact]
    public void JsonSerializerWritesSlackJsonForConcreteAndRoleTypes()
    {
        IReadOnlyList<IBlock> blocks = [new DividerBlock(), new HeaderBlock("Title")];
        var payload = new { channel = "C123", blocks };

        Assert.Equal(
            """{"channel":"C123","blocks":[{"type":"divider"},{"type":"header","text":{"type":"plain_text","text":"Title"}}]}""",
            JsonSerializer.Serialize(payload));
        Assert.Throws<NotSupportedException>(() => JsonSerializer.Deserialize<DividerBlock>("""{"type":"divider"}"""));
    }

    [Fact]
    public void ModelDefaultsAreApplied()
    {
        var message = new MessagePayload("C123", blocks: [new DividerBlock()]);

        Assert.Equal("""{"channel":"C123","blocks":[{"type":"divider"}],"text":"","mrkdwn":true}""", message.ToJson());
        Assert.Equal(string.Empty, message.Text);
        Assert.True(message.Mrkdwn);
    }

    [Fact]
    public void ValidationFailuresCarryCategoryAndPath()
    {
        var error = Assert.Throws<ValidationException>(() => new SectionBlock(fields: [new string('x', 2001)]));

        Assert.Equal(ErrorCategory.LengthExceeded, error.Category);
        Assert.Equal("SectionBlock.fields[0].text", error.Path);
        Assert.StartsWith("SectionBlock.fields[0].text: ", error.Message, StringComparison.Ordinal);
        Assert.IsAssignableFrom<ArgumentException>(error);
    }

    [Fact]
    public void MissingRequiredArgumentsAreValidationFailures()
    {
        var error = Assert.Throws<ValidationException>(() => new HeaderBlock(null!));

        Assert.Equal(ErrorCategory.MissingRequired, error.Category);
    }

    [Fact]
    public void LimitsCountUnicodeCodePoints()
    {
        _ = new HeaderBlock(string.Concat(Enumerable.Repeat("🙂", 150)));

        Assert.Throws<ValidationException>(() => new HeaderBlock(string.Concat(Enumerable.Repeat("🙂", 151))));
    }

    [Fact]
    public void NonFiniteNumbersAreRejected()
    {
        var error = Assert.Throws<ValidationException>(() => new RawNumber(double.NaN, "NaN"));

        Assert.Equal(ErrorCategory.TypeMismatch, error.Category);
        Assert.Equal("RawNumber.value", error.Path);
    }

    [Fact]
    public void WholeNumbersAreWrittenWithoutADecimalPoint()
    {
        Assert.Equal("""{"type":"raw_number","value":42,"text":"42"}""", new RawNumber(42, "42").ToJson());
        Assert.Equal("""{"type":"raw_number","value":42.5,"text":"42.5"}""", new RawNumber(42.5, "42.5").ToJson());
    }

    [Fact]
    public void AdditionalFieldsAreSentAndValidated()
    {
        var block = new DividerBlock(additionalFields: new Dictionary<string, object?>
        {
            ["future_flag"] = true,
            ["nested"] = new Dictionary<string, object?> { ["count"] = 3, ["items"] = new[] { "a", "b" } },
        });

        Assert.Equal("""{"type":"divider","future_flag":true,"nested":{"count":3,"items":["a","b"]}}""", block.ToJson());
        Assert.Throws<ValidationException>(() => new DividerBlock(additionalFields: new Dictionary<string, object?>
        {
            ["block_id"] = new string('x', 256),
        }));
    }

    [Fact]
    public void RawDispatchActionConfigRequiresTriggers()
    {
        var error = Assert.Throws<ValidationException>(() => new PlainTextInputElement("a", additionalFields: new Dictionary<string, object?>
        {
            ["dispatch_action_config"] = new Dictionary<string, object?>(),
        }));

        Assert.Equal(ErrorCategory.MissingRequired, error.Category);
        Assert.Equal("PlainTextInputElement.dispatch_action_config", error.Path);
    }

    [Fact]
    public void AdditionalFieldsCannotRepeatANamedParameter()
    {
        var error = Assert.Throws<ArgumentException>(() => new DividerBlock(
            blockId: "a",
            additionalFields: new Dictionary<string, object?> { ["block_id"] = "b" }));

        Assert.Contains("DividerBlock.block_id", error.Message, StringComparison.Ordinal);
    }

    [Fact]
    public void AdditionalFieldsMustBeJsonCompatible()
    {
        Assert.Throws<ArgumentException>(() => new DividerBlock(additionalFields: new Dictionary<string, object?>
        {
            ["when"] = DateTime.UnixEpoch,
        }));
        Assert.Throws<ValidationException>(() => new DividerBlock(additionalFields: new Dictionary<string, object?>
        {
            ["missing"] = null,
        }));
    }

    [Fact]
    public void NullCollectionItemsAreRejected()
    {
        Assert.Throws<ArgumentNullException>(() => new ActionsBlock([null!]));
    }

    [Fact]
    public void PlanTasksAreSentWithoutTheirTypeDiscriminator()
    {
        var plan = new PlanBlock("Release", tasks: [new TaskCardBlock("t1", "Build", status: TaskCardStatus.InProgress)]);

        var task = plan.ToJsonNode()["tasks"]![0]!.AsObject();
        Assert.False(task.ContainsKey("type"));
        Assert.Equal("in_progress", task["status"]!.GetValue<string>());
        Assert.Equal("task_card", Assert.Single(plan.Tasks).ToJsonNode()["type"]!.GetValue<string>());
    }

    [Fact]
    public void AttachmentColorsAcceptHexWithoutTheHash()
    {
        Assert.Equal("#36a64f", new Attachment(color: "36a64f").ToJsonNode()["color"]!.GetValue<string>());
        Assert.Equal("danger", new Attachment(color: "danger").ToJsonNode()["color"]!.GetValue<string>());
        Assert.Throws<ValidationException>(() => new Attachment(color: "blue"));
    }

    [Fact]
    public void RichTextStylesSendOnlyTheFlagsSet()
    {
        var text = new RichTextText("Hello", style: new RichTextStyle(bold: true, code: false));

        Assert.Equal("""{"type":"text","text":"Hello","style":{"bold":true,"code":false}}""", text.ToJson());
        Assert.Equal("{}", new RichTextStyle().ToJson());
    }

    [Fact]
    public void ImplicitTextOnItsOwnIsMarkdown()
    {
        Text text = "*bold*";
        Text? missing = (string?)null;

        Assert.Equal("""{"type":"mrkdwn","text":"*bold*"}""", text.ToJson());
        Assert.Null(missing);
    }

    [Fact]
    public void EnumWireValuesMatchTheSharedModel()
    {
        var model = JsonNode.Parse(System.IO.File.ReadAllText(Repository.PathTo("spec", "model.json")))!;
        var renames = new Dictionary<string, string> { ["TaskStatus"] = "TaskCardStatus" };
        foreach (var spec in model["enums"]!.AsArray())
        {
            var name = spec!["name"]!.GetValue<string>();
            var type = typeof(SlackObject).Assembly.GetExportedTypes().Single(candidate => candidate.Name == renames.GetValueOrDefault(name, name));
            var toWire = type.Assembly.GetType(type.FullName + "Extensions")!.GetMethod("ToWireValue")!;
            var actual = Enum.GetValues(type).Cast<object>().Select(value => (string)toWire.Invoke(null, [value])!);
            var expected = spec["constants"]!.AsArray().Select(constant => constant!["wire"]!.GetValue<string>());
            Assert.Equal(expected, actual);
        }
    }
}
