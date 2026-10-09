# Releasing

The Python (`slackblocks` on PyPI), TypeScript
(`@nicklambourne/slackblocks` on npm), Go
(`github.com/nicklambourne/slackblocks/go/v2`), Java
(`io.github.nicklambourne:slackblocks` on Maven Central), C# (`Slackblocks` on NuGet), Ruby (`slackblocks` on RubyGems), and Rust (`slackblocks` on crates.io)
packages are released together and always carry the same version
number.

**Current preparation state:** six languages are published at 2.5.0. The source
train targets 2.6.0 and adds Rust, but Rust's `publish = false` disables release
activation. The coordinator and every publisher now run a shared preflight that
blocks a partial six-language 2.6.0 release. PR validation and package dry runs
remain available without credentials. Complete [Rust activation](#rust-activation-and-first-publication)
before dispatching any 2.6.0 publisher. No account setup is implied by these files.

The recommended entry point is the **Coordinated Release** workflow in GitHub
Actions. Run it from `master` with one `X.Y.Z` input; it validates the shared
version and changelogs, creates all seven annotated tags in one atomic push,
dispatches each publisher at its tag, and waits for all seven runs.

When adding another language package, complete the
[new language implementation workflow](docs/docs/contributing/adding-a-language.mdx) before
activating its publisher. That guide covers registry bootstrap, dry-run checks,
coordinator integration and documentation history alongside implementation parity.

## Tag scheme

Releases are triggered by pushing tags:

| Tag | Workflow | Publishes |
|---|---|---|
| `python/vX.Y.Z` | [`.github/workflows/publish.yml`](.github/workflows/publish.yml) | `slackblocks` to PyPI |
| `ts/vX.Y.Z` | [`.github/workflows/publish-npm.yml`](.github/workflows/publish-npm.yml) | `@nicklambourne/slackblocks` to npm |
| `go/vX.Y.Z` | [`.github/workflows/publish-go.yml`](.github/workflows/publish-go.yml) | `github.com/nicklambourne/slackblocks/go/v2` to the Go module ecosystem |
| `java/vX.Y.Z` | [`.github/workflows/publish-java.yml`](.github/workflows/publish-java.yml) | `io.github.nicklambourne:slackblocks` to Maven Central |
| `csharp/vX.Y.Z` | [`.github/workflows/publish-nuget.yml`](.github/workflows/publish-nuget.yml) | `Slackblocks` to NuGet |
| `ruby/vX.Y.Z` | [`.github/workflows/publish-rubygems.yml`](.github/workflows/publish-rubygems.yml) | `slackblocks` to RubyGems |
| `rust/vX.Y.Z` | [`.github/workflows/publish-crates.yml`](.github/workflows/publish-crates.yml) | `slackblocks` to crates.io; disabled until activation |

Plain `v*` tags (used by the pre-monorepo 1.x/2.0 releases) no longer trigger
anything.

The seven publisher workflows also accept a manual dispatch at an existing,
matching language tag. The coordinator uses those entry points so each job
retains its registry-specific publisher workflow identity.

The coordinator verifies that `python/pyproject.toml`,
`typescript/package.json`, `java/pom.xml`,
`csharp/src/Slackblocks/Slackblocks.csproj`,
`ruby/lib/slackblocks/version.rb`, and `rust/Cargo.toml` agree. The shared guard
also verifies the Python, Ruby and Cargo lockfiles, Go major module path, all seven
changelogs, clean checkout, master ancestry and the complete annotated tag set.
Every publisher calls `.github/scripts/verify-release.py`; Ruby’s historical
`ruby/bin/verify-release` command delegates to the same guard. Provider-specific
installed-package checks remain in their workflows.

## One-time setup

These must be in place before the workflows can publish:

1. **GitHub environments** — create `pypi`, `npm`, `maven-central`, `nuget`, `rubygems`, and `crates-io`
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

7. **RubyGems trusted publishing** — the first gem was published in 2.5.0.
   Keep the trusted publisher for gem `slackblocks`, repository
   `nicklambourne/slackblocks`, workflow `publish-rubygems.yml`, and environment
   `rubygems`. Configure a required reviewer for the GitHub environment if
   release approval is desired. The publisher exchanges GitHub OIDC for a
   short-lived credential; it needs no RubyGems API key.

## Rust activation and first publication

The implementation train leaves publishing disabled. The following is a separate,
reviewable release-readiness change; merging an implementation PR does not authorize
a release or establish registry ownership.

1. Recheck availability of `slackblocks` on crates.io. The name was absent on
   4 October 2026, which does not reserve it. Sign in with the intended maintainer
   account and verify its email. Do not publish a placeholder crate to reserve it.
2. Create the GitHub environment `crates-io`. Restrict deployment refs to
   `rust/v*` and configure the intended reviewer policy. The workflow identity is
   repository `nicklambourne/slackblocks`, file **`publish-crates.yml`**, environment
   **`crates-io`**. The protected publish job is separate from PR/build validation;
   PR jobs have neither registry secrets nor `id-token: write`.
3. The initial publication needs an API token because trusted publishing currently
   requires an existing crate. Create a short-lived token with permission to
   publish a new crate, restricted to `slackblocks`; save it only as the environment
   secret **`CRATES_IO_BOOTSTRAP_TOKEN`**. Verify current controls using the official
   [token guidance](https://blog.rust-lang.org/2023/06/23/improved-api-tokens-for-crates-io/)
   and [trusted publishing documentation](https://crates.io/docs/trusted-publishing).
   Never place credentials in source, command arguments or release notes.
4. In a release-readiness PR, change only the library's `publish = false` to
   `publish = ["crates-io"]` in `rust/Cargo.toml`. The conformance crate stays
   unpublished. Keep the checked-in first-release policy:

   ```toml
   [package.metadata.slackblocks-release]
   authentication = "bootstrap"
   bootstrap-version = "2.6.0"
   ```

   Bootstrap is allowed only for 2.6.0 while the crate is absent. There is no
   automatic token fallback when OIDC fails. All other release prerequisites below
   must be completed in the same reviewed state: seven dated changelogs, Java's
   reproducible timestamp, matching versions/lockfiles and green checks.
5. The outgoing six-language 2.5.0 documentation is already frozen; **do not
   create it again**. Remove the current docs `Unreleased` label when the release
   is ready. Update registry installation snippets across root/package READMEs
   and guides to 2.6.0, including Rust's `cargo add slackblocks@2.6.0` alongside
   `serde_json`. Replace Rust's static unpublished badge with the live crates.io
   badge and add the live docs.rs link. Re-run extracted snippets, generated API
   checks and the complete documentation build before publication.
6. Run the existing Rust matrix and quality/coverage checks. The release workflow
   additionally checks extracted archives on MSRV/stable, minimum dependencies and
   unified serde_json features, then exercises `cargo publish --dry-run` using a
   disposable archive copy. That dry run activates only the temporary manifest;
   it never changes the source manifest or uploads a crate. Account configuration
   still requires a maintainer's verification. Register the Rust matrix/quality,
   release-build and docs checks in branch protection once their workflows exist
   on the default branch; verify actual test steps and GitHub runner metadata.
7. After a separately authorized merge/release, dispatch **Coordinated Release**
   from `master`. It creates all seven annotated tags atomically and monitors all
   seven publishers. Approve `crates-io` if configured. Rust authenticates only
   after its release guards and artifact comparison pass.
8. Verify the exact version on crates.io and its rustdoc build on docs.rs. Compile
   and execute a fresh consumer with `slackblocks = "=2.6.0"` on MSRV and stable,
   with no path/patch override. Confirm package contents and a representative
   serialized payload. Record the run URL, registry version, checksum and docs.rs
   result. A dry run cannot substitute for this installed-registry check.
9. Configure the crate's trusted publisher for the exact repository, workflow and
   environment in step 2. Revoke the bootstrap API token and delete its environment
   secret. In a follow-up readiness change, set `authentication = "trusted"` and
   remove `bootstrap-version`. The pinned official authentication action obtains
   and revokes a short-lived credential. The next coordinated release must use
   trusted mode; test its credential exchange when that authorized release runs.

### Rust artifact identity and recovery

The build job uses Cargo **1.85.0** to prepare the distributable and uploads the
`.crate` plus `release.json` containing its commit, version, toolchain, auth mode
and SHA-256. A second job downloads and reproduces it on a fresh runner on every
PR, without registry credentials. The protected publish job downloads that same run’s artifact,
checks all metadata and the checksum, then reproduces the archive with the exact
packaging toolchain. A byte mismatch stops publication before authentication.
Cargo's final publish uses the same clean checkout, lockfile and toolchain;
`--no-verify` avoids repeating compilation after the archive and consumers have
already passed verification. It does not bypass the release/artifact guards.

Registry errors fail closed. An existing target version is never uploaded again,
even if yanked. If crates.io already has the version, verify its checksum against
the recorded artifact, complete the installed-consumer/docs.rs checks, and create
a missing GitHub Release from the existing tag separately. Do not re-run bootstrap
against an existing crate. If only GitHub Release creation failed, re-run that
failed job, rather than the successful publishing job. Never move the tags.

Useful read-only/local checks (Python 3.11+, plus the documented Rust toolchains):

```sh
python3 -m unittest discover -s .github/scripts -p 'test_*.py' -v
GITHUB_EVENT_NAME=pull_request python3 .github/scripts/verify-release.py coordinator
python3 rust/bin/check-publish-dry-run.py
```

These commands neither activate publishing nor create tags. The Cargo dry run
requires a clean committed checkout; use the PR workflow for the exact proposed
commit. Run the guard's dispatch/publisher modes only when testing a concrete
release ref; those modes are also read-only but intentionally reject this disabled
train.

## Coordinated release procedure

The docs site serves the current package version live (`lastVersion:
"current"`) and lists every earlier release as a frozen snapshot in the version
dropdown. Each release therefore has to freeze the version it is moving *off*
before the new version takes over as current — otherwise that version vanishes
from the dropdown. CI enforces this: the `check:release-snapshots` guard fails
if any released `python/v*` tag other than the current package version is
missing from `docs/versions.json` (or the legacy manifest).

The outgoing six-language 2.5.0 snapshot is already frozen for the 2.6.0 Rust
train. Its recorded source commit and hash are in `rust/IMPLEMENTATION.md`. Do not
repeat the snapshot step for this train. Future releases follow the procedure below.

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
   - `ruby/lib/slackblocks/version.rb` (`VERSION`) and the own-gem entries in `ruby/Gemfile.lock`
   - `rust/Cargo.toml` (`package.version`) and the own-crate entry in `rust/Cargo.lock`
   - the pinned versions in the installation examples of `README.md`,
     `java/README.md`, `docs/docs/quick-start.mdx`, and
     `docs/docs/usage/installation.mdx`

   For a new major release, also change the Go semantic import path in
   `go/go.mod` from `/v2` to `/vN`, update every Go import in code and docs,
   and update the shared release preflight’s module-path policy as needed. A coordinated
   3.0.0 release therefore requires a Go `/v3` module; the
   existing `/v2` path cannot publish `go/v3.0.0`.
3. Add a `## [X.Y.Z] — YYYY-MM-DD` section to `python/CHANGELOG.md`,
   `typescript/CHANGELOG.md`, `go/CHANGELOG.md`, `java/CHANGELOG.md`,
   `csharp/CHANGELOG.md`, `ruby/CHANGELOG.md`, and `rust/CHANGELOG.md`. The
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
   `go/vX.Y.Z`, `java/vX.Y.Z`, `csharp/vX.Y.Z`, `ruby/vX.Y.Z`, and `rust/vX.Y.Z`,
   then dispatches and monitors all seven publishers.
   Each publisher creates its own GitHub Release after its registry step
   succeeds.
8. **Java only:** the publisher uploads the bundle and waits for Central to
   validate it, and `autoPublish` then publishes it. Central can take around
   30 minutes to serve a newly published version. Confirm it arrives at
   `https://repo1.maven.org/maven2/io/github/nicklambourne/slackblocks/X.Y.Z/`
   with the binary, sources, and Javadoc JARs, the POM, and a `.asc` signature
   for each. A published version can never be replaced, so fix any problem by
   releasing a new patch version of every package.
9. **Ruby only:** approve the `rubygems` environment deployment if it has
   required reviewers. Confirm `https://rubygems.org/gems/slackblocks/versions/X.Y.Z`
   lists the new gem, then install that exact version in a clean location and
   render a sample payload.
10. **C# only:** approve the `nuget` environment deployment if it has required
   reviewers. NuGet indexes a new version within minutes; confirm
   `https://www.nuget.org/packages/Slackblocks/X.Y.Z` lists it. NuGet versions
   cannot be deleted or re-uploaded, only unlisted.

If the coordinator is unavailable, create all seven signed tags at the same
commit and push them atomically. GitHub does not generate tag-push workflow
runs when more than three tags are pushed at once, so dispatch each publisher
explicitly at its tag after the push:

```sh
GITHUB_EVENT_NAME=workflow_dispatch GITHUB_REF=refs/heads/master REQUESTED_VERSION=X.Y.Z \
  python3 .github/scripts/verify-release.py coordinator
git tag -s python/vX.Y.Z -m "slackblocks X.Y.Z (Python)"
git tag -s ts/vX.Y.Z -m "@nicklambourne/slackblocks X.Y.Z (TypeScript)"
git tag -s go/vX.Y.Z -m "slackblocks X.Y.Z (Go)"
git tag -s java/vX.Y.Z -m "slackblocks X.Y.Z (Java)"
git tag -s csharp/vX.Y.Z -m "Slackblocks X.Y.Z (C#)"
git tag -s ruby/vX.Y.Z -m "slackblocks X.Y.Z (Ruby)"
git tag -s rust/vX.Y.Z -m "slackblocks X.Y.Z (Rust)"
git push --atomic origin python/vX.Y.Z ts/vX.Y.Z go/vX.Y.Z java/vX.Y.Z csharp/vX.Y.Z ruby/vX.Y.Z rust/vX.Y.Z
gh workflow run publish.yml --ref python/vX.Y.Z
gh workflow run publish-npm.yml --ref ts/vX.Y.Z
gh workflow run publish-go.yml --ref go/vX.Y.Z
gh workflow run publish-java.yml --ref java/vX.Y.Z
gh workflow run publish-nuget.yml --ref csharp/vX.Y.Z
gh workflow run publish-rubygems.yml --ref ruby/vX.Y.Z
gh workflow run publish-crates.yml --ref rust/vX.Y.Z
```

Tags created by the coordinator are annotated as `github-actions[bot]` but
are not signed with a maintainer's personal key; CI deliberately does not
store that private key.

## Partial-failure recovery

The seven tags are created atomically, but seven publishing targets cannot be
updated as one transaction. If one publisher fails after another succeeds,
re-run the failed publisher from its existing workflow run; never move the
tags or republish an already released version.

Inspect the registry state before re-running a failed publisher:

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
- **RubyGems** — a gem version cannot be pushed twice. If RubyGems already
  lists the version, treat publication as complete and create a missing GitHub
  Release separately. Re-run the publisher only if the gem is absent.
- **crates.io** — use the Rust artifact/recovery checks above. Re-run only while
  the target version is absent; complete a missing GitHub Release separately.
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
- RubyGems: https://rubygems.org/gems/slackblocks lists the new version.
- crates.io: https://crates.io/crates/slackblocks lists the exact version and
  https://docs.rs/slackblocks has a successful rustdoc build.
- GitHub: a Release exists for each of the seven tags with the changelog notes.

If a bad artifact was published, do not delete and re-upload: registries
reject reused file names and versions. Yank, deprecate, or unlist the broken
version and release a new coordinated patch version of **all seven** packages.
