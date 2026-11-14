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
    const domains = language === "haskell" ? ["blocks", "elements", "objects", "payloads", "components", "core", "errors"] : ["blocks"];
    for (const domain of domains) {
      const context = `${language}/${domain}`;
      await page.goto(`${origin}/slackblocks/reference/${context}`, {waitUntil: "networkidle"});
      const index = page.locator("details.reference-index");
      await index.locator("summary").click();
      const input = index.getByRole("searchbox", {name: "Filter this page’s API"});
      const entries = await index.locator("li a").evaluateAll(links => links.map(a => ({name: a.textContent, href: a.getAttribute("href")})));
      assert.equal(new Set(entries.map(e => e.href)).size, entries.length, `${context}: duplicate targets`);
      for (const {href} of entries) {
        assert.equal(await page.locator(`[id="${href.slice(1)}"]`).count(), 1, `${context}: ${href} must identify one symbol`);
      }
      for (const query of [entries[0].name, entries[Math.floor(entries.length / 2)].name, "__no_such_native_symbol__", entries[0].name, ""]) {
        await input.fill(query);
        const expected = entries.filter(e => e.name.toLowerCase().includes(query.toLowerCase())).map(e => e.name);
        await page.waitForFunction(n => document.querySelector(".reference-index [role=status]")?.textContent === `${n} matching entries`, expected.length);
        assert.deepEqual(await index.locator("li a").allTextContents(), expected, `${context}: ${query}`);
      }
    }
  }
  console.log("Native reference filters passed: six languages, repeated searches, exact results and unique symbol anchors.");
} finally {
  await browser.close();
  await new Promise(resolve => server.close(resolve));
}
