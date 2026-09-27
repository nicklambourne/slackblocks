// Reads the compiled Slackblocks assembly and its XML documentation and writes the public API as
// the JSON model shared with the Java reference: types with Markdown documentation, enum constants,
// and members with signatures, parameters, return values, and exceptions.

using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using Slackblocks;

if (args.Length != 1)
{
    Console.Error.WriteLine("usage: ReferenceTool <output.json>");
    return 2;
}

var assembly = typeof(SlackObject).Assembly;
var xmlPath = Path.ChangeExtension(assembly.Location, ".xml");
var documentation = XDocument.Load(xmlPath).Descendants("member")
    .ToDictionary(member => (string)member.Attribute("name")!, member => member);
var nullability = new NullabilityInfoContext();

var types = assembly.GetExportedTypes()
    .Where(type => !type.Name.EndsWith("Extensions", StringComparison.Ordinal) || type.IsEnum)
    .OrderBy(type => type.Name, StringComparer.Ordinal)
    .Select(DescribeType)
    .ToList();

await File.WriteAllTextAsync(args[0], JsonSerializer.Serialize(new { types }, new JsonSerializerOptions { WriteIndented = true }));
Console.WriteLine($"Described {types.Count} C# API types.");
return 0;

object DescribeType(Type type)
{
    var doc = Documentation($"T:{TypeId(type)}");
    var text = Join(Markdown(doc?.Element("summary")), Markdown(doc?.Element("remarks")));
    var relations = Relations(type);
    if (relations.Length > 0)
    {
        text = Join(text, relations);
    }

    var constants = new List<object>();
    if (type.IsEnum)
    {
        var toWire = assembly.GetType(type.FullName + "Extensions")?.GetMethod("ToWireValue");
        foreach (var field in type.GetFields(BindingFlags.Public | BindingFlags.Static))
        {
            var value = field.GetValue(null)!;
            constants.Add(new
            {
                name = field.Name,
                wire = toWire is null ? field.Name : (string)toWire.Invoke(null, [value])!,
                doc = Markdown(Documentation($"F:{TypeId(type)}.{field.Name}")?.Element("summary")),
            });
        }

        if (toWire is not null)
        {
            text = Join(text, "`ToWireValue()` returns the value Slack expects in JSON.");
        }
    }

    var members = new List<object>();
    const BindingFlags declared = BindingFlags.Public | BindingFlags.Instance | BindingFlags.Static | BindingFlags.DeclaredOnly;
    foreach (var constructor in type.GetConstructors(BindingFlags.Public | BindingFlags.Instance).OrderBy(member => member.MetadataToken))
    {
        members.Add(Member(type, constructor, type.Name, $"public {type.Name}({Parameters(constructor)})", null));
    }

    foreach (var property in type.GetProperties(declared).OrderBy(member => member.MetadataToken))
    {
        var info = nullability.Create(property);
        var modifier = type.IsInterface ? string.Empty : "public ";
        members.Add(Member(
            type,
            property,
            property.Name,
            $"{modifier}{TypeName(property.PropertyType, info)} {property.Name} {{ get; }}",
            null));
    }

    foreach (var method in type.GetMethods(declared).OrderBy(member => member.MetadataToken))
    {
        if ((method.IsSpecialName && method.Name != "op_Implicit") || IsObjectOverride(method))
        {
            continue;
        }

        var returns = TypeName(method.ReturnType, nullability.Create(method.ReturnParameter));
        var modifier = type.IsInterface ? string.Empty : method.IsStatic ? "public static " : "public ";
        var signature = method.Name == "op_Implicit"
            ? $"public static implicit operator {returns}({Parameters(method)})"
            : $"{modifier}{returns} {method.Name}({Parameters(method)})";
        var name = method.Name == "op_Implicit" ? "Implicit conversion" : method.Name;
        members.Add(Member(type, method, name, signature, method));
    }

    return new
    {
        name = type.Name,
        package = type.Namespace,
        doc = text,
        see = Array.Empty<object>(),
        constants,
        members,
    };
}

object Member(Type owner, MemberInfo member, string name, string signature, MethodInfo? method)
{
    var doc = Documentation(MemberId(owner, member));
    if (doc?.Element("inheritdoc") is not null)
    {
        doc = owner.GetInterfaces()
            .Select(contract => Documentation(MemberId(contract, contract.GetMember(member.Name).FirstOrDefault() ?? member)))
            .FirstOrDefault(found => found is not null) ?? doc;
    }

    var parameters = member is MethodBase methodBase ? methodBase.GetParameters() : [];
    return new
    {
        name,
        signature,
        parameterTypes = string.Join(", ", parameters.Select(parameter => TypeName(parameter.ParameterType, nullability.Create(parameter)))),
        doc = Join(Markdown(doc?.Element("summary")), Markdown(doc?.Element("value"))),
        @params = parameters.Select(parameter => new
        {
            name = parameter.Name,
            doc = Markdown(doc?.Elements("param").FirstOrDefault(element => (string?)element.Attribute("name") == parameter.Name)),
        }),
        returns = method is null || method.ReturnType == typeof(void) ? string.Empty : Markdown(doc?.Element("returns")),
        throws = (doc?.Elements("exception") ?? []).Select(exception => new
        {
            type = ShortCref((string)exception.Attribute("cref")!),
            doc = Markdown(exception),
        }),
    };
}

string Relations(Type type)
{
    if (type.IsEnum || type.IsInterface)
    {
        return string.Empty;
    }

    var parts = new List<string>();
    if (type.BaseType is { } baseType && baseType.Assembly == assembly && baseType.Name != nameof(SlackObject))
    {
        parts.Add($"Derives from [`{baseType.Name}`](ref:{baseType.Name}).");
    }

    var roles = type.GetInterfaces()
        .Where(contract => contract.Assembly == assembly && contract != typeof(ISlackObject))
        .Select(contract => $"[`{contract.Name}`](ref:{contract.Name})")
        .Order(StringComparer.Ordinal)
        .ToList();
    if (roles.Count > 0)
    {
        parts.Add($"Implements {string.Join(", ", roles)}.");
    }

    return string.Join(" ", parts);
}

bool IsObjectOverride(MethodInfo method) =>
    method.Name is "Equals" or "GetHashCode" or "ToString" or "Deconstruct" or "<Clone>$" or "PrintMembers";

string Parameters(MethodBase method) => string.Join(", ", method.GetParameters().Select(parameter =>
{
    var text = new StringBuilder();
    if (method.IsDefined(typeof(System.Runtime.CompilerServices.ExtensionAttribute)) && parameter.Position == 0)
    {
        text.Append("this ");
    }

    text.Append(TypeName(parameter.ParameterType, nullability.Create(parameter))).Append(' ').Append(parameter.Name);
    if (parameter.HasDefaultValue)
    {
        text.Append(" = ").Append(DefaultValue(parameter));
    }

    return text.ToString();
}));

string DefaultValue(ParameterInfo parameter)
{
    var value = parameter.DefaultValue;
    var type = Nullable.GetUnderlyingType(parameter.ParameterType) ?? parameter.ParameterType;
    return value switch
    {
        null => "null",
        bool flag => flag ? "true" : "false",
        string text => JsonSerializer.Serialize(text),
        _ when type.IsEnum => $"{type.Name}.{Enum.GetName(type, value)}",
        IFormattable number => number.ToString(null, System.Globalization.CultureInfo.InvariantCulture),
        _ => value.ToString()!,
    };
}

string TypeName(Type type, NullabilityInfo? info)
{
    if (Nullable.GetUnderlyingType(type) is { } underlying)
    {
        return TypeName(underlying, null) + "?";
    }

    var suffix = !type.IsValueType && info?.ReadState == NullabilityState.Nullable ? "?" : string.Empty;
    if (type.IsArray)
    {
        return TypeName(type.GetElementType()!, info?.ElementType) + "[]" + suffix;
    }

    var alias = type == typeof(string) ? "string"
        : type == typeof(bool) ? "bool"
        : type == typeof(int) ? "int"
        : type == typeof(long) ? "long"
        : type == typeof(double) ? "double"
        : type == typeof(object) ? "object"
        : type == typeof(void) ? "void"
        : null;
    if (alias is not null)
    {
        return alias + suffix;
    }

    if (!type.IsGenericType)
    {
        return type.Name + suffix;
    }

    var arguments = type.GetGenericArguments()
        .Select((argument, index) => TypeName(argument, info is not null && index < info.GenericTypeArguments.Length ? info.GenericTypeArguments[index] : null));
    return $"{type.Name[..type.Name.IndexOf('`', StringComparison.Ordinal)]}<{string.Join(", ", arguments)}>{suffix}";
}

XElement? Documentation(string id) => documentation.GetValueOrDefault(id);

string MemberId(Type owner, MemberInfo member) => member switch
{
    ConstructorInfo constructor => $"M:{TypeId(owner)}.#ctor{ParameterIds(constructor)}",
    PropertyInfo property => $"P:{TypeId(owner)}.{property.Name}",
    MethodInfo { Name: "op_Implicit" } method => $"M:{TypeId(owner)}.op_Implicit{ParameterIds(method)}~{TypeId(method.ReturnType)}",
    MethodInfo method => $"M:{TypeId(owner)}.{method.Name}{ParameterIds(method)}",
    _ => string.Empty,
};

string ParameterIds(MethodBase method)
{
    var parameters = method.GetParameters();
    return parameters.Length == 0 ? string.Empty : "(" + string.Join(",", parameters.Select(parameter => TypeId(parameter.ParameterType))) + ")";
}

string TypeId(Type type)
{
    if (type.IsArray)
    {
        return TypeId(type.GetElementType()!) + "[]";
    }

    if (!type.IsGenericType)
    {
        return (type.FullName ?? type.Name).Replace('+', '.');
    }

    var definition = type.GetGenericTypeDefinition().FullName!;
    var name = definition[..definition.IndexOf('`', StringComparison.Ordinal)];
    return $"{name}{{{string.Join(",", type.GetGenericArguments().Select(TypeId))}}}";
}

static string ShortCref(string cref)
{
    var name = cref[(cref.IndexOf(':', StringComparison.Ordinal) + 1)..];
    var parenthesis = name.IndexOf('(', StringComparison.Ordinal);
    if (parenthesis >= 0)
    {
        name = name[..parenthesis];
    }

    // Types keep their simple name; members keep their declaring type, such as Paginator.Create.
    var segments = name.Split('.');
    return cref.StartsWith("T:", StringComparison.Ordinal) || segments.Length < 2
        ? segments[^1]
        : $"{segments[^2]}.{segments[^1]}";
}

static string Join(params string[] parts) =>
    string.Join("\n\n", parts.Where(part => !string.IsNullOrWhiteSpace(part)));

static string Markdown(XElement? element)
{
    if (element is null)
    {
        return string.Empty;
    }

    var text = Inline(element);
    text = Regex.Replace(text, "[ \t]*\n[ \t]*", "\n");
    text = Regex.Replace(text, "\n{3,}", "\n\n");
    return text.Trim();
}

static string Inline(XElement element)
{
    var builder = new StringBuilder();
    foreach (var node in element.Nodes())
    {
        switch (node)
        {
            case XText text:
                // Braces start MDX expressions and < starts JSX, so escape them in prose as the Java doclet does.
                builder.Append(Regex.Replace(text.Value, @"\s+", " ").Replace("{", "\\{").Replace("}", "\\}").Replace("<", "&lt;"));
                break;
            case XElement { Name.LocalName: "c" } code:
                builder.Append('`').Append(code.Value).Append('`');
                break;
            case XElement { Name.LocalName: "para" } paragraph:
                builder.Append("\n\n").Append(Inline(paragraph).Trim()).Append("\n\n");
                break;
            case XElement { Name.LocalName: "list" } list:
                builder.Append("\n\n");
                foreach (var item in list.Elements("item"))
                {
                    builder.Append("- ").Append(Inline(item.Element("description") ?? item).Trim()).Append('\n');
                }

                builder.Append('\n');
                break;
            case XElement { Name.LocalName: "see" } see when see.Attribute("href") is { } href:
                var label = see.Value.Trim();
                builder.Append('[').Append(label.Length > 0 ? label : href.Value).Append("](").Append(href.Value).Append(')');
                break;
            case XElement { Name.LocalName: "see" } see when see.Attribute("langword") is { } word:
                builder.Append('`').Append(word.Value).Append('`');
                break;
            case XElement { Name.LocalName: "see" or "seealso" } see when see.Attribute("cref") is { } cref:
                var name = ShortCref(cref.Value);
                builder.Append(cref.Value.StartsWith("T:", StringComparison.Ordinal) ? $"[`{name}`](ref:{name})" : $"`{name}`");
                break;
            case XElement { Name.LocalName: "paramref" or "typeparamref" } reference:
                builder.Append('`').Append((string?)reference.Attribute("name")).Append('`');
                break;
            case XElement nested:
                builder.Append(Inline(nested));
                break;
        }
    }

    return builder.ToString();
}
