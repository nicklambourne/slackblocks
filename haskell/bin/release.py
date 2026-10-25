#!/usr/bin/env python3
"""Prepare tested Cabal/Haddock archives; publish only from the activated tag."""

import argparse
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import urllib.error
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "preflight", ROOT / ".github/scripts/verify-release.py"
)
preflight = importlib.util.module_from_spec(spec)
spec.loader.exec_module(preflight)
require = preflight.require
API = "https://hackage.haskell.org"
TOOLCHAIN = {"ghc": "9.6.7", "cabal": "3.16.1.0"}


def scratch():
    if Path("/Volumes/Repos").is_dir():
        require(
            all(
                Path("/Volumes", name).is_mount()
                for name in ("Repos", "Cache", "Scratch")
            ),
            "Required external storage is unavailable",
        )
        return "/Volumes/Scratch"
    return os.environ.get("RUNNER_TEMP")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def names(version):
    return [f"slackblocks-{version}.tar.gz", f"slackblocks-{version}-docs.tar.gz"]


def run(root, *args):
    subprocess.run(args, cwd=root / "haskell", check=True)


def identity(root):
    require(
        not preflight.git(root, "status", "--porcelain", "--untracked-files=no"),
        "Commit changes before preparing release artifacts",
    )
    for tool, expected in TOOLCHAIN.items():
        actual = subprocess.check_output([tool, "--numeric-version"], text=True).strip()
        require(actual == expected, f"Use pinned {tool} {expected}")
    # sdist may include untracked source files, so verify its complete file inventory.
    listed = subprocess.check_output(
        ["cabal", "sdist", "--list-only"], cwd=root / "haskell", text=True
    )
    tracked = set(preflight.git(root, "ls-files", "--", "haskell").splitlines())
    for name in listed.splitlines():
        require(
            "haskell/" + Path(name).as_posix().removeprefix("./") in tracked,
            f"Uncommitted archive input: {name}",
        )
    return {
        "version": preflight.validate(root),
        "commit": preflight.git(root, "rev-parse", "HEAD"),
        "toolchain": TOOLCHAIN,
        "dependencies_sha256": digest(root / "haskell/dependencies/locked-9.6.7.json"),
    }


def archive_members(data, prefix):
    result = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:*") as archive:
        for member in archive:
            require(
                not member.issym()
                and not member.islnk()
                and ".." not in Path(member.name).parts
                and (
                    member.name == prefix.rstrip("/") or member.name.startswith(prefix)
                ),
                "Unsafe archive member",
            )
            if member.isfile():
                require(member.name not in result, "Duplicate archive member")
                result[member.name] = archive.extractfile(member).read()
            else:
                require(member.isdir(), "Non-regular archive member")
    require(bool(result), "Empty archive")
    return result


def source(root, directory):
    run(root, "cabal", "check")
    run(root, "cabal", "sdist", f"--output-directory={directory}")


def pack_docs(folder, target, prefix):
    # Cabal's Hackage mode supplies portable HTML/source links. Canonicalize only
    # archive metadata, not documentation content, for reproducible assets.
    with (
        target.open("wb") as output,
        gzip.GzipFile(fileobj=output, mode="wb", filename="", mtime=0) as compressed,
    ):
        with tarfile.open(
            fileobj=compressed, mode="w", format=tarfile.USTAR_FORMAT
        ) as archive:
            for file in sorted(folder.rglob("*")):
                require(not file.is_symlink(), "Symlink in generated documentation")
                if file.is_file():
                    content = file.read_bytes()
                    entry = tarfile.TarInfo(
                        prefix + "/" + file.relative_to(folder).as_posix()
                    )
                    entry.size, entry.mode, entry.mtime = len(content), 0o644, 0
                    archive.addfile(entry, io.BytesIO(content))


def prepare(root, directory):
    record = identity(root)
    version = record["version"]
    directory.mkdir(parents=True, exist_ok=True)
    source(root, directory)
    archive, documentation = [directory / name for name in names(version)]
    members = archive_members(archive.read_bytes(), f"slackblocks-{version}/")
    with tempfile.TemporaryDirectory(prefix="haskell-release-", dir=scratch()) as temp:
        project = Path(temp)
        policy = json.loads(
            (root / "haskell/dependencies/locked-9.6.7.json").read_text()
        )
        config = (
            f"packages: {archive}\ntests: True\nindex-state: {policy['index_state']}\n"
        )
        config += (
            "constraints: "
            + ", ".join(f"any.{k} =={v}" for k, v in policy["constraints"].items())
            + "\n"
        )
        config += "package aeson\n  flags: +ordered-keymap\n"
        (project / "cabal.project").write_text(config)
        build = project / "build"
        command = ["cabal", f"--project-dir={project}", f"--builddir={build}"]
        run(root, *command, "test", "all", "--jobs=2", "--test-show-details=direct")
        plan = json.loads((build / "cache/plan.json").read_text())
        resolved = {p["pkg-name"]: p["pkg-version"] for p in plan["install-plan"]}
        require(
            all(resolved.get(k) == v for k, v in policy["constraints"].items()),
            "Release dependency resolution differs",
        )
        record["resolved_dependencies"] = resolved
        run(
            root,
            *command,
            "haddock",
            "lib:slackblocks",
            "--haddock-for-hackage",
            "--haddock-hyperlink-source",
        )
        folders = list(
            build.glob(f"build/*/ghc-9.6.7/slackblocks-{version}/doc/html/slackblocks*")
        )
        folders = [
            p
            for p in folders
            if p.is_dir() and (p / "Slackblocks-Models.html").is_file()
        ]
        require(
            len(folders) == 1,
            f"Expected one native documentation output, found {folders}",
        )
        require(
            "buildPlainText" in (folders[0] / "Slackblocks-Models.html").read_text(),
            "Native Haddock API absent",
        )
        pack_docs(folders[0], documentation, f"slackblocks-{version}-docs")
    record["source_members"] = len(members)
    record["assets"] = {p.name: digest(p) for p in (archive, documentation)}
    (directory / "release.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n"
    )
    return record


def verify(root, directory, rebuild=False):
    record = json.loads((directory / "release.json").read_text())
    require(
        all(record.get(k) == v for k, v in identity(root).items()),
        "Artifact identity differs from the checkout",
    )
    require(
        set(record["assets"]) == set(names(record["version"])),
        "Unexpected release asset inventory",
    )
    for name, expected in record["assets"].items():
        require(
            digest(directory / name) == expected, "Release artifact checksum differs"
        )
        archive_members(
            (directory / name).read_bytes(), name.removesuffix(".tar.gz") + "/"
        )
    if rebuild:
        with tempfile.TemporaryDirectory(
            prefix="haskell-reproduce-", dir=scratch()
        ) as temp:
            source(root, Path(temp))
            name = names(record["version"])[0]
            require(
                digest(Path(temp) / name) == record["assets"][name],
                "Source archive reproduction differs",
            )
    return record


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, fp, code, message, headers, newurl):
        return None


def fetch(path, token=None, absent=False, redirect=False, opener=None):
    require(
        path.startswith("/") and not path.startswith("//"),
        "Expected a Hackage-relative path",
    )
    headers = {"Accept": "application/json", "User-Agent": "slackblocks-release"}
    if token:
        headers["Authorization"] = "X-ApiKey " + token
    request = urllib.request.Request(API + path, headers=headers)
    open_request = opener or urllib.request.build_opener(NoRedirect()).open
    try:
        with open_request(request, timeout=30) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        if absent and error.code == 404:
            error.close()
            return None
        if redirect and error.code in (302, 303, 307):
            location = error.headers.get("Location")
            error.close()
            return location
        raise


def account(root):
    policy = json.loads((root / "haskell/release.json").read_text())
    require(policy["enabled"] is True, "Haskell publication is disabled")
    owner = policy.get("owner")
    require(
        isinstance(owner, str) and re.fullmatch(r"[a-zA-Z0-9_-]+", owner),
        "Verified Hackage owner is missing",
    )
    token = os.environ.get("HACKAGE_TOKEN", "")
    require(
        re.fullmatch(r"[a-zA-Z0-9]+", token) is not None,
        "Hackage API token is missing or malformed",
    )
    location = fetch("/users/account-management", token=token, redirect=True)
    require(
        location in (f"/user/{owner}/manage", f"{API}/user/{owner}/manage"),
        "Token owner differs from release policy",
    )
    enabled = json.loads(fetch(f"/user/{owner}/enabled.json", token=token))
    require(enabled.get("enabled") is True, "Hackage owner account is disabled")
    uploaders = json.loads(fetch("/packages/uploaders.json"))
    require(
        owner in {entry["username"] for entry in uploaders["members"]},
        "Owner is not a Hackage uploader",
    )
    package = fetch("/package/slackblocks.json", absent=True)
    if package is None:
        require(
            policy["first_publication"] is True,
            "Package missing outside approved first-publication mode",
        )
    else:
        maintainers = json.loads(fetch("/package/slackblocks/maintainers.json"))
        require(
            owner in {entry["username"] for entry in maintainers["members"]},
            "Owner does not maintain slackblocks",
        )
    return token


def registry_path(version, documentation=False):
    # /docs.tar is the stored tar stream; /docs/ is the rendered index.
    return (
        f"/package/slackblocks-{version}/docs.tar"
        if documentation
        else f"/package/slackblocks-{version}/slackblocks-{version}.tar.gz"
    )


def matches(remote, path, documentation=False):
    if remote is None:
        return False
    if documentation:
        prefix = path.name.removesuffix(".tar.gz") + "/"
        require(
            archive_members(remote, prefix)
            == archive_members(path.read_bytes(), prefix),
            "Published documentation content differs; do not replace it automatically",
        )
    else:
        require(
            hashlib.sha256(remote).hexdigest() == digest(path),
            "Published source archive checksum differs",
        )
    return True


def upload(path, token, documentation):
    with tempfile.TemporaryDirectory(
        prefix="hackage-credential-", dir=scratch()
    ) as temp:
        config = Path(temp) / "config"
        descriptor = os.open(config, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "w") as stream:
            stream.write(
                f"repository hackage.haskell.org\n  url: {API}/\n  secure: True\nhackage-token: {token}\n"
            )
        command = ["cabal", f"--config-file={config}", "upload", "--publish"]
        if documentation:
            command.append("--documentation")
        # Do not echo credential-bearing tool output. A failed/uncertain write
        # stops; an explicit rerun first inspects registry bytes before any write.
        result = subprocess.run([*command, str(path)], capture_output=True)
        require(
            result.returncode == 0,
            "Hackage upload failed or delivery is uncertain; inspect registry before retrying",
        )


def publish(root, directory):
    version = preflight.publisher(root, "haskell", os.environ.get("GITHUB_REF", ""))
    record = verify(root, directory, rebuild=True)
    token = account(root)
    require(record["version"] == version, "Publisher version differs")
    for documentation, name in enumerate(names(version)):
        path = directory / name
        remote = fetch(registry_path(version, bool(documentation)), absent=True)
        if not matches(remote, path, bool(documentation)):
            upload(path, token, bool(documentation))
            require(
                matches(
                    fetch(registry_path(version, bool(documentation)), absent=True),
                    path,
                    bool(documentation),
                ),
                "Published artifact is not yet verifiable; stop and inspect before retrying",
            )
    # Cabal metadata revisions are mutable independently of the immutable tar.
    original = archive_members(
        (directory / names(version)[0]).read_bytes(), f"slackblocks-{version}/"
    )
    require(
        fetch(f"/package/slackblocks-{version}/slackblocks.cabal")
        == original[f"slackblocks-{version}/slackblocks.cabal"],
        "Registry Cabal metadata differs from the tested release",
    )


def consumer(root):
    version = preflight.validate(root)
    with tempfile.TemporaryDirectory(prefix="hackage-consumer-", dir=scratch()) as temp:
        folder = Path(temp)
        shutil.copy2(root / "haskell/consumer/Main.hs", folder / "Main.hs")
        (folder / "consumer.cabal").write_text(
            f"cabal-version: 3.0\nname: registry-consumer\nversion: 0.0.0\nbuild-type: Simple\nexecutable registry-consumer\n  main-is: Main.hs\n  default-language: GHC2021\n  build-depends: base, slackblocks =={version}, aeson, containers\n"
        )
        (folder / "cabal.project").write_text("packages: .\n")
        subprocess.run(["cabal", "update"], cwd=folder, check=True)
        subprocess.run(
            ["cabal", "run", "registry-consumer", "--jobs=2"], cwd=folder, check=True
        )
        plan = json.loads((folder / "dist-newstyle/cache/plan.json").read_text())
        own = [p for p in plan["install-plan"] if p["pkg-name"] == "slackblocks"]
        require(
            len(own) == 1
            and own[0]["pkg-version"] == version
            and own[0]["pkg-src"]["type"] == "repo-tar",
            "Consumer did not resolve the public registry package",
        )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command", choices=["prepare", "verify", "readiness", "publish", "consumer"]
    )
    args = parser.parse_args()
    directory = ROOT / "haskell/dist-release"
    if args.command == "readiness":
        preflight.coordinator(
            ROOT, None, "workflow_dispatch", os.environ.get("GITHUB_REF", "")
        )
        account(ROOT)
    elif args.command == "prepare":
        prepare(ROOT, directory)
    elif args.command == "verify":
        verify(ROOT, directory, rebuild=True)
    elif args.command == "publish":
        publish(ROOT, directory)
    else:
        consumer(ROOT)
    print(f"Haskell {args.command} checks passed.")


if __name__ == "__main__":
    main()
