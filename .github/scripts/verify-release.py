#!/usr/bin/env python3
"""Read-only, shared preflight for the seven-language release train (Python 3.11+)."""

import argparse
import datetime
import json
import os
import re
import subprocess
import xml.etree.ElementTree as ET
from pathlib import Path

import tomllib

LANGUAGES = {
    "python": "python",
    "typescript": "ts",
    "go": "go",
    "java": "java",
    "csharp": "csharp",
    "ruby": "ruby",
    "rust": "rust",
}


class ReleaseError(ValueError):
    """The requested operation cannot release this checkout."""


def require(condition, message):
    if not condition:
        raise ReleaseError(message)


def match(path, pattern):
    found = re.search(pattern, path.read_text(), re.M)
    require(found is not None, f"Cannot read version from {path}")
    return found[1]


def rust_package(root):
    return tomllib.loads((root / "rust/Cargo.toml").read_text())["package"]


def validate(root, requested=None, strict=False):
    versions = {
        "Python": tomllib.loads((root / "python/pyproject.toml").read_text())["project"]["version"],
        "TypeScript": json.loads((root / "typescript/package.json").read_text())["version"],
        "Java": ET.parse(root / "java/pom.xml")
        .getroot()
        .findtext("{http://maven.apache.org/POM/4.0.0}version"),
        "C#": ET.parse(root / "csharp/src/Slackblocks/Slackblocks.csproj")
        .getroot()
        .findtext("./PropertyGroup/Version"),
        "Ruby": match(root / "ruby/lib/slackblocks/version.rb", r'^  VERSION = "([^"]+)"$'),
        "Rust": rust_package(root)["version"],
    }
    version = requested or versions["Rust"]
    require(re.fullmatch(r"\d+\.\d+\.\d+", version), "Use a stable X.Y.Z version")
    require(
        all(v == version for v in versions.values()),
        f"Coordinated versions differ from {version}: {versions}",
    )
    for label, path, key in (
        ("Python", "python/uv.lock", "package"),
        ("Rust", "rust/Cargo.lock", "package"),
        ("Rust integrations", "rust/integrations/Cargo.lock", "package"),
    ):
        own = [
            package["version"]
            for package in tomllib.loads((root / path).read_text())[key]
            if package["name"] == "slackblocks"
        ]
        require(own == [version], f"{label} lockfile must pin this package to {version}")
    ruby_locks = re.findall(
        r"^\s+slackblocks \(([^)]+)\)$", (root / "ruby/Gemfile.lock").read_text(), re.M
    )
    require(
        ruby_locks and set(ruby_locks) == {version},
        "Ruby lockfile must pin this package version",
    )
    major = version.split(".")[0]
    require(
        match(root / "go/go.mod", r"^module (.+)$")
        == f"github.com/nicklambourne/slackblocks/go/v{major}",
        "Go module major must match the release",
    )
    dates = set()
    for language in LANGUAGES:
        file = root / language / "CHANGELOG.md"
        headings = re.findall(
            r"^## \[" + re.escape(version) + r"\] — (.+)$", file.read_text(), re.M
        )
        require(
            len(headings) == 1,
            f"{file}: require exactly one {version} changelog section",
        )
        section = re.search(
            r"^## \[" + re.escape(version) + r"\] [^\n]*\n([\s\S]*?)(?=^## \[|\Z)",
            file.read_text(),
            re.M,
        )
        require(
            section is not None and section[1].strip(), f"{file}: release notes must not be empty"
        )
        date = headings[0]
        if date == "Unreleased" and not strict:
            continue
        try:
            datetime.date.fromisoformat(date)
        except ValueError as error:
            raise ReleaseError(f"{file}: require a real YYYY-MM-DD release date") from error
        require(re.fullmatch(r"\d{4}-\d{2}-\d{2}", date), f"{file}: use YYYY-MM-DD")
        dates.add(date)
    require(len(dates) <= 1, "Changelog release dates differ")
    if strict:
        require(
            rust_package(root).get("publish") == ["crates-io"],
            "Rust publication is disabled; complete the release activation checklist before releasing any language",
        )
        timestamp = match(root / "java/pom.xml", r"<project.build.outputTimestamp>([^<]+)<")
        require(
            timestamp == next(iter(dates)) + "T00:00:00Z",
            "Java output timestamp must match the coordinated release date",
        )
    return version


def git(root, *args):
    return subprocess.check_output(
        ["git", *args], cwd=root, text=True, stderr=subprocess.PIPE
    ).strip()


def tag_state(root, version, head):
    tags = [f"{prefix}/v{version}" for prefix in LANGUAGES.values()]
    existing = 0
    for tag in tags:
        present = subprocess.run(
            ["git", "show-ref", "--verify", "--quiet", f"refs/tags/{tag}"], cwd=root
        )
        if present.returncode == 1:
            continue
        require(present.returncode == 0, f"Cannot inspect {tag}")
        require(
            git(root, "rev-parse", f"refs/tags/{tag}^{{commit}}") == head,
            f"{tag} points to a different commit",
        )
        require(
            git(root, "cat-file", "-t", f"refs/tags/{tag}") == "tag",
            f"{tag} must be annotated",
        )
        existing += 1
    require(
        existing in (0, len(tags)),
        f"Partial coordinated tag set: {existing}/{len(tags)}; recover without moving public tags",
    )
    return existing


def release_commit(root):
    head = git(root, "rev-parse", "HEAD")
    require(
        not git(root, "status", "--porcelain", "--untracked-files=no"),
        "Release checkout has tracked modifications",
    )
    result = subprocess.run(["git", "merge-base", "--is-ancestor", head, "origin/master"], cwd=root)
    require(
        result.returncode == 0,
        "Release commit must be on origin/master; fetch full history first",
    )
    return head


def coordinator(root, version, event, ref):
    strict = event == "workflow_dispatch"
    require(
        event in ("pull_request", "workflow_dispatch"),
        f"Unsupported coordinator event: {event}",
    )
    version = validate(root, version, strict)
    if strict:
        require(ref == "refs/heads/master", "Dispatch coordinated releases from master")
        head = release_commit(root)
        require(
            head == git(root, "rev-parse", "origin/master"),
            "Dispatch the current master commit",
        )
        tag_state(root, version, head)
    return version


def publisher(root, language, ref):
    found = re.fullmatch(r"refs/tags/" + LANGUAGES[language] + r"/v(\d+\.\d+\.\d+)", ref)
    require(
        found is not None,
        f"{language} publication requires its exact language/vX.Y.Z tag",
    )
    version = validate(root, found[1], strict=True)
    head = release_commit(root)
    require(
        git(root, "rev-parse", ref + "^{commit}") == head,
        "Requested tag does not name the checkout",
    )
    require(
        tag_state(root, version, head) == len(LANGUAGES),
        "Create the complete seven-tag set before publishing",
    )
    return version


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("coordinator", "publisher"))
    parser.add_argument("--language", choices=LANGUAGES)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    ref = os.environ.get("GITHUB_REF", "")
    try:
        if args.command == "coordinator":
            version = coordinator(
                root,
                os.environ.get("REQUESTED_VERSION") or None,
                os.environ.get("GITHUB_EVENT_NAME", ""),
                ref,
            )
        else:
            require(args.language is not None, "Publisher requires --language")
            version = publisher(root, args.language, ref)
        if output := os.environ.get("GITHUB_OUTPUT"):
            with open(output, "a") as stream:
                stream.write(f"version={version}\n")
        print(f"Verified seven-language {args.command} preflight for {version}")
    except (ReleaseError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error)) from error


if __name__ == "__main__":
    main()
