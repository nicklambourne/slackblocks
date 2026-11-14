"""Verify public immutable tags/assets/docs; combine complete release evidence."""

import argparse
import base64
import hashlib
import json
import os
import re
import subprocess
from pathlib import Path

import release_availability as availability
from public_release import aggregate, require
from release_coordinator import PREFIXES, api


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
        result.append(dict(language=language, url=final, sha256=hashlib.sha256(data).hexdigest()))
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
            r'<option value="current" selected="">' + re.escape(version) + r"</option>", text
        ):
            raise availability.Pending("Current documentation has not deployed the release version")
        (folder / f"website-{target}.html").write_bytes(data)
        result.append(dict(version=target, url=final, sha256=hashlib.sha256(data).hexdigest()))
    return result


def public_identity(repo, version, source, folder):
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
                annotated["object"]["type"] == "commit" and annotated["object"]["sha"] == source,
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
                    len(data) == asset["size"] and asset.get("digest") == "sha256:" + digest,
                    "Public asset differs from GitHub's recorded digest",
                )
                assets.append(dict(id=asset["id"], name=asset["name"], sha256=digest))
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
        {r["language"] for r in identity["releases"]} == set(PREFIXES), "Missing public releases"
    )
    require(
        {d["language"] for d in identity.get("documentation", [])} == {"rust", "elixir", "haskell"}
        and len(identity.get("website", [])) == 2
        and version in {d["version"] for d in identity["website"]},
        "Registry or current/frozen documentation evidence is missing",
    )
    records = [
        json.loads(p.read_text()) for p in (directory / "publishers").rglob("publisher.json")
    ]
    require(
        len(records) == len(PREFIXES) and {r["language"] for r in records} == set(PREFIXES),
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
    args = parser.parse_args()
    require(
        re.fullmatch(r"\d+\.\d+\.\d+", args.version) and re.fullmatch(r"[0-9a-f]{40}", args.source),
        "Invalid release identity",
    )
    if args.complete:
        result = complete(args.directory, args.version, args.source, os.environ["GITHUB_RUN_ID"])
        availability.save(args.directory / "release-completion.json", result)
    else:
        public_identity(args.repo, args.version, args.source, args.directory)


if __name__ == "__main__":
    main()
