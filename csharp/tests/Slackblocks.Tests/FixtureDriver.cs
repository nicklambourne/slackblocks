using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using System.Text.Json.Nodes;
using Slackblocks.Objects;

namespace Slackblocks.Tests;

/// <summary>
/// Builds Slack JSON through the public constructors, using the shared model to map each wire
/// field to its named parameter. Any field that cannot be passed through a typed parameter falls
/// back to <c>additionalFields</c> and is recorded, so conformance tests can require zero
/// fallbacks for valid fixtures.
/// </summary>
internal sealed class FixtureDriver
{
    private static readonly Dictionary<string, string> Renames = new(StringComparer.Ordinal)
    {
        ["TaskStatus"] = "TaskCardStatus",
    };

    private readonly Dictionary<string, JsonObject> types = new(StringComparer.Ordinal);
    private readonly Dictionary<string, Type> clrTypes;

    public FixtureDriver()
    {
        var model = JsonNode.Parse(File.ReadAllText(Repository.PathTo("spec", "model.json")))!.AsObject();
        foreach (var item in model["types"]!.AsArray())
        {
            types[item!["name"]!.GetValue<string>()] = item.AsObject();
        }

        clrTypes = typeof(SlackObject).Assembly.GetExportedTypes()
            .Where(type => type.Namespace?.StartsWith("Slackblocks", StringComparison.Ordinal) == true)
            .ToDictionary(type => type.Name, StringComparer.Ordinal);
    }

    public List<string> Fallbacks { get; } = [];

    public Type Type(string modelName)
    {
        var name = Renames.GetValueOrDefault(modelName, modelName);
        if (clrTypes.TryGetValue(name, out var type) || clrTypes.TryGetValue("I" + name, out type))
        {
            return type;
        }

        throw new InvalidOperationException("Unknown slackblocks type " + modelName);
    }

    public object? Build(JsonNode? json, Type expected, string path)
    {
        switch (json)
        {
            case JsonArray array:
                return array.Select(item => Build(item, typeof(object), path + "[]")).ToList();
            case JsonObject obj:
                var typeName = Resolve(obj, expected, path);
                return typeName is null ? obj.DeepClone() : Construct(typeName, obj);
            default:
                return json?.DeepClone();
        }
    }

    public static string Canonical(JsonNode? node) => Internal.SlackJson.Canonical(node);

    private object Construct(string typeName, JsonObject obj)
    {
        var spec = types[typeName];
        var type = Type(typeName);
        var wireType = spec["wireType"]!.GetValue<string>();
        var constructor = type.GetConstructors().Single();
        var fieldsByParameter = spec["fields"]!.AsArray()
            .Select(field => field!.AsObject())
            .ToDictionary(field => field["method"]!.GetValue<string>(), StringComparer.Ordinal);
        var parameters = constructor.GetParameters();
        var arguments = new object?[parameters.Length];
        var additional = new Dictionary<string, object?>(StringComparer.Ordinal);
        var handled = new HashSet<string>(StringComparer.Ordinal);

        for (var index = 0; index < parameters.Length; index++)
        {
            var parameter = parameters[index];
            arguments[index] = parameter.HasDefaultValue ? parameter.DefaultValue : null;
            if (parameter.Name == "additionalFields" || !fieldsByParameter.TryGetValue(parameter.Name!, out var field))
            {
                continue;
            }

            var wire = field["wire"]!.GetValue<string>();
            if (!obj.TryGetPropertyValue(wire, out var value))
            {
                if (parameter.ParameterType.IsValueType && Nullable.GetUnderlyingType(parameter.ParameterType) is null)
                {
                    throw new InvalidOperationException($"{typeName}.{wire} is required but absent from the fixture.");
                }

                continue;
            }

            handled.Add(wire);
            if (TryArgument(field, parameter.ParameterType, value, $"{typeName}.{wire}", out var argument))
            {
                arguments[index] = argument;
            }
            else
            {
                Fallbacks.Add($"{typeName}.{wire}");
                additional[wire] = value?.DeepClone();
            }
        }

        foreach (var (key, value) in obj)
        {
            if (handled.Contains(key) || (key == "type" && JsonValue(value, out var discriminator) && discriminator == wireType))
            {
                continue;
            }

            Fallbacks.Add($"{typeName}.{key}");
            additional[key] = Build(value, typeof(object), $"{typeName}.{key}");
        }

        if (additional.Count > 0)
        {
            arguments[^1] = additional;
        }

        return Invoke(() => constructor.Invoke(arguments));
    }

    private static object Invoke(Func<object?> call)
    {
        try
        {
            return call()!;
        }
        catch (TargetInvocationException error) when (error.InnerException is not null)
        {
            System.Runtime.ExceptionServices.ExceptionDispatchInfo.Capture(error.InnerException).Throw();
            throw;
        }
    }

    private string? Resolve(JsonObject obj, Type expected, string path)
    {
        if (typeof(SlackObject).IsAssignableFrom(expected) && types.ContainsKey(expected.Name))
        {
            return expected.Name;
        }

        if (!JsonValue(obj["type"], out var wire))
        {
            return null;
        }

        var candidates = types.Values
            .Where(spec => spec["wireType"]!.GetValue<string>() == wire)
            .Select(spec => spec["name"]!.GetValue<string>())
            .Where(name => expected.IsAssignableFrom(Type(name)))
            .ToList();
        if (candidates.Count > 1 && candidates.Contains("ImageBlock") && candidates.Contains("ImageElement"))
        {
            candidates.Remove(path.Contains("elements", StringComparison.Ordinal) || path.StartsWith("ImageElement", StringComparison.Ordinal)
                ? "ImageBlock"
                : "ImageElement");
        }

        return candidates.Count switch
        {
            0 => null,
            1 => candidates[0],
            _ => throw new InvalidOperationException($"Ambiguous type for {wire} at {path}: {string.Join(", ", candidates)}"),
        };
    }

    private bool TryArgument(JsonObject field, Type parameterType, JsonNode? value, string path, out object? argument)
    {
        argument = null;
        var kind = field["kind"]!.GetValue<string>();
        var target = field["type"] is { } typeNode ? Type(typeNode.GetValue<string>()) : typeof(object);
        switch (kind)
        {
            case "string":
                if (JsonValue(value, out var text))
                {
                    argument = text;
                    return true;
                }

                return false;
            case "boolean":
                if (value is JsonValue flag && flag.GetValueKind() is JsonValueKind.True or JsonValueKind.False)
                {
                    argument = flag.GetValueKind() == JsonValueKind.True;
                    return true;
                }

                return false;
            case "int":
                if (Whole(value) is long whole && whole is >= int.MinValue and <= int.MaxValue)
                {
                    argument = (int)whole;
                    return true;
                }

                return false;
            case "long":
                if (Whole(value) is long longValue)
                {
                    argument = longValue;
                    return true;
                }

                return false;
            case "double":
            case "number":
                if (Internal.JsonValues.Number(value) is double number)
                {
                    argument = number;
                    return true;
                }

                return false;
            case "stringList":
                if (value is JsonArray strings && strings.All(item => JsonValue(item, out _)))
                {
                    argument = strings.Select(item => item!.GetValue<string>()).ToArray();
                    return true;
                }

                return false;
            case "enum":
                if (!JsonValue(value, out var wireValue))
                {
                    return false;
                }

                var extensions = target.Assembly.GetType(target.FullName + "Extensions")!;
                var toWire = extensions.GetMethod("ToWireValue")!;
                foreach (var member in Enum.GetValues(target))
                {
                    if ((string)toWire.Invoke(null, [member])! == wireValue)
                    {
                        argument = member;
                        return true;
                    }
                }

                return false;
            case "map":
                if (value is JsonObject map)
                {
                    argument = map.ToDictionary(entry => entry.Key, entry => (object?)entry.Value?.DeepClone(), StringComparer.Ordinal);
                    return true;
                }

                return false;
            case "style":
                return TryStyle(field, value, out argument);
            case "object":
                if (value is JsonObject)
                {
                    var built = Build(value, target, path);
                    if (target.IsInstanceOfType(built))
                    {
                        argument = built;
                        return true;
                    }
                }

                return false;
            case "list":
                return value is JsonArray items && TryTypedArray(items, target, path, out argument);
            case "rows":
                return value is JsonArray rows && TryRows(rows, target, path, out argument);
            case "text":
                if (IsCoercible(value, field["coerce"]!.GetValue<string>(), out var content))
                {
                    argument = ImplicitText(parameterType, content);
                    return true;
                }

                if (value is JsonObject)
                {
                    var built = Build(value, target, path);
                    if (target.IsInstanceOfType(built))
                    {
                        argument = built;
                        return true;
                    }
                }

                return false;
            case "textList":
                if (value is not JsonArray texts)
                {
                    return false;
                }

                var coerce = field["coerce"]!.GetValue<string>();
                if (texts.All(item => IsCoercible(item, coerce, out _)))
                {
                    argument = texts.Select(item => (Text)ImplicitText(typeof(Text), item!["text"]!.GetValue<string>())).ToArray();
                    return true;
                }

                return TryTypedArray(texts, target, path, out argument);
            default:
                throw new InvalidOperationException("Unknown field kind " + kind);
        }
    }

    private bool TryTypedArray(JsonArray items, Type target, string path, out object? argument)
    {
        var array = Array.CreateInstance(target, items.Count);
        for (var index = 0; index < items.Count; index++)
        {
            var built = Build(items[index], target, $"{path}[{index}]");
            if (!target.IsInstanceOfType(built))
            {
                argument = null;
                return false;
            }

            array.SetValue(built, index);
        }

        argument = array;
        return true;
    }

    private bool TryRows(JsonArray rows, Type target, string path, out object? argument)
    {
        var result = Array.CreateInstance(target.MakeArrayType(), rows.Count);
        for (var index = 0; index < rows.Count; index++)
        {
            if (rows[index] is not JsonArray row || !TryTypedArray(row, target, path + "[][]", out var cells))
            {
                argument = null;
                return false;
            }

            result.SetValue(cells, index);
        }

        argument = result;
        return true;
    }

    private static bool TryStyle(JsonObject field, JsonNode? value, out object? argument)
    {
        argument = null;
        if (value is not JsonObject flags)
        {
            return false;
        }

        var allowed = field["flags"]!.AsArray().Select(flag => flag!.GetValue<string>()).ToHashSet(StringComparer.Ordinal);
        var constructor = typeof(RichTextStyle).GetConstructors().Single();
        var parameters = constructor.GetParameters();
        var arguments = parameters.Select(parameter => (object?)null).ToArray();
        foreach (var (flag, setting) in flags)
        {
            if (!allowed.Contains(flag) || setting is not System.Text.Json.Nodes.JsonValue scalar
                || scalar.GetValueKind() is not (JsonValueKind.True or JsonValueKind.False))
            {
                return false;
            }

            var index = Array.FindIndex(parameters, parameter => parameter.Name == Camel(flag));
            arguments[index] = scalar.GetValueKind() == JsonValueKind.True;
        }

        argument = Invoke(() => constructor.Invoke(arguments));
        return true;
    }

    private static object ImplicitText(Type parameterType, string content)
    {
        var textType = Nullable.GetUnderlyingType(parameterType) ?? parameterType;
        var conversion = textType.GetMethods(BindingFlags.Public | BindingFlags.Static)
            .Single(method => method.Name == "op_Implicit" && method.GetParameters()[0].ParameterType == typeof(string));
        return Invoke(() => conversion.Invoke(null, [content]));
    }

    private static bool IsCoercible(JsonNode? value, string coerce, out string content)
    {
        content = string.Empty;
        return value is JsonObject obj
            && obj.Count == 2
            && JsonValue(obj["type"], out var type)
            && type == coerce
            && JsonValue(obj["text"], out content);
    }

    private static bool JsonValue(JsonNode? node, out string text) => Internal.JsonValues.IsString(node, out text);

    private static long? Whole(JsonNode? value)
    {
        if (value is not System.Text.Json.Nodes.JsonValue scalar || scalar.GetValueKind() != JsonValueKind.Number)
        {
            return null;
        }

        if (scalar.TryGetValue<long>(out var whole))
        {
            return whole;
        }

        if (scalar.TryGetValue<int>(out var small))
        {
            return small;
        }

        if (scalar.TryGetValue<JsonElement>(out var element))
        {
            var raw = element.GetRawText();
            return !raw.Contains('.', StringComparison.Ordinal) && !raw.Contains('e', StringComparison.OrdinalIgnoreCase)
                && element.TryGetInt64(out var parsed) ? parsed : null;
        }

        return null;
    }

    private static string Camel(string snake)
    {
        var parts = snake.Split('_');
        return parts[0] + string.Concat(parts.Skip(1).Select(part => char.ToUpperInvariant(part[0]) + part[1..]));
    }
}
