# Releasing

The Python (`slackblocks` on PyPI), TypeScript
(`@nicklambourne/slackblocks` on npm), and Go
(`github.com/nicklambourne/slackblocks/go/v2`) packages are released together
and always carry the same version number.

The recommended entry point is the **Coordinated Release** workflow in GitHub
Actions. Run it from `master` with one `X.Y.Z` input; it validates the shared
version and changelogs, creates all three annotated tags in one atomic push,
dispatches each existing publisher at its tag, and waits for all three runs.

## Tag scheme

Releases are triggered by pushing tags:

| Tag | Workflow | Publishes |
|---|---|---|
| `python/vX.Y.Z` | [`.github/workflows/publish.yml`](.github/workflows/publish.yml) | `slackblocks` to PyPI |
| `ts/vX.Y.Z` | [`.github/workflows/publish-npm.yml`](.github/workflows/publish-npm.yml) | `@nicklambourne/slackblocks` to npm |
| `go/vX.Y.Z` | [`.github/workflows/publish-go.yml`](.github/workflows/publish-go.yml) | `github.com/nicklambourne/slackblocks/go/v2` to the Go module ecosystem |

Plain `v*` tags (used by the pre-monorepo 1.x/2.0 releases) no longer trigger
anything.

The three publisher workflows also accept a manual dispatch at an existing,
matching language tag. The coordinator uses those entry points so the PyPI and
npm jobs retain their existing trusted-publisher workflow identities.

The Python and TypeScript workflows fail fast if the tag does not match their
package manifest. Every workflow also verifies that `python/pyproject.toml`
and `typescript/package.json` agree, and the Go workflow requires its tag to
match that coordinated version.

## One-time setup

These must be in place before the workflows can publish:

1. **GitHub environments** — create `pypi` and `npm` environments in the
   repository settings (Settings → Environments). The publish jobs run inside
   them; add required reviewers there if you want manual approval before
   publishing.
2. **PyPI trusted publisher** — on PyPI, add a trusted publisher for the
   `slackblocks` project pointing at this repository with workflow
   `publish.yml` and environment `pypi`. No API token is needed after that;
   `uv publish` authenticates via OIDC.
3. **npm first-publish bootstrap** — npm trusted publishing can only be
   configured on a package that already exists, so the very first publish of
   `@nicklambourne/slackblocks` must use a token:
   1. Create a granular npm access token allowed to publish new packages under
      the `@nicklambourne` scope.
   2. Add it as the repository (or `npm` environment) secret `NPM_TOKEN`.
   3. Push the `ts/v*` tag. The workflow detects `NPM_TOKEN` and publishes
      with token authentication (still with `--provenance`).
   4. Once the package exists on npm, configure trusted publishing on
      npmjs.com (package Settings → Trusted publisher: this repository,
      workflow `publish-npm.yml`, environment `npm`), then **delete the
      `NPM_TOKEN` secret**. Subsequent publishes use OIDC automatically; the
      token path only runs while the secret is present.
4. **Go requires no registry credentials** — Go modules are published by
   pushing the correctly prefixed repository tag. Because the module lives in
   the `go/` subdirectory and declares the `/v2` module path, its tags must use
   the exact form `go/vX.Y.Z`. The workflow verifies the module and creates the
   corresponding GitHub Release; consumers and the public Go proxy resolve it
   directly from the repository.

## Coordinated release procedure

The docs site serves the current package version live (`lastVersion:
"current"`) and lists every earlier release as a frozen snapshot in the version
dropdown. Each release therefore has to freeze the version it is moving *off*
before the new version takes over as current — otherwise that version vanishes
from the dropdown. CI enforces this: the `check:release-snapshots` guard fails
if any released `python/v*` tag other than the current package version is
missing from `docs/versions.json` (or the legacy manifest).

1. On one branch, freeze the **outgoing** docs version — the value currently in
   `python/pyproject.toml`, before you bump it — so it survives as a dropdown
   entry once the new version becomes current:

   ```sh
   pnpm --filter @slackblocks/docs generate
   pnpm --filter @slackblocks/docs exec docusaurus docs:version <outgoing>
   ```

   `generate` populates the gitignored API reference so the snapshot matches a
   real build; `docs:version` then copies `docs/docs` into
   `docs/versioned_docs/version-<outgoing>` and prepends `<outgoing>` to
   `docs/versions.json`. (The very first monorepo release is the exception: its
   outgoing `2.0.0` is a legacy version already frozen in the manifest.)
2. Bump the version in **both** manifests on the same branch:
   - `python/pyproject.toml` (`project.version`)
   - `typescript/package.json` (`version`)

   For a new major release, also change the Go semantic import path in
   `go/go.mod` from `/v2` to `/vN`, update every Go import in code and docs,
   and update the module-path assertion in `publish-go.yml`. A coordinated
   Python/TypeScript 3.0.0 release therefore requires a Go `/v3` module; the
   existing `/v2` path cannot publish `go/v3.0.0`.
3. Add a `## [X.Y.Z] — YYYY-MM-DD` section to `python/CHANGELOG.md`,
   `typescript/CHANGELOG.md`, and `go/CHANGELOG.md`. The publish workflows
   extract the matching section for their GitHub Release notes.
4. Merge to `master` and wait for CI to pass.
5. In GitHub Actions, open **Coordinated Release**, select **Run workflow**,
   keep the branch set to `master`, and enter `X.Y.Z`. The equivalent CLI
   command is:

   ```sh
   gh workflow run coordinated-release.yml --ref master -f version=X.Y.Z
   ```

6. The coordinator atomically pushes `python/vX.Y.Z`, `ts/vX.Y.Z`, and
   `go/vX.Y.Z`, then dispatches and monitors the three existing publishers.
   Each publisher creates its own GitHub Release after its registry step
   succeeds.

If the coordinator is unavailable, the direct tag triggers remain as a manual
fallback. Create all three signed tags at the same commit and push them
atomically:

```sh
git tag -s python/vX.Y.Z -m "slackblocks X.Y.Z (Python)"
git tag -s ts/vX.Y.Z -m "@nicklambourne/slackblocks X.Y.Z (TypeScript)"
git tag -s go/vX.Y.Z -m "slackblocks X.Y.Z (Go)"
git push --atomic origin python/vX.Y.Z ts/vX.Y.Z go/vX.Y.Z
```

Tags created by the coordinator are annotated as `github-actions[bot]` but
are not signed with a maintainer's personal key; CI deliberately does not
store that private key.

## Partial-failure recovery

The three tags are created atomically, but three external registries cannot be
updated as one transaction. If one publisher fails after another succeeds,
re-run the failed publisher from its existing workflow run; never move the
tags or republish an already released version.

All three workflows are safe to re-run from the Actions UI:

- **PyPI** — `uv publish` runs with
  `--check-url https://pypi.org/simple/slackblocks/`, so files already
  uploaded by a partially failed run are skipped instead of erroring.
- **npm** — the publish is a single atomic upload; if it failed, re-running
  simply retries it. If it already succeeded, npm rejects the duplicate
  version, which tells you the registry side is done.
- **Go** — the tag is the published module version. Re-run the workflow if only
  verification or GitHub Release creation failed; do not move or replace a
  public module tag.
- **GitHub Releases** — the release step skips itself if a release for the
  tag already exists.

After a failure, check:

- PyPI: https://pypi.org/project/slackblocks/ lists the new version with both
  the sdist and the wheel.
- npm: https://www.npmjs.com/package/@nicklambourne/slackblocks shows the new
  version (with provenance).
- Go: https://pkg.go.dev/github.com/nicklambourne/slackblocks/go/v2 lists the
  new module version.
- GitHub: a Release exists for each of the three tags with the changelog notes.

If a bad artifact was published, do not delete and re-upload: registries
reject reused file names and versions. Yank/deprecate the broken version and
release a new coordinated patch version of **all three** packages.
