"""Read-only exact-version public consumers for every maintained implementation."""

import argparse
import hashlib
import json
import os
import re
import subprocess
from pathlib import Path

import release_availability as availability

LANGUAGES = {
    "python": ("Python", "python"),
    "typescript": ("TypeScript", "ts"),
    "go": ("Go", "go"),
    "java": ("Java", "java"),
    "csharp": ("CSharp", "csharp"),
    "ruby": ("Ruby", "ruby"),
    "rust": ("Rust", "rust"),
    "php": ("PHP", "php"),
    "elixir": ("Elixir", "elixir"),
    "haskell": ("Haskell", "haskell"),
}
RUNTIMES = {
    "python": (["python3", "--version"], r"Python 3\.10\."),
    "typescript": (["node", "--version"], r"v20\.19\.0\b"),
    "go": (["go", "version"], r"go1\.22\.0\b"),
    "java": (["java", "-version"], r'version "17\.'),
    "csharp": (["dotnet", "--version"], r"8\.0\."),
    "ruby": (["ruby", "--version"], r"ruby 3\.3\."),
    "rust": (["rustc", "+1.85.0", "--version"], r"rustc 1\.85\.0\b"),
    "php": (["php", "--version"], r"PHP 8\.2\."),
    "elixir": (["elixir", "--version"], r"Elixir 1\.16\.3\b"),
    "haskell": (["ghc", "--version"], r"version 9\.6\.7\b"),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def recipe(root, language):
    page = (root / "docs/docs/recipes/layout-grid.mdx").read_text()
    title, fence = LANGUAGES[language]
    region = re.search(f"<{title}>(.*?)</{title}>", page, re.S)
    require(region is not None, f"Missing native recipe for {language}")
    code = re.search(r"^```" + fence + r"\n(.*?)\n```", region[1], re.S | re.M)
    require(code is not None, f"Missing literal example for {language}")
    expected = json.loads(re.search(r"export const recipeOutputs = (.*?);\n", page, re.S)[1])
    return code[1], expected[title.lower()]


def clean_environment(folder):
    # Isolate consumer configuration, caches and credentials from the checkout.
    removed = (
        "PYTHONPATH",
        "NODE_PATH",
        "RUBYLIB",
        "RUBYOPT",
        "BUNDLE_GEMFILE",
        "BUNDLE_PATH",
        "COMPOSER_AUTH",
        "COMPOSER",
        "CABAL_CONFIG",
        "HEX_API_KEY",
        "HACKAGE_TOKEN",
        "NUGET_API_KEY",
        "NPM_TOKEN",
        "NODE_AUTH_TOKEN",
        "GH_TOKEN",
        "GITHUB_TOKEN",
        "CARGO_REGISTRY_TOKEN",
        "MIX_ENV",
        "HEX_MIRROR",
        "HEX_UNSAFE_HTTPS",
    )
    env = {key: value for key, value in os.environ.items() if key not in removed}
    env.update(
        PIP_CONFIG_FILE=os.devnull,
        PYTHONNOUSERSITE="1",
        npm_config_userconfig=os.devnull,
        npm_config_cache=str(folder / "npm-cache"),
        COMPOSER_HOME=str(folder / "composer-home"),
        COMPOSER_CACHE_DIR=str(folder / "composer-cache"),
        CABAL_DIR=str(folder / "cabal-home"),
        GEM_HOME=str(folder / "gems"),
        GEM_PATH=str(folder / "gems"),
        CARGO_HOME=str(folder / "cargo-home"),
        CARGO_TARGET_DIR=str(folder / "target"),
        NUGET_PACKAGES=str(folder / "packages"),
        DOTNET_CLI_HOME=str(folder / "dotnet-home"),
        NUGET_HTTP_CACHE_PATH=str(folder / "nuget-cache"),
        GOPROXY="https://proxy.golang.org",
        GOSUMDB="sum.golang.org",
        GOWORK="off",
        GOENV="off",
        GOPRIVATE="",
        GONOPROXY="",
        GONOSUMDB="",
        GOTOOLCHAIN="local",
        GOMODCACHE=str(folder / "gomod"),
        GOCACHE=str(folder / "gocache"),
        MIX_HOME=str(folder / "mix-home"),
        HEX_HOME=str(folder / "hex-home"),
        MIX_BUILD_PATH=str(folder / "_build"),
        MIX_DEPS_PATH=str(folder / "deps"),
    )
    return env


def registry(language, version, source):
    urls = {
        "python": f"https://pypi.org/pypi/slackblocks/{version}/json",
        "typescript": f"https://registry.npmjs.org/@nicklambourne%2fslackblocks/{version}",
        "go": f"https://proxy.golang.org/github.com/nicklambourne/slackblocks/go/v{version.split('.')[0]}/@v/v{version}.info",
        "csharp": "https://api.nuget.org/v3-flatcontainer/slackblocks/index.json",
        "ruby": f"https://rubygems.org/api/v2/rubygems/slackblocks/versions/{version}.json",
        "rust": f"https://crates.io/api/v1/crates/slackblocks/{version}",
        "elixir": f"https://hex.pm/api/packages/slackblocks/releases/{version}",
    }
    if language == "php":
        return availability.packagist(version, source)
    if language in ("java", "haskell"):
        url = (
            f"https://repo.maven.apache.org/maven2/io/github/nicklambourne/slackblocks/{version}/slackblocks-{version}.pom"
            if language == "java"
            else f"https://hackage.haskell.org/package/slackblocks-{version}/slackblocks.cabal"
        )
        with availability.open_response(url) as response:
            return {"url": url, "sha256": hashlib.sha256(response.read()).hexdigest()}
    data = availability.json_response(urls[language])
    actual = {
        "python": lambda: data["info"]["version"],
        "typescript": lambda: data["version"],
        "go": lambda: data["Version"].removeprefix("v"),
        "csharp": lambda: version if version in data["versions"] else None,
        "ruby": lambda: data["version"],
        "rust": lambda: data["version"]["num"],
        "elixir": lambda: data["version"],
    }[language]()
    if actual is None:
        raise availability.Pending(f"Public {language} index does not contain {version}")
    require(actual == version, f"{language} registry returned a different version")
    return data


def consume(root, language, version, source, folder, wait_seconds=900):
    require(
        subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
        == source,
        "Source checkout differs from the release",
    )
    require(not folder.exists(), "Consumer workdir must be fresh")
    folder.mkdir(parents=True)
    env = clean_environment(folder)
    code, expected = recipe(root, language)
    receipt = folder / "receipt.json"
    identity = dict(
        schema=1,
        language=language,
        version=version,
        source=source,
        passed=False,
        run_id=os.environ.get("GITHUB_RUN_ID"),
        run_attempt=os.environ.get("GITHUB_RUN_ATTEMPT"),
    )
    availability.save(receipt, {**identity, "state": "failed"})
    commands = []

    def run(command, *, combined=False):
        command = list(map(str, command))
        result = subprocess.run(
            command, cwd=folder, env=env, text=True, capture_output=True, timeout=1200
        )
        index = len(commands)
        (folder / f"{index}.stdout").write_text(result.stdout)
        (folder / f"{index}.stderr").write_text(result.stderr)
        commands.append({"command": command, "exit_code": result.returncode})
        availability.finish(receipt, commands=commands)
        require(result.returncode == 0, f"{language} command {index} failed; inspect retained logs")
        return result.stdout + result.stderr if combined else result.stdout

    def write(name, content):
        path = folder / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)

    try:
        if language == "csharp":
            write("global.json", '{"sdk":{"version":"8.0.100","rollForward":"latestFeature"}}')
        runtime = run(RUNTIMES[language][0], combined=True)
        require(
            re.search(RUNTIMES[language][1], runtime),
            "Consumer is not running on the supported minimum runtime",
        )
        if language == "elixir":
            require("Erlang/OTP 26" in runtime, "Expected minimum OTP 26")
        metadata = availability.wait(
            lambda: registry(language, version, source), receipt, identity, timeout=wait_seconds
        )
        write("registry.json", json.dumps(metadata, indent=2))
        if language == "python":
            write("probe.py", code)
            run(["python3", "-m", "venv", "venv"])
            python = folder / "venv/bin/python"
            run(
                [
                    python,
                    "-m",
                    "pip",
                    "install",
                    "--isolated",
                    "--no-cache-dir",
                    "--index-url",
                    "https://pypi.org/simple",
                    "--report",
                    "install.json",
                    f"slackblocks=={version}",
                ]
            )
            report = json.loads((folder / "install.json").read_text())
            own = [p for p in report["install"] if p["metadata"]["name"].lower() == "slackblocks"]
            require(len(own) == 1, "Expected one public Python distribution")
            download = own[0]["download_info"]
            require(
                any(
                    item["url"] == download["url"]
                    and item["digests"]["sha256"] == download["archive_info"]["hashes"]["sha256"]
                    for item in metadata["urls"]
                ),
                "Python archive differs from registry metadata",
            )
            run(
                [
                    python,
                    "-c",
                    f'import importlib.metadata as m; assert m.version("slackblocks")=="{version}"',
                ]
            )
            output = run([python, "probe.py"])
        elif language == "typescript":
            write("package.json", '{"private":true,"type":"module"}\n')
            write("probe.mts", code)
            run(
                [
                    "npm",
                    "install",
                    "--prefer-online",
                    "--ignore-scripts",
                    "--save-exact",
                    "--registry",
                    "https://registry.npmjs.org",
                    f"@nicklambourne/slackblocks@{version}",
                    "typescript@5.9.3",
                ]
            )
            lock = json.loads((folder / "package-lock.json").read_text())["packages"][
                "node_modules/@nicklambourne/slackblocks"
            ]
            require(
                lock["version"] == version and lock["integrity"] == metadata["dist"]["integrity"],
                "npm package identity differs",
            )
            run(
                [
                    folder / "node_modules/.bin/tsc",
                    "--strict",
                    "--skipLibCheck",
                    "--target",
                    "es2022",
                    "--module",
                    "nodenext",
                    "probe.mts",
                ]
            )
            output = run(["node", "probe.mjs"])
            run(
                [
                    "node",
                    "-e",
                    "if (!require('@nicklambourne/slackblocks').TableBlock) throw Error('CommonJS export missing')",
                ]
            )
        elif language == "go":
            module = re.search(r"^module (\S+)", (root / "go/go.mod").read_text(), re.M)[1]
            write("main.go", code)
            write("go.mod", f"module release-consumer\n\ngo 1.22\n\nrequire {module} v{version}\n")
            run(["go", "mod", "tidy"])
            meta = json.loads(run(["go", "list", "-m", "-json", module]))
            require(
                meta["Version"] == "v" + version and "Replace" not in meta,
                "Go resolved a replacement or wrong version",
            )
            output = run(["go", "run", "-mod=readonly", "."])
        elif language == "java":
            write("Recipe.java", code)
            write(
                "settings.xml",
                f"<settings><localRepository>{folder}/m2</localRepository><mirrors><mirror><id>public-central</id><mirrorOf>*</mirrorOf><url>https://repo.maven.apache.org/maven2</url></mirror></mirrors></settings>",
            )
            write(
                "pom.xml",
                f'<project xmlns="http://maven.apache.org/POM/4.0.0"><modelVersion>4.0.0</modelVersion><groupId>release.test</groupId><artifactId>consumer</artifactId><version>1</version><dependencies><dependency><groupId>io.github.nicklambourne</groupId><artifactId>slackblocks</artifactId><version>{version}</version></dependency></dependencies></project>',
            )
            run(
                [
                    root / "java/mvnw",
                    "-B",
                    "-ntp",
                    "--settings",
                    "settings.xml",
                    "-f",
                    "pom.xml",
                    "dependency:build-classpath",
                    "-Dmdep.outputFile=classpath.txt",
                ]
            )
            cp = (folder / "classpath.txt").read_text().strip()
            require(
                f"/slackblocks/{version}/slackblocks-{version}.jar" in cp, "Wrong Java artifact"
            )
            run(["javac", "--release", "17", "-cp", cp, "Recipe.java"])
            output = run(["java", "-cp", str(folder) + os.pathsep + cp, "Recipe"])
        elif language == "csharp":
            write("global.json", '{"sdk":{"version":"8.0.100","rollForward":"latestFeature"}}')
            write(
                "NuGet.Config",
                '<configuration><packageSources><clear/><add key="public" value="https://api.nuget.org/v3/index.json"/></packageSources><fallbackPackageFolders><clear/></fallbackPackageFolders></configuration>',
            )
            write(
                "Probe.csproj",
                f'<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><OutputType>Exe</OutputType><Nullable>enable</Nullable></PropertyGroup><ItemGroup><PackageReference Include="Slackblocks" Version="[{version}]" /></ItemGroup></Project>',
            )
            write("Program.cs", code)
            run(
                [
                    "dotnet",
                    "restore",
                    "--force-evaluate",
                    "--no-cache",
                    "--configfile",
                    "NuGet.Config",
                ]
            )
            assets = json.loads((folder / "obj/project.assets.json").read_text())["libraries"]
            require(f"Slackblocks/{version}" in assets, "Wrong NuGet version")
            package = folder / "packages/slackblocks" / version
            require(
                json.loads((package / ".nupkg.metadata").read_text())["source"]
                == "https://api.nuget.org/v3/index.json",
                "NuGet package did not come from the public registry",
            )
            availability.finish(
                receipt,
                archive_sha256=hashlib.sha256(
                    (package / f"slackblocks.{version}.nupkg").read_bytes()
                ).hexdigest(),
            )
            run(["dotnet", "build", "--no-restore", "--nologo"])
            output = run(["dotnet", folder / "bin/Debug/net8.0/Probe.dll"])
        elif language == "ruby":
            write(
                "probe.rb",
                code
                + f'\nraise "wrong version" unless Gem.loaded_specs.fetch("slackblocks").version.to_s == "{version}"\n',
            )
            run(
                [
                    "gem",
                    "install",
                    "slackblocks",
                    "--version",
                    version,
                    "--no-document",
                    "--install-dir",
                    folder / "gems",
                    "--clear-sources",
                    "--source",
                    "https://rubygems.org",
                ]
            )
            cached = folder / f"gems/cache/slackblocks-{version}.gem"
            require(
                hashlib.sha256(cached.read_bytes()).hexdigest() == metadata["sha"],
                "Ruby gem differs from the registry checksum",
            )
            output = run(["ruby", "probe.rb"])
        elif language == "rust":
            write("src/main.rs", code)
            write(
                "Cargo.toml",
                f'[package]\nname="release-layout-consumer"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nslackblocks="={version}"\nserde_json="=1.0.145"\n',
            )
            meta = json.loads(run(["cargo", "+1.85.0", "metadata", "--format-version", "1"]))
            own = [p for p in meta["packages"] if p["name"] == "slackblocks"]
            require(
                len(own) == 1
                and own[0]["version"] == version
                and own[0]["source"] == "registry+https://github.com/rust-lang/crates.io-index",
                "Wrong Rust registry package",
            )
            output = run(["cargo", "+1.85.0", "run", "--locked", "--quiet"])
        elif language == "php":
            write("probe.php", code)
            write("composer.json", json.dumps({"require": {"nicklambourne/slackblocks": version}}))
            run(
                [
                    "composer",
                    "install",
                    "--no-interaction",
                    "--no-plugins",
                    "--no-scripts",
                    "--prefer-dist",
                ]
            )
            own = [
                p
                for p in json.loads((folder / "composer.lock").read_text())["packages"]
                if p["name"] == "nicklambourne/slackblocks"
            ]
            require(
                len(own) == 1
                and own[0]["version"].lstrip("v") == version
                and own[0]["source"]["reference"] == source
                and own[0]["dist"]["reference"] == source,
                "Wrong Composer package source",
            )
            output = run(["php", "probe.php"])
            # A second clean transport must also work; the publisher additionally
            # checks the complete installed file inventory against its tested ZIP.
            run(
                [
                    "composer",
                    "reinstall",
                    "nicklambourne/slackblocks",
                    "--no-interaction",
                    "--no-plugins",
                    "--no-scripts",
                    "--prefer-source",
                ]
            )
            installed = json.loads((folder / "vendor/composer/installed.json").read_text())[
                "packages"
            ]
            require(
                next(p for p in installed if p["name"] == "nicklambourne/slackblocks")[
                    "installation-source"
                ]
                == "source",
                "Composer fell back from source installation",
            )
            require(
                json.loads(run(["php", "probe.php"])) == json.loads(output),
                "Composer transports differ",
            )
        elif language == "elixir":
            run(["mix", "local.hex", "2.5.1", "--force"])
            run(["mix", "local.rebar", "--force"])
            write(
                "mix.exs",
                f'defmodule Consumer.MixProject do\n use Mix.Project\n def project, do: [app: :consumer, version: "0.0.0", deps: [{{:slackblocks, "== {version}"}}]]\n def application, do: []\nend\n',
            )
            write("probe.exs", "value = (\n" + code + "\n)\nIO.puts(Jason.encode!(value))\n")
            run(["mix", "deps.get"])
            require(
                f':slackblocks, "{version}"' in (folder / "mix.lock").read_text(),
                "Wrong Hex package",
            )
            run(["mix", "compile", "--warnings-as-errors"])
            output = run(["mix", "run", "--no-compile", "probe.exs"])
        else:
            pragmas = re.findall(r"^\{-#.*?#-\}\s*$", code, re.M)
            body = re.sub(r"^\{-#.*?#-\}\s*$", "", code, flags=re.M).lstrip()
            write(
                "Main.hs",
                "\n".join(pragmas)
                + "\nmodule Main where\nimport qualified Data.ByteString.Lazy.Char8 as Output\n"
                + body
                + "\nmain :: IO ()\nmain = either (fail . show) (Output.putStrLn . encode) example\n",
            )
            write(
                "consumer.cabal",
                f"cabal-version: 3.0\nname: release-layout-consumer\nversion: 0.0.0\nbuild-type: Simple\nexecutable probe\n  main-is: Main.hs\n  default-language: GHC2021\n  build-depends: base, slackblocks =={version}, aeson, bytestring, text\n",
            )
            write("cabal.project", "packages: .\n")

            def indexed():
                run(["cabal", "update"])
                listing = run(["cabal", "list", "--simple-output", "slackblocks"])
                if ["slackblocks", version] not in [line.split() for line in listing.splitlines()]:
                    raise availability.Pending("Secure Hackage index has not exposed the version")

            availability.wait(indexed, receipt, identity, timeout=wait_seconds)
            run(["cabal", "build", "exe:probe", "--jobs=2"])
            own = [
                p
                for p in json.loads((folder / "dist-newstyle/cache/plan.json").read_text())[
                    "install-plan"
                ]
                if p["pkg-name"] == "slackblocks"
            ]
            require(
                len(own) == 1
                and own[0]["pkg-version"] == version
                and own[0]["pkg-src"]["type"] == "repo-tar",
                "Wrong Hackage package",
            )
            output = run([run(["cabal", "list-bin", "exe:probe"]).strip()])
        payload = json.loads(output)
        require(
            payload == expected, "Public package output differs from the reviewed native recipe"
        )
        canonical = json.dumps(payload, sort_keys=True, separators=(",", ":"))
        write("layout.json", canonical + "\n")
        availability.finish(
            receipt,
            state="verified",
            passed=True,
            runtime=runtime.strip(),
            commands=commands,
            payload_sha256=hashlib.sha256(canonical.encode()).hexdigest(),
            registry_evidence="registry.json",
        )
    except availability.Pending as error:
        availability.finish(
            receipt,
            state="awaiting_availability",
            passed=False,
            error=str(error),
            commands=commands,
        )
        raise
    except Exception as error:
        availability.finish(
            receipt, state="failed", passed=False, error=str(error), commands=commands
        )
        raise


def aggregate(directory, version, source):
    receipts = [json.loads(p.read_text()) for p in directory.glob("*/receipt.json")]
    require(
        len(receipts) == len(LANGUAGES) and {r["language"] for r in receipts} == set(LANGUAGES),
        "Missing or duplicate language receipts",
    )
    require(
        all(
            r.get("passed") is True
            and r.get("state") == "verified"
            and r.get("source") == source
            and r.get("version") == version
            and r.get("runtime")
            and r.get("commands")
            and r.get("registry_evidence")
            for r in receipts
        ),
        "Incomplete or mismatched release verification",
    )
    require(
        all(
            re.search(RUNTIMES[r["language"]][1], r["runtime"])
            and all(command.get("exit_code") == 0 for command in r["commands"])
            for r in receipts
        ),
        "Minimum runtime or successful command evidence differs",
    )
    require(
        len({r.get("payload_sha256") for r in receipts}) == 1
        and re.fullmatch(r"[0-9a-f]{64}", receipts[0].get("payload_sha256", "")),
        "Cross-language payloads differ",
    )
    return dict(
        schema=1, version=version, source=source, state="verified", passed=True, languages=receipts
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", type=Path)
    parser.add_argument("--source", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--language", choices=LANGUAGES)
    parser.add_argument("--directory", type=Path, required=True)
    parser.add_argument("--aggregate", action="store_true")
    args = parser.parse_args()
    require(
        re.fullmatch(r"\d+\.\d+\.\d+", args.version) and re.fullmatch(r"[0-9a-f]{40}", args.source),
        "Invalid release identity",
    )
    if args.aggregate:
        availability.save(
            args.directory / "summary.json", aggregate(args.directory, args.version, args.source)
        )
    else:
        require(args.source_root and args.language, "Specify source and language")
        consume(
            args.source_root.resolve(),
            args.language,
            args.version,
            args.source,
            args.directory.resolve(),
        )


if __name__ == "__main__":
    main()
