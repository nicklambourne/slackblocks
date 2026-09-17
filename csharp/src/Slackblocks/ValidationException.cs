using System;

namespace Slackblocks;

/// <summary>Stable validation categories shared by every slackblocks implementation.</summary>
public enum ErrorCategory
{
    /// <summary>A string or collection falls outside Slack's documented length limits.</summary>
    LengthExceeded,

    /// <summary>A numeric value falls outside Slack's documented range.</summary>
    OutOfRange,

    /// <summary>Two fields that cannot coexist were supplied together.</summary>
    MutuallyExclusive,

    /// <summary>A value is not an accepted Slack object or scalar type.</summary>
    TypeMismatch,

    /// <summary>A required field or collection is absent.</summary>
    MissingRequired,

    /// <summary>A combination of otherwise valid values cannot be used together.</summary>
    InvalidUsage,
}

/// <summary>Conversions between <see cref="ErrorCategory"/> and the shared specification.</summary>
public static class ErrorCategoryExtensions
{
    /// <summary>Returns the kebab-case category name used by the shared conformance corpus.</summary>
    /// <param name="category">The category to convert.</param>
    /// <returns>A value such as <c>length-exceeded</c>.</returns>
    /// <exception cref="ArgumentOutOfRangeException">The value is not a defined category.</exception>
    public static string ToWireValue(this ErrorCategory category) => category switch
    {
        ErrorCategory.LengthExceeded => "length-exceeded",
        ErrorCategory.OutOfRange => "out-of-range",
        ErrorCategory.MutuallyExclusive => "mutually-exclusive",
        ErrorCategory.TypeMismatch => "type-mismatch",
        ErrorCategory.MissingRequired => "missing-required",
        ErrorCategory.InvalidUsage => "invalid-usage",
        _ => throw new ArgumentOutOfRangeException(nameof(category), category, "Unknown error category."),
    };
}

/// <summary>Thrown when a slackblocks value would break a Slack Block Kit rule.</summary>
/// <remarks>
/// This derives from <see cref="ArgumentException"/>, so code that already handles invalid
/// arguments continues to work. Catch <see cref="ValidationException"/> when you need the stable
/// <see cref="Category"/> or the <see cref="Path"/> of the invalid field. Paths start with the type
/// being constructed, for example <c>SectionBlock.fields[2].text</c>.
/// </remarks>
public sealed class ValidationException : ArgumentException
{
    /// <summary>Creates a structured validation failure.</summary>
    /// <param name="category">Stable cross-language failure category.</param>
    /// <param name="path">Dotted path of the invalid value.</param>
    /// <param name="reason">Human-readable explanation.</param>
    public ValidationException(ErrorCategory category, string path, string reason)
        : base(path + ": " + reason)
    {
        Category = category;
        Path = path;
    }

    /// <summary>Gets the stable cross-language error category.</summary>
    public ErrorCategory Category { get; }

    /// <summary>Gets the dotted path of the invalid field, starting with the type being constructed.</summary>
    public string Path { get; }
}
