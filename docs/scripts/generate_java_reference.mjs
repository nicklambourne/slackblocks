// Generates the Java API reference with the javadoc tool.
//
// javadoc parses java/src/main/java, resolves types and {@link} references, and runs
// java-reference/ReferenceDoclet.java, which writes the public API as JSON. This script renders
// that JSON as one MDX page per API area. It needs a JDK (17+) on PATH or in JAVA_HOME; Maven
// resolves the library's compile classpath through the checked-in wrapper.

import { execFileSync } from "node:child_process";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const javaRoot = path.join(repository, "java");
const outputRoot = path.join(repository, "docs/docs/reference/java");
const workDirectory = path.join(javaRoot, "target/reference");
const basePackage = "io.github.nicklambourne.slackblocks";

const domains = [
  { slug: "blocks", title: "Blocks", position: 1, packages: ["block"] },
  { slug: "elements", title: "Elements", position: 2, packages: ["element"] },
  { slug: "objects", title: "Composition Objects", position: 3, packages: ["object"] },
  { slug: "payloads", title: "Payloads And Views", position: 4, packages: ["payload"] },
  { slug: "components", title: "Components", position: 5, packages: ["component"] },
  { slug: "core", title: "Core Types", position: 6, packages: [""], names: ["Buildable", "SlackObject", "Slackblocks", "SlackblocksJson"] },
  { slug: "errors", title: "Validation And Errors", position: 7, packages: [""], names: ["ErrorCategory", "ValidationException"] },
];

function tool(name) {
  const home = process.env.JAVA_HOME;
  return home ? path.join(home, "bin", name) : name;
}

function run(command, args, options = {}) {
  try {
    return execFileSync(command, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], ...options });
  } catch (error) {
    const detail = [error.stdout, error.stderr].filter(Boolean).join("\n");
    throw new Error(`${path.basename(command)} ${args.join(" ")} failed. A JDK 17 or newer must be on PATH or in JAVA_HOME.\n${detail}`);
  }
}

async function javadocJson() {
  await rm(workDirectory, { recursive: true, force: true });
  await mkdir(workDirectory, { recursive: true });
  const classpathFile = path.join(workDirectory, "classpath.txt");
  run(path.join(javaRoot, process.platform === "win32" ? "mvnw.cmd" : "mvnw"), [
    "-B", "-ntp", "-q", "dependency:build-classpath",
    `-Dmdep.outputFile=${classpathFile}`, "-Dmdep.includeScope=compile",
  ], { cwd: javaRoot });
  const classpath = (await readFile(classpathFile, "utf8")).trim();
  const docletClasses = path.join(workDirectory, "doclet");
  run(tool("javac"), ["-d", docletClasses, path.join(scriptsDirectory, "java-reference/ReferenceDoclet.java")]);
  const output = path.join(workDirectory, "reference.json");
  run(tool("javadoc"), [
    "-quiet",
    "-doclet", "ReferenceDoclet",
    "-docletpath", docletClasses,
    "-classpath", classpath,
    "-sourcepath", path.join(javaRoot, "src/main/java"),
    "-subpackages", basePackage,
    "-exclude", `${basePackage}.internal`,
    "-o", output,
  ]);
  return JSON.parse(await readFile(output, "utf8"));
}

function anchor(name) {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, "");
}

function domainFor(type) {
  const relative = type.package === basePackage ? "" : type.package.slice(basePackage.length + 1);
  return domains.find((domain) => domain.packages.includes(relative) && (!domain.names || domain.names.includes(type.name)));
}

function resolveLinks(markdown, typeLinks) {
  return markdown.replace(/\]\(javadoc:(\w+)\)/g, (_, name) => (typeLinks.has(name) ? `](${typeLinks.get(name)})` : "]()"))
    .replace(/\[(`[^`]*`)\]\(\)/g, "$1");
}

function heading(member, overloaded) {
  const base = member.name.replace(/^[a-z]/, (letter) => letter.toUpperCase());
  if (!overloaded) return base;
  return `${base} — ${member.parameterTypes ? `\`${member.parameterTypes}\`` : "no arguments"}`;
}

function relatedTypes(signature, typeLinks) {
  return [...typeLinks.keys()]
    .filter((name) => new RegExp(`\\b${name}\\b`).test(signature))
    .sort();
}

function memberSection(member, typeLinks, overloaded) {
  let output = `### ${heading(member, overloaded)}\n\n`;
  if (member.doc) output += `${resolveLinks(member.doc, typeLinks)}\n\n`;
  output += `\`\`\`java\n${member.signature}\n\`\`\`\n\n`;
  if (member.params.length) {
    output += "| Parameter | Description |\n| --- | --- |\n";
    for (const parameter of member.params) {
      output += `| \`${parameter.name}\` | ${resolveLinks(parameter.doc, typeLinks).replace(/\n+/g, " ")} |\n`;
    }
    output += "\n";
  }
  if (member.returns) output += `**Returns:** ${resolveLinks(member.returns, typeLinks)}\n\n`;
  if (member.throws.length) {
    output += "| Throws | When |\n| --- | --- |\n";
    for (const thrown of member.throws) {
      const type = typeLinks.has(thrown.type) ? `[\`${thrown.type}\`](${typeLinks.get(thrown.type)})` : `\`${thrown.type}\``;
      output += `| ${type} | ${resolveLinks(thrown.doc, typeLinks).replace(/\n+/g, " ")} |\n`;
    }
    output += "\n";
  }
  const related = relatedTypes(member.signature, typeLinks);
  if (related.length) {
    output += `**Related types:** ${related.map((name) => `[\`${name}\`](${typeLinks.get(name)})`).join(", ")}\n\n`;
  }
  return output;
}

const reference = await javadocJson();
const byDomain = new Map(domains.map((domain) => [domain.slug, []]));
for (const type of reference.types) {
  const domain = domainFor(type);
  if (domain) byDomain.get(domain.slug).push(type);
}
const typeLinks = new Map();
for (const domain of domains) {
  for (const type of byDomain.get(domain.slug)) {
    typeLinks.set(type.name, `/reference/java/${domain.slug}#${anchor(type.name)}`);
  }
}

await rm(outputRoot, { recursive: true, force: true });
await mkdir(outputRoot, { recursive: true });

for (const domain of domains) {
  const types = byDomain.get(domain.slug).sort((a, b) => a.name.localeCompare(b.name));
  let output = `---\nsidebar_position: ${domain.position}\ntoc_max_heading_level: 3\n---\n\n# ${domain.title}\n\n`;
  output += `This page documents the public Java API for ${domain.title.toLowerCase()}, generated from the Javadoc by the javadoc tool. Values are immutable, builders are concrete and fluent, validation runs when you call \`.build()\`, and typed getters read built values back.\n\n`;
  if (domain.slug === "blocks") {
    output += "Built block values implement Slack's official `LayoutBlock` interface, so they can be passed directly to `ChatPostMessageRequest.blocks(...)` and other slack-java-sdk request builders.\n\n";
  }
  if (domain.slug === "components") {
    output += "Components expand to ordinary `List<Block>` values. The application remains responsible for channel selection, delivery options, and interaction handling through slack-java-sdk.\n\n";
  }
  for (const type of types) {
    output += `## ${type.name}\n\n`;
    output += `${resolveLinks(type.doc || `Public ${type.name} API.`, typeLinks)}\n\n`;
    for (const see of type.see) output += `See the [${see.label}](${see.url}).\n\n`;
    if (type.constants.length) {
      output += "| Constant | Slack value | Meaning |\n| --- | --- | --- |\n";
      for (const constant of type.constants) {
        output += `| \`${constant.name}\` | \`${constant.wire}\` | ${resolveLinks(constant.doc, typeLinks)} |\n`;
      }
      output += "\n";
    }
    const counts = new Map();
    for (const member of type.members) counts.set(member.name, (counts.get(member.name) ?? 0) + 1);
    for (const member of type.members) output += memberSection(member, typeLinks, counts.get(member.name) > 1);
  }
  await writeFile(path.join(outputRoot, `${domain.slug}.mdx`), output.trimEnd() + "\n");
}

const index = `---
sidebar_position: 0
---

# Java API reference

This is the complete guide to slackblocks' public Java API, generated from its Javadoc by the javadoc tool. Use it to find concrete fluent builder methods, typed getters, immutable return types, validation failures, payloads, and higher-level components.

Build a block with a concrete builder, call \`.build()\`, then pass the resulting value directly to Slack's official Java SDK. \`ChatPostMessageRequest\` keeps the channel, fallback text, threading, and delivery options where Java Slack developers already expect them.

- [Blocks](/reference/java/blocks)
- [Elements](/reference/java/elements)
- [Composition Objects](/reference/java/objects)
- [Payloads And Views](/reference/java/payloads)
- [Components](/reference/java/components)
- [Core Types](/reference/java/core)
- [Validation And Errors](/reference/java/errors)
`;
await writeFile(path.join(outputRoot, "index.mdx"), index);
console.log(`Generated ${typeLinks.size} Java API types with javadoc.`);
