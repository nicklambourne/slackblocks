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

const LANGUAGE_INDEXES = {
  python: {
    ignoredContent: '[data-language-content="typescript"], [data-language-content="go"], [data-language-content="java"], [data-language-content="csharp"]',
    ignoredRoutes: [/^reference\/(?:typescript|go|java|csharp)(?:\/|$)/],
  },
  typescript: {
    ignoredContent: '[data-language-content="python"], [data-language-content="go"], [data-language-content="java"], [data-language-content="csharp"]',
    ignoredRoutes: [
      /^reference\/(?:python|go|java|csharp)(?:\/|$)/,
      /^usage\/(?:compatibility|migration)$/,
    ],
  },
  go: {
    ignoredContent: '[data-language-content="python"], [data-language-content="typescript"], [data-language-content="java"], [data-language-content="csharp"]',
    ignoredRoutes: [
      /^reference\/(?:python|typescript|java|csharp)(?:\/|$)/,
      /^usage\/(?:compatibility|migration)$/,
    ],
  },
  java: {
    ignoredContent: '[data-language-content="python"], [data-language-content="typescript"], [data-language-content="go"], [data-language-content="csharp"]',
    ignoredRoutes: [
      /^reference\/(?:python|typescript|go|csharp)(?:\/|$)/,
      /^usage\/(?:compatibility|migration)$/,
    ],
  },
  csharp: {
    ignoredContent: '[data-language-content="python"], [data-language-content="typescript"], [data-language-content="go"], [data-language-content="java"]',
    ignoredRoutes: [
      /^reference\/(?:python|typescript|go|java)(?:\/|$)/,
      /^usage\/(?:compatibility|migration)$/,
    ],
  },
};

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
