// Renders a generated API reference model as one MDX page per API area.
//
// The Java reference (javadoc, through java-reference/ReferenceDoclet.java) and the C# reference
// (reflection and XML documentation, through csharp-reference/) both produce the same JSON model:
// types with Markdown documentation, enum constants, and members with signatures, parameters,
// return values, and exceptions. Links to other API types are written as [`Name`](ref:Name), or
// javadoc:Name for the Java doclet, and resolved here to their page and anchor.

import { mkdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";

export function anchor(name) {
  return name.toLowerCase().replace(/[^a-z0-9]+/g, "");
}

function resolveLinks(markdown, typeLinks) {
  return markdown.replace(/\]\((?:javadoc|ref):(\w+)\)/g, (_, name) => (typeLinks.has(name) ? `](${typeLinks.get(name)})` : "]()"))
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

function memberSection(member, typeLinks, overloaded, fence) {
  let output = `### ${heading(member, overloaded)}\n\n`;
  if (member.doc) output += `${resolveLinks(member.doc, typeLinks)}\n\n`;
  output += `\`\`\`${fence}\n${member.signature}\n\`\`\`\n\n`;
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

/**
 * Writes the reference pages.
 *
 * @param {object} options
 * @param {{types: object[]}} options.reference the generated API model
 * @param {{slug: string, title: string, position: number}[]} options.domains API areas, in order
 * @param {(type: object) => object | undefined} options.domainFor the area a type belongs to, if any
 * @param {string} options.language URL segment, such as "java"
 * @param {string} options.fence code fence language for signatures
 * @param {string} options.outputRoot directory that receives the MDX pages
 * @param {(domain: object) => string} options.introduction Markdown that opens each area page
 * @param {string} options.index the index page, including front matter
 * @returns {Promise<number>} the number of documented types
 */
export async function writeReferencePages({ reference, domains, domainFor, language, fence, outputRoot, introduction, index }) {
  const byDomain = new Map(domains.map((domain) => [domain.slug, []]));
  for (const type of reference.types) {
    const domain = domainFor(type);
    if (domain) byDomain.get(domain.slug).push(type);
  }
  const typeLinks = new Map();
  for (const domain of domains) {
    for (const type of byDomain.get(domain.slug)) {
      typeLinks.set(type.name, `/reference/${language}/${domain.slug}#${anchor(type.name)}`);
    }
  }

  await rm(outputRoot, { recursive: true, force: true });
  await mkdir(outputRoot, { recursive: true });

  for (const domain of domains) {
    const types = byDomain.get(domain.slug).sort((a, b) => a.name.localeCompare(b.name));
    let output = `---\nsidebar_position: ${domain.position}\ntoc_max_heading_level: 3\n---\n\n# ${domain.title}\n\n`;
    output += introduction(domain);
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
      for (const member of type.members) output += memberSection(member, typeLinks, counts.get(member.name) > 1, fence);
    }
    await writeFile(path.join(outputRoot, `${domain.slug}.mdx`), output.trimEnd() + "\n");
  }

  await writeFile(path.join(outputRoot, "index.mdx"), index);
  return typeLinks.size;
}
