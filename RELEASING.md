# Releasing

The Python (`slackblocks` on PyPI) and TypeScript (`@nicklambourne/slackblocks`
on npm) packages are released together and always carry the same version
number.

## Tag scheme

Releases are triggered by pushing tags:

| Tag | Workflow | Publishes |
|---|---|---|
| `python/vX.Y.Z` | [`.github/workflows/publish.yml`](.github/workflows/publish.yml) | `slackblocks` to PyPI |
| `ts/vX.Y.Z` | [`.github/workflows/publish-npm.yml`](.github/workflows/publish-npm.yml) | `@nicklambourne/slackblocks` to npm |

Plain `v*` tags (used by the pre-monorepo 1.x/2.0 releases) no longer trigger
anything.

Both workflows fail fast if the tag does not match the package manifest
version, or if `python/pyproject.toml` and `typescript/package.json` disagree
with each other.

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

## Coordinated release procedure

1. Bump the version in **both** manifests on one branch:
   - `python/pyproject.toml` (`project.version`)
   - `typescript/package.json` (`version`)
2. Add a `## [X.Y.Z] — YYYY-MM-DD` section to both `python/CHANGELOG.md` and
   `typescript/CHANGELOG.md`. The publish workflows extract this section for
   the GitHub Release notes.
3. Merge to `master` and wait for CI to pass.
4. Tag and push, Python first (either order works; the guards only require
   the two manifests to agree):

   ```sh
   git tag python/vX.Y.Z && git push origin python/vX.Y.Z
   git tag ts/vX.Y.Z && git push origin ts/vX.Y.Z
   ```

5. Each workflow publishes to its registry and then creates a GitHub Release
   for its tag with the changelog section as notes.

## Partial-failure recovery

Both workflows are safe to re-run from the Actions UI:

- **PyPI** — `uv publish` runs with
  `--check-url https://pypi.org/simple/slackblocks/`, so files already
  uploaded by a partially failed run are skipped instead of erroring.
- **npm** — the publish is a single atomic upload; if it failed, re-running
  simply retries it. If it already succeeded, npm rejects the duplicate
  version, which tells you the registry side is done.
- **GitHub Releases** — the release step skips itself if a release for the
  tag already exists.

After a failure, check:

- PyPI: https://pypi.org/project/slackblocks/ lists the new version with both
  the sdist and the wheel.
- npm: https://www.npmjs.com/package/@nicklambourne/slackblocks shows the new
  version (with provenance).
- GitHub: a Release exists for each of the two tags with the changelog notes.

If a bad artifact was published, do not delete and re-upload: registries
reject reused file names and versions. Yank/deprecate the broken version and
release a new patch version of **both** packages.
