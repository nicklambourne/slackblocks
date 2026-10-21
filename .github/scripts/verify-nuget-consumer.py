#!/usr/bin/env python3
"""Restore one exact public NuGet version in a fresh external consumer; never publish."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

SOURCE = "https://api.nuget.org/v3/index.json"
PROGRAM = '''using System.Text.Json;
using Slackblocks;
using Slackblocks.Blocks;
using Slackblocks.Elements;
using Slackblocks.Payloads;
var block = new SectionBlock("Review deployment", blockId: "release-section",
    accessory: new ButtonElement("Review", "review", value: "deploy-1"));
var message = new MessagePayload("C0123456", text: "Review deployment", blocks: [block]);
using var wire = JsonDocument.Parse(message.ToJson());
var section = wire.RootElement.GetProperty("blocks")[0];
if (section.GetProperty("block_id").GetString() != "release-section" ||
    section.GetProperty("accessory").GetProperty("action_id").GetString() != "review")
    throw new Exception("Native public constructor/serialization mismatch");
try { _ = new HeaderBlock(new string('x', 151)); throw new Exception("Missing validation"); }
catch (ValidationException error) {
    if (error.Category != ErrorCategory.LengthExceeded) throw;
}
Console.WriteLine("native-consumer-ok");
'''


def prepare(workdir, version):
    if not re.fullmatch(r"\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?", version):
        raise ValueError("Use an exact major.minor.patch version, optionally prerelease")
    workdir.mkdir(parents=True, exist_ok=False)
    (workdir / "NuGet.Config").write_text(
        '<configuration><packageSources><clear/><add key="nuget.org" value="' + SOURCE +
        '"/></packageSources><fallbackPackageFolders><clear/></fallbackPackageFolders></configuration>\n')
    (workdir / "Consumer.csproj").write_text('''<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable>
    <ManagePackageVersionsCentrally>false</ManagePackageVersionsCentrally>
  </PropertyGroup>
  <ItemGroup><PackageReference Include="Slackblocks" Version="[''' + version + ''']" /></ItemGroup>
</Project>
''')
    (workdir / "Program.cs").write_text(PROGRAM)


def verify_assets(workdir, version):
    assets = json.loads((workdir / "obj/project.assets.json").read_text())
    found = [name for name in assets["libraries"] if name.lower().startswith("slackblocks/")]
    if found != ["Slackblocks/" + version]:
        raise ValueError(f"Expected exactly Slackblocks/{version}, resolved {found}")
    package = workdir / "packages/slackblocks" / version.lower()
    metadata = json.loads((package / ".nupkg.metadata").read_text())
    if metadata.get("source") != SOURCE:
        raise ValueError("Package did not come from the public NuGet source")
    archive = package / ("slackblocks." + version.lower() + ".nupkg")
    return hashlib.sha256(archive.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True)
    parser.add_argument("--workdir", type=Path, required=True, help="New disposable directory, outside the checkout")
    parser.add_argument("--dotnet", default="dotnet")
    parser.add_argument("--attempts", type=int, default=1, choices=range(1, 7), help="Bounded indexing-delay retries")
    args = parser.parse_args()
    root = args.workdir.resolve()
    checkout = Path(__file__).resolve().parents[2]
    if root == checkout or checkout in root.parents:
        parser.error("--workdir must be outside the checkout")
    # Never reuse a prior restore or a locally populated package cache.
    root.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    env.pop("NUGET_API_KEY", None)
    env.update(DOTNET_CLI_TELEMETRY_OPTOUT="1", DOTNET_NOLOGO="1", DOTNET_SKIP_FIRST_TIME_EXPERIENCE="1",
               DOTNET_CLI_HOME=str(root / "dotnet-home"), NUGET_HTTP_CACHE_PATH=str(root / "http-cache"),
               NUGET_PLUGINS_CACHE_PATH=str(root / "plugins"), NUGET_FALLBACK_PACKAGES="")
    receipt = {"version": args.version, "source": SOURCE, "attempts": [], "passed": False}
    try:
        for attempt in range(1, args.attempts + 1):
            work = root / f"attempt-{attempt}"
            prepare(work, args.version)
            env["NUGET_PACKAGES"] = str(work / "packages")
            props = ["-p:ImportDirectoryBuildProps=false", "-p:ImportDirectoryBuildTargets=false"]
            restore = subprocess.run([args.dotnet, "restore", "Consumer.csproj", "--configfile", "NuGet.Config",
                                      "--no-cache", "--force-evaluate", *props], cwd=work, env=env,
                                     text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            (work / "restore.log").write_text(restore.stdout)
            receipt["attempts"].append({"number": attempt, "restore_exit_code": restore.returncode})
            if restore.returncode:
                # Only a missing version can be an indexing delay; auth/network/build errors fail immediately.
                errors = set(re.findall(r"error (NU\d+|MSB\d+|NETSDK\d+)", restore.stdout))
                if errors == {"NU1102"} and attempt < args.attempts:
                    time.sleep(10)
                    continue
                raise RuntimeError(f"Restore failed; inspect {work / 'restore.log'}")
            receipt["sha256"] = verify_assets(work, args.version)
            result = subprocess.run([args.dotnet, "run", "--project", "Consumer.csproj", "--no-restore",
                                     "--configuration", "Release", *props], cwd=work, env=env,
                                    text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            (work / "consumer.log").write_text(result.stdout)
            if result.returncode or "native-consumer-ok" not in result.stdout:
                raise RuntimeError(f"Native consumer failed; inspect {work / 'consumer.log'}")
            receipt.update(passed=True, native_consumer="constructor, serialization, validation")
            break
    finally:
        (root / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt))


if __name__ == "__main__":
    main()
