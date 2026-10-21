using System;
using System.Collections.Generic;
using Slackblocks.Elements;
using Slackblocks.Objects;
namespace Slackblocks.Tests;

public sealed class SelectPlaceholderTests
{
    [Fact]
    public void AllNativeSelectorsEnforceUnicodeBoundaries()
    {
        var factories = new Dictionary<string, Func<string, object>>
        {
            ["ExternalSelectElement"] = s => new ExternalSelectElement(placeholder: s),
            ["ExternalMultiSelectElement"] = s => new ExternalMultiSelectElement(placeholder: s),
            ["UserSelectElement"] = s => new UserSelectElement(placeholder: s),
            ["UserMultiSelectElement"] = s => new UserMultiSelectElement(placeholder: s),
            ["ChannelSelectElement"] = s => new ChannelSelectElement(placeholder: s),
            ["ChannelMultiSelectElement"] = s => new ChannelMultiSelectElement(placeholder: s),
            ["ConversationSelectElement"] = s => new ConversationSelectElement(placeholder: s),
            ["ConversationMultiSelectElement"] = s => new ConversationMultiSelectElement(placeholder: s),
            ["StaticSelectElement"] = s => new StaticSelectElement(options: [new Option("A", "a")], placeholder: s),
            ["StaticMultiSelectElement"] = s => new StaticMultiSelectElement(options: [new Option("A", "a")], placeholder: s),
        };
        foreach (var item in factories)
        {
            foreach (var unit in new[] { "x", "🚀" })
            {
                foreach (var size in new[] { 149, 150, 151 })
                {
                    var text = string.Concat(System.Linq.Enumerable.Repeat(unit, size));
                    if (size == 151)
                    {
                        var error = Assert.Throws<ValidationException>(() => item.Value(text));
                        Assert.Equal(ErrorCategory.LengthExceeded, error.Category);
                        Assert.Equal(item.Key + ".placeholder.text", error.Path);
                    }
                    else
                    {
                        Assert.NotNull(item.Value(text));
                    }
                }
            }
        }
    }
}
