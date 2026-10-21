import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const generator = process.argv[2] ?? fileURLToPath(new URL("./generate_references.mjs", import.meta.url));
const expected = ["generate:python", "generate:go", "generate:java", "generate:csharp", "generate:ruby", "check:typescript-api", "generate:rust", "generate:php", "generate:elixir"];
const temporary = mkdtempSync(path.join(process.env.TMPDIR ?? os.tmpdir(), "reference-controls-"));
try {
  const stub = path.join(temporary, "pnpm.mjs");
  writeFileSync(stub, `import { appendFileSync } from 'node:fs';
const name = process.argv[3];
const event = state => appendFileSync(process.env.TRACE, JSON.stringify({ name, state }) + '\\n');
event('start');
setTimeout(() => { event('finish'); process.exit(process.env.FAIL === name ? 17 : 0); }, process.env.FAIL === name ? 30 : 120);
`);
  for (const fail of ["", "generate:python"]) {
    const trace = path.join(temporary, fail ? "failure.jsonl" : "success.jsonl");
    const result = spawnSync(process.execPath, [generator], {
      env: { ...process.env, npm_execpath: stub, TRACE: trace, FAIL: fail }, encoding: "utf8",
    });
    assert.equal(result.status, fail ? 17 : 0, result.stderr);
    const events = readFileSync(trace, "utf8").trim().split("\n").map(JSON.parse);
    const starts = events.filter((event) => event.state === "start").map((event) => event.name);
    assert.deepEqual([...starts].sort(), (fail ? expected.slice(0, 2) : expected).toSorted());
    let active = 0;
    let peak = 0;
    for (const event of events) {
      active += event.state === "start" ? 1 : -1;
      assert.ok(active >= 0 && active <= 2, "At most two generators may run");
      peak = Math.max(peak, active);
    }
    assert.equal(active, 0, "Drain already-running generators before returning");
    assert.equal(peak, 2);
    if (fail) assert.equal(events.at(-1).name, "generate:go");
    for (const name of starts) assert.ok(result.stdout.includes(`[${name}]`));
  }
  const patch = path.join(temporary, "missing-executable.mjs");
  writeFileSync(patch, `Object.defineProperty(process, 'execPath', { value: ${JSON.stringify(path.join(temporary, "missing-node"))} });\n`);
  const result = spawnSync(process.execPath, ["--import", patch, generator], {
    env: { ...process.env, npm_execpath: stub }, encoding: "utf8",
  });
  assert.equal(result.status, 127, result.stderr);
  assert.ok(result.stdout.includes("ENOENT"));
  assert.ok(!result.stdout.includes("[generate:java]"));
  console.log("Reference scheduling controls passed: nine commands, two-child bound, failure stop/drain, spawn failure.");
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
