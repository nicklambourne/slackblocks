#!/usr/bin/env node

// Every released version must be reachable from the docs site: the current
// package version is served live (lastVersion: "current"), and every earlier
// release must have a frozen snapshot in the version dropdown. That snapshot is
// recorded either in docs/versions.json (Docusaurus-era releases, cut with
// `docusaurus docs:version`) or in docs/legacy/manifest.json (the pre-monorepo
// versions the legacy porter freezes). A release that ships without cutting its
// docs version leaves a gap — this guard fails so the gap is caught in CI
// rather than discovered by a reader.

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const docsRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));

function currentVersion() {
  const pyproject = readFileSync(
    join(docsRoot, "..", "python", "pyproject.toml"),
    "utf8",
  );
  const match = pyproject.match(/^version = "([^"]+)"$/m);
  if (!match) throw new Error("Could not read the Python package version");
  return match[1];
}

function snapshottedVersions() {
  const versions = new Set(
    JSON.parse(readFileSync(join(docsRoot, "versions.json"), "utf8")),
  );
  const manifest = JSON.parse(
    readFileSync(join(docsRoot, "legacy", "manifest.json"), "utf8"),
  );
  for (const entry of manifest.versions) versions.add(entry.version);
  return versions;
}

// Python, TypeScript, and Go are released together under one version number, so
// the `python/v*` tags remain the authoritative released-version list. Plain
// `v*` tags belong to the pre-monorepo 1.x/2.0 line and are covered by the
// legacy manifest instead.
function releasedVersions() {
  let output;
  try {
    output = execFileSync("git", ["tag", "--list", "python/v*"], {
      cwd: docsRoot,
      encoding: "utf8",
    });
  } catch (error) {
    console.warn(
      `Could not enumerate release tags (${error.message}); skipping the ` +
        "released-snapshot guard. This should only happen outside CI, where " +
        "docs.yml checks out with fetch-depth: 0.",
    );
    return null;
  }
  return [
    ...new Set(
      output
        .split("\n")
        .map((line) => line.trim())
        .filter(Boolean)
        .map((tag) => tag.replace(/^python\/v/, "")),
    ),
  ];
}

const current = currentVersion();
const released = releasedVersions();

if (released === null) {
  process.exit(0);
}

const snapshotted = snapshottedVersions();
const missing = released
  .filter((version) => version !== current)
  .filter((version) => !snapshotted.has(version))
  .sort();

if (missing.length > 0) {
  console.error(
    `Released version(s) without a frozen docs snapshot: ${missing.join(", ")}.\n` +
      "Freeze each one with `docusaurus docs:version <version>` (see RELEASING.md) " +
      "so it appears in the version dropdown, or confirm it belongs in the legacy " +
      "manifest.",
  );
  process.exitCode = 1;
} else {
  const checked = released.filter((version) => version !== current).length;
  console.log(
    `Every released version has a docs snapshot ` +
      `(${checked} checked; ${current} is the live current version).`,
  );
}
