import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const directory = path.dirname(fileURLToPath(import.meta.url));
const docs = path.resolve(directory, "../docs/reference/ruby");
const domains = ["blocks", "elements", "objects", "payloads", "components", "core", "errors"];
const pages = new Map(await Promise.all(domains.map(async (domain) => [
  domain,
  await readFile(path.join(docs, `${domain}.mdx`), "utf8"),
])));
const rendered = [...pages.values()].join("\n");
const reference = JSON.parse(execFileSync("python3", [path.join(directory, "ruby_reference_adapter.py")], {
  encoding: "utf8",
}));
const headings = [...rendered.matchAll(/^## ([A-Za-z][A-Za-z0-9]+)$/gm)].map((match) => match[1]);
assert.equal(headings.length, reference.types.length, "every public Ruby type must be documented exactly once");
assert.deepEqual(new Set(headings), new Set(reference.types.map(({ name }) => name)));

for (const [domain, type, field] of [
  ["blocks", "SectionBlock", "text"],
  ["elements", "ButtonElement", "action_id"],
  ["objects", "MarkdownText", "text"],
  ["payloads", "MessagePayload", "channel"],
]) {
  const page = pages.get(domain);
  assert.match(page, new RegExp(`^## ${type}$`, "m"));
  assert.match(page, /^### \.new$/m);
  assert.match(page, new RegExp(`^### ${field}$`, "m"));
  assert.match(page, /```ruby\ndef self\.new:/);
  assert.match(page, /\| Parameter \| Description \|/);
}
assert.match(pages.get("blocks"), /\| Throws \| When \|/);
assert.match(pages.get("blocks"), /See the \[Slack reference\]\(https:\/\/docs\.slack\.dev\/reference\/block-kit\/blocks\/section-block\)/);
assert.match(pages.get("blocks"), /def text: \(\) -> \(Text\)\?/);
assert.match(pages.get("elements"), /\| `PRIMARY` \| `:primary` \|/);
assert.match(pages.get("objects"), /^## RichTextStyle$/m);
assert.match(pages.get("components"), /^## Paginator$/m);
assert.match(pages.get("core"), /^## Value$/m);
assert.match(pages.get("errors"), /^## ValidationError$/m);
assert.doesNotMatch(rendered, /\]\((?:ref|javadoc):/);
assert.doesNotMatch(rendered, /^\s+$/m);

for (const [, domain, expectedAnchor] of rendered.matchAll(/\]\(\/reference\/ruby\/([a-z]+)#([a-z0-9]+)\)/g)) {
  const target = pages.get(domain);
  assert.ok(target, `Ruby reference links to missing ${domain}.mdx`);
  const anchors = new Set([...target.matchAll(/^## (.+)$/gm)].map((match) =>
    match[1].toLowerCase().replace(/[^a-z0-9]+/g, "")));
  assert.ok(anchors.has(expectedAnchor), `Ruby reference links to missing #${expectedAnchor} in ${domain}.mdx`);
}

const html = await readFile(path.resolve(directory, "../build/reference/ruby/blocks.html"), "utf8");
const breadcrumbs = html.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
assert.ok(breadcrumbs, "Ruby blocks page is missing breadcrumbs");
assert.doesNotMatch(breadcrumbs, />Ruby API reference</, "Ruby API breadcrumbs should omit the redundant language level");
console.log("Ruby API rendering checks passed.");
