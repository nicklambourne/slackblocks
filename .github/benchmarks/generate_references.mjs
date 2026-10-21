import { spawn } from "node:child_process";

const scripts = [
  "generate:python", "generate:go", "generate:java", "generate:csharp",
  "generate:ruby", "check:typescript-api", "generate:rust", "generate:php",
  "generate:elixir",
];
if (!process.env.npm_execpath) {
  throw new Error("Run reference generation with pnpm generate.");
}

let next = 0;
let failure = 0;
const logs = [];
async function worker() {
  while (!failure && next < scripts.length) {
    const index = next++;
    const chunks = [];
    const code = await new Promise((resolve) => {
      const child = spawn(process.execPath, [process.env.npm_execpath, "run", scripts[index]], {
        stdio: ["ignore", "pipe", "pipe"],
      });
      child.stdout.on("data", (data) => chunks.push(data));
      child.stderr.on("data", (data) => chunks.push(data));
      child.on("error", (error) => {
        chunks.push(Buffer.from(`${error.message}\n`));
        resolve(127);
      });
      child.on("close", (exitCode) => resolve(exitCode ?? 1));
    });
    logs[index] = Buffer.concat(chunks);
    if (code && !failure) failure = code;
  }
}

// Stop scheduling on failure, then drain both children before returning to the build.
await Promise.all([worker(), worker()]);
for (let index = 0; index < logs.length; index++) {
  if (logs[index]) {
    process.stdout.write(`\n[${scripts[index]}]\n`);
    process.stdout.write(logs[index]);
  }
}
process.exitCode = failure;
