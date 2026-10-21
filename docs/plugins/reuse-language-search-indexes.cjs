const fs = require("node:fs/promises");
const path = require("node:path");
const { createRequire } = require("node:module");
const searchRequire = createRequire(require.resolve("@easyops-cn/docusaurus-search-local"));
const cheerio = searchRequire("cheerio");
const { processDocInfos } = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/processDocInfos");
const { postBuildFactory } = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/postBuildFactory");

// Reuse only equivalent inputs within this build. The upstream parser and Lunr
// builder still own every generated index; no cache survives a build invocation.
module.exports = async function reuseLanguageSearchIndexes(buildData, variants) {
  if (variants.some(({ config }) => config.searchContextByPaths)) {
    for (const { config, filename } of variants) {
      await postBuildFactory(config, filename)(buildData);
    }
    return;
  }

  const started = performance.now();
  const pageLanguages = new Map();
  const groups = new Map();
  let copied = 0;
  for (const variant of variants) {
    for (const version of processDocInfos(buildData, variant.config)) {
      const inputs = [];
      for (const info of version.paths) {
        if (!pageLanguages.has(info.filePath)) {
          const $ = cheerio.load(await fs.readFile(info.filePath, "utf8"));
          pageLanguages.set(info.filePath, new Set(
            $("[data-language-content]").toArray().map((node) => $(node).attr("data-language-content")),
          ));
        }
        // All variants share the base selectors and indexing options. Their
        // only content difference is removing these other language sections.
        const removed = variant.ignoredLanguages.filter((language) =>
          pageLanguages.get(info.filePath).has(language));
        inputs.push([info.filePath, info.url, info.type, removed]);
      }
      const key = JSON.stringify([version.outDir, inputs]);
      const previous = groups.get(key);
      const destination = path.join(version.outDir, variant.filename);
      if (previous) {
        await fs.copyFile(previous, destination);
        copied++;
        continue;
      }
      // Restrict the upstream builder to this version, retaining the original
      // root for HTML paths and the same version output directory.
      const relative = path.relative(buildData.outDir, version.outDir).split(path.sep).join("/");
      await postBuildFactory(variant.config, variant.filename)({
        ...buildData,
        routesPaths: version.paths.map(({ url }) => url),
        plugins: [{
          name: "docusaurus-plugin-content-docs",
          content: { loadedVersions: [{
            isLast: relative === "",
            path: `${buildData.baseUrl}${relative}`,
            docs: version.paths.map(({ url }) => ({ permalink: url })),
          }] },
        }],
      });
      groups.set(key, destination);
    }
  }
  console.info(`[language-search] ${pageLanguages.size} HTML pages inspected; ${groups.size} indexes built; ${copied} equivalent indexes reused; ${((performance.now() - started) / 1000).toFixed(2)}s`);
};
