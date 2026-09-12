import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const referenceDirectory = path.resolve(scriptsDirectory, "../docs/reference/java");
const buildDirectory = path.resolve(scriptsDirectory, "../build/reference/java");
const files = (await readdir(referenceDirectory)).filter((file) => file.endsWith(".mdx"));
const pages = new Map(await Promise.all(files.map(async (file) => [
  file.slice(0, -4),
  await readFile(path.join(referenceDirectory, file), "utf8"),
])));
const rendered = [...pages.values()].join("\n");

for (const [domain, type, method] of [
  ["blocks", "SectionBlock", "MarkdownText"],
  ["elements", "ButtonElement", "ActionId"],
  ["objects", "MarkdownText", "Of"],
  ["payloads", "MessagePayload", "Channel"],
]) {
  const page = pages.get(domain);
  assert.ok(page, `${domain}.mdx is missing`);
  assert.match(page, new RegExp(`^## ${type}$`, "m"));
  assert.match(page, new RegExp(`^### ${method}$`, "m"));
  assert.match(page, /```java\npublic /, `${domain} must render signatures as Java code`);
  assert.match(page, /\| Parameter \| Description \|/, `${domain} must document method arguments`);
}

assert.match(pages.get("blocks"), /official `LayoutBlock` interface/);
assert.match(pages.get("components"), /ordinary `List<Block>` values/);
assert.match(pages.get("core"), /^## SlackObject$/m);
assert.match(pages.get("errors"), /^## ValidationException$/m);
assert.doesNotMatch(rendered, /^## (?:BuilderState|Validator|WireObjects|SlackObjectJsonAdapter)$/m);
assert.doesNotMatch(rendered, /Class:|Method:|Interface:/);

for (const match of rendered.matchAll(/\]\(\/reference\/java\/([a-z]+)#([a-z0-9]+)\)/g)) {
  const [, domain, expectedAnchor] = match;
  const target = pages.get(domain);
  assert.ok(target, `Java reference links to missing ${domain}.mdx`);
  const anchors = new Set([...target.matchAll(/^## (.+)$/gm)].map((heading) =>
    heading[1].toLowerCase().replace(/[^a-z0-9]+/g, ""),
  ));
  assert.ok(anchors.has(expectedAnchor), `Java reference links to missing #${expectedAnchor} in ${domain}.mdx`);
}

const blocksHtml = await readFile(path.join(buildDirectory, "blocks.html"), "utf8");
const breadcrumbs = blocksHtml.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
assert.ok(breadcrumbs, "blocks.html is missing its breadcrumbs");
assert.doesNotMatch(breadcrumbs, />Java API reference</, "Java API breadcrumbs must omit the redundant language level");

console.log("Java API rendering checks passed.");
