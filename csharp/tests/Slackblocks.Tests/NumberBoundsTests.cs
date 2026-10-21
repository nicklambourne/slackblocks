using Slackblocks.Elements;
namespace Slackblocks.Tests;

public sealed class NumberBoundsTests
{
    [Fact]
    public void LegacyNumericArgumentTypesCompileAndRetainValues()
    {
        // The previous double? parameters accepted every primitive numeric type except decimal.
        var values = new[]
        {
            new NumberInputElement(isDecimalAllowed: true, minValue: (byte)1, maxValue: (byte?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (sbyte)1, maxValue: (sbyte?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (short)1, maxValue: (short?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (ushort)1, maxValue: (ushort?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (int)1, maxValue: (int?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (uint)1, maxValue: (uint?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (long)1, maxValue: (long?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (ulong)1, maxValue: (ulong?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (nint)1, maxValue: (nint?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (nuint)1, maxValue: (nuint?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (char)1, maxValue: (char?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (float)1, maxValue: (float?)2),
            new NumberInputElement(isDecimalAllowed: true, minValue: (double)1, maxValue: (double?)2),
        };
        foreach (var value in values)
        {
            Assert.Equal("1", value.MinValue);
            Assert.Equal("2", value.MaxValue);
        }
    }

    [Fact]
    public void UnsignedBoundsPreserveExactValuesAndNullableOmission()
    {
        ulong? unsigned = ulong.MaxValue;
        nuint? nativeUnsigned = nuint.MaxValue;
        Assert.Equal("18446744073709551615", new NumberInputElement(isDecimalAllowed: false, minValue: ulong.MaxValue, maxValue: unsigned).MinValue);
        Assert.Equal("18446744073709551615", new NumberInputElement(isDecimalAllowed: false, minValue: unsigned).MinValue);
        Assert.Equal(nuint.MaxValue.ToString(System.Globalization.CultureInfo.InvariantCulture), new NumberInputElement(isDecimalAllowed: false, minValue: nuint.MaxValue, maxValue: nativeUnsigned).MaxValue);
        unsigned = null;
        nativeUnsigned = null;
        var omitted = new NumberInputElement(isDecimalAllowed: false, minValue: unsigned, maxValue: nativeUnsigned);
        Assert.Null(omitted.MinValue);
        Assert.Null(omitted.MaxValue);
        Assert.DoesNotContain("min_value", omitted.ToJson());
        Assert.DoesNotContain("max_value", omitted.ToJson());
    }

    [Fact]
    public void NullableNumericArgumentsOmitNullBounds()
    {
        // Nullable-to-reference lifting preserves omission for the integral conversion operators.
        NumberInputBound?[] values =
        [
            (byte?)null, (sbyte?)null, (short?)null, (ushort?)null, (int?)null, (uint?)null,
            (long?)null, (ulong?)null, (nint?)null, (nuint?)null, (char?)null, (float?)null, (double?)null,
        ];
        foreach (var value in values)
        {
            var omitted = new NumberInputElement(isDecimalAllowed: true, minValue: value, maxValue: value);
            Assert.Null(omitted.MinValue);
            Assert.Null(omitted.MaxValue);
            Assert.DoesNotContain("min_value", omitted.ToJson());
            Assert.DoesNotContain("max_value", omitted.ToJson());
        }
    }

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
