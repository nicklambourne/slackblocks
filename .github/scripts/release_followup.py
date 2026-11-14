"""Resume only recorded propagation waits, within 24 hours of original publication."""
import datetime
import json
import os
from pathlib import Path
import re

from release_availability import save
from release_coordinator import PREFIXES, gh, pages


UTC = datetime.timezone.utc


def eligible(records, consumers, identity, run_id, now):
    if len(records) != len(PREFIXES) or {r.get("language") for r in records} != set(PREFIXES):
        return None
    version, source = records[0].get("version", ""), records[0].get("source", "")
    if not re.fullmatch(r"\d+\.\d+\.\d+", version) or not re.fullmatch(r"[a-f0-9]{40}", source):
        return None
    if any(r.get("version") != version or r.get("source") != source or r.get("coordinator_run") != str(run_id)
           or r.get("state") not in ("publisher_verified", "awaiting_availability") or not r.get("release_started_at") for r in records):
        return None
    started = min(datetime.datetime.fromisoformat(r["release_started_at"].replace("Z", "+00:00")) for r in records)
    if not datetime.timedelta(0) <= now - started < datetime.timedelta(hours=24):
        return None
    if any(r["state"] == "awaiting_availability" for r in records):
        return version
    if not identity or identity.get("version") != version or identity.get("source") != source:
        return None
    if identity.get("state") == "awaiting_availability":
        return version
    if identity.get("state") != "verified" or len(consumers) != len(PREFIXES) or {r.get("language") for r in consumers} != set(PREFIXES):
        return None
    if any(r.get("state") not in ("verified", "awaiting_availability") or r.get("version") != version
           or r.get("source") != source or r.get("run_id") != str(run_id) for r in consumers):
        return None
    return version if any(r["state"] == "awaiting_availability" for r in consumers) else None


def main():
    repo = os.environ["GITHUB_REPOSITORY"]
    directory = Path(os.environ["RUNNER_TEMP"]) / "release-followup"
    directory.mkdir(parents=True, exist_ok=True)
    now = datetime.datetime.now(UTC)
    runs = pages(f"repos/{repo}/actions/workflows/coordinated-release.yml/runs?event=workflow_dispatch&per_page=100", "workflow_runs")
    if any(r["head_branch"] == "master" and r["status"] != "completed" for r in runs):
        return  # Let the single active coordinator continue.
    # Only the newest coordinator for a version is eligible. Old failure receipts
    # cannot restart a release after a later successful or blocked reconciliation.
    seen = set()
    for run in sorted(runs, key=lambda r: r["id"], reverse=True):
        created = datetime.datetime.fromisoformat(run["created_at"].replace("Z", "+00:00"))
        if now - created >= datetime.timedelta(hours=24):
            continue
        if run["head_branch"] != "master" or run["path"] != ".github/workflows/coordinated-release.yml":
            continue
        artifacts = pages(f"repos/{repo}/actions/runs/{run['id']}/artifacts?per_page=100", "artifacts")
        records, consumers, identity = [], [], None
        for artifact in artifacts:
            name = artifact["name"]
            if artifact["expired"] or not name.endswith("-" + str(run["run_attempt"])) or not name.startswith(("publisher-record-", "public-release-consumer-", "public-release-identity-")):
                continue
            folder = directory / str(artifact["id"])
            gh("run", "download", str(run["id"]), "--repo", repo, "--name", name, "--dir", str(folder))
            if name.startswith("publisher-record-"):
                records.extend(json.loads(p.read_text()) for p in folder.rglob("publisher.json"))
            elif name.startswith("public-release-consumer-"):
                consumers.extend(json.loads(p.read_text()) for p in folder.rglob("receipt.json"))
            else:
                identity = json.loads((folder / "identity.json").read_text())
        versions = {r.get("version") for r in records}
        if len(versions) != 1:
            continue
        version = next(iter(versions))
        if version in seen:
            continue
        seen.add(version)
        if run["conclusion"] != "failure" or not eligible(records, consumers, identity, run["id"], now):
            continue
        record = dict(version=version, previous_coordinator=run["id"], state="resume_requested", at=now.isoformat())
        save(directory / "request.json", record)
        # Resume cannot start a missing publisher or re-upload an uncertain package.
        gh("workflow", "run", "coordinated-release.yml", "--repo", repo, "--ref", "master",
           "-f", f"version={version}", "-f", "resume=true")
        return  # Bound each hourly pass to one existing release.


if __name__ == "__main__":
    main()
