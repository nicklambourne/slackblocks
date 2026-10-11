#!/usr/bin/env python3
"""Resolve sending integrations independently against the actual Hex artifact."""

import argparse
import os
import re
import shutil
import tempfile
from pathlib import Path

from package_check import PACKAGE, extract_archive, run


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", choices=["locked", "lower", "latest"], default="locked")
    args = parser.parse_args()
    scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
    if scratch and not all(Path(p).is_dir() for p in [scratch, "/Volumes/Cache"]):
        raise SystemExit("Required external storage mounts are unavailable")
    with tempfile.TemporaryDirectory(prefix="slackblocks-elixir-sending-", dir=scratch) as temp:
        root = Path(temp)
        archive = root / "slackblocks.tar"
        run(["mix", "hex.build", "--output", str(archive)])
        extract_archive(archive, root / "package")
        consumer = root / "integrations"
        shutil.copytree(
            PACKAGE / "integrations",
            consumer,
            ignore=shutil.ignore_patterns("_build", "deps"),
        )
        manifest = consumer / "mix.exs"
        text = manifest.read_text().replace('path: ".."', 'path: "../package"')
        if args.profile == "lower":
            text = (
                text.replace('"~> 1.2.2"', '"== 1.2.2"')
                .replace('">= 0.5.0 and < 1.0.0"', '"== 0.5.0"')
                .replace('"~> 3.14.0"', '"== 3.14.0"')
                .replace("deps: [", 'deps: [{:jason, "== 1.4.5"}, {:decimal, "== 3.0.0"},')
            )
        manifest.write_text(text)
        if args.profile != "locked":
            (consumer / "mix.lock").unlink()
        env = dict(os.environ)
        for key in ["MIX_BUILD_PATH", "MIX_BUILD_ROOT", "MIX_DEPS_PATH", "MIX_ENV"]:
            env.pop(key, None)
        expected = re.search(r'version:\s*"([^"]+)"', (PACKAGE / "mix.exs").read_text()).group(1)
        env["SLACKBLOCKS_EXPECTED_VERSION"] = expected
        run(["mix", "deps.get"], consumer, env)
        run(["mix", "compile", "--warnings-as-errors"], consumer, env)
        run(["mix", "test", "--warnings-as-errors"], consumer, env)
        run(
            ["mix", "compile", "--warnings-as-errors"],
            consumer,
            {**env, "MIX_ENV": "prod"},
        )
        if args.profile == "locked":
            previous_jason(root, env)
        print(f"Actual artifact sending/Ecto integration profile passed: {args.profile}")


def previous_jason(root, env):
    consumer = root / "previous-jason"
    consumer.mkdir()
    manifest = consumer / "mix.exs"
    source = """defmodule PreviousJason.MixProject do
  use Mix.Project
  def project, do: [app: :previous_jason, version: "0.0.0", deps: [{:jason, "~> 1.4"}]]
  def application, do: []
end
"""
    manifest.write_text(source)
    (consumer / "before.exs").write_text(
        '"{\\"ready\\":true}" = Jason.encode!(%{"ready" => true})\nfalse = Code.ensure_loaded?(Decimal)\n'
    )
    env = {**env, "MIX_ENV": "prod"}
    run(["mix", "deps.get"], consumer, env)
    run(["mix", "run", "before.exs"], consumer, env)
    decoder = consumer / "_build/prod/lib/jason/ebin/Elixir.Jason.Decoder.beam"
    previous_decoder = decoder.read_bytes()
    manifest.write_text(source.replace("deps: [", 'deps: [{:slackblocks, path: "../package"},'))
    (
        consumer / "after.exs"
    ).write_text("""0.1 = Slackblocks.decode!(Slackblocks.Objects.RawNumber, ~s({"type":"raw_number","text":"0.1","value":0.1})).value
true = Protocol.consolidated?(Jason.Encoder)
%{"type" => "divider"} = Slackblocks.Blocks.Divider.new!() |> Jason.encode!() |> Jason.decode!()
""")
    run(["mix", "deps.get"], consumer, env)
    run(["mix", "compile"], consumer, env)
    # Mix may or may not invalidate this optional compile-time integration.
    # Restore the genuine earlier beam to deterministically exercise that state.
    decoder.write_bytes(previous_decoder)
    (
        consumer / "stale.exs"
    ).write_text("""{:error, %ArgumentError{message: message}} = Slackblocks.decode(Slackblocks.Blocks.Divider, ~s({"type":"divider"}))
true = String.contains?(message, "mix deps.compile jason --force")
try do
  Slackblocks.decode!(Slackblocks.Blocks.Divider, ~s({"type":"divider"}))
  raise "expected actionable stale-Jason error"
rescue
  error in ArgumentError -> true = String.contains?(error.message, "mix deps.compile jason --force")
end
""")
    run(["mix", "run", "--no-compile", "stale.exs"], consumer, env)
    # These are the documented commands for an existing application's build cache.
    run(["mix", "deps.compile", "jason", "--force"], consumer, env)
    run(["mix", "compile", "--force"], consumer, env)
    run(["mix", "run", "after.exs"], consumer, env)
    print("Precompiled Jason diagnostic, documented rebuild and consolidated protocol passed")


if __name__ == "__main__":
    main()
