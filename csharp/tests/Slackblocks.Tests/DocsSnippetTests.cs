using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Text.Json.Nodes;
using System.Text.RegularExpressions;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;

namespace Slackblocks.Tests;

/// <summary>
/// Compiles every C# snippet in the documentation site and the READMEs, and checks that each
/// Using Blocks example produces the JSON documented beside it.
/// </summary>
public sealed partial class DocsSnippetTests
{
    private const string GlobalUsings = """
        global using System;
        global using System.Collections.Generic;
        global using System.IO;
        global using System.Linq;
        global using System.Net.Http;
        global using System.Threading.Tasks;
        global using Slackblocks;
        global using Slackblocks.Blocks;
        global using Slackblocks.Components;
        global using Slackblocks.Elements;
        global using Slackblocks.Objects;
        global using Slackblocks.Payloads;
        """;

    // Values that README snippets refer to from earlier snippets on the same page.
    private const string Context = """
        private static readonly SectionBlock block = new(text: "Hello");
        """;

    private static readonly Lazy<Compiled> Snippets = new(Compile);

    public static TheoryData<string> UsingBlocksSections()
    {
        var data = new TheoryData<string>();
        foreach (var title in UsingBlocks().Keys)
        {
            data.Add(title);
        }

        return data;
    }

    [Fact]
    public void EveryDocumentationSnippetCompiles()
    {
        var compiled = Snippets.Value;

        Assert.True(compiled.Errors.Count == 0, "Documentation snippets do not compile:\n" + string.Join("\n", compiled.Errors));
        Assert.True(compiled.SnippetCount >= 40, $"expected the C# guides and READMEs to contain snippets, found {compiled.SnippetCount}");
    }

    [Fact]
    public void SectionHelloExampleMatchesTheSharedJson()
    {
        var main = Snippets.Value.Assembly!.GetType("SectionHello")!.GetMethod("Main")!;
        var output = new StringWriter();
        var original = Console.Out;
        Console.SetOut(output);
        try
        {
            main.Invoke(null, null);
        }
        finally
        {
            Console.SetOut(original);
        }

        var expected = JsonNode.Parse(File.ReadAllText(Repository.PathTo("docs", "examples", "section_hello.json")));
        Assert.Equal(FixtureDriver.Canonical(expected), FixtureDriver.Canonical(JsonNode.Parse(output.ToString())));
    }

    [Theory]
    [MemberData(nameof(UsingBlocksSections))]
    public void EveryUsingBlocksSnippetProducesItsDocumentedJson(string title)
    {
        var type = Snippets.Value.Assembly!.GetType(UsingBlocksClass(title))!;
        var value = (ISlackObject)type.GetMethod("Value")!.Invoke(null, null)!;
        var expected = JsonNode.Parse(UsingBlocks()[title].Json);

        Assert.Equal(FixtureDriver.Canonical(expected), FixtureDriver.Canonical(JsonNode.Parse(value.ToJson())));
    }

    [Fact]
    public void EveryBlockSectionHasACSharpExample() => Assert.True(UsingBlocks().Count >= 21);

    private static Compiled Compile()
    {
        var sources = new List<string> { GlobalUsings, File.ReadAllText(Repository.PathTo("docs", "examples", "csharp", "SectionHello.cs")) };
        var count = 0;
        var documents = Directory.EnumerateFiles(Repository.PathTo("docs", "docs"), "*.mdx", SearchOption.AllDirectories)
            .Where(file => !file.Contains($"{Path.DirectorySeparatorChar}reference{Path.DirectorySeparatorChar}csharp{Path.DirectorySeparatorChar}", StringComparison.Ordinal))
            .Order(StringComparer.Ordinal)
            .Select(file => (File: file, Regions: CSharpSection().Matches(ReadDocument(file)).Select(match => match.Groups[1].Value).ToList()))
            .Append((Repository.PathTo("README.md"), [ReadDocument(Repository.PathTo("README.md"))]))
            .Append((Repository.PathTo("csharp", "README.md"), [ReadDocument(Repository.PathTo("csharp", "README.md"))]));
        foreach (var (file, regions) in documents)
        {
            foreach (var region in regions)
            {
                foreach (Match fence in CSharpFence().Matches(region))
                {
                    count++;
                    sources.Add(Wrap($"Doc{count}", fence.Groups[1].Value, Path.GetRelativePath(Repository.Root, file)));
                }
            }
        }

        foreach (var (title, section) in UsingBlocks())
        {
            sources.Add(Wrap(UsingBlocksClass(title), section.Code, "using_blocks.mdx", returnsBlock: true));
        }

        var references = ((string)AppContext.GetData("TRUSTED_PLATFORM_ASSEMBLIES")!)
            .Split(Path.PathSeparator)
            .Append(typeof(SlackObject).Assembly.Location)
            .Distinct()
            .Select(path => MetadataReference.CreateFromFile(path));
        var compilation = CSharpCompilation.Create(
            "DocsSnippets",
            sources.Select(source => CSharpSyntaxTree.ParseText(source, new CSharpParseOptions(LanguageVersion.CSharp12))),
            references,
            new CSharpCompilationOptions(OutputKind.DynamicallyLinkedLibrary, nullableContextOptions: NullableContextOptions.Enable));
        using var stream = new MemoryStream();
        var result = compilation.Emit(stream);
        var errors = result.Diagnostics
            .Where(diagnostic => diagnostic.Severity == DiagnosticSeverity.Error)
            .Select(diagnostic => $"{diagnostic.Location.SourceTree?.GetText().Lines[0]} {diagnostic}")
            .ToList();
        return new Compiled(result.Success ? Assembly.Load(stream.ToArray()) : null, errors, count);
    }

    private static string Wrap(string name, string snippet, string origin, bool returnsBlock = false)
    {
        var usings = new StringBuilder();
        var body = new StringBuilder();
        foreach (var line in snippet.Split('\n'))
        {
            (line.StartsWith("using ", StringComparison.Ordinal) && line.TrimEnd().EndsWith(';') && !line.Contains('(', StringComparison.Ordinal) && !line.StartsWith("using var ", StringComparison.Ordinal)
                ? usings
                : body).Append(line).Append('\n');
        }

        var method = returnsBlock
            ? $"public static ISlackObject Value()\n{{\n{body}\nreturn block;\n}}"
            : $"public static async Task Run()\n{{\n{body}\nawait Task.CompletedTask;\n}}";
        return $"// {origin}\n{usings}\n#pragma warning disable CS8321, CS0168, CS0219\npublic static class {name}\n{{\n{Context}\n{method}\n}}\n";
    }

    private static Dictionary<string, (string Code, string Json)> UsingBlocks()
    {
        var text = ReadDocument(Repository.PathTo("docs", "docs", "usage", "using_blocks.mdx"));
        var headings = Heading().Matches(text).ToList();
        var sections = new Dictionary<string, (string, string)>(StringComparer.Ordinal);
        for (var index = 0; index < headings.Count; index++)
        {
            var start = headings[index].Index + headings[index].Length;
            var end = index + 1 < headings.Count ? headings[index + 1].Index : text.Length;
            var body = text[start..end];
            var csharp = CSharpSection().Match(body);
            var json = JsonFence().Match(body);
            if (!csharp.Success || !json.Success)
            {
                continue;
            }

            var fence = CSharpFence().Match(csharp.Groups[1].Value);
            if (fence.Success && fence.Groups[1].Value.StartsWith("var block = ", StringComparison.Ordinal))
            {
                sections[headings[index].Groups[1].Value] = (fence.Groups[1].Value, json.Groups[1].Value);
            }
        }

        return sections;
    }

    // Windows checkouts may convert documents to CRLF; the fence patterns expect LF.
    private static string ReadDocument(string path) => File.ReadAllText(path).ReplaceLineEndings("\n");

    private static string UsingBlocksClass(string title) => "UsingBlocks" + Regex.Replace(title, "[^A-Za-z]", string.Empty);

    [GeneratedRegex("<CSharp>(.*?)</CSharp>", RegexOptions.Singleline)]
    private static partial Regex CSharpSection();

    [GeneratedRegex("```csharp\n(.*?)```", RegexOptions.Singleline)]
    private static partial Regex CSharpFence();

    [GeneratedRegex("```json\n(.*?)```", RegexOptions.Singleline)]
    private static partial Regex JsonFence();

    [GeneratedRegex("^## (.+)$", RegexOptions.Multiline)]
    private static partial Regex Heading();

    private sealed record Compiled(Assembly? Assembly, IReadOnlyList<string> Errors, int SnippetCount);
}
