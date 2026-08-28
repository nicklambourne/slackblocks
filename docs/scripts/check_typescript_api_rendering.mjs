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

function functionSection(contents, name) {
  const heading = `## ${name}()`;
  const start = contents.indexOf(heading);
  assert.notEqual(start, -1, `${heading} is missing from the rendered API`);
  const end = contents.indexOf("\n***", start);
  return contents.slice(start, end === -1 ? contents.length : end);
}

const fluentPages = [blocks, components, elements, objects, payloads];
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
    new RegExp("^> \\*\\*" + name + "\\*\\*&lt;`Type`&gt;", "m"),
    `${name}<Type> is not safely escaped in its declaration`,
  );
}

console.log("TypeScript API rendering checks passed.");
