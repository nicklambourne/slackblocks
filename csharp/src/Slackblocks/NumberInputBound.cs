using System.Globalization;

namespace Slackblocks;

/// <summary>A number-input bound sent as a JSON string. Numeric conversions use invariant culture.</summary>
public sealed class NumberInputBound
{
    /// <summary>Creates a bound; the containing number input validates its numeric value.</summary>
    public NumberInputBound(string value)
    {
        System.ArgumentNullException.ThrowIfNull(value);
        Value = value;
    }
    /// <summary>Gets the supplied wire string.</summary>
    public string Value { get; }
    /// <summary>Preserves a supplied decimal string.</summary>
    public static implicit operator NumberInputBound?(string? value) => value is null ? null : new(value);
    /// <summary>Converts a numeric convenience argument using invariant culture.</summary>
    public static implicit operator NumberInputBound(double value) => new(value.ToString("R", CultureInfo.InvariantCulture));
    /// <summary>Preserves nullable numeric arguments and omission.</summary>
    public static implicit operator NumberInputBound?(double? value) => value is null ? null : new(value.Value.ToString("R", CultureInfo.InvariantCulture));
    /// <summary>Preserves exact decimal arithmetic.</summary>
    public static implicit operator NumberInputBound(decimal value) => new(value.ToString(CultureInfo.InvariantCulture));
    /// <summary>Preserves an exact integer bound.</summary>
    public static implicit operator NumberInputBound(long value) => new(value.ToString(CultureInfo.InvariantCulture));
    /// <summary>Preserves an exact integer bound.</summary>
    public static implicit operator NumberInputBound(int value) => new(value.ToString(CultureInfo.InvariantCulture));
}
