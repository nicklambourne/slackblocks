"""Resume exact-source publishers without blindly repeating an upload."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import time

from release_availability import Pending, save

WORKFLOWS = {
    "python": "publish.yml",
    "typescript": "publish-npm.yml",
    "go": "publish-go.yml",
    "java": "publish-java.yml",
    "csharp": "publish-nuget.yml",
    "ruby": "publish-rubygems.yml",
    "rust": "publish-crates.yml",
    "php": "publish-packagist.yml",
    "elixir": "publish-hex.yml",
    "haskell": "publish-hackage.yml",
}
PREFIXES = {
    language: {"typescript": "ts"}.get(language, language) for language in WORKFLOWS
}
RETRY_JOBS = {
    "csharp": re.compile(r"Verify public NuGet installation"),
    "php": re.compile(r"Verify public Packagist installations"),
    "haskell": re.compile(r"Verify Hackage install \(GHC [0-9.]+\)"),
}
UPLOAD_JOBS = {
    "csharp": "Build, verify, and publish to NuGet",
    "php": "Publish PHP archive and Composer version tag",
    "haskell": "Publish Hackage package and Haddock",
}
RECEIPTS = {
    "csharp": "nuget-public-consumer",
    "php": "php-public-consumer",
    "haskell": "hackage-public-consumer-",
}

CONTRACTS = json.loads(
    (Path(__file__).resolve().parents[1] / "release-publishers.json").read_text()
)


def gh(*args):
    return subprocess.check_output(["gh", *args], text=True)


def api(path):
    return json.loads(gh("api", path))


def pages(path, field):
    return [
        item
        for page in json.loads(gh("api", "--paginate", "--slurp", path))
        for item in page[field]
    ]


def candidates(runs, tag, source, workflow):
    relevant = [
        r for r in runs if r["event"] == "workflow_dispatch" and r["head_branch"] == tag
    ]
    for run in relevant:
        if run["head_sha"] != source or run["path"] != ".github/workflows/" + workflow:
            raise ValueError(
                "Publisher run does not match the immutable source/workflow"
            )
    active = [r for r in relevant if r["status"] != "completed"]
    if len(active) > 1:
        raise ValueError("Multiple active publishers require reconciliation")
    if active:
        return active[0]
    return max(relevant, key=lambda r: r["id"], default=None)


def verify_jobs(jobs):
    if (
        not jobs
        or not any(j["conclusion"] == "success" for j in jobs)
        or any(j["status"] != "completed" for j in jobs)
    ):
        raise ValueError("Official jobs have not completed")
    for job in jobs:
        if job["conclusion"] == "skipped":
            continue
        if not job["runner_name"].startswith("GitHub Actions ") or not any(
            re.fullmatch(r"(?:ubuntu|macos|windows)-(?:latest|[0-9.]+)(?:-arm)?", label)
            for label in job["labels"]
        ):
            raise ValueError("Publisher did not execute on an authorized hosted runner")
        if any(s["status"] != "completed" for s in job["steps"]):
            raise ValueError("Publisher steps have not completed")
        if job["conclusion"] == "success" and any(
            s["conclusion"] not in ("success", "skipped") for s in job["steps"]
        ):
            raise ValueError("A green job contains an unsuccessful step")


def verify_required_steps(job, groups):
    if job["conclusion"] != "success":
        raise ValueError(f"Required publisher job did not succeed: {job['name']}")
    for group in groups:
        alternatives = [group] if isinstance(group, str) else group
        steps = [step for step in job["steps"] if step.get("name") in alternatives]
        if {step["name"] for step in steps} != set(alternatives) or len(steps) != len(
            alternatives
        ):
            raise ValueError(
                f"Missing or duplicate critical steps in {job['name']}: {alternatives}"
            )
        if (
            sum(
                step["status"] == "completed" and step["conclusion"] == "success"
                for step in steps
            )
            != 1
        ):
            raise ValueError(
                f"Critical publication/verification step did not execute: {alternatives}"
            )


def verify_publisher(language, jobs):
    verify_jobs(jobs)
    contract = CONTRACTS[language]
    by_name = {job["name"]: job for job in jobs}
    if len(by_name) != len(jobs) or not set(contract["jobs"]) <= set(by_name):
        raise ValueError("Missing or duplicate required publisher jobs")
    for name, groups in contract["jobs"].items():
        verify_required_steps(by_name[name], groups)
    for job in jobs:
        if (
            job["conclusion"] == "skipped"
            and job["name"] in contract["allowed_skipped_jobs"]
        ):
            continue
        if job["conclusion"] != "success":
            raise ValueError(f"Unreviewed publisher skip or failure: {job['name']}")


def retryable(language, jobs, receipts, version):
    failed = [j for j in jobs if j["conclusion"] not in ("success", "skipped")]
    pattern = RETRY_JOBS.get(language)
    upload = next((j for j in jobs if j["name"] == UPLOAD_JOBS.get(language)), None)
    if upload is None:
        return False
    try:
        verify_required_steps(upload, CONTRACTS[language]["jobs"][upload["name"]])
    except ValueError:
        return False
    if (
        not failed
        or not pattern
        or any(
            j["conclusion"] != "failure" or not pattern.fullmatch(j["name"])
            for j in failed
        )
    ):
        return False
    # Every failed consumer must have retained its own pending receipt. A compile
    # failure, timeout, missing receipt or uncertain upload is never auto-retried.
    pending = [
        r
        for r in receipts
        if r.get("version") == version
        and r.get("state") == "awaiting_availability"
        and r.get("passed") is False
    ]
    return len(pending) == len(failed) and all(
        r.get("state") in ("verified", "awaiting_availability") for r in receipts
    )


def latest_jobs(history):
    """A failed-job rerun retains successful uploads from an earlier attempt."""
    groups = {}
    for job in history:
        groups.setdefault(job["name"], {})[job["id"]] = job
    result = []
    for versions in groups.values():
        ordered = sorted(
            versions.values(), key=lambda job: (job.get("run_attempt", 0), job["id"])
        )
        result.append({**ordered[-1], "previous_attempts": ordered[:-1]})
    return result


def collect(repo, run, directory, language):
    jobs = latest_jobs(
        pages(
            f"repos/{repo}/actions/runs/{run['id']}/jobs?filter=all&per_page=100",
            "jobs",
        )
    )
    verify_jobs(jobs)
    artifacts = pages(
        f"repos/{repo}/actions/runs/{run['id']}/artifacts?per_page=100", "artifacts"
    )
    receipts = []
    for artifact in artifacts:
        prefix = RECEIPTS.get(language)
        if (
            prefix
            and artifact["name"].startswith(prefix)
            and artifact["name"].endswith(f"-attempt-{run['run_attempt']}")
            and not artifact["expired"]
        ):
            destination = (
                directory / f"attempt-{run['run_attempt']}" / str(artifact["id"])
            )
            if not destination.exists():
                gh(
                    "run",
                    "download",
                    str(run["id"]),
                    "--repo",
                    repo,
                    "--name",
                    artifact["name"],
                    "--dir",
                    str(destination),
                )
            receipts.extend(
                json.loads(p.read_text()) for p in destination.rglob("receipt.json")
            )
    return jobs, artifacts, receipts


def monitor(
    repo,
    language,
    version,
    source,
    directory,
    resume,
    *,
    timeout=1800,
    sleep=time.sleep,
    clock=time.monotonic,
):
    workflow = WORKFLOWS[language]
    tag = f"{PREFIXES[language]}/v{version}"
    directory.mkdir(parents=True, exist_ok=True)
    record = {
        "schema": 1,
        "language": language,
        "version": version,
        "source": source,
        "workflow": workflow,
        "tag": tag,
        "coordinator_run": os.environ.get("GITHUB_RUN_ID"),
        "tooling": os.environ.get("GITHUB_SHA"),
        "state": "observing",
        "passed": False,
    }
    path = directory / "publisher.json"
    save(path, record)
    start = clock()
    try:
        runs = pages(
            f"repos/{repo}/actions/workflows/{workflow}/runs?event=workflow_dispatch&per_page=100",
            "workflow_runs",
        )
        run = candidates(runs, tag, source, workflow)
        if run is None:
            if resume or int(os.environ.get("GITHUB_RUN_ATTEMPT", "1")) > 1:
                raise ValueError(
                    "No observable publisher after a prior attempt; reconcile the dispatch before starting another"
                )
            record["state"] = "dispatch_requested"
            save(path, record)
            # Never retry this write. If its response is uncertain, the durable
            # coordinator run and receipt force observation on a resumed attempt.
            gh("workflow", "run", workflow, "--repo", repo, "--ref", tag)
            for _ in range(12):
                sleep(5)
                run = candidates(
                    pages(
                        f"repos/{repo}/actions/workflows/{workflow}/runs?event=workflow_dispatch&per_page=100",
                        "workflow_runs",
                    ),
                    tag,
                    source,
                    workflow,
                )
                if run:
                    break
            if run is None:
                raise ValueError(
                    "Dispatch accepted but run is not observable; resume without redispatching"
                )
        record.update(
            run_id=run["id"],
            run_url=run["html_url"],
            release_started_at=run.get("created_at"),
        )
        retried = False
        while True:
            run = api(f"repos/{repo}/actions/runs/{run['id']}")
            if candidates([run], tag, source, workflow) is None:
                raise ValueError("Publisher changed its release identity")
            record.update(run_attempt=run["run_attempt"], state="running")
            save(path, record)
            if run["status"] == "completed":
                jobs, artifacts, receipts = collect(repo, run, directory, language)
                record.update(jobs=jobs, artifacts=artifacts)
                if run["conclusion"] == "success" and all(
                    j["conclusion"] in ("success", "skipped") for j in jobs
                ):
                    verify_publisher(language, jobs)
                    record.update(state="publisher_verified", passed=True)
                    save(path, record)
                    return record
                if not retried and retryable(language, jobs, receipts, version):
                    record["state"] = "retrying_availability"
                    save(path, record)
                    gh("run", "rerun", str(run["id"]), "--repo", repo, "--failed")
                    retried = True
                    # A rerun response can precede the new attempt becoming visible.
                    previous = run["run_attempt"]
                    for _ in range(12):
                        sleep(5)
                        run = api(f"repos/{repo}/actions/runs/{run['id']}")
                        if (
                            run["run_attempt"] > previous
                            or run["status"] != "completed"
                        ):
                            break
                    else:
                        raise Pending(
                            "Verification rerun not yet observable; resume later"
                        )
                elif retryable(language, jobs, receipts, version):
                    raise Pending(
                        "Registry is still propagating; successful upload retained"
                    )
                else:
                    raise ValueError(
                        "Publisher failed outside a proven availability delay; inspect retained jobs before any retry"
                    )
            if clock() - start >= timeout:
                raise Pending("Publisher still running; resume to observe the same run")
            sleep(20)
    except Pending as error:
        record.update(state="awaiting_availability", error=str(error))
        save(path, record)
        raise
    except Exception as error:
        record.update(state="blocked", error=str(error))
        save(path, record)
        raise


def initial_attempt(repo, source, current_run, version):
    if int(os.environ.get("GITHUB_RUN_ATTEMPT", "1")) > 1:
        raise ValueError("Rerun must use a new coordinator with resume selected")
    runs = pages(
        f"repos/{repo}/actions/workflows/coordinated-release.yml/runs?event=workflow_dispatch&per_page=100",
        "workflow_runs",
    )
    previous = [r for r in runs if r["head_sha"] == source and r["id"] != current_run]
    if not previous:
        return
    # A readiness/preflight failure has made no release writes. Prove that phase
    # from official jobs AND live tags/publishers before permitting a fresh start.
    for run in previous:
        if (
            run.get("path") != ".github/workflows/coordinated-release.yml"
            or run["status"] != "completed"
            or run["conclusion"] not in ("failure", "cancelled")
        ):
            raise ValueError(
                "A prior coordinator may have published; select resume or reconcile it"
            )
        jobs = pages(
            f"repos/{repo}/actions/runs/{run['id']}/jobs?filter=all&per_page=100",
            "jobs",
        )
        tags = [j for j in jobs if j["name"] == "Create coordinated tags"]
        if not tags or any(
            j["status"] != "completed" or j["conclusion"] != "skipped" for j in tags
        ):
            raise ValueError(
                "Prior tag creation is not proven unstarted; use resume or reconcile it"
            )
        if any(
            j["name"].startswith("Publish ") and j["conclusion"] != "skipped"
            for j in jobs
        ):
            raise ValueError("A prior publisher may have started; use resume")
    for prefix in [*PREFIXES.values(), ""]:
        tag = f"{prefix + '/' if prefix else ''}v{version}"
        if api(f"repos/{repo}/git/matching-refs/tags/{tag}"):
            raise ValueError(
                "Release tags already exist; use resume or reconcile the partial tag set"
            )
    for language, workflow in WORKFLOWS.items():
        tag = f"{PREFIXES[language]}/v{version}"
        publishers = pages(
            f"repos/{repo}/actions/workflows/{workflow}/runs?per_page=100",
            "workflow_runs",
        )
        if any(
            r["event"] in ("push", "workflow_dispatch")
            and (r["head_branch"] == tag or r["head_sha"] == source)
            for r in publishers
        ):
            raise ValueError("A publisher already exists; use resume or reconcile it")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--language", choices=WORKFLOWS)
    parser.add_argument("--directory", type=Path)
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--check-start", action="store_true")
    args = parser.parse_args()
    if not re.fullmatch(r"[0-9a-f]{40}", args.source) or not re.fullmatch(
        r"\d+\.\d+\.\d+", args.version
    ):
        parser.error("Expected an immutable source SHA and X.Y.Z version")
    if args.check_start:
        initial_attempt(
            args.repo, args.source, int(os.environ["GITHUB_RUN_ID"]), args.version
        )
    else:
        if not args.language or args.directory is None:
            parser.error("Publisher observation requires language and directory")
        monitor(
            args.repo,
            args.language,
            args.version,
            args.source,
            args.directory,
            args.resume,
        )


if __name__ == "__main__":
    try:
        main()
    except Pending as error:
        raise SystemExit(str(error)) from error
