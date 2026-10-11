#!/usr/bin/env python3
"""Exercise the Hex artifact from Rebar3, EUnit and an independent OTP release."""

import functools
import http.server
import os
import shutil
import subprocess
import tempfile
import threading
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]


def main():
    subprocess.run(["mix", "hex.build"], cwd=PACKAGE, check=True)
    archives = list(PACKAGE.glob("slackblocks-*.tar"))
    if len(archives) != 1:
        raise SystemExit("Expected exactly one Hex artifact")
    scratch = "/Volumes/Scratch" if Path("/Volumes/Repos").is_dir() else None
    if scratch and not Path(scratch).is_dir():
        raise SystemExit("Required scratch volume is unavailable")
    env = dict(os.environ)
    for key in [
        "MIX_BUILD_PATH",
        "MIX_BUILD_ROOT",
        "MIX_DEPS_PATH",
        "MIX_ENV",
        "REBAR_BASE_DIR",
    ]:
        env.pop(key, None)
    if scratch:
        env["REBAR_CACHE_DIR"] = "/Volumes/Cache/slackblocks-elixir/rebar"
    with tempfile.TemporaryDirectory(prefix="slackblocks-erlang-", dir=scratch) as tmp:
        root = Path(tmp)
        shutil.copytree(
            PACKAGE / "erlang",
            root / "consumer",
            ignore=shutil.ignore_patterns("_build", "_checkouts"),
        )
        consumer = root / "consumer"
        registry = root / "registry"
        (registry / "tarballs").mkdir(parents=True)
        shutil.copy2(archives[0], registry / "tarballs" / archives[0].name)
        server = http.server.ThreadingHTTPServer(
            ("127.0.0.1", 0),
            functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(registry)),
        )
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        config = consumer / "rebar.config"
        text = config.read_text().replace(
            '{slackblocks, "2.8.0-dev"}',
            '{slackblocks, {hex, slackblocks, "2.8.0-dev"}}',
        )
        text = text.replace(
            '{decimal, "3.2.0"}',
            '{decimal, "' + env.get("SLACKBLOCKS_ERLANG_DECIMAL", "3.2.0") + '"}',
        )
        text += f'\n{{elixir_opts, [{{cdn, "http://127.0.0.1:{server.server_port}"}}]}}.\n'
        config.write_text(text)
        try:
            for args in [["rebar3", "eunit"], ["rebar3", "release"]]:
                subprocess.run(args, cwd=consumer, env=env, check=True)
        finally:
            server.shutdown()
            server.server_close()
        release = consumer / "_build/default/rel/slackblocks_erlang"
        deployed = root / "deployed"
        shutil.copytree(release, deployed)
        # No checkout, compiler, Mix or Rebar3 is available to the deployed release.
        shutil.rmtree(consumer)
        subprocess.run(
            [
                str(deployed / "bin/slackblocks_erlang"),
                "foreground",
                "-noshell",
                "-eval",
                "ok=slackblocks_erlang:verify(), true='Elixir.Protocol':'consolidated?'('Elixir.Jason.Encoder'), halt(0).",
            ],
            cwd=root,
            env={**env, "PATH": "/usr/bin:/bin"},
            check=True,
            timeout=60,
        )
        print("Erlang Hex artifact, EUnit, protocol consolidation and standalone release passed")


if __name__ == "__main__":
    main()
