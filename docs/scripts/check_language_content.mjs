import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const docsRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../docs");

async function markdownFiles(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(entries.map((entry) => {
    const target = path.join(directory, entry.name);
    if (entry.isDirectory()) return markdownFiles(target);
    return /\.mdx?$/.test(entry.name) ? [target] : [];
  }));
  return nested.flat();
}

for (const file of await markdownFiles(docsRoot)) {
  const source = await readFile(file, "utf8");
  const blocks = source.match(/<LanguageContent>[\s\S]*?<\/LanguageContent>/g) ?? [];
  blocks.forEach((block, index) => {
    assert.match(block, /<Go>/, `${path.relative(docsRoot, file)} LanguageContent #${index + 1} has no Go variant`);
  });
}

console.log("Every language-switched documentation section includes Go content.");
