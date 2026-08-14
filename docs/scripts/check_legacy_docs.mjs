#!/usr/bin/env node

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const docsRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const manifest = JSON.parse(
  readFileSync(join(docsRoot, "legacy", "manifest.json"), "utf8"),
);
const versions = manifest.versions;
const siteOrigin = "https://nicklambourne.github.io";
const siteBaseUrl = "/slackblocks";

function fail(message) {
  throw new Error(message);
}

function routePath(prefix, route) {
  return route === "/" ? `/${prefix}` : `/${prefix}${route}`;
}

function currentRoute(route) {
  if (!route.startsWith("/reference/")) return route;
  const page = route.slice("/reference/".length);
  return `/reference/python/${page === "utils" ? "builder" : page}`;
}

function pageSource(version, route) {
  const relative = route === "/" ? "index.mdx" : `${route.slice(1)}.mdx`;
  return join(docsRoot, "versioned_docs", `version-${version}`, relative);
}

function routeFile(buildRoot, prefix, route) {
  const suffix = route === "/" ? "" : route;
  const path = `${prefix}${suffix}`.replace(/^\//, "");
  const candidates = [
    join(buildRoot, `${path}.html`),
    join(buildRoot, path, "index.html"),
  ];
  return candidates.find(existsSync);
}

function redirectFile(buildRoot, route) {
  const relative = route.replace(/^\//, "");
  const candidates = [
    join(buildRoot, relative, "index.html"),
    join(buildRoot, `${relative}.html`),
  ];
  return candidates.find(existsSync);
}

function expectedCanonical(prefix, route) {
  const path = routePath(prefix, route);
  return `${siteOrigin}${siteBaseUrl}${path === "/" ? "/" : path}`;
}

function assertRedirect(buildRoot, from, to) {
  const output = redirectFile(buildRoot, from);
  if (!output) fail(`Missing redirect route ${from}`);
  const rendered = readFileSync(output, "utf8");
  const target = `${siteBaseUrl}${to === "/" ? "/" : to}`;
  if (!rendered.includes(`rel="canonical" href="${target}"`)) {
    fail(`${from} does not declare ${target} as its canonical target`);
  }
  if (!rendered.includes(`http-equiv="refresh" content="0; url=${target}"`)) {
    fail(`${from} does not immediately redirect to ${target}`);
  }
}

if (versions.length !== 22) {
  fail(`Expected 22 published versions, found ${versions.length}`);
}
if (versions.some(({ version }) => version === "1.0.4" || version === "1.2.1")) {
  fail("Unpublished documentation versions are present");
}
if (!manifest.migration.aliases_enabled) {
  fail("Historical aliases are not enabled");
}
const canonicalRouteCount = versions.reduce(
  (total, version) => total + version.expected_routes.length,
  0,
);
if (canonicalRouteCount !== 294) {
  fail(`Expected 294 canonical routes, found ${canonicalRouteCount}`);
}

const tags = new Set();
const canonicalPrefixes = new Set();
for (const version of versions) {
  if (tags.has(version.tag)) fail(`Duplicate release tag ${version.tag}`);
  if (canonicalPrefixes.has(version.canonical_prefix)) {
    fail(`Duplicate canonical prefix ${version.canonical_prefix}`);
  }
  tags.add(version.tag);
  canonicalPrefixes.add(version.canonical_prefix);
  if (!/^[0-9a-f]{40}$/.test(version.tag_commit)) {
    fail(`${version.tag} has an invalid commit hash`);
  }
  for (const key of [
    "documentation_source_tree_hash",
    "python_source_tree_hash",
    "generated_snapshot_tree_hash",
  ]) {
    if (!/^[0-9a-f]{40,64}$/.test(version[key] ?? "")) {
      fail(`${version.tag} has an invalid ${key}`);
    }
  }
  if (JSON.stringify(version.language_availability) !== JSON.stringify(["python"])) {
    fail(`${version.tag} is not marked Python-only`);
  }
  const inventory = JSON.parse(
    readFileSync(join(docsRoot, "legacy", version.inventory), "utf8"),
  );
  if (inventory.gh_pages_commit !== manifest.migration.gh_pages_commit) {
    fail(`${version.tag} inventory does not match the manifest gh-pages commit`);
  }
  const inventoryRoutes = inventory.pages.map(({ route }) => route);
  if (JSON.stringify(inventoryRoutes) !== JSON.stringify(version.expected_routes)) {
    fail(`${version.tag} inventory routes differ from its manifest entry`);
  }
  for (const page of inventory.pages) {
    if (!page.published_title || !Array.isArray(page.headings)) {
      fail(`${version.tag}${page.route} has an incomplete frozen inventory`);
    }
    const sourcePath = pageSource(version.version, page.route);
    if (!existsSync(sourcePath)) fail(`Missing generated page ${sourcePath}`);
    const source = readFileSync(sourcePath, "utf8");
    const notice = `from ${version.tag} (${version.tag_commit})`;
    if (!source.includes(notice)) {
      fail(`${version.tag}${page.route} is missing its provenance notice`);
    }
    const codeBlockCount = (source.match(/^\s*```/gm) ?? []).length / 2;
    if (codeBlockCount < page.code_block_count) {
      fail(
        `${version.tag}${page.route} has ${codeBlockCount} code blocks; ` +
          `the published page had ${page.code_block_count}`,
      );
    }
    const imageTargets = [
      ...[...source.matchAll(/!\[[^\]]*\]\(([^)]+)\)/g)].map(
        (match) => match[1],
      ),
      ...[...source.matchAll(/<img\b[^>]*\bsrc="([^"]+)"/g)].map(
        (match) => match[1],
      ),
    ];
    if (imageTargets.length < page.images.length) {
      fail(`${version.tag}${page.route} lost published images`);
    }
    for (const target of imageTargets.filter((value) => value.startsWith("/img/legacy/"))) {
      if (!existsSync(join(docsRoot, "static", target))) {
        fail(`${version.tag}${page.route} references missing asset ${target}`);
      }
    }
  }
}

const registered = versions.filter(({ generated_snapshot_tree_hash: hash }) => hash);
const versionsFile = JSON.parse(readFileSync(join(docsRoot, "versions.json"), "utf8"));
const expectedVersions = registered.map(({ version }) => version);
if (JSON.stringify(versionsFile) !== JSON.stringify(expectedVersions)) {
  fail("docs/versions.json is not generated from the registered manifest entries");
}

const buildArgument = process.argv.indexOf("--build-dir");
if (buildArgument >= 0) {
  const buildRoot = resolve(process.cwd(), process.argv[buildArgument + 1]);
  if (!existsSync(buildRoot)) fail(`Build directory does not exist: ${buildRoot}`);

  for (const version of registered) {
    const inventory = JSON.parse(
      readFileSync(join(docsRoot, "legacy", version.inventory), "utf8"),
    );
    for (const page of inventory.pages) {
      const output = routeFile(buildRoot, version.canonical_prefix, page.route);
      if (!output) {
        fail(`Missing built route ${version.canonical_prefix}${page.route}`);
      }
      const rendered = readFileSync(output, "utf8");
      if (/reference\/typescript/.test(rendered)) {
        fail(
          `${version.canonical_prefix}${page.route} contains a TypeScript reference route`,
        );
      }
      for (const heading of page.headings.filter(({ id }) => id)) {
        if (!rendered.includes(`id="${heading.id}"`)) {
          fail(
            `${version.canonical_prefix}${page.route} is missing historical heading ` +
              `${heading.id}`,
          );
        }
      }
      const canonical = expectedCanonical(version.canonical_prefix, page.route);
      if (!rendered.includes(`rel="canonical" href="${canonical}"`)) {
        fail(`${version.canonical_prefix}${page.route} has no canonical URL`);
      }
      const source = readFileSync(pageSource(version.version, page.route), "utf8");
      if (source.includes("<Tabs>") && !rendered.includes('role="tablist"')) {
        fail(`${version.canonical_prefix}${page.route} did not render its tabs`);
      }
      if (/^\|.+\|$/m.test(source) && !rendered.includes("<table")) {
        fail(`${version.canonical_prefix}${page.route} did not render its tables`);
      }
    }

    for (const alias of version.old_alias_prefixes) {
      for (const route of version.expected_routes) {
        assertRedirect(
          buildRoot,
          routePath(alias, route),
          routePath(version.canonical_prefix, route),
        );
      }
    }

    const searchIndex = join(
      buildRoot,
      version.canonical_prefix,
      "search-index.json",
    );
    if (!existsSync(searchIndex)) {
      fail(`${version.canonical_prefix} has no version-scoped search index`);
    }
    const searchData = JSON.parse(readFileSync(searchIndex, "utf8"));
    const searchDocuments = searchData.flatMap(({ documents = [] }) => documents);
    const searchPrefix = `${siteBaseUrl}/${version.canonical_prefix}`;
    if (
      searchDocuments.length === 0 ||
      searchDocuments.some(({ u }) => u !== searchPrefix && !u.startsWith(`${searchPrefix}/`))
    ) {
      fail(`${version.canonical_prefix} search results cross a version boundary`);
    }
  }

  const currentRoutes = Array.from(
    new Set(versions.flatMap(({ expected_routes }) => expected_routes)),
  );
  for (const alias of ["latest", "master"]) {
    for (const route of currentRoutes) {
      assertRedirect(buildRoot, routePath(alias, route), currentRoute(route));
    }
  }
  for (const page of [
    "attachments",
    "blocks",
    "elements",
    "messages",
    "modals",
    "objects",
    "rich_text",
    "utils",
    "views",
  ]) {
    assertRedirect(
      buildRoot,
      `/reference/${page}`,
      `/reference/python/${page === "utils" ? "builder" : page}`,
    );
  }

  const currentSearchIndex = join(buildRoot, "search-index.json");
  if (!existsSync(currentSearchIndex)) fail("Current docs have no search index");
  const currentSearchData = JSON.parse(readFileSync(currentSearchIndex, "utf8"));
  const currentSearchDocuments = currentSearchData.flatMap(
    ({ documents = [] }) => documents,
  );
  if (
    currentSearchDocuments.length === 0 ||
    currentSearchDocuments.some(({ u }) => /\/v\d+\.\d+\.\d+(?:\/|$)/.test(u))
  ) {
    fail("Current search results include historical documentation");
  }

  const currentHome = routeFile(buildRoot, "", "/");
  if (!currentHome) fail("Current documentation home is missing");
  const currentHomeHtml = readFileSync(currentHome, "utf8");
  if (currentHomeHtml.includes("· Python")) {
    fail("Documentation version labels must not include a language suffix");
  }

  const forbiddenV2Symbols = [
    "AlertBlock",
    "CardBlock",
    "FeedbackButtonsBlock",
    "PlanBlock",
    "TaskCardBlock",
  ];
  const v2Blocks = readFileSync(
    pageSource("2.0.0", "/reference/blocks"),
    "utf8",
  );
  for (const symbol of forbiddenV2Symbols) {
    if (v2Blocks.includes(symbol)) {
      fail(`Post-2.0 symbol ${symbol} leaked into the v2.0.0 reference`);
    }
  }
}

console.log(
  `Legacy documentation contract covers ${versions.length} versions and ` +
    `${canonicalRouteCount} routes; ${registered.length} snapshots are registered.`,
);
