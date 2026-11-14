import assert from "node:assert/strict";
import {createServer} from "node:http";
import {readFile} from "node:fs/promises";
import path from "node:path";
import {fileURLToPath} from "node:url";
import {chromium} from "playwright";

const root = fileURLToPath(new URL("../build/", import.meta.url));
const types = {".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml"};
const server = createServer(async (request, response) => {
  const route = decodeURIComponent(new URL(request.url, "http://localhost").pathname).replace(/^\/slackblocks\//, "");
  const filename = path.resolve(root, route.replace(/^\//, ""));
  if (!filename.startsWith(root)) { response.writeHead(403).end(); return; }
  try {
    const extension = path.extname(filename);
    response.setHeader("Content-Type", types[extension] ?? "text/html");
    response.end(await readFile(extension ? filename : `${filename}.html`));
  } catch { response.writeHead(404).end(); }
});
await new Promise(resolve => server.listen(0, "127.0.0.1", resolve));
const origin = `http://127.0.0.1:${server.address().port}`;
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.route("**/*", route => route.request().url().startsWith(origin) ? route.continue() : route.abort());
  for (const language of ["java", "csharp", "ruby", "rust", "php", "haskell"]) {
    await page.goto(`${origin}/slackblocks/reference/${language}/blocks`, {waitUntil: "networkidle"});
    const index = page.locator("details.reference-index");
    await index.locator("summary").click();
    const input = index.getByRole("searchbox", {name: "Filter this page’s API"});
    const first = await index.locator("li a").first().textContent();
    await input.fill("__no_such_native_symbol__");
    await page.waitForFunction(() => document.querySelector(".reference-index [role=status]")?.textContent === "0 matching entries");
    assert.equal(await index.locator("li").count(), 0, language);
    await input.fill(first);
    const link = index.getByRole("link", {name: first, exact: true}).first();
    await link.waitFor({state: "visible"});
    const href = await link.getAttribute("href");
    assert.ok(await page.locator(`[id="${href.slice(1)}"]`).count(), `${language}: index anchor must exist`);
  }
  console.log("Native reference filters passed: six languages, empty results, labelled input and preserved anchors.");
} finally {
  await browser.close();
  await new Promise(resolve => server.close(resolve));
}
