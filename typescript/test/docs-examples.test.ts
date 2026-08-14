import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { expect, it } from "vitest";

import { payload } from "../../docs/examples/typescript/section_hello.js";

it("keeps the TypeScript docs example aligned with the JSON tab", () => {
  const expected = JSON.parse(
    readFileSync(resolve(import.meta.dirname, "../../docs/examples/section_hello.json"), "utf8"),
  );
  expect(payload).toEqual(expected);
});
