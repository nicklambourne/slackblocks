const assert = require("node:assert/strict");
const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const vm = require("node:vm");
const { createRequire } = require("node:module");
const nativeRequire = createRequire(require.resolve("@easyops-cn/docusaurus-search-local"));
const lunr = nativeRequire("lunr");
const { postBuildFactory } = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/postBuildFactory");
const pluginFile = path.resolve(__dirname, "../plugins/language-search-indexes.cjs");

async function baselinePlugin() {
  const module = { exports: {} };
  const originalRequire = createRequire(pluginFile);
  vm.runInNewContext(await fs.readFile(pluginFile, "utf8"), {
    module,
    require: (name) => name === "./reuse-language-search-indexes.cjs"
      ? async (data, variants) => {
        for (const { config, filename } of variants) {
          await postBuildFactory(config, filename)(data);
        }
      }
      : originalRequire(name),
  }, { filename: pluginFile });
  return module.exports;
}

// Internal IDs are build-global counters in the upstream scanner. Normalize
// their references together, including Lunr postings/vectors and parent links.
function canonicalIndex(indexes) {
  const ids = new Map();
  for (const { documents } of indexes) {
    for (const { i } of documents) if (!ids.has(String(i))) ids.set(String(i), String(ids.size + 1));
  }
  return indexes.map(({ documents, index }) => ({
    documents: documents.map((doc) => ({
      ...doc, i: ids.get(String(doc.i)),
      ...(doc.p === undefined ? {} : { p: ids.get(String(doc.p)) }),
    })),
    index: {
      ...index,
      fieldVectors: index.fieldVectors.map(([ref, vector]) => {
        const [field, id] = ref.split("/");
        return [`${field}/${ids.get(id)}`, vector];
      }).sort(([a], [b]) => a.localeCompare(b)),
      invertedIndex: index.invertedIndex.map(([term, posting]) => [term, {
        ...posting,
        t: Object.fromEntries(Object.entries(posting.t).map(([id, value]) => [ids.get(id), value])),
      }]),
    },
  }));
}

function searchResults(indexes, query) {
  return indexes.map(({ documents, index }) => {
    const byId = new Map(documents.map((doc) => [String(doc.i), doc]));
    return lunr.Index.load(index).search(query).map(({ ref, score }) => {
      const { i, p, ...document } = byId.get(ref);
      return { document, score };
    });
  });
}

async function verifyIndexes(expectedDir, actualDir) {
  const files = (await fs.readdir(expectedDir, { recursive: true }))
    .filter((file) => /(?:^|\/)search-index-[a-z]+\.json$/.test(file)).sort();
  const actualFiles = (await fs.readdir(actualDir, { recursive: true }))
    .filter((file) => /(?:^|\/)search-index-[a-z]+\.json$/.test(file)).sort();
  assert.deepEqual(actualFiles, files, "Index filenames/version fallback locations changed");
  for (const file of files) {
    const expected = JSON.parse(await fs.readFile(path.join(expectedDir, file), "utf8"));
    const actual = JSON.parse(await fs.readFile(path.join(actualDir, file), "utf8"));
    assert.deepEqual(canonicalIndex(actual), canonicalIndex(expected), `Document/index parity: ${file}`);
    for (const query of ["shared", "decimal", "Python", "Elixir", "Haskell", "shared decimal", "message*", "+shared -hidden"]) {
      assert.deepEqual(searchResults(actual, query), searchResults(expected, query), `Ranking/results: ${file}: ${query}`);
    }
  }
  return files.length;
}

async function main() {
  const scratch = require("node:fs").existsSync("/Volumes/Scratch") ? "/Volumes/Scratch" : os.tmpdir();
  const temp = await fs.mkdtemp(path.join(scratch, "slackblocks-search-reuse-"));
  try {
    for (const trailingSlash of [false, true]) {
      const expectedDir = path.join(temp, `native-${trailingSlash}`);
      const actualDir = path.join(temp, `reuse-${trailingSlash}`);
      const baseUrl = "/slackblocks/";
      const siteConfig = { trailingSlash, baseUrl, themeConfig: {} };
      const versions = ["", "2.1.0", "v1.0.0"].map((version) => ({
        path: `${baseUrl}${version}`, isLast: version === "", docs: [],
      }));
      const routesPaths = [];
      for (const version of versions) {
        for (const route of ["intro", "usage/erlang", "reference/python/elements", "reference/elixir/elements", "reference/haskell/elements"]) {
          const versionPart = version.path.slice(baseUrl.length);
          const relative = [versionPart, route].filter(Boolean).join("/");
          const url = `${baseUrl}${relative}${trailingSlash ? "/" : ""}`;
          routesPaths.push(url);
          version.docs.push({ permalink: url });
          const legacy = versionPart.startsWith("v1");
          const html = `<html><head><meta name="robots" content="noindex"><meta name="description" content="shared decimal"><meta name="keywords" content="message,decimal"></head><body><article><header><h1>Shared decimal</h1></header><p>Shared messages</p><div class="skip">hidden</div>${legacy ? "<p>Legacy message</p>" : '<div data-language-content="python"><p>Python decimal</p></div><div data-language-content="elixir"><p>Elixir decimal</p></div><div data-language-content="haskell"><p>Haskell decimal</p></div>'}<h2>Messages<a class="hash-link" href="#messages">#</a></h2><p>shared decimal content</p><span class="badge">Version: old</span></article></body></html>`;
          for (const outDir of [expectedDir, actualDir]) {
            const file = path.join(outDir, trailingSlash ? `${relative}/index.html` : `${relative}.html`);
            await fs.mkdir(path.dirname(file), { recursive: true });
            await fs.writeFile(file, html);
          }
        }
      }
      const buildData = {
        baseUrl, siteConfig, routesPaths,
        plugins: [{ name: "docusaurus-plugin-content-docs", content: { loadedVersions: versions } }],
      };
      const options = { searchOptions: { indexDocs: true, docsRouteBasePath: "/", forceIgnoreNoIndex: true, ignoreCssSelectors: [".skip"] } };
      const context = { siteDir: path.resolve(__dirname, ".."), siteConfig };
      const baseline = await baselinePlugin();
      await baseline(context, options).postBuild({ ...buildData, outDir: expectedDir });
      await require(pluginFile)(context, options).postBuild({ ...buildData, outDir: actualDir });
      const count = await verifyIndexes(expectedDir, actualDir);
      // Repeat with changed HTML to prove there is no cross-build stale cache.
      const file = path.join(actualDir, trailingSlash ? "intro/index.html" : "intro.html");
      await fs.writeFile(file, (await fs.readFile(file, "utf8")).replace("Python decimal", "Python refreshed"));
      await require(pluginFile)(context, options).postBuild({ ...buildData, outDir: actualDir });
      const refreshed = await fs.readFile(path.join(actualDir, "search-index-python.json"), "utf8");
      assert.ok(refreshed.includes("Python refreshed"));
      console.log(`Verified ${count} indexes, parent links, Lunr postings/vectors and ranked results (trailingSlash=${trailingSlash}).`);
    }
  } finally {
    await fs.rm(temp, { recursive: true, force: true });
  }
}
module.exports = { baselinePlugin, canonicalIndex, searchResults, verifyIndexes };
if (require.main === module) main().catch((error) => { console.error(error); process.exitCode = 1; });
