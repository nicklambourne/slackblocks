import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const read = file => readFile(path.join(root, file), "utf8");
const reference = JSON.parse(await read("docs/docs/reference/elixir/reference.json"));
const inventory = JSON.parse(await read("elixir/design/public_api.json")).modules;
const models = JSON.parse(await read("spec/model.json")).types;
const mapping = JSON.parse(await read("elixir/design/parity.json")).models;
const domains = ["blocks", "elements", "objects", "payloads", "components", "core", "errors"];
const pages = await Promise.all(domains.map(domain => read(`docs/docs/reference/elixir/${domain}.mdx`)));
const text = pages.join("\n");
const expected = Object.entries(inventory).filter(([, entry]) => !["internal", "protocol"].includes(entry.kind)).map(([name]) => name).sort();
assert.deepEqual(reference.modules.map(entry => entry.module).sort(), expected);
assert.equal(expected.length, 97);
assert.deepEqual([...text.matchAll(/^## (Slackblocks\S*) \{#/gm)].map(match => match[1]).sort(), expected);
// Compiler-generated struct/exception functions and these explicitly internal
// Style receiving-context helpers have @doc false; they are not consumer APIs.
const hidden = {
  "Slackblocks.RichText.Style": ["parse/2", "wire/3"],
  "Slackblocks.ValidationError": ["exception/1", "message/1"],
};
for (const module of reference.modules) {
  assert.ok(module.doc.trim(), `Missing module docs: ${module.module}`);
  const functions = module.functions.flatMap(member => member.arities.map(arity => `${member.name.split("/")[0]}/${arity}`));
  const publicExports = module.exports.filter(name => !name.startsWith("__struct__/") && !hidden[module.module]?.includes(name));
  assert.deepEqual(functions.sort(), publicExports.sort(), `Undocumented arity: ${module.module}`);
  assert.deepEqual(module.exports.slice().sort(), inventory[module.module].functions.slice().sort(), `Export inventory drift: ${module.module}`);
  for (const member of module.functions) {
    assert.ok(member.doc.trim(), `${module.module}.${member.name} missing docs`);
    assert.ok(member.signatures.length && member.spec.length, `${module.module}.${member.name} missing native signature/typespec`);
  }
}
let fields = 0;
for (const model of models) {
  const module = reference.modules.find(entry => entry.module === mapping[model.name]);
  assert.ok(module, model.name);
  assert.ok(module.types.some(type => type.startsWith("@type t() :: %")), `${model.name}: native struct type missing`);
  for (const field of model.fields) {
    fields++;
    assert.ok(module.doc.includes(`* \`${field.wire}\` — ${field.description}`), `${model.name}.${field.wire}: field prose missing`);
    assert.ok(module.types.some(type => type.includes(`${field.wire}:`)), `${model.name}.${field.wire}: field type missing`);
  }
}
assert.match(text, /Default: `true`/);
assert.match(text, /Optional; `nil` omits the field/);
assert.match(text, /@spec change\(/);
assert.match(text, /Decimal support|Decimal decoder/);
assert.doesNotMatch(text, /javadoc:|ref:/);
for (const [index, domain] of domains.entries()) {
  const html = await read(`docs/build/reference/elixir/${domain}.html`);
  for (const [, anchor] of pages[index].matchAll(/\{#([a-z0-9_-]+)\}/g)) assert.ok(html.includes(`id="${anchor}"`), `Missing rendered anchor: ${anchor}`);
  const breadcrumbs = html.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
  assert.ok(breadcrumbs);
  assert.doesNotMatch(breadcrumbs, />Elixir API reference</);
}
console.log(`Elixir reference: all ${expected.length} public modules, ${fields} model fields, exported arities, native specs and rendered anchors verified.`);
