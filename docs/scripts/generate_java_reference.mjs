import { mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const sourceRoot = path.join(repository, "java/src/main/java/io/github/nicklambourne/slackblocks");
const outputRoot = path.join(repository, "docs/docs/reference/java");

const domains = [
  { slug: "blocks", title: "Blocks", position: 1, directories: ["block"] },
  { slug: "elements", title: "Elements", position: 2, directories: ["element"] },
  { slug: "objects", title: "Composition Objects", position: 3, directories: ["object"] },
  { slug: "payloads", title: "Payloads And Views", position: 4, directories: ["payload"] },
  { slug: "components", title: "Components", position: 5, directories: ["component"] },
  { slug: "core", title: "Core Types", position: 6, files: ["Buildable.java", "SlackObject.java", "Slackblocks.java", "SlackblocksJson.java"] },
  { slug: "errors", title: "Validation And Errors", position: 7, files: ["ErrorCategory.java", "ValidationException.java"] },
];

function cleanJavadoc(raw) {
  const lines = raw.split("\n").map((line) => line.replace(/^\s*\* ?/, ""));
  const tags = new Map();
  const prose = [];
  let see;
  let current;
  for (const line of lines) {
    const tag = line.match(/^@(param|return|throws)\s+(\S+)?\s*(.*)$/);
    const seeTag = line.match(/^@see\s+<a href="([^"]+)">([^<]*)<\/a>/);
    if (tag) {
      const [, kind, name = "", description] = tag;
      current = kind === "return" ? "return" : `${kind}:${name}`;
      tags.set(current, kind === "return" ? `${name} ${description}`.trim() : description);
    } else if (line.startsWith("@see")) {
      current = undefined;
      const joined = line.match(/^@see\s+<a href="([^"]+)">/);
      see = seeTag ? { url: seeTag[1], label: seeTag[2] } : joined ? { url: joined[1], label: "Slack reference" } : see;
    } else if (current && line.trim() && !line.startsWith("@")) {
      tags.set(current, `${tags.get(current)} ${line.trim()}`);
    } else if (!line.includes("{@inheritDoc}")) {
      current = undefined;
      prose.push(line);
    }
  }
  for (const [key, value] of tags) {
    tags.set(key, renderInline(value));
  }
  return { prose: renderInline(prose.join("\n").trim()), tags, see };
}

function renderInline(value) {
  return value
    .replace(/<pre>\{@code\n?([\s\S]*?)\n?}<\/pre>/g, (_, code) => `\n\n\`\`\`java\n${code}\n\`\`\`\n\n`)
    .replace(/<ul>\s*/g, "\n\n")
    .replace(/\s*<li>\s*/g, "\n- ")
    .replace(/\s*<\/ul>/g, "\n\n")
    .replace(/\{@code\s+([^}]+)}/g, "`$1`")
    .replace(/\{@link\s+([^}\s]+)(?:\s+([^}]+))?}/g, (_, target, label) => `\`${label ?? target.replace(/^#/, "").replace("#", ".")}\``)
    .replace(/<p>/g, "\n\n")
    .replace(/<\/?(?:p|a|em|strong|b|i)(?:\s[^>]*)?>/g, "")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

function className(source, file) {
  return source.match(/public\s+(?:final\s+)?(?:class|interface|enum)\s+(\w+)/)?.[1]
    ?? path.basename(file, ".java");
}

function classDocumentation(source, name) {
  const declaration = new RegExp(`/\\*\\*((?:(?!\\*/)[\\s\\S])*)\\*/\\s*(?:@\\w+(?:\\([^\\n]+\\))?\\s*)*public\\s+(?:final\\s+)?(?:class|interface|enum)\\s+${name}\\b`);
  return cleanJavadoc(source.match(declaration)?.[1] ?? "");
}

function enumConstants(source) {
  const constants = [];
  for (const match of source.matchAll(/\/\*\*\s*([^*]*?)\s*\*\/\s*([A-Z][A-Z0-9_]*)\("([^"]*)"\)[,;]/g)) {
    constants.push({ name: match[2], wire: match[3], description: renderInline(match[1]) });
  }
  return constants;
}

function publicMethods(source) {
  const methods = [];
  const expression = /\/\*\*((?:(?!\*\/)[\s\S])*)\*\/\s*(?:@Override\s*)?(?:@SafeVarargs\s*)?(?:@SuppressWarnings\([^\n]+\)\s*)?public\s+((?:static\s+)?(?:final\s+)?[^;{}]+\([^;{}]*\))(?:\s+throws\s+[^{}]+)?\s*\{/g;
  for (const match of source.matchAll(expression)) {
    const signature = `public ${match[2].replace(/\s+/g, " ").trim()}`;
    if (/\b(equals|hashCode|toString|toMap|getType|getBlockId)\s*\(/.test(signature)) continue;
    const methodName = signature.match(/\b(\w+)\s*\(/)?.[1];
    if (!methodName) continue;
    methods.push({ name: methodName, signature, docs: cleanJavadoc(match[1]) });
  }
  return methods;
}

async function sourceFiles(domain) {
  const files = [...(domain.files ?? []).map((file) => path.join(sourceRoot, file))];
  for (const directory of domain.directories ?? []) {
    for (const file of await readdir(path.join(sourceRoot, directory))) {
      if (file.endsWith(".java") && file !== "package-info.java") {
        files.push(path.join(sourceRoot, directory, file));
      }
    }
  }
  return files.sort();
}

function parameterTypes(signature) {
  const parameters = signature.slice(signature.indexOf("(") + 1, signature.lastIndexOf(")"));
  if (!parameters.trim()) return "";
  return parameters
    .split(",")
    .map((parameter) => parameter.trim().replace(/\s+\w+$/, ""))
    .join(", ");
}

function methodSection(method, typeLinks, overloaded) {
  const baseHeading = method.name === "build" ? "Build" : method.name.replace(/^[a-z]/, (letter) => letter.toUpperCase());
  const parameterList = parameterTypes(method.signature);
  const heading = overloaded ? `${baseHeading} — ${parameterList ? `\`${parameterList}\`` : "no arguments"}` : baseHeading;
  let output = `### ${heading}\n\n`;
  if (method.docs.prose) output += `${linkTypes(method.docs.prose, typeLinks)}\n\n`;
  output += `\`\`\`java\n${method.signature}\n\`\`\`\n\n`;
  const parameters = [...method.docs.tags.entries()].filter(([key]) => key.startsWith("param:"));
  if (parameters.length) {
    output += "| Parameter | Description |\n| --- | --- |\n";
    for (const [key, description] of parameters) {
      output += `| \`${key.slice(6)}\` | ${linkTypes(description, typeLinks)} |\n`;
    }
    output += "\n";
  }
  const returns = method.docs.tags.get("return");
  if (returns) output += `**Returns:** ${linkTypes(returns, typeLinks)}\n\n`;
  const throws = [...method.docs.tags.entries()].filter(([key]) => key.startsWith("throws:"));
  if (throws.length) {
    output += "| Throws | When |\n| --- | --- |\n";
    for (const [key, description] of throws) {
      output += `| ${linkTypes(key.slice(7), typeLinks)} | ${linkTypes(description, typeLinks)} |\n`;
    }
    output += "\n";
  }
  const related = [...typeLinks.keys()]
    .filter((name) => new RegExp(`\\b${name}\\b`).test(method.signature))
    .sort();
  if (related.length) {
    output += `**Related types:** ${related.map((name) => `[\`${name}\`](${typeLinks.get(name)})`).join(", ")}\n\n`;
  }
  return output;
}

function linkTypes(value, typeLinks) {
  // Leave fenced code untouched; link type names only in prose.
  return value
    .split(/(```[\s\S]*?```)/)
    .map((part) => {
      if (part.startsWith("```")) return part;
      let result = part;
      for (const [name, href] of [...typeLinks].sort((a, b) => b[0].length - a[0].length)) {
        if (name === "Block") continue;
        result = result.replace(new RegExp(`(?<![\\w\`\\[])${name}(?![\\w\`\\]])`, "g"), `[\`${name}\`](${href})`);
      }
      return result;
    })
    .join("");
}

function anchor(name) {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, "");
}

await rm(outputRoot, { recursive: true, force: true });
await mkdir(outputRoot, { recursive: true });

const parsed = [];
for (const domain of domains) {
  const types = [];
  for (const file of await sourceFiles(domain)) {
    const source = await readFile(file, "utf8");
    const name = className(source, file);
    const docs = classDocumentation(source, name);
    types.push({ name, description: docs.prose, see: docs.see, constants: enumConstants(source), methods: publicMethods(source) });
  }
  parsed.push({ ...domain, types });
}
const typeLinks = new Map(parsed.flatMap((domain) => domain.types.map((type) => [type.name, `/reference/java/${domain.slug}#${anchor(type.name)}`])));

for (const domain of parsed) {
  let output = `---\nsidebar_position: ${domain.position}\ntoc_max_heading_level: 3\n---\n\n# ${domain.title}\n\n`;
  output += `This page documents the public Java API for ${domain.title.toLowerCase()}. Values are immutable, builders are concrete and fluent, and validation runs when you call \`.build()\`.\n\n`;
  if (domain.slug === "blocks") {
    output += "Built block values implement Slack's official `LayoutBlock` interface, so they can be passed directly to `ChatPostMessageRequest.blocks(...)` and other slack-java-sdk request builders.\n\n";
  }
  if (domain.slug === "components") {
    output += "Components expand to ordinary `List<Block>` values. The application remains responsible for channel selection, delivery options, and interaction handling through slack-java-sdk.\n\n";
  }
  for (const type of domain.types) {
    output += `## ${type.name}\n\n`;
    output += `${linkTypes(type.description || `Public ${type.name} API.`, typeLinks)}\n\n`;
    if (type.see) output += `See the [Slack reference](${type.see.url}).\n\n`;
    if (type.constants.length) {
      output += "| Constant | Slack value | Meaning |\n| --- | --- | --- |\n";
      for (const constant of type.constants) {
        output += `| \`${constant.name}\` | \`${constant.wire}\` | ${constant.description} |\n`;
      }
      output += "\n";
    }
    const methodCounts = new Map();
    for (const method of type.methods) {
      methodCounts.set(method.name, (methodCounts.get(method.name) ?? 0) + 1);
    }
    for (const method of type.methods) {
      output += methodSection(method, typeLinks, methodCounts.get(method.name) > 1);
    }
  }
  await writeFile(path.join(outputRoot, `${domain.slug}.mdx`), output.trimEnd() + "\n");
}

const index = `---
sidebar_position: 0
---

# Java API reference

This is the complete guide to slackblocks' public Java API. Use it to find concrete fluent builder methods, immutable return types, validation failures, payloads, and higher-level components.

Build a block with a concrete builder, call \`.build()\`, then pass the resulting value directly to Slack's official Java SDK. ` + "`" + `ChatPostMessageRequest` + "`" + ` keeps the channel, fallback text, threading, and delivery options where Java Slack developers already expect them.

- [Blocks](/reference/java/blocks)
- [Elements](/reference/java/elements)
- [Composition Objects](/reference/java/objects)
- [Payloads And Views](/reference/java/payloads)
- [Components](/reference/java/components)
- [Core Types](/reference/java/core)
- [Validation And Errors](/reference/java/errors)
`;
await writeFile(path.join(outputRoot, "index.mdx"), index);
console.log(`Generated ${parsed.reduce((total, domain) => total + domain.types.length, 0)} Java API types.`);
