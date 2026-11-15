"""Exercise pre-tag failure, propagation failure and resume without service writes."""

import datetime
import json
import os
from pathlib import Path
import unittest
from unittest.mock import patch

import release_coordinator as coordinator
import release_followup as followup
import test_release as common
from test_release_coordinator import job, publisher_jobs


class FakeGitHub:
    """Only the gh transport is replaced; production state/receipt guards run."""

    def __init__(self, source, version):
        self.source, self.version = source, version
        self.previous = []
        self.tags = False
        self.runs, self.histories = {}, {}
        self.dispatches, self.reruns = [], []

    def call(self, *args):
        if args[0] == "api":
            path = args[-1]
            if "coordinated-release.yml/runs" in path:
                result = {"workflow_runs": self.previous}
            elif "/git/matching-refs/" in path:
                result = [{"ref": path}] if self.tags else []
            elif "/actions/workflows/" in path:
                workflow = path.split("/actions/workflows/")[1].split("/")[0]
                result = {
                    "workflow_runs": [
                        r
                        for r in self.runs.values()
                        if r["path"].endswith("/" + workflow)
                    ]
                }
            elif "/actions/runs/1/jobs" in path:
                result = {
                    "jobs": [
                        dict(id=1, **job("Validate coordinated release")),
                        dict(id=2, **job("Verify registry readiness", "failure")),
                        dict(id=3, **job("Create coordinated tags", "skipped")),
                        dict(id=4, **job("Publish ${{ matrix.language }}", "skipped")),
                    ]
                }
            else:
                run_id = int(path.split("/actions/runs/")[1].split("/")[0])
                run = self.runs[run_id]
                if "/jobs?" in path:
                    result = {"jobs": self.histories[run_id]}
                elif "/artifacts?" in path:
                    result = {"artifacts": []}
                    if run["head_branch"].startswith("csharp/"):
                        result["artifacts"] = [
                            dict(
                                id=run_id * 10 + run["run_attempt"],
                                name=f"nuget-public-consumer-attempt-{run['run_attempt']}",
                                expired=False,
                            )
                        ]
                else:
                    result = run
            return json.dumps([result] if "--slurp" in args else result)
        if args[:2] == ("workflow", "run"):
            workflow = args[2]
            language = next(
                lang for lang, file in coordinator.WORKFLOWS.items() if file == workflow
            )
            assert self.tags and language not in self.dispatches
            self.dispatches.append(language)
            run_id = 100 + list(coordinator.WORKFLOWS).index(language)
            self.runs[run_id] = dict(
                id=run_id,
                event="workflow_dispatch",
                head_branch=f"{coordinator.PREFIXES[language]}/v{self.version}",
                head_sha=self.source,
                path=".github/workflows/" + workflow,
                status="completed",
                conclusion="failure" if language == "csharp" else "success",
                html_url=f"https://github.com/fixture/actions/runs/{run_id}",
                run_attempt=1,
                created_at="2026-11-15T10:00:00Z",
            )
            jobs = publisher_jobs(language)
            for number, item in enumerate(jobs):
                item["id"] = run_id * 100 + number
            if language == "csharp":
                self.fail_consumer(jobs)
            self.histories[run_id] = jobs
            return ""
        if args[:2] == ("run", "rerun"):
            run_id = int(args[2])
            run = self.runs[run_id]
            assert run["head_branch"].startswith("csharp/") and args[-1] == "--failed"
            self.reruns.append(run_id)
            run["run_attempt"] += 1
            run["conclusion"] = "success" if run["run_attempt"] == 3 else "failure"
            jobs = publisher_jobs("csharp")[1:]  # Never rerun the successful upload.
            for number, item in enumerate(jobs):
                item.update(
                    id=run_id * 100 + run["run_attempt"] * 10 + number,
                    run_attempt=run["run_attempt"],
                )
            if run["conclusion"] == "failure":
                self.fail_consumer(jobs)
            self.histories[run_id].extend(jobs)
            return ""
        if args[:2] == ("run", "download"):
            run = self.runs[int(args[2])]
            directory = Path(args[args.index("--dir") + 1])
            directory.mkdir(parents=True)
            pending = run["conclusion"] == "failure"
            (directory / "receipt.json").write_text(
                json.dumps(
                    dict(
                        version=self.version,
                        state="awaiting_availability" if pending else "verified",
                        passed=not pending,
                    )
                )
            )
            return ""
        raise AssertionError(f"Unexpected gh call: {args}")

    @staticmethod
    def fail_consumer(jobs):
        for item in jobs:
            if item["name"] == "Verify public NuGet installation":
                item["conclusion"] = "failure"
                item["steps"][0]["conclusion"] = "failure"
            elif item["name"] == "Create GitHub Release":
                item["conclusion"] = "skipped"
                for step in item["steps"]:
                    step["conclusion"] = "skipped"


class RecoveryTests(unittest.TestCase):
    setUp = common.ReleaseTests.setUp
    git = common.ReleaseTests.git
    commit = common.ReleaseTests.commit
    tags = common.ReleaseTests.tags

    def test_failed_readiness_then_pending_upload_then_resume_preserves_publication(
        self,
    ):
        source = self.git("rev-parse", "HEAD")
        version = common.VERSION
        service = FakeGitHub(source, version)
        evidence = self.root / "evidence"
        with (
            patch.object(coordinator, "gh", side_effect=service.call),
            patch.dict(
                os.environ,
                {
                    "GITHUB_RUN_ID": "10",
                    "GITHUB_RUN_ATTEMPT": "1",
                    "GITHUB_SHA": source,
                },
            ),
        ):
            coordinator.initial_attempt("fixture/repo", source, 1, version)
            service.previous = [
                dict(
                    id=1,
                    head_sha=source,
                    path=".github/workflows/coordinated-release.yml",
                    status="completed",
                    conclusion="failure",
                )
            ]
            coordinator.initial_attempt("fixture/repo", source, 10, version)
            self.tags()
            service.tags = True
            original_tags = self.git("show-ref", "--tags")
            with self.assertRaises(ValueError):
                coordinator.initial_attempt("fixture/repo", source, 11, version)
            for language in coordinator.WORKFLOWS:
                folder = evidence / "initial" / language
                if language == "csharp":
                    with self.assertRaises(coordinator.Pending):
                        coordinator.monitor(
                            "fixture/repo",
                            language,
                            version,
                            source,
                            folder,
                            False,
                            sleep=lambda _: None,
                        )
                else:
                    coordinator.monitor(
                        "fixture/repo",
                        language,
                        version,
                        source,
                        folder,
                        False,
                        sleep=lambda _: None,
                    )
            records = [
                json.loads(p.read_text())
                for p in (evidence / "initial").glob("*/publisher.json")
            ]
            self.assertEqual(
                followup.eligible(
                    records,
                    [],
                    None,
                    10,
                    datetime.datetime(2026, 11, 15, 12, tzinfo=followup.UTC),
                ),
                version,
            )
            self.assertEqual(len(service.dispatches), 10)
            self.assertEqual(len(service.reruns), 1)
            self.assertEqual(
                common.release.coordinator(
                    self.root,
                    version,
                    "workflow_dispatch",
                    "refs/heads/master",
                    resume=True,
                ),
                version,
            )
            with patch.dict(os.environ, {"GITHUB_RUN_ID": "11"}):
                for language in coordinator.WORKFLOWS:
                    coordinator.monitor(
                        "fixture/repo",
                        language,
                        version,
                        source,
                        evidence / "publishers" / language,
                        True,
                        sleep=lambda _: None,
                    )
            records = [
                json.loads(p.read_text())
                for p in (evidence / "publishers").glob("*/publisher.json")
            ]
            self.assertTrue(
                all(r["passed"] and r["coordinator_run"] == "11" for r in records)
            )
            self.assertEqual(len(service.dispatches), 10)
            self.assertEqual(len(service.reruns), 2)
            self.assertEqual(len(set(service.reruns)), 1)
            self.assertEqual(self.git("show-ref", "--tags"), original_tags)
            nuget = next(r for r in records if r["language"] == "csharp")
            upload = next(
                j
                for j in nuget["jobs"]
                if j["name"] == coordinator.UPLOAD_JOBS["csharp"]
            )
            self.assertEqual(upload["run_attempt"], 1)
            self.assertEqual(upload["previous_attempts"], [])
