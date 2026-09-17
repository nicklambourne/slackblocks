using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Globalization;
using System.Text.Json.Nodes;
using Slackblocks.Objects;

namespace Slackblocks.Internal;

/// <summary>
/// Collects the named arguments of one generated constructor into Slack JSON, then validates it.
/// Each method returns the value the constructor should expose through its typed property.
/// </summary>
internal sealed class WireBuilder
{
    private readonly string name;
    private readonly JsonObject values = new();
    private readonly HashSet<string> named = new(StringComparer.Ordinal);

    public WireBuilder(string name, string wireType)
    {
        this.name = name;
        if (wireType.Length > 0)
        {
            values["type"] = wireType;
        }
    }

    public string? String(string field, string? value)
    {
        if (value is not null)
        {
            Set(field, JsonValue.Create(value));
        }

        return value;
    }

    public bool? Boolean(string field, bool? value)
    {
        if (value is bool set)
        {
            Set(field, JsonValue.Create(set));
        }

        return value;
    }

    public int? Int(string field, int? value)
    {
        if (value is int set)
        {
            Set(field, JsonValue.Create(set));
        }

        return value;
    }

    public long? Long(string field, long? value)
    {
        if (value is long set)
        {
            Set(field, JsonValue.Create(set));
        }

        return value;
    }

    public double? Double(string field, double? value)
    {
        if (value is double set)
        {
            if (!double.IsFinite(set))
            {
                throw new ValidationException(ErrorCategory.TypeMismatch, Path(field), "expected a finite number");
            }

            Set(field, JsonValue.Create(set));
        }

        return value;
    }

    public double Number(string field, double value) => Double(field, value)!.Value;

    public TEnum? Enum<TEnum>(string field, TEnum? value, Func<TEnum, string> toWireValue)
        where TEnum : struct
    {
        if (value is TEnum set)
        {
            Set(field, JsonValue.Create(toWireValue(set)));
        }

        return value;
    }

    public TEnum Enum<TEnum>(string field, TEnum value, Func<TEnum, string> toWireValue)
        where TEnum : struct
    {
        Set(field, JsonValue.Create(toWireValue(value)));
        return value;
    }

    public T? Object<T>(string field, T? value)
        where T : class, ISlackObject
    {
        if (value is not null)
        {
            Set(field, Node(value));
        }

        return value;
    }

    public IReadOnlyList<T> List<T>(string field, IEnumerable<T>? items)
        where T : class, ISlackObject
    {
        if (items is null)
        {
            return Array.Empty<T>();
        }

        var copy = new List<T>();
        var array = new JsonArray();
        foreach (var item in items)
        {
            var index = copy.Count;
            if (item is null)
            {
                throw new ArgumentNullException(field, $"{Path(field)}[{index}] is null.");
            }

            copy.Add(item);
            array.Add(Node(item));
        }

        Set(field, array);
        return new ReadOnlyCollection<T>(copy);
    }

    public IReadOnlyList<string> Strings(string field, IEnumerable<string>? items)
    {
        if (items is null)
        {
            return Array.Empty<string>();
        }

        var copy = new List<string>();
        var array = new JsonArray();
        foreach (var item in items)
        {
            if (item is null)
            {
                throw new ArgumentNullException(field, $"{Path(field)}[{copy.Count}] is null.");
            }

            copy.Add(item);
            array.Add(JsonValue.Create(item));
        }

        Set(field, array);
        return new ReadOnlyCollection<string>(copy);
    }

    public IReadOnlyList<IReadOnlyList<T>> Rows<T>(string field, IEnumerable<IEnumerable<T>>? rows)
        where T : class, ISlackObject
    {
        if (rows is null)
        {
            return Array.Empty<IReadOnlyList<T>>();
        }

        var copy = new List<IReadOnlyList<T>>();
        var array = new JsonArray();
        foreach (var row in rows)
        {
            var rowIndex = copy.Count;
            if (row is null)
            {
                throw new ArgumentNullException(field, $"{Path(field)}[{rowIndex}] is null.");
            }

            var cells = new List<T>();
            var rowArray = new JsonArray();
            foreach (var cell in row)
            {
                if (cell is null)
                {
                    throw new ArgumentNullException(field, $"{Path(field)}[{rowIndex}][{cells.Count}] is null.");
                }

                cells.Add(cell);
                rowArray.Add(Node(cell));
            }

            copy.Add(new ReadOnlyCollection<T>(cells));
            array.Add(rowArray);
        }

        Set(field, array);
        return new ReadOnlyCollection<IReadOnlyList<T>>(copy);
    }

    public TText? Text<TText>(string field, TText? value, string coerce)
        where TText : Text
    {
        if (value is null)
        {
            return null;
        }

        var resolved = (TText)Resolve(value, coerce);
        Set(field, Node(resolved));
        return resolved;
    }

    public IReadOnlyList<Text> TextList(string field, IEnumerable<Text>? items, string coerce)
    {
        if (items is null)
        {
            return Array.Empty<Text>();
        }

        var copy = new List<Text>();
        var array = new JsonArray();
        foreach (var item in items)
        {
            if (item is null)
            {
                throw new ArgumentNullException(field, $"{Path(field)}[{copy.Count}] is null.");
            }

            var resolved = Resolve(item, coerce);
            copy.Add(resolved);
            array.Add(Node(resolved));
        }

        Set(field, array);
        return new ReadOnlyCollection<Text>(copy);
    }

    public IReadOnlyDictionary<string, object?>? Map(string field, IReadOnlyDictionary<string, object?>? value)
    {
        if (value is null)
        {
            return null;
        }

        var copy = new Dictionary<string, object?>(value, StringComparer.Ordinal);
        Set(field, WireValues.ToNode(copy, Path(field))!);
        return new ReadOnlyDictionary<string, object?>(copy);
    }

    /// <summary>Adds additional wire fields, optionally transforms the result, and validates it.</summary>
    public JsonObject Build(IReadOnlyDictionary<string, object?>? additionalFields, Action<JsonObject>? transform = null)
    {
        if (additionalFields is not null)
        {
            foreach (var (field, value) in additionalFields)
            {
                if (string.IsNullOrEmpty(field))
                {
                    throw new ArgumentException("Additional field names must not be empty.", nameof(additionalFields));
                }

                if (named.Contains(field))
                {
                    throw new ArgumentException(
                        $"{Path(field)} is already set by a named parameter; set it in one place.",
                        nameof(additionalFields));
                }

                values[field] = WireValues.ToNode(value, Path(field));
            }
        }

        transform?.Invoke(values);
        Validator.Validate(name, values);
        return values;
    }

    private static Text Resolve(Text value, string coerce) =>
        coerce == "plain_text" && value is MarkdownText { FromImplicitString: true } implicitText
            ? new PlainText(implicitText.Text)
            : value;

    private static JsonNode Node(ISlackObject value) =>
        value is SlackObject slackObject ? slackObject.Wire.DeepClone() : value.ToJsonNode();

    private void Set(string field, JsonNode node)
    {
        values[field] = node;
        named.Add(field);
    }

    private string Path(string field) => name + "." + field;
}

/// <summary>Converts .NET values supplied as additional fields into Slack JSON.</summary>
internal static class WireValues
{
    public static JsonNode? ToNode(object? value, string path)
    {
        switch (value)
        {
            case null:
                return null;
            case JsonNode node:
                return node.DeepClone();
            case string text:
                return JsonValue.Create(text);
            case bool flag:
                return JsonValue.Create(flag);
            case byte or sbyte or short or ushort or int or uint or long:
                return JsonValue.Create(Convert.ToInt64(value, CultureInfo.InvariantCulture));
            case ulong unsigned:
                return JsonValue.Create(unsigned);
            case decimal number:
                return JsonValue.Create(number);
            case float or double:
                var real = Convert.ToDouble(value, CultureInfo.InvariantCulture);
                if (!double.IsFinite(real))
                {
                    throw new ValidationException(ErrorCategory.TypeMismatch, path, "expected a finite number");
                }

                return JsonValue.Create(real);
            case SlackObject slackObject:
                return slackObject.Wire.DeepClone();
            case ISlackObject other:
                return other.ToJsonNode();
            case IReadOnlyDictionary<string, object?> map:
                var fromReadOnly = new JsonObject();
                foreach (var (key, nested) in map)
                {
                    fromReadOnly[key] = ToNode(nested, path + "." + key);
                }

                return fromReadOnly;
            case IDictionary<string, object?> map:
                var fromDictionary = new JsonObject();
                foreach (var (key, nested) in map)
                {
                    fromDictionary[key] = ToNode(nested, path + "." + key);
                }

                return fromDictionary;
            case IEnumerable items:
                var array = new JsonArray();
                var index = 0;
                foreach (var item in items)
                {
                    array.Add(ToNode(item, $"{path}[{index++}]"));
                }

                return array;
            default:
                throw new ArgumentException(
                    $"{path}: {value.GetType().FullName} cannot be written as Slack JSON; use a string, number, "
                    + "boolean, list, dictionary, JsonNode, or slackblocks value.");
        }
    }
}
