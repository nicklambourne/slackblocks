import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeReferencePages } from "./reference_pages.mjs";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const adapter = path.join(scriptsDirectory, "ruby_reference_adapter.py");
const reference = JSON.parse(execFileSync("python3", [adapter], { encoding: "utf8" }));

const domains = [
  { slug: "blocks", title: "Blocks", position: 1, package: "block" },
  { slug: "elements", title: "Elements", position: 2, package: "element" },
  { slug: "objects", title: "Composition Objects", position: 3, package: "object" },
  { slug: "payloads", title: "Payloads And Views", position: 4, package: "payload" },
  { slug: "components", title: "Components", position: 5, package: "component" },
  { slug: "core", title: "Core Types", position: 6, package: "core" },
  { slug: "errors", title: "Validation And Errors", position: 7, package: "error" },
];

const index = [
  "---",
  "sidebar_position: 0",
  "---",
  "",
  "# Ruby API reference",
  "",
  "Public Ruby constructors, fields, helpers, and errors are generated from the gem's model-derived naming and type metadata.",
  "",
  "- [Blocks](/reference/ruby/blocks)",
  "- [Elements](/reference/ruby/elements)",
  "- [Composition Objects](/reference/ruby/objects)",
  "- [Payloads And Views](/reference/ruby/payloads)",
  "- [Components](/reference/ruby/components)",
  "- [Core Types](/reference/ruby/core)",
  "- [Validation And Errors](/reference/ruby/errors)",
  "",
].join("\n");

const count = await writeReferencePages({
  reference,
  domains,
  domainFor: (type) => domains.find((domain) => domain.package === type.package),
  language: "ruby",
  fence: "ruby",
  outputRoot: path.join(repository, "docs/docs/reference/ruby"),
  introduction: (domain) => "This page documents the Ruby API for " + domain.title.toLowerCase() +
    ". Values use keyword-only construction, validate before freezing, and serialize to Slack JSON.\n\n",
  index,
  preserveMemberNames: true,
});
console.log("Generated " + count + " Ruby API types from package reference metadata.");
