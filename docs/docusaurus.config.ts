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

const legacyManifest = JSON.parse(
  readFileSync(new URL("./legacy/manifest.json", import.meta.url), "utf8"),
) as {
  versions: Array<{
    version: string;
    canonical_prefix: string;
    generated_snapshot_tree_hash: string | null;
  }>;
};

const legacyVersionConfiguration = Object.fromEntries(
  legacyManifest.versions
    .filter(({ generated_snapshot_tree_hash: hash }) => Boolean(hash))
    .map(({ version, canonical_prefix: path }) => [
      version,
      {
        badge: false,
        label: `${version} · Python`,
        path,
      },
    ]),
);

if (typescriptPackage.version !== pythonVersion) {
  throw new Error(
    `Package versions must match: Python is ${pythonVersion}, TypeScript is ${typescriptPackage.version}`,
  );
}

const TYPESCRIPT_CLASS_NAMES = new Set([
  "InvalidUsageError",
  "LengthError",
  "MissingRequiredError",
  "MutualExclusivityError",
  "RangeError",
  "TypeMismatchError",
]);

const TYPESCRIPT_INTERFACE_NAMES = new Set([
  "ActionInput",
  "AxisConfigInput",
  "ButtonInput",
  "CardBlockInput",
  "ChartSegmentInput",
  "ConfirmationInput",
  "DataPointInput",
  "DataSeriesInput",
  "FactorySettings",
  "FeedbackButtonInput",
  "JsonObject",
  "MarkdownOptions",
  "MessageInput",
  "OptionGroupInput",
  "OptionInput",
  "PlainTextOptions",
  "RichTextStyle",
  "SectionBlockInput",
  "WorkflowButtonInput",
]);

const TYPESCRIPT_TYPE_ALIAS_NAMES = new Set([
  "AlertLevel",
  "BlockKitPayload",
  "ContainerWidth",
  "ElementInput",
  "ErrorCategory",
  "JsonPrimitive",
  "JsonValue",
  "SlackCompatibleBlock",
  "SlackIconName",
  "SlackObject",
  "SlackWire",
  "TextLike",
  "TextObject",
  "TaskStatus",
]);

const TYPESCRIPT_DOMAIN_TITLES: Record<string, string> = {
  blocks: "Blocks",
  elements: "Elements",
  errors: "Errors",
  messages: "Messages",
  objects: "Composition Objects",
  "rich-text": "Rich Text",
  utilities: "Utilities",
  views: "Views",
};

function typescriptDomainTitle({ rawName }: { rawName: string }): string {
  return TYPESCRIPT_DOMAIN_TITLES[rawName] ?? rawName;
}

function legacyTypeScriptRedirect(path: string): string | undefined {
  const match = path.match(
    /^\/reference\/typescript\/(?:blocks|elements|errors|messages|objects|rich-text|utilities|views)\/([^/]+)$/,
  );
  if (!match) return undefined;

  const name = match[1];
  const kind = TYPESCRIPT_CLASS_NAMES.has(name)
    ? "classes"
    : TYPESCRIPT_INTERFACE_NAMES.has(name)
      ? "interfaces"
      : TYPESCRIPT_TYPE_ALIAS_NAMES.has(name)
        ? "type-aliases"
        : "functions";
  return `/reference/typescript/${kind}/${name}`;
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
          editUrl: ({ version, docPath }) =>
            version === "current"
              ? `https://github.com/nicklambourne/slackblocks/tree/master/docs/${docPath}`
              : undefined,
          showLastUpdateTime: true,
          lastVersion: "current",
          versions: {
            current: {
              badge: false,
              label: pythonVersion,
              path: "",
            },
            ...legacyVersionConfiguration,
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
        entryPoints: [
          "../typescript/src/blocks.ts",
          "../typescript/src/elements.ts",
          "../typescript/src/objects.ts",
          "../typescript/src/rich-text.ts",
          "../typescript/src/messages.ts",
          "../typescript/src/views.ts",
          "../typescript/src/utilities-reference.ts",
          "../typescript/src/errors.ts",
        ],
        entryPointStrategy: "expand",
        sortEntryPoints: false,
        router: "module",
        plugin: ["./scripts/typedoc-domain-groups.mjs"],
        tsconfig: "../typescript/tsconfig.typedoc.json",
        out: "docs/reference/typescript",
        readme: "./typescript-api-intro.md",
        mergeReadme: true,
        pageTitleTemplates: {
          member: "{name}",
          module: typescriptDomainTitle,
        },
        parametersFormat: "table",
        interfacePropertiesFormat: "table",
        classPropertiesFormat: "table",
        typeDeclarationFormat: "table",
        tableColumnSettings: {
          hideInherited: true,
          hideSources: true,
        },
        disableSources: true,
        excludeExternals: true,
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
        createRedirects: legacyTypeScriptRedirect,
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
      style: "light",
      copyright: `Copyright © ${new Date().getFullYear()} Nicholas Lambourne`,
    },
  },
};

export default config;
