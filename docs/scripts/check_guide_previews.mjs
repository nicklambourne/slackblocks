import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const docsDirectory = path.resolve(scriptsDirectory, "../docs/usage");
const staticDirectory = path.resolve(scriptsDirectory, "../static");

function sections(source) {
  return new Map(
    source
      .split(/^## /m)
      .slice(1)
      .map((section) => {
        const [title, ...body] = section.split("\n");
        return [title.trim(), body.join("\n")];
      }),
  );
}

const usingBlocks = sections(
  await readFile(path.join(docsDirectory, "using_blocks.mdx"), "utf8"),
);
const blockSections = [...usingBlocks].filter(([title]) =>
  title.endsWith(" Block"),
);

assert.ok(blockSections.length > 0, "Using Blocks has no block sections");

for (const [title, body] of blockSections) {
  assert.match(
    body,
    /<TabItem value=\{"slack-ui"\} label=\{"Slack UI"\}>/,
    `${title} is missing its Slack UI tab`,
  );
  assert.match(
    body,
    /!\[[^\]]+\]\([^)]+\)/,
    `${title} is missing its Slack UI preview image`,
  );
}

const guide = await readFile(path.join(docsDirectory, "using_blocks.mdx"), "utf8");
for (const [, asset] of guide.matchAll(/!\[[^\]]+\]\((\/img\/usage\/[^)]+)\)/g)) {
  await access(path.join(staticDirectory, asset));
}

const catalog = JSON.parse(await readFile(path.join(scriptsDirectory, "../examples/recipes/catalog.json"), "utf8"));
for (const { id, title } of catalog) {
  const source = await readFile(path.join(docsDirectory, `../recipes/${id}.mdx`), "utf8");
  assert.ok(source.includes(`<RecipeOutput recipe="${id}" outputs={recipeOutputs} />`), `${title} lacks its executable output preview`);
}

const layouts = await readFile(path.join(docsDirectory, "layouts.mdx"), "utf8");
assert.match(layouts, /!\[[^\]]+\]\(\/img\/usage\/layout-anatomy\.svg\)/, "Layouts needs its anatomy diagram");
await access(path.join(staticDirectory, "img/usage/layout-anatomy.svg"));
assert.match(layouts, /<RecipeOutput recipe="layout-basic" outputs=/, "Layouts needs its tested example preview");

console.log("Guide preview checks passed.");
