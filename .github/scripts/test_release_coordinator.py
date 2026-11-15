"""Simulated dispatch/recovery; these tests never call a publishing service."""

import copy
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import release_coordinator as c
import test_release as common


def publisher(**changes):
    run = dict(
        id=42,
        event="workflow_dispatch",
        head_branch="php/v4.0.0",
        head_sha="a" * 40,
        path=".github/workflows/publish-packagist.yml",
        status="completed",
        conclusion="success",
        html_url="https://github.com/example/actions/runs/42",
        run_attempt=1,
    )
    return {**run, **changes}


def job(name="publish", conclusion="success"):
    return dict(
        name=name,
        conclusion=conclusion,
        status="completed",
        runner_name="GitHub Actions 123",
        labels=["ubuntu-latest"],
        steps=[dict(status="completed", conclusion=conclusion)],
    )


def publisher_jobs(language="php"):
    jobs = []
    for number, (name, groups) in enumerate(c.CONTRACTS[language]["jobs"].items()):
        steps = []
        for group in groups:
            for index, step in enumerate([group] if isinstance(group, str) else group):
                steps.append(
                    dict(
                        name=step,
                        status="completed",
                        conclusion="success" if index == 0 else "skipped",
                    )
                )
        jobs.append({**job(name), "id": number + 1, "run_attempt": 1, "steps": steps})
    return jobs


def upload_job(language="php"):
    return next(
        j for j in publisher_jobs(language) if j["name"] == c.UPLOAD_JOBS[language]
    )


class ResumeTests(unittest.TestCase):
    def test_foreign_source_path_and_multiple_active_runs_are_rejected(self):
        for changes in ({"head_sha": "b" * 40}, {"path": "wrong.yml"}):
            with self.assertRaises(ValueError):
                c.candidates(
                    [publisher(**changes)],
                    "php/v4.0.0",
                    "a" * 40,
                    "publish-packagist.yml",
                )
        with self.assertRaises(ValueError):
            c.candidates(
                [publisher(status="in_progress"), publisher(id=43, status="queued")],
                "php/v4.0.0",
                "a" * 40,
                "publish-packagist.yml",
            )

    def test_runner_and_unfinished_jobs_fail_closed(self):
        for jobs in (
            [],
            [{**job(), "status": "in_progress"}],
            [{**job(), "runner_name": "private-host"}],
            [{**job(), "steps": [{"status": "in_progress"}]}],
        ):
            with self.assertRaises(ValueError):
                c.verify_jobs(jobs)
        c.verify_jobs([job()])

    def test_rerun_reconciles_successful_earlier_upload_and_new_consumer(self):
        upload = {**job("publish"), "id": 1, "run_attempt": 1}
        failed = {**job("consumer", "failure"), "id": 2, "run_attempt": 1}
        passed = {**job("consumer"), "id": 3, "run_attempt": 2}
        latest = c.latest_jobs([passed, upload, failed])
        self.assertEqual({j["id"] for j in latest}, {1, 3})
        self.assertEqual(
            next(j for j in latest if j["id"] == 3)["previous_attempts"], [failed]
        )
        c.verify_jobs(latest)
        with self.assertRaises(ValueError):
            c.verify_jobs(
                [
                    {
                        **upload,
                        "steps": [{"status": "completed", "conclusion": "failure"}],
                    }
                ]
            )

    def test_only_proven_pending_consumers_can_retry(self):
        jobs = [upload_job(), job("Verify public Packagist installations", "failure")]
        receipt = dict(version="4.0.0", state="awaiting_availability", passed=False)
        self.assertTrue(c.retryable("php", jobs, [receipt], "4.0.0"))
        for receipts in (
            [],
            [{**receipt, "version": "3.2.0"}],
            [{**receipt, "state": "failed"}],
        ):
            self.assertFalse(c.retryable("php", jobs, receipts, "4.0.0"))
        self.assertFalse(
            c.retryable(
                "php",
                [job("Publish PHP archive and Composer version tag", "failure")],
                [receipt],
                "4.0.0",
            )
        )
        self.assertFalse(
            c.retryable(
                "php",
                [job("Verify public Packagist installations", "timed_out")],
                [receipt],
                "4.0.0",
            )
        )

    def test_success_is_observed_without_dispatch_or_rerun(self):
        with (
            tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp,
            patch.object(c, "pages", return_value=[publisher()]),
            patch.object(c, "api", return_value=publisher()),
            patch.object(
                c,
                "collect",
                return_value=(
                    publisher_jobs(),
                    [{"id": 7, "digest": "sha256:abc"}],
                    [],
                ),
            ),
            patch.object(c, "gh") as gh,
        ):
            result = c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True)
            gh.assert_not_called()
            self.assertTrue(result["passed"])
            self.assertEqual(
                json.loads((Path(temp) / "publisher.json").read_text())["artifacts"][0][
                    "id"
                ],
                7,
            )

    def test_missing_run_on_resume_never_dispatches(self):
        with (
            tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp,
            patch.object(c, "pages", return_value=[]),
            patch.object(c, "gh") as gh,
            self.assertRaises(ValueError),
        ):
            c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True)
        gh.assert_not_called()

    def test_uncertain_dispatch_is_recorded_and_not_retried(self):
        with (
            tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp,
            patch.object(c, "pages", return_value=[]),
            patch.object(
                c, "gh", side_effect=RuntimeError("network response lost")
            ) as gh,
        ):
            with self.assertRaises(RuntimeError):
                c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), False)
            self.assertEqual(gh.call_count, 1)
            record = json.loads((Path(temp) / "publisher.json").read_text())
            self.assertFalse(record["passed"])
            self.assertEqual(record["source"], "a" * 40)
            self.assertEqual(record["state"], "blocked")

    def test_prior_tag_or_publication_attempt_requires_resume(self):
        run = {**publisher(id=3), "path": ".github/workflows/coordinated-release.yml"}
        with (
            patch.object(c, "pages", return_value=[run]),
            self.assertRaises(ValueError),
        ):
            c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")

    def test_pretag_restart_requires_completed_unstarted_writes_and_no_live_tags_or_publishers(
        self,
    ):
        run = {
            **publisher(id=3, conclusion="failure"),
            "path": ".github/workflows/coordinated-release.yml",
        }
        skipped = [
            job("Create coordinated tags", "skipped"),
            job("Publish php", "skipped"),
        ]

        def observe(jobs=skipped, publishers=()):
            def pages(path, field):
                if "coordinated-release.yml/runs" in path:
                    return [run]
                if "/jobs?" in path:
                    return jobs
                return list(publishers)

            return pages

        with (
            patch.object(c, "pages", side_effect=observe()),
            patch.object(c, "api", return_value=[]),
            patch.dict(os.environ, {"GITHUB_RUN_ATTEMPT": "1"}),
        ):
            c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")
        for jobs in (
            [],
            [job("Create coordinated tags", "failure")],
            [*skipped, job("Publish python")],
            [{**skipped[0], "status": "queued"}],
        ):
            with (
                self.subTest(jobs=jobs),
                patch.object(c, "pages", side_effect=observe(jobs)),
                patch.object(c, "api", return_value=[]),
                self.assertRaises(ValueError),
            ):
                c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")
        with (
            patch.object(c, "pages", side_effect=observe()),
            patch.object(c, "api", return_value=[{"ref": "refs/tags/python/v4.0.0"}]),
            self.assertRaises(ValueError),
        ):
            c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")
        for event in ("push", "workflow_dispatch"):
            with (
                patch.object(
                    c, "pages", side_effect=observe(publishers=[publisher(event=event)])
                ),
                patch.object(c, "api", return_value=[]),
                self.assertRaises(ValueError),
            ):
                c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")
        with (
            patch.object(c, "pages", side_effect=observe()),
            patch.object(c, "api", side_effect=RuntimeError("permission denied")),
            self.assertRaises(RuntimeError),
        ):
            c.initial_attempt("owner/repo", "a" * 40, 4, "4.0.0")

    def test_each_language_requires_its_jobs_and_critical_steps(self):
        self.assertEqual(set(c.CONTRACTS), set(c.WORKFLOWS))
        for language in c.WORKFLOWS:
            jobs = publisher_jobs(language)
            c.verify_publisher(language, jobs)
            for index, required in enumerate(jobs):
                for broken in (
                    jobs[:index] + jobs[index + 1 :],
                    jobs + [required],
                    [
                        {**j, "conclusion": "skipped"} if i == index else j
                        for i, j in enumerate(jobs)
                    ],
                ):
                    with (
                        self.subTest(language=language, job=required["name"]),
                        self.assertRaises(ValueError),
                    ):
                        c.verify_publisher(language, broken)
                for step in range(len(required["steps"])):
                    if required["steps"][step]["conclusion"] != "success":
                        continue
                    broken = copy.deepcopy(jobs)
                    broken[index]["steps"][step]["conclusion"] = "skipped"
                    with self.assertRaises(ValueError):
                        c.verify_publisher(language, broken)
            with self.assertRaises(ValueError):
                c.verify_publisher(
                    language, jobs + [job("Unexpected release gate", "skipped")]
                )
        c.verify_publisher(
            "php", publisher_jobs() + [job("PHP release checks", "skipped")]
        )

    def test_upload_alternatives_require_exactly_one_executed_path(self):
        for language in ("typescript", "rust"):
            jobs = publisher_jobs(language)
            for required in jobs:
                for step in required["steps"]:
                    step["conclusion"] = "success"
            with self.assertRaises(ValueError):
                c.verify_publisher(language, jobs)
        skipped = upload_job()
        skipped["steps"][-1]["conclusion"] = "skipped"
        receipt = dict(version="4.0.0", state="awaiting_availability", passed=False)
        self.assertFalse(
            c.retryable(
                "php",
                [skipped, job("Verify public Packagist installations", "failure")],
                [receipt],
                "4.0.0",
            )
        )

    def test_contract_critical_step_names_exist_in_publishers(self):
        for language, workflow in c.WORKFLOWS.items():
            text = (common.ROOT / ".github/workflows" / workflow).read_text()
            for name, groups in c.CONTRACTS[language]["jobs"].items():
                self.assertIn(name.split(" (")[0], text)
                for group in groups:
                    for step in [group] if isinstance(group, str) else group:
                        self.assertIn("name: " + step, text)

    def test_running_publisher_is_retained_at_deadline(self):
        run = publisher(status="in_progress", conclusion=None)
        with (
            tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp,
            patch.object(c, "pages", return_value=[run]),
            patch.object(c, "api", return_value=run),
            patch.object(c, "gh") as gh,
        ):
            with self.assertRaises(c.Pending):
                c.monitor(
                    "owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True, timeout=0
                )
            gh.assert_not_called()
            record = json.loads((Path(temp) / "publisher.json").read_text())
            self.assertEqual(record["run_id"], 42)
            self.assertFalse(record["passed"])

    def test_pending_retry_never_repeats_publish(self):
        run = publisher(conclusion="failure")
        jobs = [upload_job(), job("Verify public Packagist installations", "failure")]
        receipt = dict(version="4.0.0", state="awaiting_availability", passed=False)
        with (
            tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp,
            patch.object(c, "pages", return_value=[run]),
            patch.object(
                c,
                "api",
                side_effect=[run, publisher(run_attempt=2), publisher(run_attempt=2)],
            ),
            patch.object(
                c,
                "collect",
                side_effect=[(jobs, [], [receipt]), (publisher_jobs(), [], [])],
            ),
            patch.object(c, "gh") as gh,
        ):
            result = c.monitor(
                "owner/repo",
                "php",
                "4.0.0",
                "a" * 40,
                Path(temp),
                True,
                sleep=lambda _: None,
            )
            self.assertTrue(result["passed"])
            gh.assert_called_once_with(
                "run", "rerun", "42", "--repo", "owner/repo", "--failed"
            )


class ResumePreflightTests(unittest.TestCase):
    setUp = common.ReleaseTests.setUp
    git = common.ReleaseTests.git
    commit = common.ReleaseTests.commit
    tags = common.ReleaseTests.tags

    def test_resume_accepts_tagged_ancestor_but_never_wrong_tags(self):
        self.tags()
        source = self.git("rev-parse", "HEAD")
        self.git("commit", "--allow-empty", "-m", "advance master")
        self.git("update-ref", "refs/remotes/origin/master", "HEAD")
        self.git("checkout", "--detach", source)
        self.assertEqual(
            common.release.coordinator(
                self.root,
                common.VERSION,
                "workflow_dispatch",
                "refs/heads/master",
                resume=True,
            ),
            common.VERSION,
        )
        with self.assertRaises(ValueError):
            common.release.coordinator(
                self.root, common.VERSION, "workflow_dispatch", "refs/heads/master"
            )
        self.git("tag", "-d", "python/v" + common.VERSION)
        with self.assertRaises(ValueError):
            common.release.coordinator(
                self.root,
                common.VERSION,
                "workflow_dispatch",
                "refs/heads/master",
                resume=True,
            )
