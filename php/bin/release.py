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
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location(
    "release_preflight", ROOT / ".github/scripts/verify-release.py"
)
preflight = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(preflight)
FILES = {"composer.json", *["php/" + name for name in preflight.php_manifest.PACKAGE_FILES]}
ARCHIVE = "slackblocks-php.zip"


def temporary_directory(prefix):
    scratch = None
    if Path("/Volumes/Repos").exists():
        for volume in ("Repos", "Cache", "Scratch"):
            preflight.require(
                os.path.ismount("/Volumes/" + volume),
                "Required external volume is missing: " + volume,
            )
        scratch = "/Volumes/Scratch"
        os.environ.setdefault("COMPOSER_CACHE_DIR", "/Volumes/Cache/composer-slackblocks")
    return tempfile.TemporaryDirectory(prefix=prefix, dir=scratch)


def run(*args, cwd, env=None, input=None):
    executable = shutil.which(args[0])
    preflight.require(executable is not None, f"Required executable missing: {args[0]}")
    return subprocess.check_output(
        [executable, *args[1:]], cwd=cwd, env=env, input=input, text=True
    ).strip()


def digest(content):
    return hashlib.sha256(content).hexdigest()


def policy(root):
    return json.loads((root / "php/release.json").read_text())


def contents(root):
    paths = preflight.git(
        root, "ls-tree", "-r", "--name-only", "HEAD", "composer.json", "php"
    ).splitlines()
    selected = {}
    for name in paths:
        if (
            name in FILES
            or name.startswith("php/src/")
            and name.endswith(".php")
            or name == "php/examples/consumer.php"
        ):
            entry = preflight.git(root, "ls-tree", "HEAD", name)
            preflight.require(
                entry.startswith("100644 "), f"Non-regular PHP distribution file: {name}"
            )
            selected[name] = subprocess.check_output(["git", "show", "HEAD:" + name], cwd=root)
    preflight.require(
        selected.keys() >= FILES and "php/src/MessagePayload.php" in selected,
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
    archive = directory / ARCHIVE
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
    archive = directory / ARCHIVE
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
        preflight.require(package.namelist() == sorted(source), "PHP archive inventory mismatch")
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


def consumer(root, directory, registry=False, source=False):
    record = verify(root, directory)
    with temporary_directory(prefix="slackblocks-php-consumer-") as temporary:
        project = Path(temporary)
        manifest = {
            "require": {record["package"]: record["version"]},
            "config": {"allow-plugins": False},
        }
        if not registry:
            metadata = json.loads((root / "composer.json").read_text())
            for key in ["require-dev", "autoload-dev", "scripts", "config", "archive"]:
                metadata.pop(key, None)
            archive = directory / ARCHIVE
            metadata.update(
                version=record["version"],
                source={"type": "git", "url": str(root), "reference": record["source_commit"]},
                dist={
                    "type": "zip",
                    "url": archive.resolve().as_uri(),
                    "shasum": hashlib.sha1(archive.read_bytes()).hexdigest(),
                },
            )
            manifest["repositories"] = [
                {"type": "package", "package": metadata},
                {"packagist.org": False},
            ]
        (project / "composer.json").write_text(json.dumps(manifest))
        run(
            "composer",
            "install",
            "--no-interaction",
            "--prefer-source" if source else "--prefer-dist",
            "--no-scripts",
            "--no-plugins",
            cwd=project,
        )
        installed = project / "vendor" / record["package"]
        preflight.require(not installed.is_symlink(), "Installed PHP package must not be a symlink")
        installed_metadata = json.loads((project / "vendor/composer/installed.json").read_text())[
            "packages"
        ]
        installed_package = next(
            item for item in installed_metadata if item["name"] == record["package"]
        )
        preflight.require(
            installed_package["installation-source"] == ("source" if source else "dist"),
            "Composer used the wrong installation transport",
        )
        if source:
            preflight.require(
                run("git", "rev-parse", "HEAD", cwd=installed) == record["source_commit"],
                "Installed source commit differs",
            )
        else:
            actual = {
                path.relative_to(installed).as_posix()
                for path in installed.rglob("*")
                if path.is_file()
            }
            preflight.require(actual == set(record["files"]), "Installed PHP inventory differs")
        preflight.require(
            not any((installed / name).is_symlink() for name in record["files"]),
            "Installed PHP files must not be symlinks",
        )
        if registry:
            commit = record["source_commit"]
            packages = json.loads((project / "composer.lock").read_text())["packages"]
            package = next(item for item in packages if item["name"] == record["package"])
            preflight.require(
                package["source"]["reference"] == commit and package["dist"]["reference"] == commit,
                "Packagist source/dist references differ",
            )
            preflight.require(
                package["source"]["url"].removesuffix(".git")
                == "https://github.com/" + record["repository"],
                "Packagist source repository differs",
            )
            expected_url = preflight.php_manifest.DIST_URL
            preflight.require(
                package["dist"]["type"] == "zip"
                and package["dist"]["url"]
                in (expected_url, expected_url.replace("%prettyVersion%", "v" + record["version"])),
                "Packagist archive URL differs",
            )
        for name, checksum in record["files"].items():
            preflight.require(
                (installed / name).is_file()
                and digest((installed / name).read_bytes()) == checksum,
                f"Installed PHP file differs: {name}",
            )
        run(
            "php",
            str(installed / "php/examples/consumer.php"),
            str(project / "vendor/autoload.php"),
            cwd=project,
        )
    return record


def push_tag(root, record, remote, apply=False):
    """Expose the tested source commit to Composer, refusing changed or lightweight tags."""
    tag = "v" + record["version"]
    content = (
        f"object {record['source_commit']}\ntype commit\ntag {tag}\n"
        f"tagger Slackblocks release <release@slackblocks.invalid> {record['source_timestamp']} +0000\n\n"
        f"Slackblocks PHP {record['version']}\n\nSource: https://github.com/{record['repository']}/commit/{record['source_commit']}\n"
    )
    expected = run("git", "mktag", cwd=root, input=content)
    refs = run("git", "ls-remote", remote, "refs/tags/" + tag, cwd=root)
    if refs:
        preflight.require(
            refs.split()[0] == expected,
            "Existing Composer version tag differs; never move a published tag",
        )
        return "unchanged"
    command = ["git", "push", "--atomic", "--porcelain"]
    if not apply:
        command.append("--dry-run")
    run(*command, remote, expected + ":refs/tags/" + tag, cwd=root)
    return "published" if apply else "dry-run"


def release_info(root, repository, tag):
    # The tag endpoint only returns published releases. List all pages so retries
    # can find drafts as well; lookup failures must never authorize creation.
    pages = json.loads(
        run(
            "gh",
            "api",
            "--paginate",
            "--slurp",
            f"repos/{repository}/releases?per_page=100",
            cwd=root,
        )
    )
    matches = [release for page in pages for release in page if release["tag_name"] == tag]
    preflight.require(len(matches) <= 1, "Multiple PHP releases use the same tag")
    return matches[0] if matches else None


def release_assets(root, directory, record):
    """Create/complete the PHP release, verify exact bytes, and never overwrite an asset."""
    tag = "php/v" + record["version"]
    repository = record["repository"]
    info = release_info(root, repository, tag)
    if info is None:
        # Public assets must exist before the conventional vX.Y.Z tag is exposed.
        with temporary_directory(prefix="slackblocks-php-notes-") as temporary:
            notes = Path(temporary) / "notes.md"
            changelog = (root / "php/CHANGELOG.md").read_text()
            section = (
                changelog.split("## [" + record["version"] + "]", 1)[1]
                .split("\n", 1)[1]
                .split("\n## [", 1)[0]
                .strip()
            )
            preflight.require(bool(section), "PHP release notes are empty")
            notes.write_text(
                section + "\n\nPackagist publication is verified by the PHP publishing workflow.\n"
            )
            run(
                "gh",
                "release",
                "create",
                tag,
                "--repo",
                repository,
                "--verify-tag",
                "--latest=false",
                "--draft",
                "--title",
                "PHP v" + record["version"],
                "--notes-file",
                str(notes),
                cwd=root,
            )
        info = release_info(root, repository, tag)
    preflight.require(
        info is not None and info["tag_name"] == tag and not info["prerelease"],
        "PHP release has unexpected tag or prerelease metadata",
    )
    names = [asset["name"] for asset in info["assets"]]
    preflight.require(len(names) == len(set(names)), "Duplicate PHP release asset names")
    required = (ARCHIVE, "release.json")
    preflight.require(
        info["draft"] or set(required) <= set(names),
        "Published PHP release is missing required assets",
    )

    def verify_asset(name):
        with temporary_directory(prefix="slackblocks-php-upload-") as temporary:
            run(
                "gh",
                "release",
                "download",
                tag,
                "--repo",
                repository,
                "--pattern",
                name,
                "--dir",
                temporary,
                cwd=root,
            )
            preflight.require(
                (Path(temporary) / name).read_bytes() == (directory / name).read_bytes(),
                "Published PHP asset differs; never overwrite release files: " + name,
            )

    # Check all pre-existing files before completing a partial draft. Public
    # releases are never edited, including repositories with immutable releases.
    for name in required:
        if name in names:
            verify_asset(name)
    for name in required:
        if name not in names:
            run(
                "gh",
                "release",
                "upload",
                tag,
                str(directory / name),
                "--repo",
                repository,
                cwd=root,
            )
            verify_asset(name)
    if info["draft"]:
        run(
            "gh",
            "release",
            "edit",
            tag,
            "--repo",
            repository,
            "--draft=false",
            "--latest=false",
            cwd=root,
        )
        info = release_info(root, repository, tag)
        preflight.require(
            info is not None and not info["draft"], "PHP release assets are not public yet"
        )


def publish(root, directory, apply=False):
    record = verify(root, directory)
    remote = "https://github.com/" + record["repository"] + ".git"
    # Check tag identity and rehearse the push before creating assets. A dry run
    # cannot prove that server-side tag rules will allow the eventual write.
    result = push_tag(root, record, remote)
    if not apply:
        return result
    check_registration(policy(root))
    release_assets(root, directory, record)
    return push_tag(root, record, remote, apply=True)


def check_registration(settings, opener=urllib.request.urlopen):
    request = urllib.request.Request(
        "https://packagist.org/packages/" + settings["package"] + ".json",
        headers={"User-Agent": "slackblocks-release (github.com/nicklambourne/slackblocks)"},
    )
    try:
        with opener(request, timeout=30) as response:
            package = json.load(response)["package"]
    except urllib.error.HTTPError as error:
        error.close()
        raise
    preflight.require(package["name"] == settings["package"], "Packagist package identity differs")
    repository = package["repository"].removesuffix(".git").rstrip("/")
    preflight.require(
        repository == "https://github.com/" + settings["repository"],
        "Packagist points to the wrong monorepo",
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
    parser.add_argument("--source", action="store_true", help="Verify a native source installation")
    args = parser.parse_args()
    if args.command == "readiness":
        preflight.coordinator(ROOT, None, "workflow_dispatch", os.environ.get("GITHUB_REF", ""))
        check_registration(policy(ROOT))
        with temporary_directory(prefix="slackblocks-php-readiness-") as temporary:
            record = prepare(ROOT, Path(temporary))
            result = push_tag(ROOT, record, "https://github.com/" + record["repository"] + ".git")
    elif args.command == "prepare":
        result = prepare(ROOT, args.directory)
    elif args.command == "verify":
        result = verify(ROOT, args.directory)
    elif args.command == "consumer":
        result = consumer(ROOT, args.directory, source=args.source)
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
            consumer(ROOT, args.directory, registry=True, source=True)
        else:
            result = publish(ROOT, args.directory, args.apply)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
