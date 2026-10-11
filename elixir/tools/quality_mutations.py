#!/usr/bin/env python3
"""Prove API classification and coverage fail closed in an isolated scratch checkout."""

import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]


def run(root, env, args, failure=None):
    result = subprocess.run(
        [shutil.which("mix") or "mix", *args],
        cwd=root,
        env=env,
        text=True,
        capture_output=True,
    )
    output = result.stdout + result.stderr
    if failure is None and result.returncode != 0:
        raise RuntimeError(output)
    if failure is not None and (result.returncode == 0 or failure not in output):
        raise RuntimeError(f"Expected failing guard {failure!r}, got:\n{output}")
    print("Verified", failure or " ".join(args), flush=True)


def main():
    scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
    if scratch and not Path(scratch).is_dir():
        raise SystemExit("Required /Volumes/Scratch mount is unavailable")
    with tempfile.TemporaryDirectory(prefix="slackblocks-elixir-mutations-", dir=scratch) as temp:
        root = Path(temp) / "elixir"
        root.mkdir()
        shutil.copytree(PACKAGE.parent / "spec", Path(temp) / "spec")
        for name in ["lib", "test", "design", "conformance", "tools"]:
            shutil.copytree(
                PACKAGE / name,
                root / name,
                ignore=shutil.ignore_patterns("__pycache__"),
            )
        for name in ["mix.exs", "mix.lock"]:
            shutil.copy2(PACKAGE / name, root / name)
        env = {
            **os.environ,
            "MIX_ENV": "test",
            "MIX_BUILD_ROOT": str(root / "_build"),
            "MIX_DEPS_PATH": os.environ.get("MIX_DEPS_PATH", str(PACKAGE / "deps")),
        }
        env.pop("MIX_BUILD_PATH", None)
        run(root, env, ["test", "test/api_contract_test.exs", "--warnings-as-errors"])
        probe = root / "lib/slackblocks/probe.ex"
        probe.write_text(
            "defmodule Slackblocks.QualityProbe do\n  def produce, do: Slackblocks.Blocks.Divider.new!()\nend\n"
        )
        run(
            root,
            env,
            ["test", "test/api_contract_test.exs"],
            "Unclassified production module: Slackblocks.QualityProbe",
        )
        probe.unlink()
        color = root / "lib/slackblocks/color.ex"
        original = color.read_text()
        color.write_text(
            original.removesuffix("end\n") + "  def new_public_producer, do: named!(:good)\nend\n"
        )
        run(
            root,
            env,
            ["test", "test/api_contract_test.exs"],
            "Unclassified public function or removed API in Slackblocks.Color",
        )
        color.write_text(original)
        run(
            root,
            env,
            [
                "test",
                "--cover",
                "--export-coverage",
                "slackblocks",
                "--warnings-as-errors",
            ],
        )
        run(root, env, ["run", "tools/coverage.exs"])
        # A classified, never-executed production module must be missing from old
        # data, then count as zero coverage when a new export includes it.
        probe.write_text(
            "defmodule Slackblocks.QualityProbe do\n  @moduledoc false\n"
            + "\n".join(f"  def unused_{i}(value), do: value + {i}" for i in range(200))
            + "\nend\n"
        )
        inventory = root / "design/public_api.json"
        data = json.loads(inventory.read_text())
        data["modules"]["Slackblocks.QualityProbe"] = {
            "kind": "internal",
            "reason": "Deliberate scratch-only untouched coverage probe",
            "functions": [f"unused_{i}/1" for i in range(200)],
        }
        inventory.write_text(json.dumps(data))
        run(root, env, ["compile", "--warnings-as-errors"])
        run(
            root,
            env,
            ["run", "tools/coverage.exs"],
            "Missing production modules from coverage",
        )
        run(
            root,
            env,
            [
                "test",
                "--cover",
                "--export-coverage",
                "slackblocks",
                "--warnings-as-errors",
            ],
        )
        run(
            root,
            env,
            ["run", "tools/coverage.exs"],
            "Handwritten executable-line coverage is below 90%",
        )
    print("All API and coverage mutation guards passed")


if __name__ == "__main__":
    main()
