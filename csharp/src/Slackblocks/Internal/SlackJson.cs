using System;
using System.Linq;
using System.Text;
using System.Text.Encodings.Web;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;

namespace Slackblocks.Internal;

/// <summary>JSON writing shared by every value.</summary>
internal static class SlackJson
{
    // Slack payloads are not embedded in HTML, so characters such as < and & need no escaping.
    private static readonly JsonSerializerOptions Options = new()
    {
        Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
    };

    public static string Write(JsonNode node) => node.ToJsonString(Options);

    /// <summary>Returns JSON with object keys sorted, so equal values compare equal as text.</summary>
    public static string Canonical(JsonNode? node)
    {
        var builder = new StringBuilder();
        AppendCanonical(builder, node);
        return builder.ToString();
    }

    private static void AppendCanonical(StringBuilder builder, JsonNode? node)
    {
        switch (node)
        {
            case JsonObject obj:
                builder.Append('{');
                var first = true;
                foreach (var entry in obj.OrderBy(entry => entry.Key, StringComparer.Ordinal))
                {
                    if (!first)
                    {
                        builder.Append(',');
                    }

                    first = false;
                    builder.Append(JsonSerializer.Serialize(entry.Key, Options)).Append(':');
                    AppendCanonical(builder, entry.Value);
                }

                builder.Append('}');
                break;
            case JsonArray array:
                builder.Append('[');
                for (var index = 0; index < array.Count; index++)
                {
                    if (index > 0)
                    {
                        builder.Append(',');
                    }

                    AppendCanonical(builder, array[index]);
                }

                builder.Append(']');
                break;
            case null:
                builder.Append("null");
                break;
            default:
                builder.Append(node.ToJsonString(Options));
                break;
        }
    }
}

/// <summary>Writes slackblocks values with <see cref="JsonSerializer"/>.</summary>
/// <remarks>Reading slackblocks values from JSON is not supported.</remarks>
internal sealed class SlackObjectJsonConverterFactory : JsonConverterFactory
{
    public override bool CanConvert(Type typeToConvert) => typeof(ISlackObject).IsAssignableFrom(typeToConvert);

    public override JsonConverter CreateConverter(Type typeToConvert, JsonSerializerOptions options) =>
        (JsonConverter)Activator.CreateInstance(typeof(SlackObjectJsonConverter<>).MakeGenericType(typeToConvert))!;
}

/// <summary>Writes one slackblocks value type as its Slack JSON.</summary>
internal sealed class SlackObjectJsonConverter<T> : JsonConverter<T>
    where T : ISlackObject
{
    public override T Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) =>
        throw new NotSupportedException(
            "slackblocks values are constructed through their constructors; reading them from JSON is not supported.");

    public override void Write(Utf8JsonWriter writer, T value, JsonSerializerOptions options)
    {
        if (value is SlackObject slackObject)
        {
            slackObject.Wire.WriteTo(writer, options);
        }
        else
        {
            value.ToJsonNode().WriteTo(writer, options);
        }
    }
}

/// <summary>Helpers for reading JSON scalar values regardless of how they were created.</summary>
internal static class JsonValues
{
    public static bool IsString(JsonNode? node, out string value)
    {
        if (node is JsonValue scalar && scalar.GetValueKind() == JsonValueKind.String)
        {
            value = scalar.GetValue<string>();
            return true;
        }

        value = string.Empty;
        return false;
    }

    public static bool IsTrue(JsonNode? node) =>
        node is JsonValue scalar && scalar.GetValueKind() == JsonValueKind.True;

    public static double? Number(JsonNode? node)
    {
        if (node is not JsonValue scalar || scalar.GetValueKind() != JsonValueKind.Number)
        {
            return null;
        }

        if (scalar.TryGetValue<double>(out var asDouble))
        {
            return asDouble;
        }

        if (scalar.TryGetValue<long>(out var asLong))
        {
            return asLong;
        }

        if (scalar.TryGetValue<int>(out var asInt))
        {
            return asInt;
        }

        if (scalar.TryGetValue<decimal>(out var asDecimal))
        {
            return (double)asDecimal;
        }

        return scalar.TryGetValue<float>(out var asFloat) ? asFloat : null;
    }

    public static int CodePoints(string value) => value.EnumerateRunes().Count();
}
