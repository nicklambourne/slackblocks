using System.Diagnostics.CodeAnalysis;

namespace Slackblocks.Objects;

/// <summary>
/// A Slack text composition object: either <see cref="PlainText"/> or <see cref="MarkdownText"/>.
/// </summary>
/// <remarks>
/// A string converts to <see cref="Text"/> implicitly, so a field typed <see cref="Text"/> accepts
/// <c>"Hello"</c> directly. The field decides the representation: most such fields send
/// <c>mrkdwn</c>, and the few that Slack requires as plain text, such as an option's label, send
/// <c>plain_text</c>. On its own, a converted string is <see cref="MarkdownText"/>.
/// </remarks>
public abstract class Text : SlackObject
{
    private protected Text()
    {
    }

    /// <summary>Converts a string to text whose representation the receiving field decides.</summary>
    /// <param name="text">The text content.</param>
    /// <returns>A text object, or <see langword="null"/> when <paramref name="text"/> is null.</returns>
    [return: NotNullIfNotNull(nameof(text))]
    public static implicit operator Text?(string? text) => text is null ? null : MarkdownText.FromString(text);
}

/// <content>Conversions for <see cref="PlainText"/>.</content>
public sealed partial class PlainText
{
    /// <summary>Converts a string to a <c>plain_text</c> object.</summary>
    /// <param name="text">The text content.</param>
    /// <returns>Plain text, or <see langword="null"/> when <paramref name="text"/> is null.</returns>
    /// <exception cref="ValidationException">The text breaks a Slack limit.</exception>
    [return: NotNullIfNotNull(nameof(text))]
    public static implicit operator PlainText?(string? text) => text is null ? null : new PlainText(text);
}

/// <content>Conversions for <see cref="MarkdownText"/>.</content>
public sealed partial class MarkdownText
{
    /// <summary>Gets a value indicating whether this text came from an implicit string conversion.</summary>
    /// <remarks>Fields that require <c>plain_text</c> convert such text rather than rejecting it.</remarks>
    internal bool FromImplicitString { get; private init; }

    /// <summary>Converts a string to a <c>mrkdwn</c> object.</summary>
    /// <param name="text">The text content.</param>
    /// <returns>Markdown text, or <see langword="null"/> when <paramref name="text"/> is null.</returns>
    /// <exception cref="ValidationException">The text breaks a Slack limit.</exception>
    [return: NotNullIfNotNull(nameof(text))]
    public static implicit operator MarkdownText?(string? text) => text is null ? null : new MarkdownText(text);

    internal static MarkdownText FromString(string text) => new(text) { FromImplicitString = true };
}
