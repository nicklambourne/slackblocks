#!/usr/bin/env python3
"""Exercise the checker dispatcher without running or skipping any real CI gates."""

import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]


def main():
    scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
    if scratch and not Path(scratch).is_dir():
        raise SystemExit("Required /Volumes/Scratch mount is unavailable")
    elixir = shutil.which("elixir")
    if not elixir:
        raise SystemExit("Elixir is required for the checker dispatch test")
    with tempfile.TemporaryDirectory(prefix="slackblocks-check-profiles-", dir=scratch) as temp:
        root = Path(temp)
        (root / "integrations").mkdir()
        tool = (
            f"#!{sys.executable}\n"
            "import json, os, sys\n"
            "from pathlib import Path\n"
            "with open(os.environ['CHECK_COMMANDS'], 'a') as output:\n"
            "    output.write(json.dumps([Path(sys.argv[0]).name, sys.argv[1:]]) + '\\n')\n"
            "sys.exit(int(os.environ.get('CHECK_EXIT', '0')))\n"
        )
        for name in ["mix", "python-quality", "ruff"]:
            path = root / name
            path.write_text(tool)
            path.chmod(0o755)
        env = {
            **os.environ,
            "PATH": str(root) + os.pathsep + os.environ["PATH"],
            "SLACKBLOCKS_PYTHON": str(root / "python-quality"),
            "SLACKBLOCKS_RUFF": str(root / "ruff"),
        }

        def dispatch(profile, **extra):
            log = root / f"{profile or 'default'}.jsonl"
            log.unlink(missing_ok=True)
            args = [elixir, str(PACKAGE / "tools/check.exs")]
            if profile:
                args += ["--profile", profile]
            result = subprocess.run(
                args,
                cwd=root,
                env={**env, "CHECK_COMMANDS": str(log), **extra},
                text=True,
                capture_output=True,
            )
            commands = (
                [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []
            )
            return result, commands

        full_result, full = dispatch(None)
        ci_result, ci = dispatch("ci-quality")
        assert full_result.returncode == ci_result.returncode == 0, (
            full_result.stderr,
            ci_result.stderr,
        )
        duplicated = [
            ["python-quality", ["tools/erlang_check.py"]],
            *[
                ["python-quality", ["tools/integration_check.py", "--profile", profile]]
                for profile in ["locked", "lower", "latest"]
            ],
        ]
        assert all(full.count(command) == 1 for command in duplicated)
        assert ci == [command for command in full if command not in duplicated]
        for command in [
            ["mix", ["dialyzer"]],
            [
                "mix",
                ["test", "--cover", "--export-coverage", "slackblocks", "--warnings-as-errors"],
            ],
            ["mix", ["run", "tools/coverage.exs"]],
            ["python-quality", ["tools/quality_mutations.py"]],
            ["python-quality", ["tools/package_check.py"]],
            ["python-quality", ["generator/generate.py", "--check"]],
        ]:
            assert command in ci, command
        failed, commands = dispatch("ci-quality", CHECK_EXIT="23")
        assert failed.returncode == 23 and len(commands) == 1, failed.stdout
        assert "status 23" in failed.stdout
        invalid, commands = dispatch("unknown")
        assert invalid.returncode != 0 and commands == []
    print(
        "Checker dispatch: default full preserved, CI omits only four duplicate consumers, failures propagate"
    )


if __name__ == "__main__":
    main()
