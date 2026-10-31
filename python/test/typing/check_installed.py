"""Check the installed wheel from outside the source tree, including negative types."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path


def main() -> None:
    fixtures = Path(__file__).resolve().parent
    scratch = Path("/Volumes/Scratch")
    with tempfile.TemporaryDirectory(dir=scratch if scratch.is_dir() else None) as directory:
        environment = {k: v for k, v in os.environ.items() if k not in {"MYPYPATH", "PYTHONPATH"}}
        for name, expected_errors in (("positive", 0), ("negative", 11)):
            target = Path(directory) / f"{name}.py"
            target.write_text((fixtures / f"{name}.py").read_text())
            result = subprocess.run(
                [
                    sys.executable,
                    "-m",
                    "mypy",
                    "--strict",
                    "--no-incremental",
                    "--cache-dir",
                    os.devnull,
                    "--no-error-summary",
                    str(target),
                ],
                cwd=directory,
                env=environment,
                text=True,
                capture_output=True,
                check=False,
            )
            errors = [line for line in result.stdout.splitlines() if ": error:" in line]
            if (
                result.returncode != int(bool(expected_errors))
                or len(errors) != expected_errors
                or any("[arg-type]" not in line for line in errors)
            ):
                raise SystemExit(f"{name} contract failed:\n{result.stdout}{result.stderr}")
            print(f"{name}: {len(errors)} expected argument errors; no import errors")


if __name__ == "__main__":
    main()
