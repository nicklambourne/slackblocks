#!/usr/bin/env python3
"""Deterministic PHP distribution; publishing requires explicit activation and --apply."""

import argparse
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import tempfile
import time
import zipfile
import urllib.request
import urllib.error
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location(
    "release_preflight", ROOT / ".github/scripts/verify-release.py"
)
preflight = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(preflight)
FILES = {
    "composer.json",
    "README.md",
    "IMPLEMENTATION.md",
    "CHANGELOG.md",
    "LICENSE",
    "LICENSE.BSD-3-Clause",
}


def temporary_directory(prefix):
    scratch = None
    if Path("/Volumes/Repos").exists():
        for volume in ("Repos", "Cache", "Scratch"):
            preflight.require(
                os.path.ismount("/Volumes/" + volume),
                "Required external volume is missing: " + volume,
            )
        scratch = "/Volumes/Scratch"
        os.environ.setdefault(
            "COMPOSER_CACHE_DIR", "/Volumes/Cache/composer-slackblocks"
        )
    return tempfile.TemporaryDirectory(prefix=prefix, dir=scratch)


def run(*args, cwd, env=None):
    executable = shutil.which(args[0])
    preflight.require(executable is not None, f"Required executable missing: {args[0]}")
    return subprocess.check_output(
        [executable, *args[1:]], cwd=cwd, env=env, text=True
    ).strip()


def digest(content):
    return hashlib.sha256(content).hexdigest()


def policy(root):
    return json.loads((root / "php/release.json").read_text())


def contents(root):
    paths = preflight.git(root, "ls-files", "php").splitlines()
    selected = {}
    for path in paths:
        name = path.removeprefix("php/")
        if (
            name in FILES
            or name.startswith("src/")
            and name.endswith(".php")
            or name == "examples/consumer.php"
        ):
            file = root / path
            preflight.require(
                not file.is_symlink(), f"Symlink in PHP distribution: {path}"
            )
            selected[name] = subprocess.check_output(
                ["git", "show", "HEAD:" + path], cwd=root
            )
    preflight.require(
        FILES <= selected.keys() and "src/MessagePayload.php" in selected,
        "Missing PHP distribution files",
    )
    return dict(sorted(selected.items()))


def zip_bytes(files, target):
    with zipfile.ZipFile(target, "w", compression=zipfile.ZIP_STORED) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data)


def prepare(root, directory):
    version = preflight.validate(root)
    preflight.require(
        not preflight.git(root, "status", "--porcelain", "--untracked-files=no"),
        "Commit changes before preparing a release artifact",
    )
    files = contents(root)
    directory.mkdir(parents=True, exist_ok=True)
    archive = directory / f"slackblocks-{version}.zip"
    zip_bytes(files, archive)
    record = {
        "version": version,
        "source_commit": preflight.git(root, "rev-parse", "HEAD"),
        "source_timestamp": preflight.git(root, "show", "-s", "--format=%ct", "HEAD"),
        "package": policy(root)["package"],
        "repository": policy(root)["repository"],
        "sha256": digest(archive.read_bytes()),
        "files": {name: digest(data) for name, data in files.items()},
    }
    (directory / "release.json").write_text(json.dumps(record, indent=2) + "\n")
    return record


def verify(root, directory):
    record = json.loads((directory / "release.json").read_text())
    version = preflight.validate(root)
    expected = {
        "version": version,
        "source_commit": preflight.git(root, "rev-parse", "HEAD"),
        "source_timestamp": preflight.git(root, "show", "-s", "--format=%ct", "HEAD"),
        "package": policy(root)["package"],
        "repository": policy(root)["repository"],
    }
    preflight.require(
        all(record.get(key) == value for key, value in expected.items()),
        "PHP artifact metadata mismatch",
    )
    archive = directory / f"slackblocks-{version}.zip"
    preflight.require(
        digest(archive.read_bytes()) == record["sha256"],
        "PHP artifact checksum mismatch",
    )
    source = contents(root)
    preflight.require(
        record["files"] == {name: digest(data) for name, data in source.items()},
        "PHP source inventory mismatch",
    )
    with zipfile.ZipFile(archive) as package:
        preflight.require(
            package.namelist() == sorted(source), "PHP archive inventory mismatch"
        )
        preflight.require(
            all(package.read(name) == data for name, data in source.items()),
            "PHP archive differs from source",
        )
    with temporary_directory(prefix="slackblocks-php-reproduce-") as temporary:
        rebuilt = Path(temporary) / "rebuild.zip"
        zip_bytes(source, rebuilt)
        preflight.require(
            rebuilt.read_bytes() == archive.read_bytes(),
            "PHP archive reproduction mismatch",
        )
    return record


def consumer(root, directory, registry=False):
    record = verify(root, directory)
    with temporary_directory(prefix="slackblocks-php-consumer-") as temporary:
        project = Path(temporary)
        manifest = {
            "require": {record["package"]: record["version"]},
            "config": {"allow-plugins": False},
        }
        if not registry:
            metadata = json.loads((root / "php/composer.json").read_text())
            for key in ["require-dev", "autoload-dev", "scripts", "config", "archive"]:
                metadata.pop(key, None)
            archive = directory / f"slackblocks-{record['version']}.zip"
            metadata.update(
                version=record["version"],
                dist={
                    "type": "zip",
                    "url": archive.resolve().as_uri(),
                    "shasum": hashlib.sha1(archive.read_bytes()).hexdigest(),
                },
            )
            manifest["repositories"] = [{"type": "package", "package": metadata}]
        (project / "composer.json").write_text(json.dumps(manifest))
        run(
            "composer",
            "install",
            "--no-interaction",
            "--prefer-dist",
            "--no-scripts",
            "--no-plugins",
            cwd=project,
        )
        installed = project / "vendor" / record["package"]
        preflight.require(
            not installed.is_symlink(), "Installed PHP package must not be a symlink"
        )
        actual = {
            str(path.relative_to(installed)).replace("\\", "/")
            for path in installed.rglob("*")
            if path.is_file()
        }
        preflight.require(
            actual == set(record["files"]), "Installed PHP inventory differs"
        )
        preflight.require(
            not any(path.is_symlink() for path in installed.rglob("*")),
            "Installed PHP files must not be symlinks",
        )
        if registry:
            checkout = project / "expected-distribution"
            distribution(root, directory, checkout)
            commit = run("git", "rev-parse", "HEAD", cwd=checkout)
            packages = json.loads((project / "composer.lock").read_text())["packages"]
            package = next(
                item for item in packages if item["name"] == record["package"]
            )
            preflight.require(
                package["source"]["reference"] == commit
                and package["dist"]["reference"] == commit,
                "Packagist source/dist references differ",
            )
            preflight.require(
                package["source"]["url"].removesuffix(".git")
                == "https://github.com/" + record["repository"],
                "Packagist source repository differs",
            )
        for name, checksum in record["files"].items():
            preflight.require(
                digest((installed / name).read_bytes()) == checksum,
                f"Installed PHP file differs: {name}",
            )
        run(
            "php",
            str(installed / "examples/consumer.php"),
            str(project / "vendor/autoload.php"),
            cwd=project,
        )
    return record


def distribution(root, directory, destination):
    """Create an independent, deterministic tagged commit; never rewrite default-branch history."""
    record = verify(root, directory)
    destination.mkdir()
    with zipfile.ZipFile(directory / f"slackblocks-{record['version']}.zip") as archive:
        # verify() checks every exact path before extraction.
        archive.extractall(destination)
    run("git", "init", "--quiet", "--object-format=sha1", cwd=destination)
    run("git", "config", "core.autocrlf", "false", cwd=destination)
    run("git", "config", "core.attributesFile", os.devnull, cwd=destination)
    run("git", "add", ".", cwd=destination)
    env = os.environ.copy()
    for role in ["AUTHOR", "COMMITTER"]:
        env[f"GIT_{role}_NAME"] = "Slackblocks release"
        env[f"GIT_{role}_EMAIL"] = "release@slackblocks.invalid"
        env[f"GIT_{role}_DATE"] = record["source_timestamp"] + " +0000"
    message = f"Slackblocks PHP {record['version']}\n\nSource: https://github.com/nicklambourne/slackblocks/commit/{record['source_commit']}"
    run(
        "git",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.autocrlf=false",
        "commit",
        "--quiet",
        "-m",
        message,
        cwd=destination,
        env=env,
    )
    tag = "v" + record["version"]
    run(
        "git",
        "-c",
        "tag.gpgsign=false",
        "tag",
        "-a",
        tag,
        "-m",
        message,
        cwd=destination,
        env=env,
    )
    return record, tag


def push_tag(directory, remote, tag, apply=False):
    expected = run("git", "rev-parse", "refs/tags/" + tag, cwd=directory)
    refs = run("git", "ls-remote", remote, "refs/tags/" + tag, cwd=directory)
    if refs:
        preflight.require(
            refs.split()[0] == expected,
            "Existing distribution tag differs; never move a published tag",
        )
        return "unchanged"
    command = ["git", "push", "--atomic", "--porcelain"]
    if not apply:
        command.append("--dry-run")
    run(*command, remote, "refs/tags/" + tag, cwd=directory)
    return "published" if apply else "dry-run"


def check_registration(settings, opener=urllib.request.urlopen):
    request = urllib.request.Request(
        "https://packagist.org/packages/" + settings["package"] + ".json",
        headers={
            "User-Agent": "slackblocks-release (github.com/nicklambourne/slackblocks)"
        },
    )
    try:
        with opener(request, timeout=30) as response:
            package = json.load(response)["package"]
    except urllib.error.HTTPError as error:
        error.close()
        raise
    preflight.require(
        package["name"] == settings["package"], "Packagist package identity differs"
    )
    repository = package["repository"].removesuffix(".git").rstrip("/")
    preflight.require(
        repository == "https://github.com/" + settings["repository"],
        "Packagist points to the wrong distribution repository",
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command",
        choices=[
            "prepare",
            "verify",
            "consumer",
            "publish",
            "verify-registry",
            "readiness",
        ],
    )
    parser.add_argument("--directory", type=Path, default=ROOT / "php/dist")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    if args.command == "readiness":
        preflight.coordinator(
            ROOT, None, "workflow_dispatch", os.environ.get("GITHUB_REF", "")
        )
        check_registration(policy(ROOT))
        with temporary_directory(prefix="slackblocks-php-readiness-") as temporary:
            artifact = Path(temporary) / "artifact"
            prepare(ROOT, artifact)
            checkout = Path(temporary) / "distribution"
            record, tag = distribution(ROOT, artifact, checkout)
            result = push_tag(
                checkout, "git@github.com:" + record["repository"] + ".git", tag
            )
    elif args.command == "prepare":
        result = prepare(ROOT, args.directory)
    elif args.command == "verify":
        result = verify(ROOT, args.directory)
    elif args.command == "consumer":
        result = consumer(ROOT, args.directory)
    else:
        preflight.publisher(ROOT, "php", os.environ.get("GITHUB_REF", ""))
        if args.command == "verify-registry":
            # Indexing can lag a webhook. No credentials or registry writes here.
            for attempt in range(12):
                try:
                    result = consumer(ROOT, args.directory, registry=True)
                    break
                except subprocess.CalledProcessError:
                    if attempt == 11:
                        raise
                    time.sleep(15)
        else:
            with temporary_directory(prefix="slackblocks-php-push-") as temporary:
                checkout = Path(temporary) / "distribution"
                record, tag = distribution(ROOT, args.directory, checkout)
                result = push_tag(
                    checkout,
                    "git@github.com:" + record["repository"] + ".git",
                    tag,
                    args.apply,
                )
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
