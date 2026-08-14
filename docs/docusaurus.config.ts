import type { Config } from "@docusaurus/types";
import type { Options as ClassicPresetOptions } from "@docusaurus/preset-classic";
import { themes as prismThemes } from "prism-react-renderer";
import { readFileSync } from "node:fs";

const pythonProject = readFileSync(
  new URL("../python/pyproject.toml", import.meta.url),
  "utf8",
);
const pythonVersion = pythonProject.match(/^version = "([^"]+)"$/m)?.[1];

if (!pythonVersion) {
  throw new Error("Could not read the Python package version");
}

const typescriptPackage = JSON.parse(
  readFileSync(new URL("../typescript/package.json", import.meta.url), "utf8"),
) as { version: string };

if (typescriptPackage.version !== pythonVersion) {
  throw new Error(
    `Package versions must match: Python is ${pythonVersion}, TypeScript is ${typescriptPackage.version}`,
  );
}

const config: Config = {
  title: "slackblocks",
  tagline: "Validated Slack Block Kit construction for Python and TypeScript",
  favicon: "img/sb.png",
  url: "https://nicklambourne.github.io",
  baseUrl: "/slackblocks/",
  organizationName: "nicklambourne",
  projectName: "slackblocks",
  trailingSlash: false,
  onBrokenLinks: "throw",
  onBrokenAnchors: "throw",
  markdown: {
    format: "detect",
    hooks: {
      onBrokenMarkdownLinks: "throw",
    },
  },
  presets: [
    [
      "classic",
      {
        docs: {
          routeBasePath: "/",
          sidebarPath: "./sidebars.ts",
          editUrl: "https://github.com/nicklambourne/slackblocks/tree/master/docs/",
          showLastUpdateTime: true,
          lastVersion: "current",
          versions: {
            current: {
              badge: false,
              label: pythonVersion,
              path: "",
            },
            "1.0": {
              badge: false,
              label: "1.0.0",
              path: "1.0.0",
            },
          },
        },
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies ClassicPresetOptions,
    ],
  ],
  plugins: [
    [
      "docusaurus-plugin-typedoc",
      {
        name: "TypeScript API reference",
        entryPoints: ["../typescript/src/index.ts"],
        tsconfig: "../typescript/tsconfig.typedoc.json",
        out: "docs/reference/typescript",
        readme: "./typescript-api-intro.md",
        mergeReadme: true,
        excludePrivate: true,
        excludeInternal: true,
        sidebar: {
          autoConfiguration: false,
        },
      },
    ],
    [
      "@docusaurus/plugin-client-redirects",
      {
        redirects: [
          { from: "/latest", to: "/" },
          { from: "/latest/contributing", to: "/contributing" },
          { from: "/latest/usage/compatibility", to: "/usage/compatibility" },
          { from: "/latest/usage/cookbook", to: "/usage/cookbook" },
          { from: "/latest/usage/installation", to: "/usage/installation" },
          { from: "/latest/usage/migration", to: "/usage/migration" },
          { from: "/latest/usage/sending_messages", to: "/usage/sending_messages" },
          { from: "/latest/usage/troubleshooting", to: "/usage/troubleshooting" },
          { from: "/latest/usage/using_blocks", to: "/usage/using_blocks" },
          { from: "/latest/reference/attachments", to: "/reference/python/attachments" },
          { from: "/latest/reference/blocks", to: "/reference/python/blocks" },
          { from: "/latest/reference/elements", to: "/reference/python/elements" },
          { from: "/latest/reference/messages", to: "/reference/python/messages" },
          { from: "/latest/reference/modals", to: "/reference/python/modals" },
          { from: "/latest/reference/objects", to: "/reference/python/objects" },
          { from: "/latest/reference/rich_text", to: "/reference/python/rich_text" },
          { from: "/latest/reference/utils", to: "/reference/python/builder" },
          { from: "/latest/reference/views", to: "/reference/python/views" },
          { from: "/reference/attachments", to: "/reference/python/attachments" },
          { from: "/reference/blocks", to: "/reference/python/blocks" },
          { from: "/reference/elements", to: "/reference/python/elements" },
          { from: "/reference/messages", to: "/reference/python/messages" },
          { from: "/reference/modals", to: "/reference/python/modals" },
          { from: "/reference/objects", to: "/reference/python/objects" },
          { from: "/reference/rich_text", to: "/reference/python/rich_text" },
          { from: "/reference/utils", to: "/reference/python/builder" },
          { from: "/reference/views", to: "/reference/python/views" },
        ],
      },
    ],
  ],
  themes: [
    [
      "@easyops-cn/docusaurus-search-local",
      {
        hashed: true,
        indexDocs: true,
      },
    ],
  ],
  themeConfig: {
    image: "img/sb.png",
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.vsDark,
      additionalLanguages: ["python", "typescript", "json", "toml"],
    },
    navbar: {
      title: "slackblocks",
      logo: { alt: "slackblocks logo", src: "img/sb.png" },
      items: [
        { type: "html", value: "language-selector", position: "left" },
        {
          type: "docsVersionDropdown",
          position: "left",
          className: "docs-version-dropdown",
          dropdownActiveClassDisabled: true,
        },
        {
          href: "https://github.com/nicklambourne/slackblocks",
          position: "right",
          className: "header-github-link",
          "aria-label": "GitHub repository",
        },
      ],
    },
    colorMode: {
      defaultMode: "light",
      respectPrefersColorScheme: false,
    },
    footer: {
      style: "dark",
      copyright: `Copyright © ${new Date().getFullYear()} Nicholas Lambourne`,
    },
  },
};

export default config;
