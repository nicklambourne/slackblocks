import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const referenceDirectory = path.resolve(
  scriptsDirectory,
  "../docs/reference/python",
);
const files = (await readdir(referenceDirectory)).filter((file) =>
  file.endsWith(".mdx"),
);
const pages = new Map(
  await Promise.all(
    files.map(async (file) => [
      file.slice(0, -4),
      await readFile(path.join(referenceDirectory, file), "utf8"),
    ]),
  ),
);
const rendered = [...pages.values()].join("\n");

for (const [domain, name] of [
  ["elements", "Element"],
  ["objects", "TextLike"],
]) {
  const target = pages.get(domain);
  assert.ok(target, `${domain}.mdx is missing`);
  assert.match(target, new RegExp(`^## ${name}$`, "m"), `${name} lacks an API section`);
  assert.match(
    rendered,
    new RegExp(
      `<a href="/reference/python/${domain}#${name.toLowerCase()}">${name}</a>`,
    ),
    `${name} is not linked from argument types`,
  );
}

assert.match(
  pages.get("objects"),
  /^TextLike = str \| Text$/m,
  "TextLike must show its useful alias declaration",
);

for (const match of rendered.matchAll(
  /<a href="\/reference\/python\/([a-z_]+)#([a-z0-9_]+)">([^<]+)<\/a>/g,
)) {
  const [, domain, anchor, name] = match;
  const target = pages.get(domain);
  assert.ok(target, `${name} links to missing ${domain}.mdx`);
  assert.match(
    target,
    new RegExp(`^## ${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`, "m"),
    `${name} links to missing #${anchor} in ${domain}.mdx`,
  );
}

console.log("Python API rendering checks passed.");
