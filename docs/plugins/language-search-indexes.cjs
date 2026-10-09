const {
  processPluginOptions,
} = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/processPluginOptions");
const {
  postBuildFactory,
} = require("@easyops-cn/docusaurus-search-local/dist/server/server/utils/postBuildFactory");

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

const LANGUAGES = ["python", "typescript", "go", "java", "csharp", "ruby", "rust"];
const LANGUAGE_INDEXES = Object.fromEntries(LANGUAGES.map((language) => {
  const others = LANGUAGES.filter((candidate) => candidate !== language);
  return [language, {
    ignoredContent: others.map((other) => `[data-language-content="${other}"]`).join(", "),
    ignoredRoutes: [
      new RegExp(`^reference/(?:${others.join("|")})(?:/|$)`),
      ...(["python", "rust"].includes(language) ? [] : [/^usage\/(?:compatibility|migration)$/]),
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
      for (const [
        language,
        { ignoredContent, ignoredRoutes },
      ] of Object.entries(LANGUAGE_INDEXES)) {
        const config = {
          ...baseConfig,
          ignoreCssSelectors: [
            ...baseConfig.ignoreCssSelectors,
            ignoredContent,
          ],
          ignoreFiles: [...baseConfig.ignoreFiles, ...ignoredRoutes],
        };

        await postBuildFactory(
          config,
          `search-index-${language}.json`,
        )(buildData);
      }
    },
  };
};
