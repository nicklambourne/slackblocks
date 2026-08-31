import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

async function readDocuments(language) {
  const index = JSON.parse(
    await readFile(
      new URL(`../build/search-index-${language}.json`, import.meta.url),
      "utf8",
    ),
  );
  return index.flatMap(({ documents }) => documents);
}

const pythonDocuments = await readDocuments("python");
const typescriptDocuments = await readDocuments("typescript");
const pythonText = pythonDocuments.map(({ t }) => t).join("\n");
const typescriptText = typescriptDocuments.map(({ t }) => t).join("\n");

assert.equal(
  pythonDocuments.some(({ u }) => u.includes("/reference/typescript")),
  false,
);
assert.equal(
  typescriptDocuments.some(({ u }) => u.includes("/reference/python")),
  false,
);
assert.equal(
  typescriptDocuments.some(({ u }) =>
    /\/usage\/(?:compatibility|migration)$/.test(u),
  ),
  false,
);

assert.match(pythonText, /Python 3\.10 or newer is required/);
assert.doesNotMatch(pythonText, /Node\.js 20\.19 or newer is required/);
assert.match(typescriptText, /Node\.js 20\.19 or newer is required/);
assert.doesNotMatch(typescriptText, /Python 3\.10 or newer is required/);

console.log("Search indexes are scoped to their selected languages.");
