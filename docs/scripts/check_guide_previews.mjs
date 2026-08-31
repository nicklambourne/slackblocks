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

const cookbook = sections(
  await readFile(path.join(docsDirectory, "cookbook.mdx"), "utf8"),
);
for (const title of [
  "Build status notification",
  "Rich-formatted alert",
  "Confirmation modal",
  "Multi-column status report (table)",
]) {
  const body = cookbook.get(title);
  assert.ok(body, `Recipe Book is missing ${title}`);
  assert.match(body, /!\[[^\]]+\]\(\/img\/usage\/[^)]+\)/, `${title} lacks a preview`);
}

console.log("Guide preview checks passed.");
