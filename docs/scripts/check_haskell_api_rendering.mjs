import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const read = file => readFile(new URL(`../../${file}`, import.meta.url), "utf8");
const reference = JSON.parse(await read("docs/docs/reference/haskell/reference.json"));
const inventory = JSON.parse(await read("haskell/conformance/public-api.json"));
const model = JSON.parse(await read("spec/model.json"));
const expected = [...new Set(Object.values(inventory.exports).flat())].sort();
assert.deepEqual(reference.exports, inventory.exports);
assert.deepEqual(reference.entries.map(entry => entry.name), expected);
const byName = new Map(reference.entries.map(entry => [entry.name, entry]));
let fields = 0;
for (const type of model.types) {
  const name = inventory.models[type.name].type;
  const nativeName = name.split(".").at(-1);
  const prefix = nativeName[0].toLowerCase() + nativeName.slice(1);
  assert.ok(byName.get(name).doc.includes(type.description));
  for (const field of type.fields) {
    const selector = `${prefix}${field.method[0].toUpperCase()}${field.method.slice(1)}`;
    const entry = byName.get(`Slackblocks.Models.${selector}`);
    assert.ok(entry, `${type.name}.${field.wire}: missing native selector`);
    assert.match(entry.doc, field.required ? /Required\./ : /Optional; Nothing omits the field\. Factory default:/);
    if (field.limits) assert.ok(entry.doc.includes(field.limits));
    assert.ok(entry.signatures.some(signature => signature.includes("::")));
    fields++;
  }
}
for (const domain of ["blocks", "elements", "objects", "payloads", "components", "core", "errors"]) {
  const mdx = await read(`docs/docs/reference/haskell/${domain}.mdx`);
  const html = await read(`docs/build/reference/haskell/${domain}.html`);
  for (const entry of reference.entries.filter(entry => entry.domain === domain)) {
    assert.ok(entry.doc.trim() && entry.signatures.length, entry.name);
    assert.ok(mdx.includes(`{#${entry.anchor}}`), entry.name);
    assert.ok(html.includes(`id="${entry.anchor}"`), `${entry.name}: rendered anchor absent`);
    for (const signature of entry.signatures) {
      assert.ok(mdx.includes(signature), `${entry.name}: native signature absent`);
      assert.doesNotMatch(signature, /\.Internal\.|[a-z][a-z0-9-]*-\d[^\s:]*:/, `${entry.name}: private dependency name leaked`);
    }
  }
  assert.doesNotMatch(mdx, /\]\((?:javadoc|ref):/);
  const breadcrumbs = html.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
  assert.ok(breadcrumbs);
  assert.doesNotMatch(breadcrumbs, />Haskell API reference</);
}
for (const name of ["paginate", "accordionSection", "workflowFromURL", "blockKitBuilderURL", "rgbColor"]) {
  assert.ok(byName.get(`Slackblocks.Helpers.${name}`).doc.length > 40);
}
assert.match(byName.get("Slackblocks.Json.decode").doc, /duplicate keys/);
assert.match(byName.get("Slackblocks.Models.buildPlanTask").doc, /pending/);
console.log(`Haskell reference: ${expected.length} compiled public symbols, ${fields} field descriptions/defaults and every rendered anchor verified.`);
