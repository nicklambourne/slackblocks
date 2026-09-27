import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const rubyLib = path.join(repository, "ruby/lib");
const ruby = process.env.RUBY_BIN ?? "ruby";
const guides = [
  "index.mdx",
  "quick-start.mdx",
  "usage/installation.mdx",
  "usage/using_blocks.mdx",
  "usage/sending_messages.mdx",
  "usage/cookbook.mdx",
  "usage/troubleshooting.mdx",
];
const prelude = [
  "$LOADED_FEATURES << 'slack-ruby-client.rb'",
  "module Slack; module Web; class Client",
  "  def initialize(token:); raise 'missing token' if token.empty?; end",
  "  def chat_postMessage(**payload); payload; end",
  "end; end; end",
].join("\n");
let checked = 0;
for (const guide of guides) {
  const source = await readFile(path.join(repository, "docs/docs", guide), "utf8");
  const sections = source.match(/<Ruby>[\s\S]*?<\/Ruby>/g) ?? [];
  for (const [sectionIndex, section] of sections.entries()) {
    for (const [snippetIndex, snippet] of [...section.matchAll(/```ruby\n([\s\S]*?)\n```/g)].entries()) {
      const code = snippet[1];
      if (!code.includes('require "slackblocks"')) continue;
      const output = execFileSync(ruby, ["-I", rubyLib, "-e", `${prelude}\n${code}`], {
        encoding: "utf8",
        env: { ...process.env, SLACK_BOT_TOKEN: "test-token" },
        stdio: ["ignore", "pipe", "pipe"],
      });
      if (output.trim()) {
        const payload = JSON.parse(output.trim().split("\n").at(-1));
        assert.ok(payload && typeof payload === "object", `${guide} Ruby snippet must emit a JSON object`);
        if (guide === "usage/using_blocks.mdx" && sectionIndex > 0 && sectionIndex < 22) {
          assert.equal(typeof payload.type, "string", `${guide} block ${sectionIndex} must emit a block`);
        }
      }
      checked++;
    }
  }
}
assert.ok(checked === 28, `only ${checked} Ruby guide snippets were exercised`);
console.log(`Executed ${checked} Ruby guide snippets.`);
