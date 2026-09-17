using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json.Nodes;

namespace Slackblocks.Tests;

public sealed class ConformanceTests
{
    private static readonly (string Prefix, string Type)[] UntypedRoots =
    [
        ("attachments/", "Attachment"),
        ("messages/message_response", "MessageResponse"),
        ("messages/webhook_message", "WebhookMessage"),
        ("messages/message_", "MessagePayload"),
        ("objects/confirmation", "Confirmation"),
        ("objects/conversation_filter", "ConversationFilter"),
        ("objects/dispatch_action_configuration", "DispatchActionConfiguration"),
        ("objects/input_parameter", "InputParameter"),
        ("objects/option_group", "OptionGroup"),
        ("objects/option_", "Option"),
        ("objects/slack_file", "SlackFile"),
        ("objects/trigger", "Trigger"),
        ("objects/workflow", "Workflow"),
    ];

    private static readonly FixtureDriver Driver = new();

    public static TheoryData<string> ValidFixtureIds()
    {
        var data = new TheoryData<string>();
        foreach (var fixture in Manifest()["fixtures"]!.AsArray())
        {
            data.Add(fixture!["id"]!.GetValue<string>());
        }

        return data;
    }

    [Fact]
    public void ImplementsTheSharedSpecificationVersion() =>
        Assert.Equal(SlackblocksInfo.SpecVersion, Manifest()["spec_version"]!.GetValue<string>());

    [Theory]
    [MemberData(nameof(ValidFixtureIds))]
    public void EverySharedValidFixtureBuildsThroughNamedParameters(string id)
    {
        var expected = JsonNode.Parse(File.ReadAllText(Repository.PathTo("spec", "fixtures", "valid", id + ".json")));
        var driver = new FixtureDriver();
        var built = driver.Build(expected, RootType(driver, id), id);

        Assert.Empty(driver.Fallbacks);
        var value = Assert.IsAssignableFrom<SlackObject>(built);
        Assert.Equal(FixtureDriver.Canonical(expected), FixtureDriver.Canonical(JsonNode.Parse(value.ToJson())));
    }

    [Fact]
    public void EveryValidFixtureFileIsListedInTheManifest()
    {
        var root = Repository.PathTo("spec", "fixtures", "valid");
        var onDisk = Directory.EnumerateFiles(root, "*.json", SearchOption.AllDirectories)
            .Select(file => Path.GetRelativePath(root, file).Replace('\\', '/')[..^".json".Length])
            .Order(StringComparer.Ordinal)
            .ToList();
        var listed = Manifest()["fixtures"]!.AsArray()
            .Select(fixture => fixture!["id"]!.GetValue<string>())
            .Order(StringComparer.Ordinal)
            .ToList();

        Assert.Equal(onDisk, listed);
    }

    [Fact]
    public void ReleasedConformanceSkipListIsEmpty()
    {
        var entries = File.ReadAllLines(Repository.PathTo("csharp", "conformance", "skiplist.txt"))
            .Where(line => !string.IsNullOrWhiteSpace(line) && !line.TrimStart().StartsWith('#'));
        Assert.Empty(entries);
    }

    internal static Type RootType(FixtureDriver driver, string id)
    {
        foreach (var (prefix, type) in UntypedRoots)
        {
            if (id.StartsWith(prefix, StringComparison.Ordinal))
            {
                return driver.Type(type);
            }
        }

        return typeof(object);
    }

    private static JsonObject Manifest() =>
        JsonNode.Parse(File.ReadAllText(Repository.PathTo("spec", "manifest.json")))!.AsObject();
}
