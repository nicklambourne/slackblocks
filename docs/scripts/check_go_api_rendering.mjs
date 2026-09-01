import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const referenceDirectory = path.resolve(scriptsDirectory, "../docs/reference/go");
const buildDirectory = path.resolve(scriptsDirectory, "../build/reference/go");
const files = (await readdir(referenceDirectory)).filter((file) =>
  file.endsWith(".mdx"),
);
const pages = new Map(
  await Promise.all(
    files.map(async (file) => [
      file.slice(0, -4),
      await readFile(path.join(referenceDirectory, file), "utf8"),
    ]),
  ),
);
const rendered = [...pages.values()].join("\n");

for (const [domain, constructor, builder, field] of [
  ["blocks", "NewSectionBlock", "SectionBlockBuilder", "Text"],
  ["elements", "NewButton", "ButtonBuilder", "ActionID"],
  ["objects", "NewMarkdown", "MarkdownBuilder", "Verbatim"],
  ["payloads", "NewMessage", "MessageBuilder", "Channel"],
]) {
  const page = pages.get(domain);
  assert.ok(page, `${domain}.mdx is missing`);
  assert.match(page, new RegExp(`^## ${constructor}$`, "m"));
  assert.match(
    page,
    new RegExp(
      "^func " + constructor + "\\(\\) \\*" + builder + "$",
      "m",
    ),
    `${constructor} must return ${builder}`,
  );
  assert.match(page, new RegExp(`^### ${builder}$`, "m"));
  assert.match(
    page,
    new RegExp(
      "^func \\(b \\*" +
        builder +
        "\\) " +
        field +
        "\\([^\\n]*\\) \\*" +
        builder +
        "$",
      "m",
    ),
    `${builder} must document its ${field} fluent method`,
  );
}

assert.doesNotMatch(
  rendered,
  /\t(?:object|coercions|validator) /,
  "Generated reference must not expose private implementation fields",
);
assert.doesNotMatch(
  rendered,
  /func \(b \*Builder\)/,
  "Generated reference must not expose the removed generic Builder API",
);
assert.match(
  pages.get("core"),
  /^## Buildable$/m,
  "Core reference must document the common Buildable contract",
);
assert.match(
  pages.get("blocks"),
  /invalid fields are rejected at compile time/,
  "Concrete builder compile-time guarantees must be explained",
);
assert.equal(pages.has("slack-go"), false, "obsolete adapter page must not exist");
assert.match(
  pages.get("blocks"),
  /implements `slack\.Block`/,
  "Block reference must explain direct slack.Block compatibility",
);
assert.match(
  pages.get("blocks"),
  /slack\.MsgOptionBlocks/,
  "Block reference must show the native slack-go option path",
);
assert.match(
  pages.get("components"),
  /SlackBlocks\(\)/,
  "Component reference must document native block expansion",
);

for (const match of rendered.matchAll(
  /\]\(\/reference\/go\/([a-z]+)#([a-z0-9-]+)\)/g,
)) {
  const [, domain, anchor] = match;
  const target = pages.get(domain);
  assert.ok(target, `Go reference links to missing ${domain}.mdx`);
  const anchors = new Set(
    [...target.matchAll(/^## (.+)$/gm)].map((heading) =>
      heading[1].toLowerCase().replace(/[^a-z0-9]+/g, ""),
    ),
  );
  assert.ok(
    anchors.has(anchor),
    `Go reference links to missing #${anchor} in ${domain}.mdx`,
  );
}

const blocksHtml = await readFile(path.join(buildDirectory, "blocks.html"), "utf8");
const breadcrumbs = blocksHtml.match(
  /<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/,
)?.[0];
assert.ok(breadcrumbs, "blocks.html is missing its breadcrumbs");
assert.doesNotMatch(
  breadcrumbs,
  />Go API reference</,
  "Go API breadcrumbs must omit the redundant language level",
);

console.log("Go API rendering checks passed.");
