const {
  processPluginOptions,
} = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/processPluginOptions");
const reuseLanguageSearchIndexes = require("./reuse-language-search-indexes.cjs");

const DEFAULT_SEARCH_OPTIONS = {
  indexDocs: true,
  indexBlog: true,
  indexPages: false,
  docsRouteBasePath: ["docs"],
  blogRouteBasePath: ["blog"],
  language: ["en"],
  docsDir: ["docs"],
  blogDir: ["blog"],
  removeDefaultStopWordFilter: [],
  removeDefaultStemmer: false,
  ignoreFiles: [],
  ignoreCssSelectors: [],
  forceIgnoreNoIndex: false,
};

const LANGUAGES = ["python", "typescript", "go", "java", "csharp", "ruby", "rust", "php", "elixir"];
const LANGUAGE_INDEXES = Object.fromEntries(LANGUAGES.map((language) => {
  const others = LANGUAGES.filter((candidate) => candidate !== language);
  return [language, {
    ignoredLanguages: others,
    ignoredContent: others.map((other) => `[data-language-content="${other}"]`).join(", "),
    ignoredRoutes: [
      ...(language === "elixir" ? [] : [/^usage\/erlang$/]),
      new RegExp(`^reference/(?:${others.join("|")})(?:/|$)`),
      ...(["python", "rust", "php", "elixir"].includes(language) ? [] : [/^usage\/(?:compatibility|migration)$/]),
    ],
  }];
}));

module.exports = function languageSearchIndexes(context, options) {
  const baseConfig = processPluginOptions(
    {
      ...DEFAULT_SEARCH_OPTIONS,
      ...options.searchOptions,
    },
    context,
  );

  return {
    name: "slackblocks-language-search-indexes",
    async postBuild(buildData) {
      const variants = Object.entries(LANGUAGE_INDEXES).map(([
        language,
        { ignoredContent, ignoredRoutes, ignoredLanguages },
      ]) => {
        const config = {
          ...baseConfig,
          ignoreCssSelectors: [
            ...baseConfig.ignoreCssSelectors,
            ignoredContent,
          ],
          ignoreFiles: [...baseConfig.ignoreFiles, ...ignoredRoutes],
        };

        return { config, ignoredLanguages, filename: `search-index-${language}.json` };
      });
      await reuseLanguageSearchIndexes(buildData, variants);
    },
  };
};
