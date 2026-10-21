"""Measurement-only Ruby native bundle comparison; never merge this harness."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

BASE = "a96dcce13d921325716949e38953a0e02f0bc6c2"
SOURCE = "34237ba0b90cec56618788cd4f559b90fd0febbf"
ROOT = Path.cwd() / "source"
RESULT = Path.cwd() / "benchmark-results"
RESULT.mkdir(exist_ok=True)
META = RESULT / "metadata.json"


def capture(args, cwd=ROOT):
    return subprocess.check_output(args, cwd=cwd, text=True, encoding="utf-8").strip()


def phase(name, args, cwd=ROOT / "ruby", env=None):
    start = time.perf_counter()
    began = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (RESULT / f"{name}.log").open("w", encoding="utf-8") as log:
        p = subprocess.run(args, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT)
    record = {"phase": name, "seconds": time.perf_counter() - start,
              "started_at": began, "exit_code": p.returncode, "command": args}
    with (RESULT / "phases.jsonl").open("a", encoding="utf-8") as f:
        f.write(json.dumps(record) + "\n")
    print((RESULT / f"{name}.log").read_text(encoding="utf-8"), flush=True)
    if p.returncode:
        raise RuntimeError(f"{name} failed: {p.returncode}")


if sys.argv[1] == "prepare":
    assert capture(["git", "rev-parse", "HEAD"]) == SOURCE
    assert not capture(["git", "status", "--porcelain"])
    mode = os.environ["BENCH_MODE"]
    assert mode in ("baseline", "candidate")
    if mode == "baseline":
        (ROOT / "ruby/Gemfile").write_text(capture(["git", "show", f"{BASE}:ruby/Gemfile"]) + "\n")
    assert os.environ.get("BUNDLE_WITHOUT", "") == ("quality" if mode == "candidate" else "")
    files = capture(["git", "ls-files", "ruby", "spec", "docs/examples/ruby", "README.md"]).splitlines()
    hashes = {f: hashlib.sha256((ROOT / f).read_bytes()).hexdigest() for f in files if f != "ruby/Gemfile"}
    metadata = {"source_commit": SOURCE, "baseline_commit": BASE,
                "harness_commit": capture(["git", "rev-parse", "HEAD"], Path.cwd() / "harness"),
                "mode": mode, "temperature": os.environ["BENCH_TEMP"], "pair": os.environ["BENCH_PAIR"],
                "runner": {k: os.environ.get(k) for k in ("RUNNER_NAME", "RUNNER_OS", "RUNNER_ARCH", "ImageOS", "ImageVersion")},
                "inputs": hashes, "lock_sha256": hashes["ruby/Gemfile.lock"],
                "gemfile_sha256": hashlib.sha256((ROOT / "ruby/Gemfile").read_bytes()).hexdigest()}
    META.write_text(json.dumps(metadata, indent=2) + "\n")
else:
    metadata = json.loads(META.read_text())
    metadata["ruby"] = capture(["ruby", "--version"])
    metadata["bundler"] = capture(["ruby", "-S", "bundle", "--version"], ROOT / "ruby")
    assert "ruby 3.4.11 " in metadata["ruby"]
    metadata["bundle_config"] = capture(["ruby", "-S", "bundle", "config", "list"], ROOT / "ruby")
    metadata["bundle_specs"] = json.loads(capture(["ruby", "-S", "bundle", "exec", "ruby", "-rjson", "-e",
        'puts JSON.generate(Gem.loaded_specs.transform_values { |s| s.version.to_s })'], ROOT / "ruby"))
    assert metadata["bundle_specs"]["minitest"] == "5.27.0"
    for name in ("rbs", "standard", "steep"):
        assert (name in metadata["bundle_specs"]) == (metadata["mode"] == "baseline")
    cache = ROOT / "ruby/vendor/bundle"
    metadata["installed_cache_bytes"] = sum(p.stat().st_size for p in cache.rglob("*") if p.is_file())
    META.write_text(json.dumps(metadata, indent=2) + "\n")
    # Both timed profiles use the same locked Minitest and complete test inventory.
    phase("tests", ["ruby", "-S", "bundle", "exec", "ruby", "-I", "test", "-e",
                    'Dir[File.expand_path("test/*_test.rb")].sort.each { |path| require path }'])
    test_log = (RESULT / "tests.log").read_text()
    counts = re.search(r"(\d+) runs, (\d+) assertions, (\d+) failures, (\d+) errors, (\d+) skips", test_log)
    assert counts and tuple(map(int, counts.groups())) == (419, 1598, 0, 0, 0)
    metadata["tests"] = list(map(int, counts.groups()))
    package = RESULT / "slackblocks.gem"
    phase("package_build", ["ruby", "-S", "gem", "build", "slackblocks.gemspec", "--output", str(package)])
    metadata["archive_files"] = json.loads(capture(["ruby", "-rrubygems/package", "-rjson", "-e",
        'puts JSON.generate(Gem::Package.new(ARGV.fetch(0)).contents.sort)', str(package)]))
    metadata["archive_sha256"] = hashlib.sha256(package.read_bytes()).hexdigest()
    install = Path(os.environ["RUNNER_TEMP"]) / "ruby-benchmark-consumer"
    assert not install.exists()
    clean = {k: v for k, v in os.environ.items() if not k.startswith("BUNDLE_") and k not in ("GEM_HOME", "GEM_PATH", "RUBYOPT", "RUBYLIB")}
    clean.update(GEM_HOME=str(install), GEM_PATH=str(install))
    phase("package_install", ["ruby", "-S", "gem", "install", str(package), "--install-dir", str(install), "--no-document", "--ignore-dependencies"], cwd=RESULT, env=clean)
    phase("package_consumer", ["ruby", "-rjson", "-rslackblocks", "-e", '''
      value = Slackblocks::MessagePayload.new(channel: "C123", blocks: [Slackblocks::SectionBlock.new(text: "Installed")])
      abort "wrong output" unless JSON.parse(value.to_json).dig("blocks", 0, "text", "text") == "Installed"
      abort "checkout loaded" if $LOADED_FEATURES.any? { |f| f.start_with?(ARGV.fetch(0)) }
      puts value.to_json
      ''', str(ROOT / "ruby")], cwd=RESULT, env=clean)
    metadata["installed_files"] = {str(p.relative_to(install / "gems/slackblocks-2.8.0")): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in (install / "gems/slackblocks-2.8.0").rglob("*") if p.is_file()}
    metadata["complete"] = True
    META.write_text(json.dumps(metadata, indent=2) + "\n")
