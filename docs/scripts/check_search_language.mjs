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
const goDocuments = await readDocuments("go");
const javaDocuments = await readDocuments("java");
const csharpDocuments = await readDocuments("csharp");
const pythonText = pythonDocuments.map(({ t }) => t).join("\n");
const typescriptText = typescriptDocuments.map(({ t }) => t).join("\n");
const goText = goDocuments.map(({ t }) => t).join("\n");
const javaText = javaDocuments.map(({ t }) => t).join("\n");
const csharpText = csharpDocuments.map(({ t }) => t).join("\n");

assert.equal(
  pythonDocuments.some(({ u }) => u.includes("/reference/typescript")),
  false,
);
assert.equal(
  typescriptDocuments.some(({ u }) => u.includes("/reference/python")),
  false,
);
assert.equal(
  goDocuments.some(({ u }) => /\/reference\/(?:python|typescript|java|csharp)/.test(u)),
  false,
  "Go search leaked another language's API reference",
);
assert.equal(
  javaDocuments.some(({ u }) => /\/reference\/(?:python|typescript|go|csharp)/.test(u)),
  false,
  "Java search leaked another language's API reference",
);
assert.equal(
  csharpDocuments.some(({ u }) => /\/reference\/(?:python|typescript|go|java)/.test(u)),
  false,
  "C# search leaked another language's API reference",
);
assert.equal(
  pythonDocuments.some(({ u }) => u.includes("/reference/csharp")),
  false,
  "Python search leaked the C# API reference",
);
assert.equal(
  typescriptDocuments.some(({ u }) => u.includes("/reference/csharp")),
  false,
  "TypeScript search leaked the C# API reference",
);
assert.equal(
  pythonDocuments.some(({ u }) => u.includes("/reference/go")),
  false,
  "Python search leaked the Go API reference",
);
assert.equal(
  typescriptDocuments.some(({ u }) => u.includes("/reference/go")),
  false,
  "TypeScript search leaked the Go API reference",
);
assert.equal(
  pythonDocuments.some(({ u }) => u.includes("/reference/java")),
  false,
  "Python search leaked the Java API reference",
);
assert.equal(
  typescriptDocuments.some(({ u }) => u.includes("/reference/java")),
  false,
  "TypeScript search leaked the Java API reference",
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
assert.match(goText, /Go 1\.22 or newer is required/);
assert.doesNotMatch(goText, /Python 3\.10 or newer is required/);
assert.doesNotMatch(goText, /Node\.js 20\.19 or newer is required/);
assert.match(javaText, /Java 17 or newer is required/);
assert.doesNotMatch(javaText, /Python 3\.10 or newer is required/);
assert.doesNotMatch(javaText, /Node\.js 20\.19 or newer is required/);
assert.doesNotMatch(javaText, /Go 1\.22 or newer is required/);
assert.doesNotMatch(javaText, /\.NET 8 or newer is required/);
assert.match(csharpText, /\.NET 8 or newer is required/);
assert.doesNotMatch(csharpText, /Python 3\.10 or newer is required/);
assert.doesNotMatch(csharpText, /Node\.js 20\.19 or newer is required/);
assert.doesNotMatch(csharpText, /Go 1\.22 or newer is required/);
assert.doesNotMatch(csharpText, /Java 17 or newer is required/);

console.log("Search indexes are scoped to their selected languages.");
