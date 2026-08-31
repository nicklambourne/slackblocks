import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const referenceDirectory = path.resolve(
  scriptsDirectory,
  "../docs/reference/typescript",
);
const referenceFiles = (await readdir(referenceDirectory)).filter((file) =>
  file.endsWith(".md"),
);

for (const file of referenceFiles) {
  const contents = await readFile(path.join(referenceDirectory, file), "utf8");
  assert.doesNotMatch(
    contents,
    /\\[<>]/,
    `${file} contains a backslash-escaped generic delimiter`,
  );
}

const utilities = await readFile(
  path.join(referenceDirectory, "utilities.md"),
  "utf8",
);
const elements = await readFile(
  path.join(referenceDirectory, "elements.md"),
  "utf8",
);
const blocks = await readFile(
  path.join(referenceDirectory, "blocks.md"),
  "utf8",
);
const components = await readFile(
  path.join(referenceDirectory, "components.md"),
  "utf8",
);
const payloads = await readFile(
  path.join(referenceDirectory, "payloads.md"),
  "utf8",
);
const objects = await readFile(
  path.join(referenceDirectory, "objects.md"),
  "utf8",
);

const errors = await readFile(
  path.join(referenceDirectory, "errors.md"),
  "utf8",
);
const blocksHtml = await readFile(
  path.resolve(scriptsDirectory, "../build/reference/typescript/blocks.html"),
  "utf8",
);

function functionSection(contents, name) {
  const heading = `## ${name}()`;
  const start = contents.indexOf(heading);
  assert.notEqual(start, -1, `${heading} is missing from the rendered API`);
  const end = contents.indexOf("\n***", start);
  return contents.slice(start, end === -1 ? contents.length : end);
}

const fluentPages = [blocks, components, elements, objects, payloads];
const apiPages = [...fluentPages, utilities, errors];

assert.doesNotMatch(
  apiPages.join("\n"),
  /^> /m,
  "TypeScript signatures and declarations must not render as blockquotes",
);
assert.doesNotMatch(
  apiPages.join("\n"),
  /^#{2,3} See$/m,
  "Slack reference links must remain inline instead of creating See headings",
);
assert.doesNotMatch(
  fluentPages.join("\n"),
  /^(?:Starts a fluent|Fields accepted by|Optional behavior for)\b/m,
  "Public fluent API entries must use descriptive, reader-facing prose",
);
for (const [page, pattern, domain] of [
  [blocks, /holds interactive controls such as buttons, select/, "blocks"],
  [elements, /can submit an action, open a URL/, "elements"],
  [objects, /core text run used by Slack's structured rich-text API/, "objects"],
  [payloads, /message payload for Slack Web API methods/, "payloads"],
]) {
  assert.match(page, pattern, `${domain} must render its expanded API descriptions`);
}
assert.match(
  blocks,
  /^```ts\nfunction ActionsBlock\(\): FluentBuilder</m,
  "Function signatures must render as TypeScript code blocks",
);
assert.match(
  errors,
  /^```ts\nnew InvalidUsageError\(path, message\): InvalidUsageError;\n```$/m,
  "Constructor signatures must render as TypeScript code blocks",
);
assert.match(
  objects,
  /^```ts\ntype TextLike = string \| TextObject;\n```$/m,
  "Type aliases must render as TypeScript code blocks",
);

const toc = blocksHtml.match(
  /<ul class="table-of-contents table-of-contents__left-border">[\s\S]*?<\/ul>/,
)?.[0];
const breadcrumbs = blocksHtml.match(
  /<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/,
)?.[0];

assert.ok(toc, "blocks.html is missing its right-side table of contents");
assert.doesNotMatch(
  toc,
  />[A-Z][A-Za-z0-9_$]*\(\)<\//,
  "TypeScript factory names in the right-side navigation must omit parentheses",
);
assert.ok(breadcrumbs, "blocks.html is missing its breadcrumbs");
assert.doesNotMatch(
  breadcrumbs,
  />TypeScript API reference</,
  "TypeScript API breadcrumbs must omit the redundant language layer",
);
assert.match(
  blocksHtml,
  /<a href="https:\/\/github\.com\/nicklambourne">Nicholas Lambourne<\/a>/,
  "The footer author credit must link to the GitHub profile",
);

const blockFactories = [...blocks.matchAll(/^## ([A-Z][\w$]*Block)\(\)$/gm)];
assert.ok(blockFactories.length > 0, "No block factories were rendered");

for (const [, name] of blockFactories) {
  const section = functionSection(blocks, name);
  assert.match(
    section,
    /^### Chainable setters$/m,
    `${name} must show its fluent setters directly`,
  );
  assert.doesNotMatch(
    section,
    /^### Input fields$/m,
    `${name} must not document the retired object-input convention`,
  );
  assert.match(
    section,
    /^### Validation and errors$/m,
    `${name} must document build-time validation`,
  );
}

const channelMultiSelect = functionSection(elements, "ChannelMultiSelect");
assert.match(
  channelMultiSelect,
  /^### Chainable setters$/m,
  "Named input interfaces must be expanded beside their fluent builders",
);
assert.match(
  channelMultiSelect,
  /^\| `\.initialChannels\(\.\.\.values\)` \|/m,
  "Fluent setter tables must retain named interface fields",
);

assert.doesNotMatch(
  fluentPages.join("\n"),
  /^## [a-z][A-Za-z0-9_$]*\(\)$/m,
  "The fluent reference must not render legacy lowercase factories",
);

for (const property of [
  "initialChannels",
  "initialDateTime",
  "minQueryLength",
  "minLines",
  "maxLines",
]) {
  assert.ok(
    elements.includes("`." + property + "("),
    `${property} is missing from the rendered element setter tables`,
  );
}

for (const contents of fluentPages) {
  for (const [, name] of contents.matchAll(/^## ([A-Z][\w$]*)\(\)$/gm)) {
    const section = functionSection(contents, name);
    assert.match(section, /^### Chainable setters$/m, `${name} lacks setter details`);
    assert.match(
      section,
      /^### Validation and errors$/m,
      `${name} lacks validation details`,
    );
  }
}

for (const name of ["SlackObject", "SlackWire"]) {
  const anchor = name.toLowerCase();
  assert.match(
    utilities,
    new RegExp(`^## ${name}&lt;Type&gt; \\{#${anchor}\\}$`, "m"),
    `${name}<Type> is missing from the Utilities headings`,
  );
  assert.match(
    utilities,
    new RegExp("^```ts\\ntype " + name + "<Type> =", "m"),
    `${name}<Type> is not rendered as valid TypeScript`,
  );
}

for (const [page, name] of [
  [blocks, "AlertLevel"],
  [blocks, "ContainerWidth"],
  [blocks, "TaskStatus"],
  [objects, "RichTextStyle"],
  [objects, "SlackIconName"],
  [objects, "TextLike"],
  [utilities, "JsonObject"],
]) {
  assert.match(page, new RegExp(`^## ${name}$`, "m"), `${name} lacks an API section`);
  const link = `/reference/typescript/${
    page === blocks ? "blocks" : page === objects ? "objects" : "utilities"
  }#${name.toLowerCase()}`;
  assert.match(
    fluentPages.join("\n"),
    new RegExp(`<a href="${link}">${name}</a>`),
    `${name} is not linked from fluent setter types`,
  );
}

console.log("TypeScript API rendering checks passed.");
