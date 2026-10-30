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
const rubyDocuments = await readDocuments("ruby");
const pythonText = pythonDocuments.map(({ t }) => t).join("\n");
const typescriptText = typescriptDocuments.map(({ t }) => t).join("\n");
const goText = goDocuments.map(({ t }) => t).join("\n");
const javaText = javaDocuments.map(({ t }) => t).join("\n");
const csharpText = csharpDocuments.map(({ t }) => t).join("\n");
const rubyText = rubyDocuments.map(({ t }) => t).join("\n");

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

assert.equal(
  rubyDocuments.some(({ u }) => /\/reference\/(?:python|typescript|go|java|csharp)/.test(u)),
  false,
  "Ruby search leaked another language's API reference",
);
for (const [language, documents] of [
  ["python", pythonDocuments],
  ["typescript", typescriptDocuments],
  ["go", goDocuments],
  ["java", javaDocuments],
  ["csharp", csharpDocuments],
]) {
  assert.equal(documents.some(({ u }) => u.includes("/reference/ruby")), false,
    `${language} search leaked the Ruby API reference`);
}

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

assert.match(rubyText, /MRI Ruby 3\.3 or newer is required/);
assert.doesNotMatch(rubyText, /Python 3\.10 or newer is required/);

console.log("Search indexes are scoped to their selected languages.");

const rustDocuments = await readDocuments("rust");
const rustText = rustDocuments.map(({t}) => t).join("\n");
const languages = new Map([
  ["python", pythonDocuments], ["typescript", typescriptDocuments],
  ["go", goDocuments], ["java", javaDocuments], ["csharp", csharpDocuments],
  ["ruby", rubyDocuments], ["rust", rustDocuments],
]);
for (const [language, documents] of languages) {
  for (const other of languages.keys()) {
    if (other === language) continue;
    assert.equal(documents.some(({u}) => u.includes(`/reference/${other}`)), false,
      `${language} search leaked the ${other} API reference`);
  }
}
assert.match(rustText, /Rust 1\.85 or newer is required/);
assert.doesNotMatch(rustText, /MRI Ruby 3\.3 or newer is required/);
assert.doesNotMatch(rustText, /Python 3\.10 or newer is required/);
assert.ok(rustDocuments.some(({u}) => u.includes("/reference/rust")));
assert.ok(rustDocuments.some(({u}) => u.endsWith("/usage/compatibility")));

const allLanguages = ["python", "typescript", "go", "java", "csharp", "ruby", "rust", "php", "elixir", "haskell"];
const nativeGuidance = {
  python: "Collection coercion can retain the caller's list",
  typescript: "Factory serialization produces a wire snapshot",
  go: "A typed nil nested builder is a type error",
  java: "Varargs collection setters append",
  csharp: "Optional properties use nullable types",
  ruby: "Native values freeze their copied nested content",
  rust: "clone().into_builder()",
  php: "Readonly native values",
  elixir: "Model change/2 replaces supplied fields atomically",
  haskell: "Field records have no ToJSON",
};
for (const language of allLanguages) {
  const documents = await readDocuments(language);
  const blockGuide = documents
    .filter(({ u }) => u === "/slackblocks/usage/using_blocks")
    .map(({ t }) => t).join(" ").replace(/\s+/g, " ");
  for (const [owner, guidance] of Object.entries(nativeGuidance)) {
    assert.equal(blockGuide.toLowerCase().includes(guidance.toLowerCase()), owner === language,
      `${language} search has incorrect visibility for ${owner} value guidance`);
  }
  assert.equal(documents.some(({ u }) => u === "/slackblocks/usage/language-semantics"), false,
    `${language} still indexes the retired cross-language semantics page`);
  for (const other of allLanguages.filter((candidate) => candidate !== language)) {
    assert.equal(documents.some(({ u }) => u.includes(`/reference/${other}`)), false, `${language} leaked ${other} reference`);
  }
  if (language === "elixir") {
    const text = documents.map(({ t }) => t).join("\n");
    assert.ok(/Elixir 1\.16\.3\+/.test(text), "Elixir installation requirements absent from search");
    assert.ok(documents.some(({ u }) => u.includes("/reference/elixir/blocks")));
    assert.ok(documents.some(({ u }) => u.endsWith("/usage/erlang")));
    assert.doesNotMatch(text, /PHP 8\.2 or newer is required/);
  } else {
    assert.equal(documents.some(({ u }) => u.endsWith("/usage/erlang")), false);
  }
  if (language === "php") {
    const text = documents.map(({ t }) => t).join("\n");
    assert.match(text, /PHP 8\.2 or newer is required/);
    assert.ok(documents.some(({ u }) => u.includes("/usage/compatibility")));
    assert.ok(documents.some(({ u }) => u.includes("/reference/php/blocks")));
  }
}

const haskellDocuments = await readDocuments("haskell");
const haskellText = haskellDocuments.map(({ t }) => t).join("\n");
assert.match(haskellText, /minimum is GHC 9\.6\.7/);
assert.match(haskellText, /buildSectionBlock/);
assert.ok(haskellDocuments.some(({ u }) => u.includes("/reference/haskell/blocks")));
assert.ok(haskellDocuments.some(({ u }) => u.endsWith("/usage/compatibility")));
assert.doesNotMatch(haskellText, /PHP 8\.2 or newer is required/);
assert.doesNotMatch(haskellText, /Elixir 1\.16\.3 or newer/);
