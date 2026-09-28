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

function runSnippet(code) {
  const output = execFileSync(ruby, ["-I", rubyLib, "-e", `${prelude}\n${code}`], {
    encoding: "utf8",
    env: { ...process.env, SLACK_BOT_TOKEN: "test-token" },
    stdio: ["ignore", "pipe", "pipe"],
  });
  return output.trim() ? JSON.parse(output.trim().split("\n").at(-1)) : null;
}

function runOtherSnippets(source) {
  let count = 0;
  for (const section of source.matchAll(/<Ruby>[\s\S]*?<\/Ruby>/g)) {
    for (const snippet of section[0].matchAll(/```ruby\n([\s\S]*?)\n```/g)) {
      if (!snippet[1].includes('require "slackblocks"')) continue;
      runSnippet(snippet[1]);
      count++;
    }
  }
  return count;
}

let checked = 0;
for (const guide of guides) {
  const source = await readFile(path.join(repository, "docs/docs", guide), "utf8");
  if (guide === "usage/using_blocks.mdx") {
    const headings = [...source.matchAll(/^## (.+) Block$/gm)];
    assert.ok(headings.length > 0, "Ruby block guide has no block sections");
    for (const [index, heading] of headings.entries()) {
      const section = source.slice(heading.index, headings[index + 1]?.index);
      const rubySection = section.match(/<Ruby>([\s\S]*?)<\/Ruby>/)?.[1];
      const code = rubySection?.match(/```ruby\n([\s\S]*?)\n```/)?.[1];
      const json = section.match(/```json\n([\s\S]*?)\n```/)?.[1];
      assert.ok(code && json, `${heading[1]} Block needs Ruby and JSON examples`);
      assert.deepEqual(runSnippet(code), JSON.parse(json), `${heading[1]} Block Ruby differs from documented JSON`);
      checked++;
    }
    const nextSection = source.indexOf("\n## ", headings.at(-1).index + 3);
    if (nextSection >= 0) checked += runOtherSnippets(source.slice(nextSection));
  } else {
    checked += runOtherSnippets(source);
  }
}
assert.equal(checked, 28, `only ${checked} Ruby guide snippets were exercised`);
console.log(`Executed ${checked} Ruby guide snippets; block payloads match the documented JSON.`);
