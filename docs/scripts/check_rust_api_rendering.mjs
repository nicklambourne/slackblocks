import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
const base=new URL("../docs/reference/rust/",import.meta.url);
const inventory=JSON.parse(await readFile(new URL("../../rust/conformance/api-inventory.json",import.meta.url),"utf8"));
const pages=new Map(await Promise.all((await readdir(base)).filter(f=>f.endsWith(".mdx")&&f!=="index.mdx").map(async file=>[file.slice(0,-4),await readFile(new URL(file,base),"utf8")])));
const source=[...pages.values()].join("\n");
const headings=[...source.matchAll(/^## ([A-Za-z_][A-Za-z0-9_]*)$/gm)].map(m=>m[1]);
assert.equal(headings.length,Object.keys(inventory).length,"Every public Rust export appears exactly once");
assert.deepEqual(headings.sort(),Object.keys(inventory).sort());
for(const [domain,heading] of [["blocks","SectionBlockBuilder"],["elements","ButtonElement"],["components","block_kit_builder_url"],["components","Paginator"],["core","JsonNumber"],["errors","ValidationError"]]) {
  assert.match(pages.get(domain),new RegExp(`^## ${heading}$`,"m"));
}
assert.match(pages.get("blocks"),/### into_builder/);
assert.match(pages.get("blocks"),/fn text\(mut self, value: impl Into<TextInput>\)/);
assert.match(pages.get("blocks"),/\*\*Errors\*\*/);
assert.doesNotMatch(source,/\]\((?:ref|javadoc):/);
for(const [,domain,anchor] of source.matchAll(/\]\(\/reference\/rust\/([a-z]+)#([a-z0-9_]+)\)/g)) {
  const target=pages.get(domain);assert.ok(target,`Missing reference domain ${domain}`);
  assert.ok([...target.matchAll(/^## (.+)$/gm)].some(m=>m[1].toLowerCase().replace(/[^a-z0-9_]+/g,"")===anchor),`Missing ${domain}#${anchor}`);
}
for (const [domain, page] of pages) {
  const rendered = await readFile(new URL(`../build/reference/rust/${domain}.html`, import.meta.url), "utf8");
  for (const [, name] of page.matchAll(/^## (.+)$/gm)) {
    assert.ok(rendered.includes(`id="${name.toLowerCase()}"`), `Missing rendered anchor ${domain}#${name.toLowerCase()}`);
  }
}
const html=await readFile(new URL("../build/reference/rust/blocks.html",import.meta.url),"utf8");
const breadcrumbs=html.match(/<nav[^>]+aria-label="Breadcrumbs">[\s\S]*?<\/nav>/)?.[0];
assert.ok(breadcrumbs,"Missing Rust breadcrumbs");assert.doesNotMatch(breadcrumbs,/>Rust API reference</);
assert.match(html,/id="sectionblock"/);
console.log(`Rust API rendering covers ${headings.length} actual public exports and native signatures.`);
