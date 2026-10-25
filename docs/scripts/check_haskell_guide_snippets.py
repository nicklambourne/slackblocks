#!/usr/bin/env python3
"""Compile and execute the literal Haskell programs displayed in current guides."""

import decimal
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
FENCE = re.compile(r'^```haskell(?: title="([^"]+)")?\n(.*?)\n```', re.M | re.S)


def main():
    files = [ROOT / "README.md", ROOT / "haskell/README.md"]
    files += sorted(
        p
        for p in (ROOT / "docs/docs").rglob("*.mdx")
        if "reference" not in p.relative_to(ROOT / "docs/docs").parts
    )
    cases, counts = [], {}
    for path in files:
        source = path.read_text(encoding="utf-8")
        snippets = list(FENCE.finditer(source))
        if snippets:
            counts[path.relative_to(ROOT).as_posix()] = len(snippets)
        for index, match in enumerate(snippets):
            title, code = match.groups()
            label = f"{path.relative_to(ROOT)}:{index + 1}"
            if title:
                if (
                    title != "Send.hs (integration-tested)"
                    or path.name != "sending_messages.mdx"
                ):
                    raise ValueError(f"Unrecognized externally tested snippet: {label}")
                assert (
                    code
                    == (ROOT / "haskell/integrations/src/Send.hs")
                    .read_text(encoding="utf-8")
                    .rstrip()
                ), "HTTP example drifted from executed source"
                continue
            expected = None
            if path.name == "using_blocks.mdx":
                heading = re.findall(r"^## (.+)$", source[: match.start()], re.M)[-1]
                if heading.endswith(" Block"):
                    following = source[match.end() :].split("\n## ")[0]
                    wire = re.search(r"```json\n(.*?)\n```", following, re.S)
                    assert wire, f"{label} has no expected JSON"
                    assert not re.search(r"\b(fromValue|decode)\b", code), (
                        "Block examples must use native construction"
                    )
                    expected = json.loads(wire[1], parse_float=decimal.Decimal)
            cases.append((label, code, expected))
    inventory = json.loads((ROOT / "docs/examples/haskell/coverage.json").read_text())
    assert counts == inventory, (
        f"Review the literal Haskell snippet inventory: {counts}"
    )
    assert sum(expected is not None for _, _, expected in cases) == 21, (
        "Every documented block requires a native example"
    )
    cases.append(
        (
            "section_hello.hs",
            (ROOT / "docs/examples/haskell/section_hello.hs").read_text(),
            json.loads((ROOT / "docs/examples/section_hello.json").read_text()),
        )
    )
    scratch = os.environ.get("RUNNER_TEMP")
    if Path("/Volumes/Repos").is_dir():
        assert all(
            Path("/Volumes", name).is_mount() for name in ("Repos", "Cache", "Scratch")
        ), "External volumes required"
        scratch = "/Volumes/Scratch"
    build = os.environ.get("SLACKBLOCKS_HASKELL_BUILD", "dist-docs")
    cabal = ["cabal", f"--builddir={build}"]
    subprocess.run(
        [*cabal, "build", "lib:slackblocks", "--jobs=2"],
        cwd=ROOT / "haskell",
        check=True,
    )
    with tempfile.TemporaryDirectory(prefix="haskell-guide-", dir=scratch) as directory:
        temp = Path(directory)
        for index, (_, code, _) in enumerate(cases):
            pragmas = re.findall(r"^\{-#.*?#-\}\s*$", code, re.M)
            code = re.sub(r"^\{-#.*?#-\}\s*$", "", code, flags=re.M).lstrip()
            assert not re.search(r"^module ", code, re.M), (
                "Guide programs should not hard-code a module name"
            )
            source = (
                "\n".join(pragmas)
                + f"\nmodule Snippet{index} (main) where\nimport qualified Data.ByteString.Lazy.Char8 as SnippetOutput\n"
                + code
            )
            if not re.search(r"^main\s*(?:::|=)", code, re.M):
                assert re.search(r"^example\s*(?:::|=)", code, re.M), (
                    "A fragment must expose its example result"
                )
                source += "\nmain :: IO ()\nmain = either (fail . show) (SnippetOutput.putStrLn . encode) example\n"
            (temp / f"Snippet{index}.hs").write_text(source, encoding="utf-8")
        driver = "module Main where\nimport System.Environment (getArgs)\n"
        driver += "".join(f"import qualified Snippet{i}\n" for i in range(len(cases)))
        driver += (
            "main = do\n  args <- getArgs\n  case args of\n"
            + "".join(f'    ["{i}"] -> Snippet{i}.main\n' for i in range(len(cases)))
            + '    _ -> fail "Expected snippet index"\n'
        )
        (temp / "Main.hs").write_text(driver, encoding="utf-8")
        executable = temp / ("guides.exe" if os.name == "nt" else "guides")
        subprocess.run(
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
                str(temp),
                str(temp / "Main.hs"),
                "-o",
                str(executable),
            ],
            cwd=ROOT / "haskell",
            check=True,
        )
        for index, (label, _, expected) in enumerate(cases):
            output = subprocess.check_output(
                [str(executable), str(index)], text=True, timeout=30
            )
            actual = json.loads(output, parse_float=decimal.Decimal)
            if expected is not None:
                assert actual == expected, (
                    f"{label} differs from its documented JSON: {actual!r}"
                )
    print(
        f"{len(cases)} literal Haskell programs executed; 21 block examples and section_hello match independent JSON."
    )


if __name__ == "__main__":
    main()
