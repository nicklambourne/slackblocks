using System;
using System.Collections.Generic;
using Slackblocks.Blocks;
using Slackblocks.Objects;
namespace Slackblocks.Tests;

public sealed class SparseTableTests
{
    [Fact]
    public void GetterRetainsSkippedPositionsAndDefensiveCopies()
    {
        var right = new ColumnSettings(align: ColumnAlign.Right);
        ColumnSettings?[] settings = [null, right, null];
        var table = new TableBlock(rows: [[new RawText("A")]], columnSettings: settings);
        settings[0] = right;
        Assert.Equal(new ColumnSettings?[] { null, right, null }, table.ColumnSettings);
    }
    [Fact]
    public void RejectsInvalidNonNullSettings()
    {
        foreach (var invalid in new object[] { false, 1, "right", new Dictionary<string, object?> { ["align"] = "invalid" }, new Dictionary<string, object?> { ["is_wrapped"] = 1 } })
        {
            var error = Assert.Throws<ValidationException>(() => new TableBlock(rows: [[new RawText("A")]], additionalFields: new Dictionary<string, object?> { ["column_settings"] = new object[] { invalid } }));
            Assert.Equal(ErrorCategory.TypeMismatch, error.Category);
        }
    }
}
