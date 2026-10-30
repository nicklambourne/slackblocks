#!/usr/bin/env python3
"""Execute every literal recipe and compare the output used by its preview.

--record is an explicit authoring operation; CI never refreshes expected output.
Only Python's automatically generated UUID block IDs are replaced in snapshots.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[2]
EXAMPLES = ROOT / "docs/examples/recipes"
LANGUAGES = dict(
    Python="python",
    TypeScript="ts",
    Go="go",
    Java="java",
    CSharp="csharp",
    Ruby="ruby",
    Rust="rust",
    PHP="php",
    Elixir="elixir",
    Haskell="haskell",
)


def run(args, cwd=ROOT, **kwargs):
    result = subprocess.run(
        [str(arg) for arg in args], cwd=cwd, text=True, capture_output=True, **kwargs
    )
    if result.returncode:
        raise RuntimeError(f"{' '.join(map(str, args))}\n{result.stdout}\n{result.stderr}")
    return result.stdout


def normalize(value):
    if isinstance(value, dict):
        return {
            key: "<generated-block-id>"
            if key == "block_id"
            and isinstance(item, str)
            and re.fullmatch(r"[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}", item)
            else normalize(item)
            for key, item in value.items()
        }
    return [normalize(item) for item in value] if isinstance(value, list) else value


def descendants(value):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from descendants(item)
    elif isinstance(value, list):
        for item in value:
            yield from descendants(item)


def check_result(recipe, value, language):
    """Independent intent checks prevent a passing program with the wrong output."""
    if recipe == "builder-preview":
        assert isinstance(value, str)
        url = urlsplit(value)
        assert url.scheme == "https" and url.netloc == "app.slack.com"
        assert url.path.startswith("/block-kit-builder/")
        payload = json.loads(unquote(url.fragment))
        assert any(obj.get("type") == "section" for obj in descendants(payload))
        return
    assert isinstance(value, dict), (recipe, value)
    objects = list(descendants(value))
    types = {obj.get("type") for obj in objects if isinstance(obj.get("type"), str)}
    required = {
        "build-notification": "header",
        "approval-request": "actions",
        "rich-alert": "rich_text",
        "confirmation-modal": "input",
        "status-table": "table",
        "parse-json": "section",
        "workflow-button": "workflow_button",
        "accordion-pagination": "container",
        "layout-grid": "layout",
    }
    if recipe in required:
        assert required[recipe] in types, (recipe, types)
    if recipe == "approval-request":
        assert {obj.get("action_id") for obj in objects if obj.get("type") == "button"} >= {
            "approve",
            "reject",
        }
    elif recipe == "confirmation-modal":
        assert value["type"] == "modal" and value["submit"]["text"]
    elif recipe == "threaded-reply":
        assert re.fullmatch(r"\d+\.\d+", value["thread_ts"])
    elif recipe == "ephemeral-response":
        assert value["response_type"] == "ephemeral"
    elif recipe == "status-table":
        assert any(len(obj.get("rows", [])) >= 2 for obj in objects)
    elif recipe == "colored-attachment":
        assert value["attachments"][0]["color"]
    elif recipe == "validation-errors":
        assert value == {"category": "length-exceeded", "handled": True}
    elif recipe == "accordion-pagination":
        assert any(obj.get("type") == "button" and obj.get("value") == "2" for obj in objects)
    elif recipe == "layout-grid":
        assert value["type"] == "layout" and value["label"] == "Service status"
        assert value["is_label_visible"] is True and value["grid_lines"] == "rows"
        assert len(value["rows"]) == 2 and len(value["fallback"]) == 1
        for row, service, status in zip(value["rows"], ["API", "Worker"], ["Healthy", "Delayed"]):
            left, right = row["items"]
            assert left["width"] == "wide" and right["width"] == "narrow"
            assert right["horizontal_alignment"] == "end"
            assert left["block"]["text"]["text"] == service
            assert right["block"]["text"]["text"] == status
        assert value["fallback"][0]["text"]["text"] == "API: Healthy; Worker: Delayed"
    elif recipe == "metadata":
        fields = value["metadata"]["event_payload"]
        assert any(item is False for item in fields.values())
        if language == "Java":
            assert "missing" not in fields  # Java rejects metadata nulls.
        else:
            assert any(item is None for item in fields.values())
        assert value["text"] in {"Updated", "Complete"}


def sources():
    catalog = json.loads((EXAMPLES / "catalog.json").read_text())
    result = {language: {} for language in LANGUAGES}
    root = (ROOT / "docs/docs/usage/cookbook.mdx").read_text()
    for item in catalog:
        name = item["id"]
        assert f"](/usage/recipes/{name})" in root, f"Missing root table link: {name}"
        source = (ROOT / "docs/docs/recipes" / f"{name}.mdx").read_text()
        assert f'<RecipeOutput recipe="{name}" outputs={{recipeOutputs}} />' in source
        preview = re.search(r"export const recipeOutputs = (.*?);\n", source, re.S)
        assert preview and set(json.loads(preview[1])) == {key.lower() for key in LANGUAGES}
        for language, fence in LANGUAGES.items():
            regions = re.findall(f"<{language}>(.*?)</{language}>", source, re.S)
            assert len(regions) == 1, (name, language, "missing/duplicate language")
            snippets = re.findall(rf"^```{fence}\n(.*?)\n```", regions[0], re.S | re.M)
            assert len(snippets) == 1, (name, language, "expected one complete program")
            result[language][name] = snippets[0]
    assert len(catalog) == 15, "Review the complete recipe catalogue when changing it"
    return result


def execute(language, cases, temp):
    env = dict(os.environ)
    if language == "Python":
        env["PYTHONPATH"] = str(ROOT / "python")
        return {name: run([sys.executable, "-c", code], env=env) for name, code in cases.items()}
    if language == "Ruby":
        return {
            name: run(
                [
                    os.environ.get("RUBY_BIN", "ruby"),
                    "-I",
                    ROOT / "ruby/lib",
                    "-e",
                    code,
                ]
            )
            for name, code in cases.items()
        }
    if language == "PHP":
        return {
            name: run(
                ["php", "-d", "zend.assertions=1", "-d", "assert.exception=1"],
                cwd=ROOT / "php",
                input=code,
            )
            for name, code in cases.items()
        }
    if language == "TypeScript":
        run(["pnpm", "--filter", "@nicklambourne/slackblocks", "build"])
        (temp / "node_modules").symlink_to(ROOT / "docs/node_modules", target_is_directory=True)
        for name, code in cases.items():
            (temp / f"{name}.mts").write_text(code)
        run(
            [
                ROOT / "docs/node_modules/.bin/tsc",
                "--strict",
                "--skipLibCheck",
                "--target",
                "es2022",
                "--module",
                "nodenext",
                *temp.glob("*.mts"),
            ]
        )
        return {name: run(["node", temp / f"{name}.mjs"]) for name in cases}
    if language == "Go":
        (temp / "go.mod").write_text(
            f"module recipes\n\ngo 1.22\n\nrequire github.com/nicklambourne/slackblocks/go/v3 v3.1.0\nreplace github.com/nicklambourne/slackblocks/go/v3 => {ROOT / 'go'}\n"
        )
        for name, code in cases.items():
            (temp / name).mkdir()
            (temp / name / "main.go").write_text(code)
        run(["go", "mod", "tidy"], cwd=temp)
        return {name: run(["go", "run", "-mod=readonly", "./" + name], cwd=temp) for name in cases}
    if language == "Java":
        cp = temp / "classpath.txt"
        run(
            [
                ROOT / "java/mvnw",
                "-B",
                "-ntp",
                "-q",
                "compile",
                "dependency:build-classpath",
                f"-Dmdep.outputFile={cp}",
                "-Dmdep.includeScope=compile",
            ],
            cwd=ROOT / "java",
        )
        classpath = os.pathsep.join(
            [str(ROOT / "java/target/classes"), cp.read_text().strip(), str(temp)]
        )
        names = {name: "Recipe" + str(index) for index, name in enumerate(cases)}
        for name, code in cases.items():
            (temp / f"{names[name]}.java").write_text(
                code.replace("public final class Recipe", f"public final class {names[name]}")
            )
        java_bin = Path(env["JAVA_HOME"]) / "bin" if "JAVA_HOME" in env else Path("")
        run(
            [
                java_bin / "javac" if "JAVA_HOME" in env else "javac",
                "--release",
                "17",
                "-cp",
                classpath,
                *temp.glob("*.java"),
            ]
        )
        return {
            name: run(
                [
                    java_bin / "java" if "JAVA_HOME" in env else "java",
                    "-cp",
                    classpath,
                    names[name],
                ]
            )
            for name in cases
        }
    if language == "CSharp":
        (temp / "Recipes.csproj").write_text(
            f'<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><OutputType>Exe</OutputType><Nullable>enable</Nullable></PropertyGroup><ItemGroup><ProjectReference Include="{ROOT / "csharp/src/Slackblocks/Slackblocks.csproj"}" /></ItemGroup></Project>'
        )
        for index, code in enumerate(cases.values()):
            imports, body = [], []
            for line in code.splitlines():
                (
                    imports
                    if line.startswith("using ") and not line.startswith("using var ")
                    else body
                ).append(line)
            (temp / f"Recipe{index}.cs").write_text(
                "\n".join(imports)
                + f"\nstatic class Recipe{index} {{ public static void Run() {{\n"
                + "\n".join(body)
                + "\n}}"
            )
        (temp / "Program.cs").write_text(
            "switch (args[0]) {\n"
            + "\n".join(
                f'case "{name}": Recipe{index}.Run(); break;' for index, name in enumerate(cases)
            )
            + '\ndefault: throw new System.ArgumentException("Unknown recipe");\n}'
        )
        run(["dotnet", "build", "--nologo"], cwd=temp)
        return {
            name: run(
                [
                    "dotnet",
                    "--roll-forward",
                    "Major",
                    temp / "bin/Debug/net8.0/Recipes.dll",
                    name,
                ]
            )
            for name in cases
        }
    if language == "Rust":
        (temp / "src/bin").mkdir(parents=True)
        (temp / "Cargo.toml").write_text(
            f'[package]\nname="slackblocks-recipe-examples"\nversion="0.0.0"\nedition="2024"\n[workspace]\nresolver="3"\n[dependencies]\nslackblocks={{path={json.dumps(str(ROOT / "rust"))}}}\nserde_json="1.0.145"\n'
        )
        for name, code in cases.items():
            (temp / "src/bin" / f"{name}.rs").write_text(code)
        cargo = ["cargo", "+" + env.get("RUST_GUIDE_TOOLCHAIN", "stable")]
        run([*cargo, "build", "--bins"], cwd=temp)
        target = json.loads(run([*cargo, "metadata", "--no-deps", "--format-version=1"], cwd=temp))[
            "target_directory"
        ]
        return {name: run([Path(target) / "debug" / name]) for name in cases}
    if language == "Elixir":
        run(["mix", "compile", "--warnings-as-errors"], cwd=ROOT / "elixir")
        for name, code in cases.items():
            (temp / f"{name}.exs").write_text(
                "value = (\n" + code + "\n)\nIO.puts(Jason.encode!(value))\n"
            )
        return {
            name: run(
                ["mix", "run", "--no-compile", "--no-deps-check", temp / f"{name}.exs"],
                cwd=ROOT / "elixir",
            )
            for name in cases
        }
    if language == "Haskell":
        cabal = [
            "cabal",
            "--builddir=" + env.get("SLACKBLOCKS_HASKELL_BUILD", "dist-docs"),
        ]
        run([*cabal, "build", "lib:slackblocks", "--jobs=2"], cwd=ROOT / "haskell")
        for index, code in enumerate(cases.values()):
            pragmas = re.findall(r"^\{-#.*?#-\}\s*$", code, re.M)
            code = re.sub(r"^\{-#.*?#-\}\s*$", "", code, flags=re.M).lstrip()
            (temp / f"Recipe{index}.hs").write_text(
                "\n".join(pragmas)
                + f"\nmodule Recipe{index} where\nimport qualified Data.ByteString.Lazy.Char8 as Output\n"
                + code
                + "\nmain :: IO ()\nmain = either (fail . show) (Output.putStrLn . encode) example\n"
            )
        (temp / "Main.hs").write_text(
            "module Main where\nimport System.Environment (getArgs)\n"
            + "".join(f"import qualified Recipe{i}\n" for i in range(len(cases)))
            + "main = do\n  args <- getArgs\n  case args of\n"
            + "".join(f'    ["{name}"] -> Recipe{i}.main\n' for i, name in enumerate(cases))
            + '    _ -> fail "Unknown recipe"\n'
        )
        run(
            [
                *cabal,
                "exec",
                "--",
                "ghc",
                "-O0",
                "-package",
                "slackblocks",
                "-XOverloadedStrings",
                f"-i{temp}",
                "-outputdir",
                temp,
                temp / "Main.hs",
                "-o",
                temp / "recipes",
            ],
            cwd=ROOT / "haskell",
        )
        return {name: run([temp / "recipes", name]) for name in cases}
    raise AssertionError(language)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--language", choices=LANGUAGES)
    parser.add_argument("--record", action="store_true")
    args = parser.parse_args()
    scratch = os.environ.get("RUNNER_TEMP")
    if Path("/Volumes/Repos").exists():
        assert all(Path("/Volumes", item).is_mount() for item in ("Repos", "Cache", "Scratch"))
        scratch = "/Volumes/Scratch"
    for language, cases in sources().items():
        if args.language and args.language != language:
            continue
        with tempfile.TemporaryDirectory(prefix="slackblocks-recipes-", dir=scratch) as directory:
            outputs = execute(language, cases, Path(directory))
            actual = {}
            for name, output in outputs.items():
                try:
                    value = json.loads(output)
                except json.JSONDecodeError:
                    assert output.strip().startswith("https://app.slack.com/block-kit-builder/"), (
                        language,
                        name,
                        output,
                    )
                    value = output.strip()
                check_result(name, value, language)
                actual[name] = normalize(value) if language == "Python" else value
            for name, value in actual.items():
                page = ROOT / "docs/docs/recipes" / f"{name}.mdx"
                source = page.read_text()
                match = re.search(r"export const recipeOutputs = (.*?);\n", source, re.S)
                expected = json.loads(match[1])
                if args.record:
                    expected[language.lower()] = value
                    page.write_text(
                        source[: match.start(1)]
                        + json.dumps(expected, ensure_ascii=False, indent=2)
                        + source[match.end(1) :]
                    )
                else:
                    assert value == expected[language.lower()], (
                        f"{language} {name}: output differs from its reviewed preview"
                    )
            print(
                f"{language}: all {len(cases)} literal recipes executed; output and intent checks passed",
                flush=True,
            )


if __name__ == "__main__":
    main()
