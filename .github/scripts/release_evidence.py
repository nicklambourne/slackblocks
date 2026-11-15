"""Verify public immutable tags/assets/docs; combine complete release evidence."""

import argparse
import base64
import hashlib
import io
import json
import os
import re
import subprocess
import tarfile
import zipfile
from pathlib import Path

import release_availability as availability
from public_release import aggregate, require
from release_coordinator import PREFIXES, api


def expected_assets(language, version):
    return {
        "php": {"release.json", "slackblocks-php.zip"},
        "elixir": {"release.json", "slackblocks.tar"},
        "haskell": {
            "release.json",
            f"slackblocks-{version}.tar.gz",
            f"slackblocks-{version}-docs.tar.gz",
        },
    }.get(language, set())


def tar_files(data):
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:*") as archive:
        for member in archive:
            name = Path(member.name)
            require(
                not name.is_absolute()
                and ".." not in name.parts
                and (member.isfile() or member.isdir()),
                "Unsafe release archive member",
            )
            if member.isfile():
                require(member.name not in files, "Duplicate archive member")
                files[member.name] = archive.extractfile(member).read()
    require(files, "Empty release archive")
    return files


def verify_asset_contents(language, version, source, repo, data, root):
    require(
        set(data) == expected_assets(language, version),
        f"Missing or unexpected {language} release assets",
    )
    if not data:
        return
    record = json.loads(data["release.json"])
    commit_key = "source_commit" if language == "php" else "commit"
    require(
        record.get("version") == version and record.get(commit_key) == source,
        "Release manifest differs from immutable source/version",
    )
    archives = {
        name: hashlib.sha256(blob).hexdigest()
        for name, blob in data.items()
        if name != "release.json"
    }
    expected = (
        record.get("assets")
        if language == "haskell"
        else {next(iter(archives)): record.get("sha256")}
    )
    require(archives == expected, "Release manifest archive checksum/inventory differs")
    tracked = set(
        subprocess.check_output(
            ["git", "ls-tree", "-r", "--name-only", source], cwd=root, text=True
        ).splitlines()
    )

    def source_content(path):
        require(path in tracked, f"Archive input is not tracked in the release: {path}")
        return (root / path).read_bytes()

    if language == "php":
        require(
            record.get("repository") == repo
            and record.get("package") == "nicklambourne/slackblocks",
            "Wrong PHP package identity",
        )
        with zipfile.ZipFile(io.BytesIO(data["slackblocks-php.zip"])) as archive:
            require(
                len(archive.namelist()) == len(set(archive.namelist())),
                "Duplicate PHP archive member",
            )
            files = {name: archive.read(name) for name in archive.namelist()}
        require(
            record.get("files")
            == {name: hashlib.sha256(blob).hexdigest() for name, blob in files.items()},
            "PHP file checksums differ",
        )
        required = {
            name
            for name in tracked
            if name.startswith("php/src/") and name.endswith(".php")
        }
        required |= {
            "composer.json",
            "php/IMPLEMENTATION.md",
            "php/README.md",
            "php/CHANGELOG.md",
            "php/LICENSE",
            "php/LICENSE.BSD-3-Clause",
            "php/examples/consumer.php",
        }
        require(set(files) == required, "PHP archive source inventory differs")
        require(
            all(blob == source_content(name) for name, blob in files.items()),
            "PHP archive differs from tagged source",
        )
    elif language == "elixir":
        require(
            record.get("lock_sha256")
            == hashlib.sha256(source_content("elixir/mix.lock")).hexdigest(),
            "Hex lockfile differs from tagged source",
        )
        outer = tar_files(data["slackblocks.tar"])
        files = tar_files(outer["contents.tar.gz"])
        roots = {
            "lib",
            "mix.exs",
            "README.md",
            "CHANGELOG.md",
            "LICENSE",
            "LICENSE.BSD-3-Clause",
        }
        required = {
            name.removeprefix("elixir/")
            for name in tracked
            if name.startswith("elixir/") and name.split("/")[1] in roots
        }
        require(set(files) == required, "Hex archive source inventory differs")
        require(
            all(
                blob == source_content("elixir/" + name) for name, blob in files.items()
            ),
            "Hex archive differs from tagged source",
        )
    else:
        require(
            record.get("dependencies_sha256")
            == hashlib.sha256(
                source_content("haskell/dependencies/locked-9.6.7.json")
            ).hexdigest(),
            "Hackage dependency inputs differ from tagged source",
        )
        prefix = f"slackblocks-{version}/"
        files = tar_files(data[f"slackblocks-{version}.tar.gz"])
        require(
            all(name.startswith(prefix) for name in files), "Wrong Hackage archive root"
        )
        files = {name.removeprefix(prefix): blob for name, blob in files.items()}
        require(
            record.get("source_members") == len(files),
            "Hackage source inventory differs",
        )
        required = {
            name.removeprefix("haskell/")
            for name in tracked
            if name.startswith("haskell/src/") and name.endswith(".hs")
        }
        require(
            required | {"slackblocks.cabal", "README.md", "LICENSE"} <= set(files),
            "Hackage source modules are missing",
        )
        require(
            all(
                blob == source_content("haskell/" + name)
                for name, blob in files.items()
            ),
            "Hackage archive differs from tagged source",
        )
        docs = tar_files(data[f"slackblocks-{version}-docs.tar.gz"])
        prefix = f"slackblocks-{version}-docs/"
        require(
            all(name.startswith(prefix) for name in docs)
            and prefix + "Slackblocks-Models.html" in docs,
            "Missing Hackage API documentation asset",
        )


def documentation(version, folder):
    urls = {
        "rust": f"https://docs.rs/slackblocks/{version}/slackblocks/index.html",
        "elixir": f"https://hexdocs.pm/slackblocks/{version}/Slackblocks.html",
        "haskell": f"https://hackage.haskell.org/package/slackblocks-{version}/docs/Slackblocks-Models.html",
    }
    result = []
    for language, url in urls.items():
        with availability.open_response(url) as response:
            data, final = response.read(), response.url
        text = data.decode()
        require(
            version in final
            and version in text
            and "slackblocks" in text.lower()
            and "<html" in text.lower()
            and len(data) > 10000,
            "Missing or wrong registry API documentation",
        )
        (folder / f"{language}.html").write_bytes(data)
        result.append(
            dict(language=language, url=final, sha256=hashlib.sha256(data).hexdigest())
        )
    return result


def website(repo, version, source, folder):
    base = "https://nicklambourne.github.io/slackblocks/"
    result = []
    versions = api(f"repos/{repo}/contents/docs/versions.json?ref={source}")
    frozen = json.loads(base64.b64decode(versions["content"]))[0]
    for target in (version, frozen):
        url = base + target + "/usage/installation"
        try:
            response = availability.open_response(url)
            marker = "docs-version-" + target
        except availability.Pending as error:
            if target != version or getattr(error.__cause__, "code", None) != 404:
                raise
            response = availability.open_response(base + "usage/installation")
            marker = "docs-version-current"
        with response:
            data, final = response.read(), response.url
        text = data.decode()
        require(marker in text, "Deployed documentation version marker differs")
        if marker == "docs-version-current" and not re.search(
            r'<option value="current" selected="">' + re.escape(version) + r"</option>",
            text,
        ):
            raise availability.Pending(
                "Current documentation has not deployed the release version"
            )
        (folder / f"website-{target}.html").write_bytes(data)
        result.append(
            dict(version=target, url=final, sha256=hashlib.sha256(data).hexdigest())
        )
    return result


def public_identity(repo, version, source, folder, source_root):
    require(
        subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=source_root, text=True
        ).strip()
        == source,
        "Identity source checkout differs",
    )
    require(
        not subprocess.check_output(
            ["git", "status", "--porcelain", "--untracked-files=no"],
            cwd=source_root,
            text=True,
        ).strip(),
        "Identity source checkout has tracked modifications",
    )
    folder.mkdir(parents=True, exist_ok=True)
    receipt = folder / "identity.json"
    identity = dict(version=version, source=source)

    def check():
        releases = []
        for language, prefix in PREFIXES.items():
            tag = f"{prefix}/v{version}"
            ref = api(f"repos/{repo}/git/ref/tags/{tag}")
            require(ref["object"]["type"] == "tag", "Release tag is not annotated")
            annotated = api(f"repos/{repo}/git/tags/{ref['object']['sha']}")
            require(
                annotated["object"]["type"] == "commit"
                and annotated["object"]["sha"] == source,
                "Release tag targets a different source",
            )
            release = api(f"repos/{repo}/releases/tags/{tag}")
            require(
                release["tag_name"] == tag
                and not release["draft"]
                and not release["prerelease"]
                and release["published_at"]
                and release["body"].strip(),
                "Incomplete public GitHub release",
            )
            assets = []
            names = [asset["name"] for asset in release["assets"]]
            require(
                len(names) == len(set(names))
                and set(names) == expected_assets(language, version),
                f"Missing or unexpected {language} release assets",
            )
            downloaded = {}
            for asset in release["assets"]:
                require(Path(asset["name"]).name == asset["name"], "Unsafe asset name")
                data = subprocess.check_output(
                    [
                        "gh",
                        "api",
                        f"repos/{repo}/releases/assets/{asset['id']}",
                        "-H",
                        "Accept: application/octet-stream",
                    ]
                )
                digest = hashlib.sha256(data).hexdigest()
                require(
                    len(data) == asset["size"]
                    and asset.get("digest") == "sha256:" + digest,
                    "Public asset differs from GitHub's recorded digest",
                )
                downloaded[asset["name"]] = data
                assets.append(dict(id=asset["id"], name=asset["name"], sha256=digest))
            verify_asset_contents(
                language, version, source, repo, downloaded, source_root
            )
            releases.append(
                dict(
                    language=language,
                    tag=tag,
                    commit=source,
                    url=release["html_url"],
                    assets=assets,
                )
            )
        alias = api(f"repos/{repo}/git/ref/tags/v{version}")["object"]
        if alias["type"] == "tag":
            alias = api(f"repos/{repo}/git/tags/{alias['sha']}")["object"]
        require(
            alias["type"] == "commit" and alias["sha"] == source,
            "Composer alias differs from release source",
        )
        return dict(
            releases=releases,
            documentation=documentation(version, folder),
            website=website(repo, version, source, folder),
        )

    result = availability.wait(check, receipt, identity)
    availability.finish(receipt, state="verified", passed=True, **result)


def complete(directory, version, source, coordinator_run):
    public = aggregate(directory / "consumers", version, source)
    identity = json.loads((directory / "identity/identity.json").read_text())
    require(
        identity.get("passed") is True
        and identity.get("state") == "verified"
        and identity.get("version") == version
        and identity.get("source") == source,
        "Public release identity is incomplete",
    )
    require(
        {r["language"] for r in identity["releases"]} == set(PREFIXES),
        "Missing public releases",
    )
    require(
        {d["language"] for d in identity.get("documentation", [])}
        == {"rust", "elixir", "haskell"}
        and len(identity.get("website", [])) == 2
        and version in {d["version"] for d in identity["website"]},
        "Registry or current/frozen documentation evidence is missing",
    )
    records = [
        json.loads(p.read_text())
        for p in (directory / "publishers").rglob("publisher.json")
    ]
    require(
        len(records) == len(PREFIXES)
        and {r["language"] for r in records} == set(PREFIXES),
        "Missing or duplicate publisher records",
    )
    require(
        all(
            r.get("passed") is True
            and r.get("state") == "publisher_verified"
            and r.get("source") == source
            and r.get("version") == version
            and r.get("coordinator_run") == str(coordinator_run)
            and r.get("run_id")
            for r in records
        ),
        "Publishers are pending, failed or from a different coordinator",
    )
    require(
        all(r.get("run_id") == str(coordinator_run) for r in public["languages"]),
        "Consumers belong to another coordinator",
    )
    return dict(
        schema=1,
        version=version,
        source=source,
        complete=True,
        publishers=records,
        public=public,
        identity=identity,
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--directory", required=True, type=Path)
    parser.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY"))
    parser.add_argument("--complete", action="store_true")
    parser.add_argument("--source-root", type=Path)
    args = parser.parse_args()
    require(
        re.fullmatch(r"\d+\.\d+\.\d+", args.version)
        and re.fullmatch(r"[0-9a-f]{40}", args.source),
        "Invalid release identity",
    )
    if args.complete:
        result = complete(
            args.directory, args.version, args.source, os.environ["GITHUB_RUN_ID"]
        )
        availability.save(args.directory / "release-completion.json", result)
    else:
        require(
            args.source_root,
            "Public identity verification requires the immutable source checkout",
        )
        public_identity(
            args.repo,
            args.version,
            args.source,
            args.directory,
            args.source_root.resolve(),
        )


if __name__ == "__main__":
    main()
