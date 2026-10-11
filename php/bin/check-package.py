#!/usr/bin/env python3
"""Install the actual Composer archive outside the checkout, without a path repository."""

import hashlib
import json
import os
import re
import shutil
import subprocess
import tempfile
import zipfile
from pathlib import Path

import package_manifest

ROOT = Path(__file__).resolve().parents[2]
package_manifest.check(ROOT)
VERSION = re.search(r"PACKAGE = '([^']+)'", (ROOT / "php/src/Version.php").read_text())[1]
SCRATCH = None
if Path("/Volumes/Repos").exists():
    for volume in ("Repos", "Cache", "Scratch"):
        if not os.path.ismount("/Volumes/" + volume):
            raise SystemExit("Required external volume is missing: " + volume)
    SCRATCH = "/Volumes/Scratch"
    os.environ.setdefault("COMPOSER_CACHE_DIR", "/Volumes/Cache/composer-slackblocks")


def run(*cmd, cwd=ROOT, env=None):
    executable = shutil.which(cmd[0])
    if executable is None:
        raise SystemExit("Required tool not found: " + cmd[0])
    subprocess.run([executable, *cmd[1:]], cwd=cwd, check=True, env=env)


with tempfile.TemporaryDirectory(dir=SCRATCH, prefix="slackblocks-php-package-") as directory:
    work = Path(directory)
    run("composer", "archive", "--format=zip", "--file=slackblocks", "--dir=" + str(work))
    archive = work / "slackblocks.zip"
    with zipfile.ZipFile(archive) as package:
        names = package.namelist()
        allowed = {"composer.json", *["php/" + name for name in package_manifest.PACKAGE_FILES]}
        unexpected = [
            p
            for p in names
            if p.startswith("/")
            or ".." in Path(p).parts
            or (
                not p.endswith("/")
                and not (
                    p in allowed
                    or p.startswith("php/src/")
                    and p.endswith(".php")
                    or p == "php/examples/consumer.php"
                )
            )
        ]
        if unexpected:
            raise SystemExit(
                "Unexpected package contents: "
                + repr(unexpected[:12])
                + f" ({len(unexpected)} total)"
            )
        manifest = json.loads(package.read("composer.json"))
        required = [
            "php/src/MessagePayload.php",
            "php/src/Internal/Schema.php",
            "php/src/Builder.php",
            "php/LICENSE",
            "php/README.md",
        ]
        if not set(required) <= set(names):
            raise SystemExit("Missing package runtime/license/docs files")
    consumer = work / "consumer"
    consumer.mkdir()
    # A Composer package repository supplies registry metadata for the unchanged ZIP.
    # Its dist URL points to the tested artifact, never the source checkout.
    metadata = {
        k: v
        for k, v in manifest.items()
        if k not in ["require-dev", "autoload-dev", "scripts", "config", "archive"]
    }
    metadata.update(
        version=VERSION,
        dist={
            "type": "zip",
            "url": archive.as_uri(),
            "shasum": hashlib.sha1(archive.read_bytes()).hexdigest(),
        },
    )
    (consumer / "composer.json").write_text(
        json.dumps(
            {
                "repositories": [{"type": "package", "package": metadata}],
                "require": {manifest["name"]: VERSION},
                "config": {"allow-plugins": False},
            }
        )
    )
    run("composer", "install", "--no-interaction", "--prefer-dist", "--no-scripts", cwd=consumer)
    autoloader = consumer / "vendor/autoload.php"
    installed = consumer / "vendor" / manifest["name"]
    if installed.is_symlink():
        raise SystemExit("Package evidence must not use a checkout symlink")
    run("php", str(installed / "php/examples/consumer.php"), str(autoloader), cwd=consumer)
    # Standalone autoload must not resolve any library class from the monorepo.
    script = consumer / "location.php"
    script.write_text(
        "<?php require __DIR__.'/vendor/autoload.php'; echo (new ReflectionClass(Slackblocks\\MessagePayload::class))->getFileName();"
    )
    location = subprocess.check_output(["php", str(script)], cwd=consumer, text=True)
    if not Path(location).resolve().is_relative_to(installed.resolve()):
        raise SystemExit("Consumer loaded the source checkout")
    print(
        f"Archive verified: {len(names)} entries; SHA256 {hashlib.sha256(archive.read_bytes()).hexdigest()}"
    )

    # Prove real VCS discovery uses the root manifest, conventional tags and custom
    # dist template. A namespaced workflow tag must not become a Composer version.
    vcs = work / "vcs"
    vcs.mkdir()
    with zipfile.ZipFile(archive) as package:
        package.extractall(vcs)
    descriptor = json.loads((vcs / "composer.json").read_text())
    descriptor["dist"]["url"] = work.as_uri() + "/downloads/php/%prettyVersion%/slackblocks-php.zip"
    (vcs / "composer.json").write_text(json.dumps(descriptor))
    artifact_dir = work / "downloads" / "php" / ("v" + VERSION)
    artifact_dir.mkdir(parents=True)
    shutil.copyfile(archive, artifact_dir / "slackblocks-php.zip")
    (vcs / "other-language.txt").write_text("Source installs retain the monorepo layout.\n")
    run("git", "init", "--quiet", "--initial-branch=main", cwd=vcs)
    run("git", "add", ".", cwd=vcs)
    run(
        "git",
        "-c",
        "user.name=Package test",
        "-c",
        "user.email=package@example.invalid",
        "-c",
        "commit.gpgsign=false",
        "commit",
        "--quiet",
        "-m",
        "Package fixture",
        cwd=vcs,
    )
    run("git", "-c", "tag.gpgsign=false", "tag", "php/v" + VERSION, cwd=vcs)
    discovery = work / "discovery"
    discovery.mkdir()
    (discovery / "composer.json").write_text(
        json.dumps(
            {
                "repositories": [{"type": "vcs", "url": str(vcs)}, {"packagist.org": False}],
            }
        )
    )
    available = json.loads(
        subprocess.check_output(
            [shutil.which("composer"), "show", "--all", "--format=json", manifest["name"]],
            cwd=discovery,
            text=True,
        )
    )["versions"]
    if available != ["dev-main"]:
        raise SystemExit("Workflow tag unexpectedly exposed a stable Composer version")
    run("git", "-c", "tag.gpgsign=false", "tag", "v" + VERSION, cwd=vcs)
    reference = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=vcs, text=True).strip()
    for mode, constraint in (("dist", VERSION), ("source", VERSION), ("source", "dev-main")):
        project = work / (mode + "-" + constraint)
        project.mkdir()
        (project / "composer.json").write_text(
            json.dumps(
                {
                    "repositories": [{"type": "vcs", "url": str(vcs)}, {"packagist.org": False}],
                    "require": {manifest["name"]: constraint},
                    "config": {"allow-plugins": False},
                }
            )
        )
        run(
            "composer",
            "install",
            "--no-interaction",
            "--no-scripts",
            "--no-plugins",
            "--prefer-" + mode,
            cwd=project,
        )
        locked = json.loads((project / "composer.lock").read_text())["packages"][0]
        if locked["source"]["reference"] != reference or locked["dist"]["reference"] != reference:
            raise SystemExit("Composer source/dist provenance differs")
        installed = project / "vendor" / manifest["name"]
        if (installed / "other-language.txt").exists() != (mode == "source"):
            raise SystemExit("Composer did not use the requested distribution/source transport")
        run(
            "php",
            str(installed / "php/examples/consumer.php"),
            str(project / "vendor/autoload.php"),
            cwd=project,
        )
        print(f"Root VCS manifest verified: {mode}, {constraint}, {reference}")
