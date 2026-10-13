using Slackblocks.Elements;
namespace Slackblocks.Tests;

public sealed class NumberBoundsTests
{
    [Fact]
    public void BoundsAreStringsAndNumericCallsStillCompile()
    {
        var value = new NumberInputElement(isDecimalAllowed: true, minValue: -1.5, maxValue: "10.25");
        Assert.Equal("-1.5", value.MinValue); Assert.Equal("10.25", value.MaxValue);
        Assert.Contains("\"min_value\":\"-1.5\"", value.ToJson());
        Assert.Equal("0", new NumberInputElement(isDecimalAllowed: false, minValue: 0).MinValue);
        Assert.Equal("9007199254740993", new NumberInputElement(isDecimalAllowed: true, minValue: 9007199254740993L).MinValue);
        Assert.Null(new NumberInputElement(isDecimalAllowed: true).MinValue);
    }
    [Fact]
    public void RejectsNonfiniteInvalidAndInvertedBounds()
    {
        foreach (var value in new[] { "NaN", "Infinity", "invalid", "" }) Assert.Throws<ValidationException>(() => new NumberInputElement(isDecimalAllowed: true, minValue: value));
        Assert.Throws<ValidationException>(() => new NumberInputElement(isDecimalAllowed: true, minValue: double.NaN));
        Assert.Throws<ValidationException>(() => new NumberInputElement(isDecimalAllowed: true, minValue: "10", maxValue: "2"));
    }
    [Fact]
    public void DecimalStringOrderingIsExact()
    {
        foreach (var pair in new[] { new[] { "9007199254740993", "9007199254740992" }, new[] { "0.10000000000000001", "0.1" }, new[] { "-9007199254740992", "-9007199254740993" }, new[] { "-0.1", "-0.10000000000000001" }, new[] { "1e-1000", "9e-1001" } })
        {
            Assert.Throws<ValidationException>(() => new NumberInputElement(isDecimalAllowed: true, minValue: pair[0], maxValue: pair[1]));
            _ = new NumberInputElement(isDecimalAllowed: true, minValue: pair[1], maxValue: pair[0]);
        }
        _ = new NumberInputElement(isDecimalAllowed: true, minValue: "0.1000", maxValue: "1e-1");
    }
}
