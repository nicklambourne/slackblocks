import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { expect, it } from "vitest";

import * as slackblocks from "@nicklambourne/slackblocks";

import { payload } from "../../docs/examples/typescript/section_hello.js";

// Block sections in using_blocks.mdx whose TypeScript snippet cannot be
// executed and compared against one JSON tab by this harness. Higher-level
// components have multiple examples and variable output rather than one block
// plus one JSON tab; their payloads are covered by components.test.ts.
const EXCLUDED_SECTIONS = new Set(["Higher-Level Components"]);

const usingBlocksPath = resolve(
  import.meta.dirname,
  "../../docs/docs/usage/using_blocks.mdx",
);

type BlockSection = { title: string; code: string; expected: unknown };

function blockSections(): BlockSection[] {
  const contents = readFileSync(usingBlocksPath, "utf8");
  const headings = [...contents.matchAll(/^## (.+)$/gm)];
  const sections: BlockSection[] = [];
  headings.forEach((heading, index) => {
    const title = heading[1] ?? "";
    if (EXCLUDED_SECTIONS.has(title)) return;
    const body = contents.slice(
      heading.index + heading[0].length,
      headings[index + 1]?.index ?? contents.length,
    );
    const code = body.match(/```ts\n([\s\S]*?)```/)?.[1];
    const expected = body.match(/```json\n([\s\S]*?)```/)?.[1];
    if (code && expected) {
      sections.push({ title, code, expected: JSON.parse(expected) });
    }
  });
  return sections;
}

function evaluateSnippet(code: string): unknown {
  const importStatement = /import\s*\{([\s\S]*?)\}\s*from\s*"@nicklambourne\/slackblocks";\s*/g;
  const names = [...code.matchAll(importStatement)].flatMap((statement) =>
    (statement[1] ?? "")
      .split(",")
      .map((name) => name.trim())
      .filter(Boolean),
  );
  const body = code.replace(importStatement, "").trim().replace(/;$/, "");
  const factory = new Function(...names, `return (${body});`);
  return factory(
    ...names.map((name) => (slackblocks as Record<string, unknown>)[name]),
  );
}

it("keeps the TypeScript docs example aligned with the JSON tab", () => {
  const expected = JSON.parse(
    readFileSync(resolve(import.meta.dirname, "../../docs/examples/section_hello.json"), "utf8"),
  );
  expect(payload).toEqual(expected);
});

it("covers every block section in the using-blocks guide", () => {
  const titles = [...readFileSync(usingBlocksPath, "utf8").matchAll(/^## (.+)$/gm)]
    .map((heading) => heading[1] ?? "")
    .filter((title) => !EXCLUDED_SECTIONS.has(title));
  expect(blockSections().map(({ title }) => title)).toEqual(titles);
});

for (const { title, code, expected } of blockSections()) {
  it(`keeps the ${title} snippet aligned with the JSON tab`, () => {
    expect(evaluateSnippet(code)).toEqual(expected);
  });
}
