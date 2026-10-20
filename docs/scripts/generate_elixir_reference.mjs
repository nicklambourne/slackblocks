// Render actual Elixir docs, signatures and typespecs; no Java-shaped API adapter.
import { execFileSync } from "node:child_process";
import { mkdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const stdout = execFileSync("mix", ["run", "tools/reference.exs"], { cwd: path.join(root, "elixir"), encoding: "utf8", maxBuffer: 16 * 1024 * 1024 });
const reference = JSON.parse(stdout.trim().split("\n").at(-1));
const domains = [
  ["blocks", "Blocks"], ["elements", "Elements"], ["objects", "Composition Objects and Rich Text"],
  ["payloads", "Messages, Attachments and Views"], ["components", "Components and Utilities"],
  ["core", "JSON, Validation and Types"], ["errors", "Validation Errors"],
];
function domain(name) {
  if (name.startsWith("Slackblocks.Blocks.")) return "blocks";
  if (name.startsWith("Slackblocks.Elements.")) return "elements";
  if (name.startsWith("Slackblocks.Objects.") || name.startsWith("Slackblocks.RichText.")) return "objects";
  if (name.startsWith("Slackblocks.Views.") || /\.(?:Message|EphemeralMessage|MessageResponse|WebhookMessage|Attachment)$/.test(name)) return "payloads";
  if (name.startsWith("Slackblocks.Components.") || name === "Slackblocks.Color") return "components";
  if (name === "Slackblocks.ValidationError") return "errors";
  return "core";
}
const anchor = (name) => name.toLowerCase().replaceAll(".", "-");
const prose = (text) => text.replace(/^#/gm, "##").replace(/(?:^ {4}.*(?:\n|$))+/gm, block => "```elixir\n" + block.replace(/^ {4}/gm, "").trimEnd() + "\n```\n");
const fence = (text) => `\`\`\`elixir\n${text}\n\`\`\`\n\n`;
const output = path.join(root, "docs/docs/reference/elixir");
await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });
for (const [index, [slug, title]] of domains.entries()) {
  let text = `---\nsidebar_position: ${index + 1}\n---\n\n# ${title}\n\nGenerated from compiled Elixir documentation and native typespecs. Values are immutable; constructors and changes use keyword options.\n\n`;
  for (const module of reference.modules.filter((entry) => domain(entry.module) === slug)) {
    text += `## ${module.module} {#${anchor(module.module)}}\n\n${prose(module.doc)}\n\n`;
    if (module.types.length) text += `### Types\n\n${fence(module.types.join("\n\n"))}`;
    for (const member of module.functions) {
      text += `### ${member.name} {#${anchor(module.module)}-${member.name.replaceAll("!", "-bang").replaceAll("/", "-")}}\n\n${prose(member.doc)}\n\n${fence(member.signatures.join("\n"))}`;
      if (member.spec.length) text += fence(member.spec.join("\n"));
    }
  }
  await writeFile(path.join(output, `${slug}.mdx`), text);
}
await writeFile(path.join(output, "index.mdx"), `---\nsidebar_position: 0\n---\n\n# Elixir API reference\n\nElixir 1.16.3+ with compatible OTP 26.2+ on 64-bit systems. The first release is being prepared for 2.8.0; see [installation](/usage/installation). These pages cover all public value structs, tagged and raising constructors/changes, typespecs and helpers.\n\n${domains.map(([slug, title]) => `- [${title}](/reference/elixir/${slug})`).join("\n")}\n\nThe same modules are callable from Erlang; see [Erlang integration](/usage/erlang). Jason encoding revalidates values. Checked decoding preserves number tokens and rejects duplicate keys; existing Jason build caches may need the documented rebuild step.\n`);
await writeFile(path.join(output, "reference.json"), JSON.stringify(reference, null, 2) + "\n");
console.log(`Generated ${reference.modules.length} native Elixir modules from compiled docs and typespecs.`);
