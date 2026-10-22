#!/usr/bin/env python3
"""Disabled-by-default, immutable Hex publication and recovery from one tested commit."""

import argparse
import hashlib
import importlib.util
import json
import os
import subprocess
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

from package_check import PACKAGE_ROOTS, extract_archive

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location(
    "preflight", ROOT / ".github/scripts/verify-release.py"
)
preflight = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(preflight)
require = preflight.require
TOOLCHAIN = {"elixir": "1.16.3", "otp": "26.2", "hex": "2.5.1"}
ARCHIVE = "slackblocks.tar"
API = "https://hex.pm/api"


def scratch():
    if Path("/Volumes/Repos").is_dir():
        require(
            all(Path(f"/Volumes/{p}").is_dir() for p in ["Cache", "Scratch"]),
            "External storage is unavailable",
        )
        return "/Volumes/Scratch"
    return None


def run(root, *args):
    subprocess.run(
        args,
        cwd=root / "elixir",
        check=True,
        env={**os.environ, "SLACKBLOCKS_SOURCE_REF": preflight.git(root, "rev-parse", "HEAD")},
    )


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def metadata(root):
    output = subprocess.check_output(
        ["mix", "run", "tools/release_metadata.exs"], cwd=root / "elixir", text=True
    )
    info = json.loads(output.splitlines()[-1])
    version = preflight.validate(root)
    require(
        info["version"] == info["runtime_version"] == version, "Actual Mix/runtime version differs"
    )
    require(
        info["compiler"] == TOOLCHAIN["elixir"]
        and info["otp"] == "26"
        and info["hex"] == TOOLCHAIN["hex"],
        "Use the pinned release compiler/OTP",
    )
    require(
        info["spec_version"]
        == json.loads((root / "spec/manifest.json").read_text())["spec_version"],
        "Compiled spec version differs",
    )
    require(info["package"] == "slackblocks", "Actual Mix package name differs")
    require(info["elixir"] == ">= 1.16.3 and < 2.0.0", "Elixir support range differs")
    require(set(info["licenses"]) == {"MIT", "BSD-3-Clause"}, "Both licenses must ship")
    require(
        info["dependencies"] == {"jason": ">= 1.4.5 and < 2.0.0", "decimal": "~> 2.4 or ~> 3.0"},
        "Runtime dependency requirements differ",
    )
    return info


def package(root, destination):
    run(root, "mix", "hex.build", "--output", str(destination))
    with tempfile.TemporaryDirectory(prefix="slackblocks-hex-inspect-", dir=scratch()) as tmp:
        extract_archive(destination, Path(tmp) / "package")


def identity(root):
    require(
        not preflight.git(root, "status", "--porcelain", "--untracked-files=no"),
        "Commit changes before preparing the release artifact",
    )
    # Hex packages source files even when they are untracked or ignored by Git.
    # Limit this check to the archive allowlist so generated docs/dist remain usable.
    untracked = preflight.git(
        root, "ls-files", "--others", "--", *(f"elixir/{name}" for name in sorted(PACKAGE_ROOTS))
    )
    require(not untracked, f"Untracked package files must be committed or removed: {untracked}")
    return {
        "version": preflight.validate(root),
        "commit": preflight.git(root, "rev-parse", "HEAD"),
        "toolchain": TOOLCHAIN,
        "lock_sha256": digest(root / "elixir/mix.lock"),
    }


def prepare(root, directory):
    record = {**identity(root), "metadata": metadata(root)}
    directory.mkdir(parents=True, exist_ok=True)
    package(root, directory / ARCHIVE)
    record["sha256"] = digest(directory / ARCHIVE)
    (directory / "release.json").write_text(json.dumps(record, indent=2) + "\n")
    return record


def verify(root, directory, rebuild=False):
    record = json.loads((directory / "release.json").read_text())
    require(
        all(record.get(k) == v for k, v in identity(root).items()),
        "Artifact metadata differs from this commit/version/toolchain/lock",
    )
    require((directory / ARCHIVE).is_file(), "Tested Hex archive is missing")
    require(digest(directory / ARCHIVE) == record.get("sha256"), "Hex archive checksum differs")
    if rebuild:
        require(
            record.get("metadata") == metadata(root),
            "Actual Mix metadata differs from the tested artifact",
        )
        with tempfile.TemporaryDirectory(prefix="slackblocks-hex-rebuild-", dir=scratch()) as tmp:
            rebuilt = Path(tmp) / ARCHIVE
            package(root, rebuilt)
            require(digest(rebuilt) == record["sha256"], "Hex package reproduction differs")
    return record


def api(path, key=None, absent=False, opener=urllib.request.urlopen):
    headers = {"Accept": "application/json", "User-Agent": "slackblocks-release"}
    if key:
        headers["Authorization"] = key
    request = urllib.request.Request(API + path, headers=headers)
    try:
        with opener(request, timeout=30) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        error.close()
        if absent and error.code == 404:
            return None
        # Never print authenticated response bodies or credentials.
        raise preflight.ReleaseError(
            f"Hex request failed with HTTP {error.code}: {path}"
        ) from error


def registry(record):
    release = api(f"/packages/slackblocks/releases/{record['version']}", absent=True)
    if release is not None:
        require(
            release.get("version") == record["version"]
            and release.get("checksum") == record["sha256"],
            "Existing Hex release identity differs; never replace it",
        )
        require(not release.get("retirement"), "Existing Hex release is retired")
    return release


def account(root):
    policy = json.loads((root / "elixir/release.json").read_text())
    key = os.environ.get("HEX_API_KEY")
    require(bool(key), "Protected hex environment requires HEX_API_KEY")
    # /auth's package probe receives a string, while Hex's publisher checks a
    # resolved package and accepts either api:write or its exact package scope.
    # Inspect only the authenticated key; never print its metadata or responses.
    auth = api("/auth?domain=api&resource=read", key)
    auth_key = auth.get("key", {})
    require(
        auth_key.get("owner") == {"type": "user", "name": policy["owner"]},
        "Hex key owner differs from the reviewed personal publisher",
    )
    name = auth_key.get("name")
    require(isinstance(name, str) and bool(name), "Hex active key name is missing")
    info = api("/keys/" + urllib.parse.quote(name, safe=""), key)
    require(
        info.get("authing_key") is True and info.get("name") == name,
        "Hex active key identity differs",
    )
    permissions = info.get("permissions", [])
    write = any(
        p.get("domain") == "api" and p.get("resource") in (None, "write")
        for p in permissions
    )
    package = any(
        p.get("domain") == "package" and p.get("resource") == "hexpm/slackblocks"
        for p in permissions
    )
    existing = api("/packages/slackblocks", absent=True)
    require(write or (existing is not None and package), "Hex key lacks publish permission")
    if write:
        api("/auth?domain=api&resource=write", key)
    if existing is not None:
        owner = api("/packages/slackblocks/owners/" + urllib.parse.quote(policy["owner"], safe=""), key)
        require(
            owner.get("username") == policy["owner"] and owner.get("level") in ("full", "maintainer"),
            "Hex package owner differs from the reviewed personal publisher",
        )


def readiness(root):
    version = preflight.coordinator(root, None, "workflow_dispatch", "refs/heads/master")
    account(root)
    with tempfile.TemporaryDirectory(prefix="slackblocks-hex-readiness-", dir=scratch()) as tmp:
        record = prepare(root, Path(tmp))
        registry(record)
    run(root, "mix", "hex.publish", "--dry-run", "--yes")
    print(f"Hex account, permissions, name state and full dry run verified for {version}")


def wait_registry(record, attempts=12, sleep=time.sleep):
    for attempt in range(attempts):
        result = registry(record)
        if result is not None:
            return result
        if attempt + 1 < attempts:
            sleep(5)
    raise preflight.ReleaseError("Hex version did not become visible within the bounded wait")


def docs_available(version, opener=urllib.request.urlopen):
    url = f"https://slackblocks.hexdocs.pm/{version}/Slackblocks.html"
    try:
        with opener(url, timeout=30) as response:
            body = response.read().decode()
            return response.status == 200 and "Slackblocks" in body and version in body
    except urllib.error.HTTPError as error:
        error.close()
        if error.code == 404:
            return False
        raise preflight.ReleaseError(f"HexDocs returned HTTP {error.code}") from error


def wait_docs(version, attempts=12, sleep=time.sleep):
    for attempt in range(attempts):
        if docs_available(version):
            return
        if attempt + 1 < attempts:
            sleep(5)
    raise preflight.ReleaseError("HexDocs did not become visible within the bounded wait")


def publish(root, directory):
    preflight.publisher(root, "elixir", os.environ.get("GITHUB_REF", ""))
    record = verify(root, directory, rebuild=True)
    account(root)
    release = registry(record)
    if release is None:
        try:
            run(root, "mix", "hex.publish", "package", "--yes")
        except subprocess.CalledProcessError:
            # An upload may have succeeded despite a lost response. Re-query identity;
            # never retry the write automatically or use Hex's replacement window.
            release = wait_registry(record)
        else:
            release = wait_registry(record)
    if not release.get("has_docs"):
        run(root, "mix", "hex.publish", "docs", "--yes")
    wait_docs(record["version"])
    print(f"Hex package checksum and HexDocs verified for {record['version']}")


def consumer(root, version):
    require(preflight.validate(root) == version, "Registry consumer version differs")
    with tempfile.TemporaryDirectory(prefix="slackblocks-hex-consumer-", dir=scratch()) as tmp:
        target = Path(tmp)
        (target / "mix.exs").write_text(
            """defmodule RegistryConsumer.MixProject do
  use Mix.Project
  def project, do: [app: :registry_consumer, version: "0.0.0", deps: [{:slackblocks, "== """
            + version
            + """"}]]
  def application, do: []
end
"""
        )
        (target / "verify.exs").write_text(
            '''"'''
            + version
            + """" = Slackblocks.version()
{:ok, section} = Slackblocks.Blocks.Section.new(text: "Hello from Hex")
%{"text" => %{"text" => "Hello from Hex"}} = Jason.decode!(Jason.encode!(section))
^section = Slackblocks.decode!(Slackblocks.Blocks.Section, Slackblocks.encode!(section))
{:error, %Slackblocks.ValidationError{}} = Slackblocks.Blocks.Section.change(section, text: nil)
true = Protocol.consolidated?(Jason.Encoder)
"""
        )
        env = {
            k: v
            for k, v in os.environ.items()
            if k not in {"MIX_BUILD_PATH", "MIX_BUILD_ROOT", "MIX_DEPS_PATH", "HEX_API_KEY"}
        }
        env["MIX_ENV"] = "prod"
        for args in [["mix", "deps.get"], ["mix", "run", "verify.exs"]]:
            subprocess.run(args, cwd=target, env=env, check=True)
        require(
            "{:hex, :slackblocks," in (target / "mix.lock").read_text(),
            "Consumer did not resolve from Hex",
        )
    print(f"Fresh production consumer installed {version} from Hex")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command", choices=["prepare", "verify", "readiness", "publish", "consumer"]
    )
    parser.add_argument("--directory", type=Path, default=ROOT / "elixir/dist")
    args = parser.parse_args()
    try:
        if args.command == "prepare":
            prepare(ROOT, args.directory.resolve())
        elif args.command == "verify":
            verify(ROOT, args.directory.resolve(), rebuild=True)
        elif args.command == "readiness":
            readiness(ROOT)
        elif args.command == "publish":
            publish(ROOT, args.directory.resolve())
        else:
            consumer(ROOT, preflight.validate(ROOT))
    except (preflight.ReleaseError, subprocess.CalledProcessError, urllib.error.URLError) as error:
        raise SystemExit(str(error)) from error


if __name__ == "__main__":
    main()
