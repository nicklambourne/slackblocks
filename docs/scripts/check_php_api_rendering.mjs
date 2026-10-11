import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
const directory = path.dirname(fileURLToPath(import.meta.url));
const domains = ["blocks", "elements", "objects", "payloads", "components", "core", "errors"];
const pages = new Map(await Promise.all(domains.map(async (domain) => [domain, await readFile(path.resolve(directory, `../docs/reference/php/${domain}.mdx`), "utf8")])));
const text = [...pages.values()].join("\n");
const inventory = JSON.parse(execFileSync("php", [path.resolve(directory, "../../php/bin/api-inventory.php")], { encoding: "utf8" }));
const reference = JSON.parse(execFileSync("php", [path.resolve(directory, "php_reference_adapter.php")], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 }));
function meaningful(description, location) {
  assert.ok(description.trim().length > 1, `Missing documentation: ${location}`);
  assert.doesNotMatch(description, /(?:\/\*|\*\/|\/$|^(?:Optional|Required)\.$)/, `Malformed or placeholder documentation: ${location}`);
}
for (const type of reference.types) {
  meaningful(type.doc, type.name);
  const api = inventory[type.name];
  const expected = [...Object.keys(api.properties).map((name) => `$${name}`), ...Object.keys(api.methods)];
  assert.deepEqual(type.members.map((member) => member.name).sort(), expected.sort(), `Public members missing: ${type.name}`);
  for (const member of type.members) {
    const location = `${type.name}.${member.name}`;
    meaningful(member.doc, location);
    for (const parameter of member.params) meaningful(parameter.doc, `${location}(${parameter.name})`);
    if (/public readonly (?:\??array|array\|null) \$/.test(member.signature)) {
      assert.match(member.signature, /@var list</, `Missing collection element type: ${location}`);
    }
    for (const thrown of member.throws) meaningful(thrown.doc, `${location} throws ${thrown.type}`);
  }
  for (const constant of type.constants) meaningful(constant.doc, `${type.name}::${constant.name}`);
}
for (const [type, method] of [["Color", "hex"], ["JsonObject", "fromJson"], ["Value", "fromJson"], ["Value", "with"], ["Paginator", "create"]]) {
  const member = reference.types.find((entry) => entry.name === type).members.find((entry) => entry.name === method);
  assert.ok(member.throws.some((entry) => entry.type === "ValidationError"), `Lost @throws for ${type}.${method}`);
}
const headings = [...text.matchAll(/^## (\w+)$/gm)].map((match) => match[1]);
assert.deepEqual(headings.sort(), Object.keys(inventory).sort(), "Every independently discovered public PHP type must appear once");
assert.match(pages.get("blocks"), /### __construct/);
assert.match(pages.get("blocks"), /public readonly (?:\?Text|Text\|null) \$text/);
assert.match(pages.get("core"), /### with/);
assert.match(pages.get("core"), /### fromJson/);
assert.match(pages.get("components"), /## Paginator/);
assert.match(pages.get("errors"), /## ErrorCategory/);
assert.match(text, /\| Throws \| When \|/);
assert.match(text, /\| `Primary` \| `'primary'` \|/);
assert.doesNotMatch(text, /\]\((?:ref|javadoc):/);
for (const [, domain, anchor] of text.matchAll(/\]\(\/reference\/php\/([a-z]+)#([a-z0-9_]+)\)/g)) {
  assert.ok(pages.has(domain));
  assert.match(pages.get(domain).toLowerCase(), new RegExp(`^## ${anchor}$`, "m"));
}
const html = await readFile(path.resolve(directory, "../build/reference/php/blocks.html"), "utf8");
assert.match(html, /id="sectionblock"/);
const breadcrumbs = html.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
assert.ok(breadcrumbs);
assert.doesNotMatch(breadcrumbs, />PHP API reference</);
console.log("PHP native reference completeness, links and rendered breadcrumbs passed.");
