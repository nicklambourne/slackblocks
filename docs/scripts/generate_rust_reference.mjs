// The model owns Rust naming/type resolution. Syn reads actual public signatures
// and rustdoc comments, including handwritten helpers; no Rust source regex parser.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeReferencePages, anchor } from "./reference_pages.mjs";
const repository=path.resolve(path.dirname(fileURLToPath(import.meta.url)),"../..");
const metadata=JSON.parse(await readFile(path.join(repository,"rust/generated/reference.json"),"utf8"));
const inventory=JSON.parse(await readFile(path.join(repository,"rust/conformance/api-inventory.json"),"utf8"));
const reference=JSON.parse(execFileSync("cargo",["+stable","run","--quiet","--locked","--manifest-path",path.join(repository,"rust/Cargo.toml"),"-p","slackblocks-conformance","--bin","api_reference"],{encoding:"utf8",maxBuffer:16*1024*1024}));
assert.deepEqual(reference.types.map(t=>t.name).sort(),Object.keys(inventory).sort());
const domains=[
  {slug:"blocks",title:"Blocks",position:1},
  {slug:"elements",title:"Elements",position:2},
  {slug:"objects",title:"Composition Objects",position:3},
  {slug:"payloads",title:"Payloads and Views",position:4},
  {slug:"components",title:"Components and Utilities",position:5},
  {slug:"core",title:"Core Types",position:6},
  {slug:"errors",title:"Validation and Errors",position:7},
];
const byName=new Map();
for(const t of metadata.types) {
  const domain={block:"blocks",element:"elements",object:"objects",payload:"payloads"}[t.package];
  assert.ok(domain,`Unmapped Rust package ${t.package}`);
  byName.set(t.rustName,domain);byName.set(`${t.rustName}Builder`,domain);
}
for(const name of ["Accordion","AccordionSection","AccordionSectionBuilder","Paginator","PaginatorBuilder","AttachmentColor","BuilderPayload","block_kit_builder_url"]) byName.set(name,"components");
for(const name of ["ValidationError","ErrorCategory"]) byName.set(name,"errors");
for(const name of ["JsonNumber","RichTextStyle","PlainTextInput","TextInput","VERSION","SPEC_VERSION",...Object.keys(metadata.roles)]) byName.set(name,"core");
const model=JSON.parse(await readFile(path.join(repository,"spec/model.json"),"utf8"));
for(const e of model.enums)byName.set(e.name,"core");
assert.deepEqual([...byName.keys()].sort(),Object.keys(inventory).sort(),"Every Rust export needs an explicit reference domain");
function clean(text) {
  return text.replace(/(```[\s\S]*?```|`[^`]*`)|[{}<>]/g, (match, code) => code ?? ({"{": "&#123;", "}": "&#125;", "<": "&lt;", ">": "&gt;"}[match]))
    .replace(/^#+ (.+)$/gm,"**$1**")
    .replace(/\[(`[^`]+`)\]/g,(_,label)=>{
      const name=label.slice(1,-1).split("::")[0];
      return byName.has(name)?`[${label}](/reference/rust/${byName.get(name)}#${anchor(name)})`:label;
    });
}
for(const type of reference.types) {
  type.doc=clean(type.doc);
  for(const member of type.members)member.doc=clean(member.doc);
  for(const constant of type.constants)constant.doc=clean(constant.doc);
  const e=model.enums.find(e=>e.name===type.name);
  if(e)type.constants=e.constants.map(c=>({name:c.wire.split("_").map(w=>w[0].toUpperCase()+w.slice(1)).join(""),wire:c.wire,doc:c.description}));
}
const index=`---
sidebar_position: 0
---

# Rust API reference

Rust 2024 with MSRV 1.85. Values own their data and expose borrowed getters.
Consuming builders return \`Result<T, ValidationError>\`; \`Clone\` and
\`into_builder()\` support safe editing. Every modeled wire value implements
Serde serialization and checked deserialization. JSON ingress is also available
through \`TryFrom<serde_json::Value>\`.

This is the unreleased 2.6.0 implementation; see [installation](/usage/installation)
for use from a checkout. Run \`cargo doc --manifest-path rust/Cargo.toml --no-deps\`
for the full native rustdoc reference, including trait implementations.

${domains.map(d=>`- [${d.title}](/reference/rust/${d.slug})`).join("\n")}
`;
const count=await writeReferencePages({reference,domains,domainFor:t=>domains.find(d=>d.slug===byName.get(t.name)),language:"rust",fence:"rust",outputRoot:path.join(repository,"docs/docs/reference/rust"),introduction:()=>"Generated from the actual Rust signatures and rustdoc comments. Builders validate on `build()`; `into_builder()` preserves omissions. Error categories and paths are stable; prose may improve.\n\n",index,preserveMemberNames:true,constantValueLabel:"Wire value or payload type"});
console.log(`Generated ${count} Rust exports from public syntax and rustdoc comments.`);
