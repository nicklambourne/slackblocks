#!/usr/bin/env python3
"""Build the actual Hex archive and test an independent unpacked consumer."""

import io
import os
import shutil
from pathlib import Path
import subprocess
import tarfile
import tempfile

PACKAGE = Path(__file__).resolve().parents[1]


def run(args, cwd=PACKAGE, env=None):
    executable = shutil.which(args[0]) or args[0]
    subprocess.run([executable, *args[1:]], cwd=cwd, env=env, check=True)


def main():
    run(["mix", "hex.build"])
    archives = list(PACKAGE.glob("slackblocks-*.tar"))
    if len(archives) != 1:
        raise SystemExit(f"Expected one package artifact, found {archives}")
    volume_host = Path("/Volumes/Repos").is_dir()
    if volume_host and not all(
        Path(p).is_dir() for p in ["/Volumes/Cache", "/Volumes/Scratch"]
    ):
        raise SystemExit("Required external storage volumes are unavailable")
    scratch = "/Volumes/Scratch" if volume_host else None
    with tempfile.TemporaryDirectory(
        prefix="slackblocks-elixir-package-", dir=scratch
    ) as tmp:
        root = Path(tmp)
        source = root / "package"
        source.mkdir()
        with tarfile.open(archives[0]) as outer:
            data = outer.extractfile("contents.tar.gz").read()
            metadata = outer.extractfile("metadata.config").read().decode()
        for expected in ["MIT", "BSD-3-Clause", "jason", "decimal", "slackblocks"]:
            if expected not in metadata:
                raise SystemExit(f"Package metadata is missing {expected}")
        with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as inner:
            names = set(inner.getnames())
            for required in [
                "mix.exs",
                "README.md",
                "CHANGELOG.md",
                "LICENSE",
                "LICENSE.BSD-3-Clause",
                "lib/slackblocks.ex",
            ]:
                if required not in names:
                    raise SystemExit(f"Package is missing {required}")
            for member in inner.getmembers():
                name = Path(member.name)
                if (
                    name.is_absolute()
                    or ".." in name.parts
                    or member.issym()
                    or member.islnk()
                ):
                    raise SystemExit(f"Unsafe package member {name}")
                if name.parts[0] not in {
                    "lib",
                    "mix.exs",
                    "README.md",
                    "CHANGELOG.md",
                    "LICENSE",
                    "LICENSE.BSD-3-Clause",
                }:
                    raise SystemExit(f"Unexpected package member {name}")
            inner.extractall(source)
        consumer = root / "consumer"
        (consumer / "test").mkdir(parents=True)
        decimal = os.environ.get("SLACKBLOCKS_DECIMAL", "~> 2.4 or ~> 3.0")
        (consumer / "mix.exs").write_text(
            '''defmodule ArtifactConsumer.MixProject do
  use Mix.Project
  def project, do: [app: :artifact_consumer, version: "0.0.0", deps: [
    {:slackblocks, path: "../package"},
    {:decimal, "'''
            + decimal
            + """", override: true}
  ]]
  def application, do: []
end
"""
        )
        (consumer / "test/test_helper.exs").write_text("ExUnit.start()\n")
        (consumer / "test/foundation_test.exs").write_text(
            (PACKAGE / "test/foundation_test.exs").read_text()
        )
        (consumer / "test/helpers_test.exs").write_text(
            (PACKAGE / "test/helpers_test.exs").read_text()
        )
        env = dict(os.environ)
        for key in ["MIX_BUILD_PATH", "MIX_BUILD_ROOT", "MIX_DEPS_PATH", "MIX_ENV"]:
            env.pop(key, None)
        run(["mix", "deps.get"], consumer, env)
        run(["mix", "test", "--warnings-as-errors"], consumer, env)
        run(
            ["mix", "compile", "--warnings-as-errors"],
            consumer,
            {**env, "MIX_ENV": "prod"},
        )
        # Call the compiled public API from Erlang, with the dependency application started.
        beams = [str(p) for p in (consumer / "_build/prod/lib").glob("*/ebin")]
        # A script avoids cmd.exe interpreting parentheses in a .bat -e argument.
        probe = consumer / "runtime_path.exs"
        probe.write_text("IO.puts(:code.lib_dir(:elixir))\n")
        elixir_lib = Path(
            subprocess.check_output(
                [shutil.which("elixir") or "elixir", str(probe)], env=env, text=True
            ).strip()
        ).parent
        beams += [str(p) for p in elixir_lib.glob("*/ebin")]
        expr = '{ok,_}=application:ensure_all_started(slackblocks), {ok,S}=\'Elixir.Slackblocks.Blocks.Section\':new([{text,<<"Erlang">>}]), {ok,W}=\'Elixir.Slackblocks\':to_map(S), <<"section">> = maps:get(<<"type">>,W), halt(0).'
        run(["erl", "-noshell", "-pa", *beams, "-eval", expr], consumer, env)
        print(
            "Actual Hex artifact: independent tests, production compilation and Erlang calls passed"
        )


if __name__ == "__main__":
    main()
