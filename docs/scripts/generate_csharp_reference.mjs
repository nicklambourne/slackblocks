// Generates the C# API reference from the compiled library.
//
// csharp-reference/ is a small .NET tool that loads the Slackblocks assembly and its XML
// documentation and writes the public API as JSON. This script renders that JSON as one MDX page
// per API area with the renderer shared with the Java reference. It needs the .NET 8 SDK or newer
// on PATH, or in DOTNET_ROOT.

import { execFileSync } from "node:child_process";
import { mkdir, readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeReferencePages } from "./reference_pages.mjs";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const toolProject = path.join(scriptsDirectory, "csharp-reference/ReferenceTool.csproj");
const outputRoot = path.join(repository, "docs/docs/reference/csharp");
const workDirectory = path.join(repository, "csharp/artifacts/reference");

const domains = [
  { slug: "blocks", title: "Blocks", position: 1, namespace: "Slackblocks.Blocks" },
  { slug: "elements", title: "Elements", position: 2, namespace: "Slackblocks.Elements" },
  { slug: "objects", title: "Composition Objects", position: 3, namespace: "Slackblocks.Objects" },
  { slug: "payloads", title: "Payloads And Views", position: 4, namespace: "Slackblocks.Payloads" },
  { slug: "components", title: "Components", position: 5, namespace: "Slackblocks.Components" },
  { slug: "core", title: "Core Types", position: 6, namespace: "Slackblocks", names: ["ISlackObject", "SlackObject", "SlackblocksInfo"] },
  { slug: "errors", title: "Validation And Errors", position: 7, namespace: "Slackblocks", names: ["ErrorCategory", "ValidationException"] },
];

function dotnet() {
  const root = process.env.DOTNET_ROOT;
  return root ? path.join(root, "dotnet") : "dotnet";
}

async function referenceJson() {
  await mkdir(workDirectory, { recursive: true });
  const output = path.join(workDirectory, "reference.json");
  try {
    execFileSync(dotnet(), ["run", "--configuration", "Release", "--project", toolProject, "--", output], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
      env: { ...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1" },
    });
  } catch (error) {
    const detail = [error.stdout, error.stderr].filter(Boolean).join("\n");
    throw new Error(`dotnet run ${toolProject} failed. The .NET 8 SDK or newer must be on PATH or in DOTNET_ROOT.\n${detail}`);
  }
  return JSON.parse(await readFile(output, "utf8"));
}

function domainFor(type) {
  return domains.find((domain) => domain.namespace === type.package && (!domain.names || domain.names.includes(type.name)));
}

function introduction(domain) {
  let text = `This page documents the public C# API for ${domain.title.toLowerCase()}, generated from the XML documentation of the compiled library. Values are immutable, constructors take named arguments and validate them, and read-only properties return what was passed.\n\n`;
  if (domain.slug === "blocks") {
    text += "Every block implements `IBlock`, so blocks of any kind share one collection type, and values serialize as Slack JSON with `ToJson()` or `System.Text.Json`.\n\n";
  }
  if (domain.slug === "components") {
    text += "Components return ordinary `IReadOnlyList<IBlock>` values that spread into any block collection. The application remains responsible for delivery and interaction handling.\n\n";
  }
  return text;
}

const index = `---
sidebar_position: 0
---

# C# API reference

This is the complete guide to slackblocks' public C# API, generated from the XML documentation of the compiled library. Use it to find constructor parameters and their limits, typed properties, role interfaces, validation failures, payloads, and higher-level components.

Construct a value with named arguments, and its constructor validates them against Slack's documented rules. Put blocks in a \`MessagePayload\` or \`WebhookMessage\` and send its \`ToJson()\` with \`HttpClient\` or the Slack client library you already use.

- [Blocks](/reference/csharp/blocks)
- [Elements](/reference/csharp/elements)
- [Composition Objects](/reference/csharp/objects)
- [Payloads And Views](/reference/csharp/payloads)
- [Components](/reference/csharp/components)
- [Core Types](/reference/csharp/core)
- [Validation And Errors](/reference/csharp/errors)
`;

const count = await writeReferencePages({
  reference: await referenceJson(),
  domains,
  domainFor,
  language: "csharp",
  fence: "csharp",
  outputRoot,
  introduction,
  index,
});
console.log(`Generated ${count} C# API types from XML documentation.`);
