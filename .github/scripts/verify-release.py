#!/usr/bin/env python3
"""Read-only, shared preflight for the ten-language release train (Python 3.11+)."""

import argparse
import datetime
import importlib.util
import json
import os
import re
import subprocess
import xml.etree.ElementTree as ET
from pathlib import Path

import tomllib

_php_spec = importlib.util.spec_from_file_location(
    "php_package_manifest",
    Path(__file__).resolve().parents[2] / "php/bin/package_manifest.py",
)
php_manifest = importlib.util.module_from_spec(_php_spec)
_php_spec.loader.exec_module(php_manifest)

LANGUAGES = {
    "python": "python",
    "typescript": "ts",
    "go": "go",
    "java": "java",
    "csharp": "csharp",
    "ruby": "ruby",
    "rust": "rust",
    "php": "php",
    "elixir": "elixir",
    "haskell": "haskell",
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
        "Python": tomllib.loads((root / "python/pyproject.toml").read_text())[
            "project"
        ]["version"],
        "TypeScript": json.loads((root / "typescript/package.json").read_text())[
            "version"
        ],
        "Java": ET.parse(root / "java/pom.xml")
        .getroot()
        .findtext("{http://maven.apache.org/POM/4.0.0}version"),
        "C#": ET.parse(root / "csharp/src/Slackblocks/Slackblocks.csproj")
        .getroot()
        .findtext("./PropertyGroup/Version"),
        "Ruby": match(
            root / "ruby/lib/slackblocks/version.rb", r'^  VERSION = "([^"]+)"$'
        ),
        "Rust": rust_package(root)["version"],
        "Java constant": match(
            root
            / "java/src/main/java/io/github/nicklambourne/slackblocks/Slackblocks.java",
            r'\bVERSION = "([^"]+)"',
        ),
        "C# constant": match(
            root / "csharp/src/Slackblocks/SlackblocksInfo.cs", r'\bVersion = "([^"]+)"'
        ),
        "PHP": match(root / "php/src/Version.php", r"PACKAGE = '([^']+)'"),
    }
    versions["Elixir"] = match(root / "elixir/mix.exs", r'version:\s*"([^"]+)"')
    versions["Haskell"] = match(
        root / "haskell/slackblocks.cabal", r"^version:\s*(\S+)$"
    )
    versions["Haskell HTTP consumer"] = match(
        root / "haskell/integrations/transport.cabal", r"^\s*slackblocks ==([^,\s]+),?$"
    )
    versions["PHP public API inventory"] = json.loads(
        (root / "php/conformance/api-inventory.json").read_text()
    )["Version"]["constants"]["PACKAGE"]
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
        (
            "Rust documentation consumer",
            "docs/examples/multiselect-compat/rust/Cargo.lock",
            "package",
        ),
    ):
        own = [
            package["version"]
            for package in tomllib.loads((root / path).read_text())[key]
            if package["name"] == "slackblocks"
        ]
        require(
            own == [version], f"{label} lockfile must pin this package to {version}"
        )
    ruby_locks = re.findall(
        r"^\s+slackblocks \(([^)]+)\)$", (root / "ruby/Gemfile.lock").read_text(), re.M
    )
    require(
        ruby_locks and set(ruby_locks) == {version},
        "Ruby lockfile must pin this package version",
    )
    php = json.loads((root / "php/composer.json").read_text())
    policy = json.loads((root / "php/release.json").read_text())
    require(
        php["name"] == policy["package"] == "nicklambourne/slackblocks",
        "PHP package identity differs",
    )
    require(
        "version" not in php,
        "Composer versions must be inferred from conventional monorepo tags",
    )
    require(
        policy["repository"] == "nicklambourne/slackblocks"
        and policy["environment"] == "packagist",
        "PHP monorepo publication policy differs",
    )
    require((root / "composer.json").is_file(), "Root Composer manifest is missing")
    try:
        php_manifest.check(root)
    except ValueError as error:
        raise ReleaseError(str(error)) from error
    integration = json.loads((root / "php/integrations/composer.json").read_text())
    require(
        integration["require"][php["name"]] == version,
        "PHP integration constraint must match package version",
    )
    require(
        integration["repositories"][0]["options"]["versions"][php["name"]] == version,
        "PHP integration path version differs",
    )
    locked = json.loads((root / "php/integrations/composer.lock").read_text())
    own = [
        package["version"]
        for package in locked["packages"]
        if package["name"] == php["name"]
    ]
    require(own == [version], "PHP integration lockfile must pin package version")
    require(type(policy.get("enabled")) is bool, "PHP activation must be boolean")
    if strict:
        require(
            policy.get("enabled") is True,
            "PHP publication is disabled; verify monorepo registration, Packagist and protected environment before releasing any language",
        )
    elixir = json.loads((root / "elixir/release.json").read_text())
    require(type(elixir.get("enabled")) is bool, "Elixir activation must be boolean")
    require(
        elixir.get("auth_mode") == "api-key"
        and elixir.get("package") == "slackblocks"
        and elixir.get("environment") == "hex",
        "Elixir publication requires explicit api-key authentication and Hex identity",
    )
    require(
        match(
            root / "elixir/integrations/mix.exs",
            r'\{:slackblocks, "== ([^"]+)", path: "\.\."\}',
        )
        == version,
        "Elixir integration requirement must match package version",
    )
    require(
        match(root / "elixir/erlang/rebar.config", r'\{slackblocks, "([^"]+)"\}')
        == version,
        "Erlang consumer requirement must match Elixir version",
    )
    require(
        match(root / "elixir/lib/slackblocks.ex", r'def spec_version, do: "([^"]+)"')
        == json.loads((root / "spec/manifest.json").read_text())["spec_version"],
        "Elixir spec version differs from the shared manifest",
    )
    require(
        not any(
            line.strip() and not line.lstrip().startswith("#")
            for line in (root / "elixir/conformance/skiplist.txt")
            .read_text()
            .splitlines()
        ),
        "Elixir release skip list must be empty",
    )
    if strict:
        require(
            elixir["enabled"],
            "Elixir publication is disabled; complete the train and Hex readiness before releasing any language",
        )
        require(
            isinstance(elixir.get("owner"), str)
            and re.fullmatch(r"[a-zA-Z0-9_-]+", elixir["owner"]),
            "Elixir release requires the verified Hex owner username",
        )
    haskell = json.loads((root / "haskell/release.json").read_text())
    require(
        type(haskell.get("enabled")) is bool
        and type(haskell.get("first_publication")) is bool,
        "Haskell activation and first-publication flags must be boolean",
    )
    require(
        haskell.get("auth_mode") == "api-token"
        and haskell.get("package") == "slackblocks"
        and haskell.get("environment") == "hackage"
        and match(root / "haskell/slackblocks.cabal", r"^name:\s*(\S+)$")
        == "slackblocks",
        "Haskell publication requires explicit Hackage identity and API-token authentication",
    )
    require(
        match(root / "haskell/src/Slackblocks/Models.hs", r'^specVersion = "([^"]+)"$')
        == json.loads((root / "spec/manifest.json").read_text())["spec_version"],
        "Haskell spec version differs from the shared manifest",
    )
    require(
        not (root / "haskell/conformance/skiplist.txt").read_text().strip(),
        "Haskell release skip list must be empty",
    )
    if strict:
        require(
            haskell["enabled"],
            "Haskell publication is disabled; verify Hackage owner, token and protected environment before releasing any language",
        )
        require(
            isinstance(haskell.get("owner"), str)
            and re.fullmatch(r"[a-zA-Z0-9_-]+", haskell["owner"]),
            "Haskell release requires the verified Hackage owner username",
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
            section is not None and section[1].strip(),
            f"{file}: release notes must not be empty",
        )
        date = headings[0]
        if date == "Unreleased" and not strict:
            continue
        try:
            datetime.date.fromisoformat(date)
        except ValueError as error:
            raise ReleaseError(
                f"{file}: require a real YYYY-MM-DD release date"
            ) from error
        require(re.fullmatch(r"\d{4}-\d{2}-\d{2}", date), f"{file}: use YYYY-MM-DD")
        dates.add(date)
    require(len(dates) <= 1, "Changelog release dates differ")
    if strict:
        require(
            rust_package(root).get("publish") == ["crates-io"],
            "Rust publication is disabled; complete the release activation checklist before releasing any language",
        )
        timestamp = match(
            root / "java/pom.xml", r"<project.build.outputTimestamp>([^<]+)<"
        )
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
    composer_tag = f"refs/tags/v{version}"
    present = subprocess.run(
        ["git", "show-ref", "--verify", "--quiet", composer_tag], cwd=root
    )
    require(present.returncode in (0, 1), "Cannot inspect Composer version tag")
    if present.returncode == 0:
        require(
            existing == len(tags),
            "Composer version tag must follow all coordinated tags",
        )
        require(
            git(root, "cat-file", "-t", composer_tag) == "tag",
            "Composer version tag must be annotated",
        )
        require(
            git(root, "rev-parse", composer_tag + "^{commit}") == head,
            "Composer version tag points to a different commit",
        )
    return existing


def release_commit(root):
    head = git(root, "rev-parse", "HEAD")
    require(
        not git(root, "status", "--porcelain", "--untracked-files=no"),
        "Release checkout has tracked modifications",
    )
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", head, "origin/master"], cwd=root
    )
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
    found = re.fullmatch(
        r"refs/tags/" + LANGUAGES[language] + r"/v(\d+\.\d+\.\d+)", ref
    )
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
        "Create the complete ten-tag set before publishing",
    )
    return version


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("validate", "coordinator", "publisher"))
    parser.add_argument("--language", choices=LANGUAGES)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    ref = os.environ.get("GITHUB_REF", "")
    try:
        if args.command == "validate":
            version = validate(root)
        elif args.command == "coordinator":
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
        print(f"Verified ten-language {args.command} preflight for {version}")
    except (ReleaseError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error)) from error


if __name__ == "__main__":
    main()
