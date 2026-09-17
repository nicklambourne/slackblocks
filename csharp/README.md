# slackblocks for .NET

[![NuGet](https://img.shields.io/nuget/v/Slackblocks?logo=nuget)](https://www.nuget.org/packages/Slackblocks)
[![.NET CI](https://github.com/nicklambourne/slackblocks/actions/workflows/dotnet.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions/workflows/dotnet.yml)

Validated, immutable Slack Block Kit construction for .NET 8 and newer.

The NuGet package uses the same version as the Python, TypeScript, Go, and Java packages and is released only as part of the coordinated slackblocks release train.

## Installation

```bash
dotnet add package Slackblocks
```

## Build a block

Every value is an immutable class. Pass named arguments to its constructor, which validates them against Slack's documented rules:

```csharp
using Slackblocks.Blocks;
using Slackblocks.Elements;

var block = new SectionBlock(
    text: "A deployment is ready for review.",
    accessory: new ButtonElement("Review", "review_deployment", value: "deploy-482"));
```

Required arguments come first, so the common ones can be passed positionally. Strings convert to Slack text objects: a field such as a section's text sends `mrkdwn`, and a field Slack requires as plain text, such as a button label, sends `plain_text`. Pass `new PlainText(...)` or `new MarkdownText(...)` when you need options such as `emoji` or `verbatim`.

Known Slack limits, required fields, mutually exclusive fields, and composition restrictions are checked in the constructor. A failure throws `ValidationException` with a stable `ErrorCategory` and field path:

```csharp
using Slackblocks;
using Slackblocks.Blocks;

try
{
    _ = new HeaderBlock(new string('x', 151));
}
catch (ValidationException error)
{
    Console.WriteLine($"{error.Category}: {error.Path}"); // LengthExceeded: HeaderBlock.text.text
}
```

## Read values

Properties return what you passed. Optional values are nullable, and collections are read-only lists that are empty when unset:

```csharp
using Slackblocks.Elements;

var review = (ButtonElement)block.Accessory!;
Console.WriteLine(review.ActionId); // review_deployment
Console.WriteLine(block.Fields.Count); // 0
```

## JSON

`ToJson()` returns compact Slack JSON, and `ToJsonNode()` returns a mutable copy. Values also serialize directly with `System.Text.Json`, including through role interfaces such as `IBlock`:

```csharp
using System.Text.Json;
using Slackblocks.Blocks;

IBlock[] blocks = [new HeaderBlock("Deploy 482"), block];
string json = JsonSerializer.Serialize(new { channel = "C0123456", text = "Deploy 482", blocks });
```

Use `MessagePayload`, `WebhookMessage`, `MessageResponse`, `ModalView`, and `HomeTabView` when you need a complete, validated payload.

## Send a message

slackblocks builds payloads and leaves delivery to you. Post a `WebhookMessage` to an incoming webhook with `HttpClient`:

```csharp
using System.Text;
using Slackblocks.Payloads;

using var http = new HttpClient();
var message = new WebhookMessage(text: "A deployment is ready", blocks: [block]);
using var content = new StringContent(message.ToJson(), Encoding.UTF8, "application/json");
using var response = await http.PostAsync(Environment.GetEnvironmentVariable("SLACK_WEBHOOK_URL"), content);
response.EnsureSuccessStatusCode();
```

For the Web API, send a `MessagePayload` to `chat.postMessage` with a bot token. When you already use a client library such as SlackNet, pass the JSON from `ToJson()` to its raw request support. Several names, such as `SectionBlock`, exist in both libraries, so alias one namespace if a file imports both.

## Fields Slack adds later

Every constructor accepts `additionalFields` for Slack fields that have no named parameter yet. The values are validated with the rest of the object, and a field cannot be set both there and through its named parameter:

```csharp
using Slackblocks.Blocks;

var divider = new DividerBlock(additionalFields: new Dictionary<string, object?> { ["future_flag"] = true });
```

## Higher-level components

`Paginator`, `Accordion`, and `AccordionSection` return ordinary blocks, so their results spread into any block collection:

```csharp
using Slackblocks.Blocks;
using Slackblocks.Components;
using Slackblocks.Payloads;

IBlock[] results = [new SectionBlock(text: "one"), new SectionBlock(text: "two"), new SectionBlock(text: "three")];
var page = new MessagePayload(
    "C0123456",
    blocks: [new HeaderBlock("Results"), .. Paginator.Create("results", results, pageSize: 2)]);
```

## Compatibility and conformance

- .NET 8 and newer, with no dependencies beyond the framework.
- Versioned in lockstep with every supported slackblocks language.
- All shared valid fixtures and invalid-case categories are mandatory; the C# skip list is empty.
- Value types are generated from `java/generator/model.json`, the model the Java package is generated from, so both expose the same fields, limits, and documentation.

## Development

```bash
cd csharp
dotnet test
dotnet format --verify-no-changes
```

The build treats warnings as errors with the .NET analyzers enabled. The tests run every shared conformance fixture and every C# documentation snippet.

The Block Kit model is generated. Edit `java/generator/model.json`, regenerate the Java package as its README describes, then run:

```bash
python3 csharp/generator/generate_models.py
```

See the [C# documentation](https://nicklambourne.github.io/slackblocks/?language=csharp), [API reference](https://nicklambourne.github.io/slackblocks/reference/csharp), and repository-level [contributing guide](https://nicklambourne.github.io/slackblocks/contributing).
