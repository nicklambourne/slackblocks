# Releasing

The Python (`slackblocks` on PyPI), TypeScript
(`@nicklambourne/slackblocks` on npm), Go
(`github.com/nicklambourne/slackblocks/go/v2`), Java
(`io.github.nicklambourne:slackblocks` on Maven Central), and C# (`Slackblocks`
on NuGet) packages are released together and always carry the same version
number.

The recommended entry point is the **Coordinated Release** workflow in GitHub
Actions. Run it from `master` with one `X.Y.Z` input; it validates the shared
version and changelogs, creates all five annotated tags in one atomic push,
dispatches each publisher at its tag, and waits for all five runs.

## Tag scheme

Releases are triggered by pushing tags:

| Tag | Workflow | Publishes |
|---|---|---|
| `python/vX.Y.Z` | [`.github/workflows/publish.yml`](.github/workflows/publish.yml) | `slackblocks` to PyPI |
| `ts/vX.Y.Z` | [`.github/workflows/publish-npm.yml`](.github/workflows/publish-npm.yml) | `@nicklambourne/slackblocks` to npm |
| `go/vX.Y.Z` | [`.github/workflows/publish-go.yml`](.github/workflows/publish-go.yml) | `github.com/nicklambourne/slackblocks/go/v2` to the Go module ecosystem |
| `java/vX.Y.Z` | [`.github/workflows/publish-java.yml`](.github/workflows/publish-java.yml) | `io.github.nicklambourne:slackblocks` to Maven Central |
| `csharp/vX.Y.Z` | [`.github/workflows/publish-nuget.yml`](.github/workflows/publish-nuget.yml) | `Slackblocks` to NuGet |

Plain `v*` tags (used by the pre-monorepo 1.x/2.0 releases) no longer trigger
anything.

The five publisher workflows also accept a manual dispatch at an existing,
matching language tag. The coordinator uses those entry points so each job
retains its registry-specific publisher workflow identity.

The Python, TypeScript, Java, and C# workflows fail fast if the tag does not
match their package manifest. Every workflow also verifies that
`python/pyproject.toml`, `typescript/package.json`, `java/pom.xml`, and
`csharp/src/Slackblocks/Slackblocks.csproj` agree; the Go workflow requires its
tag to match that coordinated version.

## One-time setup

These must be in place before the workflows can publish:

1. **GitHub environments** — create `pypi`, `npm`, `maven-central`, and `nuget`
   environments in the repository settings (Settings → Environments). The
   publish jobs run inside them; add required reviewers there if you want
   manual approval before publishing.
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
5. **Maven Central namespace, credentials, and signing**
   1. Sign in to the [Central Portal](https://central.sonatype.com) with the
      `nicklambourne` GitHub account.
   2. Open **Namespaces** and add `io.github.nicklambourne`. The portal shows a
      verification key. Create a temporary **public** repository named exactly
      that key under `github.com/nicklambourne`, return to the portal, and
      select **Verify Namespace**. Delete the temporary repository once the
      namespace shows as verified. The first publish fails until this is done.
   3. Open **View Account** → **Generate User Token**. Store the token's
      username and password as `MAVEN_CENTRAL_USERNAME` and
      `MAVEN_CENTRAL_TOKEN` in the `maven-central` environment.
   4. Create a dedicated release signing key with an expiry, and note its key ID
      and expiry date in this file:

      ```sh
      gpg --quick-gen-key "slackblocks release signing <maintainer-email>" rsa4096 sign 2y
      gpg --list-secret-keys --keyid-format long
      gpg --keyserver hkps://keyserver.ubuntu.com --send-keys <KEY_ID>
      gpg --keyserver hkps://keys.openpgp.org --send-keys <KEY_ID>
      gpg --armor --export-secret-keys <KEY_ID>
      ```

      The current key is `16B380B037C8DC16` (fingerprint
      `3897 1573 4B85 1218 2C42  B197 16B3 80B0 37C8 DC16`), created
      2026-09-15 and expiring 2028-09-14. Extend it with
      `gpg --quick-set-expire 16B380B037C8DC16 2y` before then, and publish it
      to both keyservers again.

      keys.openpgp.org emails a confirmation link before it serves the key.
      Central fetches public keys from these servers to verify signatures, so
      publish before the first release and again after extending the expiry.
   5. Store the armored private key as `MAVEN_GPG_PRIVATE_KEY` and its
      passphrase as `MAVEN_GPG_PASSPHRASE` in the `maven-central` environment.

   The Java publisher signs the POM and all three JARs, uploads the bundle, and
   waits until Central has **validated** it. `java/pom.xml` sets
   `autoPublish` to `true`, so a validated bundle publishes without further
   action; the first Java release, 2.3.0, was published by hand. Java CI's
   **Signed Maven Central bundle** job signs a dry-run bundle with a throwaway
   key on every relevant pull request, so signing problems surface before a
   release.
6. **NuGet account and trusted publishing**
   1. Sign in to [nuget.org](https://www.nuget.org) as `nicklambourne` with
      two-factor authentication enabled.
   2. Under **Trusted Publishing**, add a policy for repository owner
      `nicklambourne`, repository `slackblocks`, workflow file
      `publish-nuget.yml`, and environment `nuget`.
   3. Set the repository (or `nuget` environment) variable `NUGET_USER` to the
      nuget.org username. `NuGet/login` exchanges the workflow's OIDC token for
      a short-lived API key for that user, so no long-lived key is stored.
   4. If nuget.org refuses the first push of the new `Slackblocks` package ID
      through trusted publishing, bootstrap it the way npm needed: create an API
      key scoped to push new packages matching `Slackblocks`, store it as the
      `nuget` environment secret `NUGET_API_KEY`, and run the release. The
      publisher uses the key while the secret exists and trusted publishing
      otherwise, so **delete `NUGET_API_KEY`** once the package exists.

   The C# publisher runs the full test suite, packs with package validation, and
   pushes the package and its symbols with `--skip-duplicate`. .NET CI's
   **NuGet package** job packs and pushes to a local feed on every relevant pull
   request, so packaging problems surface before a release.

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
2. Bump the version in every package manifest and the Java and C# version
   constants on the same branch:
   - `python/pyproject.toml` (`project.version`), and the `slackblocks` entry in
     `python/uv.lock`
   - `typescript/package.json` (`version`)
   - `java/pom.xml` (`project.version`)
   - `java/src/main/java/io/github/nicklambourne/slackblocks/Slackblocks.java`
     (`VERSION`; Java CI verifies that it matches the POM)
   - `csharp/src/Slackblocks/Slackblocks.csproj` (`Version`)
   - `csharp/src/Slackblocks/SlackblocksInfo.cs` (`Version`; the C# tests verify
     that it matches the project)
   - the pinned versions in the installation examples of `README.md`,
     `java/README.md`, `docs/docs/quick-start.mdx`, and
     `docs/docs/usage/installation.mdx`

   For a new major release, also change the Go semantic import path in
   `go/go.mod` from `/v2` to `/vN`, update every Go import in code and docs,
   and update the module-path assertion in `publish-go.yml`. A coordinated
   3.0.0 release therefore requires a Go `/v3` module; the
   existing `/v2` path cannot publish `go/v3.0.0`.
3. Add a `## [X.Y.Z] — YYYY-MM-DD` section to `python/CHANGELOG.md`,
   `typescript/CHANGELOG.md`, `go/CHANGELOG.md`, `java/CHANGELOG.md`, and
   `csharp/CHANGELOG.md`. The
   publish workflows extract the matching section for their GitHub Release
   notes. Pull requests may use `Unreleased` while the release is prepared, but
   replace it with the release date before dispatching: the coordinator rejects
   headings without a date, and dates that differ between changelogs.
4. Set `project.build.outputTimestamp` in `java/pom.xml` to the same date,
   such as `2026-09-20T00:00:00Z`. The value makes the Java JARs reproducible,
   and the coordinator rejects a release whose timestamp does not match the
   changelog date.
5. Merge to `master` and wait for CI to pass.
6. In GitHub Actions, open **Coordinated Release**, select **Run workflow**,
   keep the branch set to `master`, and enter `X.Y.Z`. The equivalent CLI
   command is:

   ```sh
   gh workflow run coordinated-release.yml --ref master -f version=X.Y.Z
   ```

7. The coordinator atomically pushes `python/vX.Y.Z`, `ts/vX.Y.Z`,
   `go/vX.Y.Z`, `java/vX.Y.Z`, and `csharp/vX.Y.Z`, then dispatches and
   monitors all five publishers.
   Each publisher creates its own GitHub Release after its registry step
   succeeds.
8. **Java only:** the publisher uploads the bundle and waits for Central to
   validate it, and `autoPublish` then publishes it. Central can take around
   30 minutes to serve a newly published version. Confirm it arrives at
   `https://repo1.maven.org/maven2/io/github/nicklambourne/slackblocks/X.Y.Z/`
   with the binary, sources, and Javadoc JARs, the POM, and a `.asc` signature
   for each. A published version can never be replaced, so fix any problem by
   releasing a new patch version of every package.
9. **C# only:** approve the `nuget` environment deployment if it has required
   reviewers. NuGet indexes a new version within minutes; confirm
   `https://www.nuget.org/packages/Slackblocks/X.Y.Z` lists it. NuGet versions
   cannot be deleted or re-uploaded, only unlisted.

If the coordinator is unavailable, the direct tag triggers remain as a manual
fallback. Create all five signed tags at the same commit and push them
atomically:

```sh
git tag -s python/vX.Y.Z -m "slackblocks X.Y.Z (Python)"
git tag -s ts/vX.Y.Z -m "@nicklambourne/slackblocks X.Y.Z (TypeScript)"
git tag -s go/vX.Y.Z -m "slackblocks X.Y.Z (Go)"
git tag -s java/vX.Y.Z -m "slackblocks X.Y.Z (Java)"
git tag -s csharp/vX.Y.Z -m "Slackblocks X.Y.Z (C#)"
git push --atomic origin python/vX.Y.Z ts/vX.Y.Z go/vX.Y.Z java/vX.Y.Z csharp/vX.Y.Z
```

Tags created by the coordinator are annotated as `github-actions[bot]` but
are not signed with a maintainer's personal key; CI deliberately does not
store that private key.

## Partial-failure recovery

The five tags are created atomically, but five external registries cannot be
updated as one transaction. If one publisher fails after another succeeds,
re-run the failed publisher from its existing workflow run; never move the
tags or republish an already released version.

All five workflows are safe to re-run from the Actions UI:

- **PyPI** — `uv publish` runs with
  `--check-url https://pypi.org/simple/slackblocks/`, so files already
  uploaded by a partially failed run are skipped instead of erroring.
- **npm** — the publish is a single atomic upload; if it failed, re-running
  simply retries it. If it already succeeded, npm rejects the duplicate
  version, which tells you the registry side is done.
- **Go** — the tag is the published module version. Re-run the workflow if only
  verification or GitHub Release creation failed; do not move or replace a
  public module tag.
- **Maven Central** — published versions are immutable. If the Central
  Portal shows the deployment as **validated** but not published, publish or
  drop it in the portal rather than re-running the workflow: a second upload of
  the same version is rejected. Re-run the workflow only if no deployment for
  the version exists, or after dropping a failed one. If Central already lists
  the version, treat the registry step as complete.
- **NuGet** — the push uses `--skip-duplicate`, so a version NuGet already holds
  is skipped rather than failing, and re-running completes whatever is missing,
  such as the symbols package or the GitHub Release.
- **GitHub Releases** — the release step skips itself if a release for the
  tag already exists.

After a failure, check:

- PyPI: https://pypi.org/project/slackblocks/ lists the new version with both
  the sdist and the wheel.
- npm: https://www.npmjs.com/package/@nicklambourne/slackblocks shows the new
  version (with provenance).
- Go: https://pkg.go.dev/github.com/nicklambourne/slackblocks/go/v2 lists the
  new module version.
- Maven Central: https://central.sonatype.com/artifact/io.github.nicklambourne/slackblocks
  lists the new artifact version with binary, source, and Javadoc JARs.
- NuGet: https://www.nuget.org/packages/Slackblocks lists the new version.
- GitHub: a Release exists for each of the five tags with the changelog notes.

If a bad artifact was published, do not delete and re-upload: registries
reject reused file names and versions. Yank, deprecate, or unlist the broken
version and release a new coordinated patch version of **all five** packages.
