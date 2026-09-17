using System.Text.Json.Nodes;
using System.Text.Json.Serialization;
using Slackblocks.Internal;

namespace Slackblocks.Objects;

/// <summary>Formatting flags for inline rich text such as text, links, and mentions.</summary>
/// <remarks>
/// Only the flags you set are sent. Text and links support <c>bold</c>, <c>italic</c>,
/// <c>strike</c>, and <c>code</c>; channel, user, and user group mentions support <c>bold</c>,
/// <c>italic</c>, <c>strike</c>, <c>highlight</c>, <c>client_highlight</c>, and <c>unlink</c>.
/// </remarks>
[JsonConverter(typeof(SlackObjectJsonConverterFactory))]
public sealed class RichTextStyle : SlackObject
{
    /// <summary>Creates a style with the given flags.</summary>
    /// <param name="bold">Whether the text is bold.</param>
    /// <param name="italic">Whether the text is italic.</param>
    /// <param name="strike">Whether the text is struck through.</param>
    /// <param name="code">Whether the text is shown as inline code.</param>
    /// <param name="highlight">Whether a mention is highlighted.</param>
    /// <param name="clientHighlight">Whether a mention is highlighted for the viewing user.</param>
    /// <param name="unlink">Whether a mention is shown without a link.</param>
    public RichTextStyle(
        bool? bold = null,
        bool? italic = null,
        bool? strike = null,
        bool? code = null,
        bool? highlight = null,
        bool? clientHighlight = null,
        bool? unlink = null)
    {
        var wire = new JsonObject();
        Add(wire, "bold", Bold = bold);
        Add(wire, "italic", Italic = italic);
        Add(wire, "strike", Strike = strike);
        Add(wire, "code", Code = code);
        Add(wire, "highlight", Highlight = highlight);
        Add(wire, "client_highlight", ClientHighlight = clientHighlight);
        Add(wire, "unlink", Unlink = unlink);
        Initialize(wire);
    }

    /// <summary>Gets whether the text is bold, or <see langword="null"/> when not set.</summary>
    public bool? Bold { get; }

    /// <summary>Gets whether the text is italic, or <see langword="null"/> when not set.</summary>
    public bool? Italic { get; }

    /// <summary>Gets whether the text is struck through, or <see langword="null"/> when not set.</summary>
    public bool? Strike { get; }

    /// <summary>Gets whether the text is inline code, or <see langword="null"/> when not set.</summary>
    public bool? Code { get; }

    /// <summary>Gets whether a mention is highlighted, or <see langword="null"/> when not set.</summary>
    public bool? Highlight { get; }

    /// <summary>Gets whether a mention is highlighted for the viewer, or <see langword="null"/> when not set.</summary>
    public bool? ClientHighlight { get; }

    /// <summary>Gets whether a mention is shown without a link, or <see langword="null"/> when not set.</summary>
    public bool? Unlink { get; }

    private static void Add(JsonObject wire, string flag, bool? value)
    {
        if (value is bool set)
        {
            wire[flag] = set;
        }
    }
}
