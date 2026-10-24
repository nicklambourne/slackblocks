using System.Collections.Generic;
using Slackblocks.Elements;

namespace Slackblocks.Tests;

public sealed class ButtonPromptTests
{
    [Fact]
    public void NativeFieldsAndLegacyPositionalExtensionsArePreserved()
    {
        var oldSignature = typeof(ButtonElement).GetConstructor([
            typeof(Slackblocks.Objects.PlainText), typeof(string), typeof(string), typeof(string),
            typeof(ButtonStyle?), typeof(Slackblocks.Objects.Confirmation), typeof(string),
            typeof(IReadOnlyDictionary<string, object?>)
        ]);
        Assert.NotNull(oldSignature);
        var oldCall = Assert.IsType<ButtonElement>(oldSignature.Invoke([new Slackblocks.Objects.PlainText("Ask"), "ask", null, null, null, null, null, null]));
        Assert.Equal("ask", oldCall.ActionId);
        var legacy = new ButtonElement("Ask", "ask", null, null, null, null, null, new Dictionary<string, object?> { ["custom"] = "kept" });
        Assert.Equal("kept", legacy.ToJsonNode()["custom"]!.GetValue<string>());
        var button = new ButtonElement("Ask", agentPrompt: "Ask Slackbot", agentPromptDisplay: new string('x', 80));
        Assert.Equal("Ask Slackbot", button.AgentPrompt);
        Assert.Equal(new string('x', 80), button.AgentPromptDisplay);
        var wire = button.ToJsonNode();
        Assert.Equal("Ask Slackbot", wire["agent_prompt"]!.GetValue<string>());
        wire["agent_prompt"] = "Changed";
        Assert.Equal("Ask Slackbot", button.ToJsonNode()["agent_prompt"]!.GetValue<string>());
        Assert.False(new ButtonElement("Ask", agentPrompt: "Ask Slackbot").ToJsonNode().ContainsKey("agent_prompt_display"));
    }
}
