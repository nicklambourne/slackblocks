import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const referenceDirectory = path.resolve(scriptsDirectory, "../docs/reference/csharp");
const buildDirectory = path.resolve(scriptsDirectory, "../build/reference/csharp");
const files = (await readdir(referenceDirectory)).filter((file) => file.endsWith(".mdx"));
const pages = new Map(await Promise.all(files.map(async (file) => [
  file.slice(0, -4),
  await readFile(path.join(referenceDirectory, file), "utf8"),
])));
const rendered = [...pages.values()].join("\n");

for (const [domain, type, member] of [
  ["blocks", "SectionBlock", "Accessory"],
  ["elements", "ButtonElement", "ActionId"],
  ["objects", "MarkdownText", "Verbatim"],
  ["payloads", "MessagePayload", "Channel"],
]) {
  const page = pages.get(domain);
  assert.ok(page, `${domain}.mdx is missing`);
  assert.match(page, new RegExp(`^## ${type}$`, "m"));
  assert.match(page, new RegExp(`^### ${type}$`, "m"), `${type} must document its constructor`);
  assert.match(page, new RegExp(`^### ${member}$`, "m"));
  assert.match(page, /```csharp\npublic /, `${domain} must render signatures as C# code`);
  assert.match(page, /\| Parameter \| Description \|/, `${domain} must document constructor arguments`);
}

assert.match(pages.get("blocks"), /Slack allows at most 3000 characters\./, "limits must be documented");
assert.match(pages.get("blocks"), /\| Throws \| When \|/, "constructor exceptions must be documented");
assert.match(pages.get("blocks"), /See the \[Slack reference\]\(https:\/\/docs\.slack\.dev\/reference\/block-kit\/blocks\/section-block\)/);
assert.match(pages.get("blocks"), /^- Required: `text`\.$/m, "required fields must be listed");
assert.match(pages.get("blocks"), /^## IBlock$/m, "role interfaces must be documented");
assert.match(pages.get("elements"), /^\| `Primary` \| `primary` \|/m, "enum members must be documented with their Slack values");
assert.match(pages.get("blocks"), /```csharp\npublic IElement\? Accessory \{ get; \}\n```/, "signatures must carry nullability");
assert.match(pages.get("objects"), /public static implicit operator PlainText\?\(string\? text\)/, "string conversions must be documented");
assert.match(pages.get("components"), /^## Paginator$/m);
assert.match(pages.get("core"), /^## SlackObject$/m);
assert.match(pages.get("errors"), /^## ValidationException$/m);
assert.match(pages.get("blocks"), /generated from the XML documentation of the compiled library/);
assert.doesNotMatch(rendered, /^## (?:WireBuilder|Validator|SlackJson|WireValues|\w+Extensions)$/m);
assert.doesNotMatch(rendered, /\]\(ref:/, "type links must be resolved");
assert.doesNotMatch(rendered, /<\/?(?:see|c|para|list|item|summary|remarks)\b/, "XML documentation must be rendered as Markdown");
assert.doesNotMatch(rendered, /^\s+$/m, "pages must not contain whitespace-only lines");
assert.doesNotMatch(rendered, /^### \w+ — $/m, "overload headings must name their parameters");

for (const match of rendered.matchAll(/\]\(\/reference\/csharp\/([a-z]+)#([a-z0-9]+)\)/g)) {
  const [, domain, expectedAnchor] = match;
  const target = pages.get(domain);
  assert.ok(target, `C# reference links to missing ${domain}.mdx`);
  const anchors = new Set([...target.matchAll(/^## (.+)$/gm)].map((heading) =>
    heading[1].toLowerCase().replace(/[^a-z0-9]+/g, ""),
  ));
  assert.ok(anchors.has(expectedAnchor), `C# reference links to missing #${expectedAnchor} in ${domain}.mdx`);
}

const blocksHtml = await readFile(path.join(buildDirectory, "blocks.html"), "utf8");
const breadcrumbs = blocksHtml.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
assert.ok(breadcrumbs, "blocks.html is missing its breadcrumbs");
assert.doesNotMatch(breadcrumbs, />C# API reference</, "C# API breadcrumbs must omit the redundant language level");

console.log("C# API rendering checks passed.");
