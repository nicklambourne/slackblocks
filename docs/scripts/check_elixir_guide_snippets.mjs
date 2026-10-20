import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const read = (file) => readFile(path.join(root, file), "utf8");
async function guides(directory) {
  const entries = await readdir(path.join(root, directory), { withFileTypes: true });
  return (await Promise.all(entries.filter(entry => entry.name !== "reference").map(entry => {
    const file = `${directory}/${entry.name}`;
    return entry.isDirectory() ? guides(file) : /\.mdx?$/.test(file) ? [file] : [];
  }))).flat();
}
const manifest = JSON.parse(await read("docs/examples/elixir/coverage.json"));
const counts = {};
const cases = [];
const sending = [...(await read("elixir/integrations/README.md")).matchAll(/^```elixir\n([\s\S]*?)\n```/gm)].map(match => match[1]);
const sent = [];
for (const file of ["README.md", "elixir/README.md", ...await guides("docs/docs")]) {
  const source = await read(file);
  const snippets = [...source.matchAll(/^```elixir(?: title="([^"]+)")?\n([\s\S]*?)\n```/gm)];
  if (!snippets.length) continue;
  counts[file] = snippets.length;
  const packageCodes = [];
  for (const [index, match] of snippets.entries()) {
    const [, title, code] = match;
    if (title) {
      assert.equal(file, "docs/docs/usage/sending_messages.mdx");
      assert.ok(title.endsWith("(integration-tested)"));
      assert.equal(code, sending[sent.length], `Sending snippet drift: ${title}`);
      sent.push(code);
      continue;
    }
    if (file === "elixir/README.md") { packageCodes.push(code); continue; }
    let expected = null;
    if (file.endsWith("using_blocks.mdx")) {
      const heading = [...source.slice(0, match.index).matchAll(/^## (.+)$/gm)].at(-1)?.[1];
      if (heading?.endsWith(" Block")) {
        const tail = source.slice(match.index + match[0].length).split(/^## /m)[0];
        const json = tail.match(/```json\n([\s\S]*?)\n```/);
        assert.ok(json, `${heading}: missing JSON tab`);
        assert.doesNotMatch(code, /Slackblocks\.(?:decode|from_map)/, "Block guides must construct native values");
        expected = JSON.parse(json[1]);
      }
    }
    cases.push({ label: `${file}:${index + 1}`, code, expected });
  }
  if (packageCodes.length) cases.push({ label: file, code: packageCodes.join("\n\n"), expected: null });
}
assert.deepEqual(counts, manifest, "Review the explicit snippet inventory when adding/removing examples");
assert.equal(sent.length, 2);
assert.equal(cases.filter(item => item.expected !== null).length, 21, "Every block must match its documented JSON");
cases.push({ label: "section_hello.exs", code: await read("docs/examples/elixir/section_hello.exs"), expected: JSON.parse(await read("docs/examples/section_hello.json")) });
const erlang = await read("docs/docs/usage/erlang.mdx");
for (const file of ["elixir/erlang/src/slackblocks_erlang.erl", "elixir/erlang/rebar.config"]) {
  const code = [...erlang.matchAll(/^```erlang title="([^"]+)"\n([\s\S]*?)\n```/gm)].find(match => match[1] === file);
  assert.ok(code, `Missing Erlang source: ${file}`);
  assert.equal(code[2], (await read(file)).trimEnd(), `Erlang example drift: ${file}`);
}
process.stdout.write(execFileSync("mix", ["run", "tools/guide_snippets.exs"], {
  cwd: path.join(root, "elixir"), encoding: "utf8", input: JSON.stringify(cases), maxBuffer: 8 * 1024 * 1024,
}));
console.log("All 21 native block examples match their JSON; sending and Erlang examples match integration-tested source.");
