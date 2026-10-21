import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

// The version selector is rendered on the server for the default language, Python, which every
// released version documents. Each snapshot in versions.json must therefore be offered; a
// snapshot missing here is published but unreachable from the selector.
const versions = JSON.parse(
  await readFile(new URL("../versions.json", import.meta.url), "utf8"),
);
const html = await readFile(new URL("../build/index.html", import.meta.url), "utf8");
const select = html.match(/<select[^>]*class="docs-version-select[^"]*"[^>]*>[\s\S]*?<\/select>/)?.[0];
assert.ok(select, "index.html is missing the version selector");
const offered = new Set([...select.matchAll(/<option[^>]*value="([^"]+)"/g)].map((match) => match[1]));

for (const version of versions) {
  assert.ok(offered.has(version), `version ${version} has a docs snapshot but the selector does not offer it`);
}

console.log(`The version selector offers all ${versions.length} documentation snapshots.`);

// PHP starts in 2.7.0; frozen versions must not advertise it.
for (const version of versions.filter((value) => {
  const [major, minor] = value.split(".").map(Number);
  return major < 2 || (major === 2 && minor < 7);
})) {
  const sidebar = await readFile(new URL(`../versioned_sidebars/version-${version}-sidebars.json`, import.meta.url), "utf8");
  assert.doesNotMatch(sidebar, /reference\/php/, `${version} advertises an unpublished PHP implementation`);
}
const currentPhp = await readFile(new URL("../build/reference/php.html", import.meta.url), "utf8");
assert.match(currentPhp, /PHP API reference/);

// Elixir begins in 2.8.0; older source snapshots contain no Elixir panels/reference.
for (const version of versions.filter(value => Number(value.split(".")[0]) < 2 || (Number(value.split(".")[0]) === 2 && Number(value.split(".")[1]) < 8))) {
  const guide = await readFile(new URL(`../versioned_docs/version-${version}/index.mdx`, import.meta.url), "utf8");
  assert.doesNotMatch(guide, /<Elixir>|reference\/elixir/, `${version} gained Elixir content`);
}
const currentElixir = await readFile(new URL("../build/reference/elixir.html", import.meta.url), "utf8");
assert.match(currentElixir, /Elixir API reference/);
