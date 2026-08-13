#!/usr/bin/env node

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const docsRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const manifest = JSON.parse(
  readFileSync(join(docsRoot, "legacy", "manifest.json"), "utf8"),
);
const versions = manifest.versions;

function fail(message) {
  throw new Error(message);
}

if (versions.length !== 22) fail(`Expected 22 published versions, found ${versions.length}`);
if (versions.some(({ version }) => version === "1.0.4" || version === "1.2.1")) {
  fail("Unpublished documentation versions are present");
}
const canonicalRouteCount = versions.reduce(
  (total, version) => total + version.expected_routes.length,
  0,
);
if (canonicalRouteCount !== 294) {
  fail(`Expected 294 canonical routes, found ${canonicalRouteCount}`);
}

for (const version of versions) {
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
}

const registered = versions.filter(({ generated_snapshot_tree_hash: hash }) => hash);
const versionsFile = JSON.parse(readFileSync(join(docsRoot, "versions.json"), "utf8"));
if (registered.length > 0) {
  const expectedVersions = registered.map(({ version }) => version);
  if (JSON.stringify(versionsFile) !== JSON.stringify(expectedVersions)) {
    fail("docs/versions.json is not generated from the registered manifest entries");
  }
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

const buildArgument = process.argv.indexOf("--build-dir");
if (buildArgument >= 0) {
  const buildRoot = resolve(process.cwd(), process.argv[buildArgument + 1]);
  if (!existsSync(buildRoot)) fail(`Build directory does not exist: ${buildRoot}`);
  for (const version of registered) {
    for (const route of version.expected_routes) {
      const output = routeFile(buildRoot, version.canonical_prefix, route);
      if (!output) fail(`Missing built route ${version.canonical_prefix}${route}`);
      const rendered = readFileSync(output, "utf8");
      if (/reference\/typescript/.test(rendered)) {
        fail(`${version.canonical_prefix}${route} contains a TypeScript reference route`);
      }
      if (route.startsWith("/reference/")) {
        const inventory = JSON.parse(
          readFileSync(join(docsRoot, "legacy", version.inventory), "utf8"),
        );
        const page = inventory.pages.find((candidate) => candidate.route === route);
        for (const anchor of page.api_anchors) {
          if (!rendered.includes(`id="${anchor}"`)) {
            fail(`${version.canonical_prefix}${route} is missing historical anchor ${anchor}`);
          }
        }
      }
    }
  }
}

console.log(
  `Legacy documentation contract covers ${versions.length} versions and ${canonicalRouteCount} routes; ${registered.length} snapshots are registered.`,
);
