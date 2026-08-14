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

function functionSection(contents, name) {
  const heading = `## ${name}()`;
  const start = contents.indexOf(heading);
  assert.notEqual(start, -1, `${heading} is missing from the rendered API`);
  const end = contents.indexOf("\n***", start);
  return contents.slice(start, end === -1 ? contents.length : end);
}

const blockFactories = [
  ...blocks.matchAll(/^## ([A-Za-z_$][\w$]*Block)\(\)$/gm),
];
assert.ok(blockFactories.length > 0, "No block factories were rendered");

for (const [, name] of blockFactories) {
  const section = functionSection(blocks, name);
  assert.match(
    section,
    /^### Input fields$/m,
    `${name} must show its object fields directly`,
  );
  assert.doesNotMatch(
    section,
    /^### Parameters$/m,
    `${name} must not group its object fields under a parameter`,
  );
  assert.match(
    section,
    /^### Settings$/m,
    `${name} must keep factory settings separate from its input fields`,
  );
}

const channelMultiSelect = functionSection(elements, "channelMultiSelect");
assert.match(
  channelMultiSelect,
  /^### Input fields$/m,
  "Named input interfaces must be expanded beside their factories",
);
assert.match(
  channelMultiSelect,
  /^\| `initialChannels\?` \|/m,
  "Expanded input tables must retain named interface fields",
);

assert.doesNotMatch(
  elements,
  /\| `input` \| \[`ActionInput`\]/,
  "Element factories must not render an opaque ActionInput parameter",
);

for (const property of [
  "initialChannels",
  "initialDateTime",
  "minQueryLength",
  "minLines",
  "maxLines",
]) {
  const requiredProperty = "`" + property;
  assert.ok(
    elements.includes(requiredProperty + "` |") ||
      elements.includes(requiredProperty + "?` |"),
    `${property} is missing from the rendered element input tables`,
  );
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
