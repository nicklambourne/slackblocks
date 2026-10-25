#!/usr/bin/env python3
"""Run all ten native multi-select consumers; validate JSON without Slack traffic."""

import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EXAMPLES = ROOT / "docs/examples/multiselect-compat"


def run(args, cwd=ROOT, env=None):
    subprocess.run([str(arg) for arg in args], cwd=cwd, env=env, check=True)


def probe(language, args, directory, cwd=ROOT, env=None):
    output = directory / f"{language}.jsonl"
    try:
        with output.open("w") as stream:
            subprocess.run(
                [str(arg) for arg in args], cwd=cwd, env=env, stdout=stream, check=True
            )
        run([sys.executable, EXAMPLES / "check.py", output])
    except subprocess.CalledProcessError:
        print(output.read_text(), file=sys.stderr)
        raise
    print(f"Verified {language}: 15 native multi-select payloads", flush=True)


def java_tool(name):
    return (
        Path(os.environ["JAVA_HOME"]) / "bin" / name
        if "JAVA_HOME" in os.environ
        else name
    )


scratch = None
if Path("/Volumes/Repos").exists():
    for mount in ("Repos", "Cache", "Scratch"):
        assert os.path.ismount("/Volumes/" + mount), f"Missing /Volumes/{mount}"
    scratch = "/Volumes/Scratch"

with tempfile.TemporaryDirectory(
    prefix="slackblocks-multiselect-", dir=scratch
) as temporary:
    directory = Path(temporary)
    probe(
        "Python",
        [sys.executable, EXAMPLES / "python.py"],
        directory,
        env={**os.environ, "PYTHONPATH": str(ROOT / "python")},
    )
    run(["pnpm", "--filter", "@nicklambourne/slackblocks", "build"])
    probe(
        "TypeScript",
        ["node", EXAMPLES / "node.mjs"],
        directory,
        env={
            **os.environ,
            "SLACKBLOCKS_MODULE": (ROOT / "typescript/dist/index.mjs").as_uri(),
        },
    )
    probe("Ruby", ["ruby", "-I", ROOT / "ruby/lib", EXAMPLES / "ruby.rb"], directory)
    probe(
        "PHP",
        ["php", EXAMPLES / "php.php", ROOT / "php/vendor/autoload.php"],
        directory,
    )
    probe("Go", ["go", "run", "-mod=readonly", "."], directory, cwd=EXAMPLES / "go")
    probe(
        "Rust",
        [
            "cargo",
            "+1.85.0",
            "run",
            "--locked",
            "--quiet",
            "--manifest-path",
            EXAMPLES / "rust/Cargo.toml",
        ],
        directory,
    )

    project = EXAMPLES / "csharp/Multiselect.csproj"
    run(["dotnet", "build", project, "--nologo"])
    probe(
        "C#",
        [
            "dotnet",
            "run",
            "--no-build",
            "--roll-forward",
            "Major",
            "--project",
            project,
        ],
        directory,
    )

    classpath_file = directory / "classpath.txt"
    wrapper = ROOT / "java" / ("mvnw.cmd" if os.name == "nt" else "mvnw")
    run(
        [
            wrapper,
            "-B",
            "-ntp",
            "-q",
            "compile",
            "dependency:build-classpath",
            f"-Dmdep.outputFile={classpath_file}",
            "-Dmdep.includeScope=compile",
        ],
        cwd=ROOT / "java",
    )
    classpath = (
        str(ROOT / "java/target/classes")
        + os.pathsep
        + classpath_file.read_text().strip()
    )
    classes = directory / "classes"
    classes.mkdir()
    run(
        [
            java_tool("javac"),
            "--release",
            "17",
            "-cp",
            classpath,
            "-d",
            classes,
            EXAMPLES / "java/Multiselect.java",
        ]
    )
    probe(
        "Java",
        [
            java_tool("java"),
            "-cp",
            str(classes) + os.pathsep + classpath,
            "Multiselect",
        ],
        directory,
    )

    run(["mix", "compile", "--warnings-as-errors"], cwd=ROOT / "elixir")
    probe(
        "Elixir",
        ["mix", "run", "--no-compile", "--no-deps-check", EXAMPLES / "elixir.exs"],
        directory,
        cwd=ROOT / "elixir",
    )

    haskell_build = os.environ.get("SLACKBLOCKS_HASKELL_BUILD", "dist-docs")
    executable = directory / (
        "haskell-probe.exe" if os.name == "nt" else "haskell-probe"
    )
    run(
        [
            "cabal",
            f"--builddir={haskell_build}",
            "build",
            "lib:slackblocks",
            "--jobs=2",
        ],
        cwd=ROOT / "haskell",
    )
    run(
        [
            "cabal",
            f"--builddir={haskell_build}",
            "exec",
            "--",
            "ghc",
            "-O0",
            "-package",
            "slackblocks",
            "-outputdir",
            directory,
            EXAMPLES / "haskell.hs",
            "-o",
            executable,
        ],
        cwd=ROOT / "haskell",
    )
    probe("Haskell", [executable], directory)

print(
    "Executed all ten multi-select consumers: 150 payloads; no Slack acceptance inferred."
)
