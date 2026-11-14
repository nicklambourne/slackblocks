"""Simulated dispatch/recovery; these tests never call a publishing service."""
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import release_coordinator as c
import test_release as common


def publisher(**changes):
    run = dict(id=42, event="workflow_dispatch", head_branch="php/v4.0.0", head_sha="a" * 40,
               path=".github/workflows/publish-packagist.yml", status="completed", conclusion="success",
               html_url="https://github.com/example/actions/runs/42", run_attempt=1)
    return {**run, **changes}


def job(name="publish", conclusion="success"):
    return dict(name=name, conclusion=conclusion, status="completed", runner_name="GitHub Actions 123",
                labels=["ubuntu-latest"], steps=[dict(status="completed", conclusion=conclusion)])


class ResumeTests(unittest.TestCase):
    def test_foreign_source_path_and_multiple_active_runs_are_rejected(self):
        for changes in ({"head_sha": "b" * 40}, {"path": "wrong.yml"}):
            with self.assertRaises(ValueError):
                c.candidates([publisher(**changes)], "php/v4.0.0", "a" * 40, "publish-packagist.yml")
        with self.assertRaises(ValueError):
            c.candidates([publisher(status="in_progress"), publisher(id=43, status="queued")], "php/v4.0.0", "a" * 40, "publish-packagist.yml")

    def test_runner_and_unfinished_jobs_fail_closed(self):
        for jobs in ([], [{**job(), "status": "in_progress"}], [{**job(), "runner_name": "private-host"}],
                     [{**job(), "steps": [{"status": "in_progress"}]}]):
            with self.assertRaises(ValueError):
                c.verify_jobs(jobs)
        c.verify_jobs([job()])

    def test_rerun_reconciles_successful_earlier_upload_and_new_consumer(self):
        upload = {**job("publish"), "id": 1, "run_attempt": 1}
        failed = {**job("consumer", "failure"), "id": 2, "run_attempt": 1}
        passed = {**job("consumer"), "id": 3, "run_attempt": 2}
        latest = c.latest_jobs([passed, upload, failed])
        self.assertEqual({j["id"] for j in latest}, {1, 3})
        self.assertEqual(next(j for j in latest if j["id"] == 3)["previous_attempts"], [failed])
        c.verify_jobs(latest)
        with self.assertRaises(ValueError):
            c.verify_jobs([{**upload, "steps": [{"status": "completed", "conclusion": "failure"}]}])

    def test_only_proven_pending_consumers_can_retry(self):
        jobs = [job("Publish PHP archive and Composer version tag"), job("Verify public Packagist installations", "failure")]
        receipt = dict(version="4.0.0", state="awaiting_availability", passed=False)
        self.assertTrue(c.retryable("php", jobs, [receipt], "4.0.0"))
        for receipts in ([], [{**receipt, "version": "3.2.0"}], [{**receipt, "state": "failed"}]):
            self.assertFalse(c.retryable("php", jobs, receipts, "4.0.0"))
        self.assertFalse(c.retryable("php", [job("Publish PHP archive and Composer version tag", "failure")], [receipt], "4.0.0"))
        self.assertFalse(c.retryable("php", [job("Verify public Packagist installations", "timed_out")], [receipt], "4.0.0"))

    def test_success_is_observed_without_dispatch_or_rerun(self):
        with tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp, \
             patch.object(c, "pages", return_value=[publisher()]), patch.object(c, "api", return_value=publisher()), \
             patch.object(c, "collect", return_value=([job()], [{"id": 7, "digest": "sha256:abc"}], [])), \
             patch.object(c, "gh") as gh:
            result = c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True)
            gh.assert_not_called()
            self.assertTrue(result["passed"])
            self.assertEqual(json.loads((Path(temp) / "publisher.json").read_text())["artifacts"][0]["id"], 7)

    def test_missing_run_on_resume_never_dispatches(self):
        with tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp, \
             patch.object(c, "pages", return_value=[]), patch.object(c, "gh") as gh, self.assertRaises(ValueError):
            c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True)
        gh.assert_not_called()

    def test_uncertain_dispatch_is_recorded_and_not_retried(self):
        with tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp, \
             patch.object(c, "pages", return_value=[]), patch.object(c, "gh", side_effect=RuntimeError("network response lost")) as gh:
            with self.assertRaises(RuntimeError):
                c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), False)
            self.assertEqual(gh.call_count, 1)
            record = json.loads((Path(temp) / "publisher.json").read_text())
            self.assertFalse(record["passed"])
            self.assertEqual(record["source"], "a" * 40)
            self.assertEqual(record["state"], "blocked")

    def test_previous_coordinator_requires_resume(self):
        with patch.object(c, "pages", return_value=[publisher(id=3)]), self.assertRaises(ValueError):
            c.initial_attempt("owner/repo", "a" * 40, 4)

    def test_running_publisher_is_retained_at_deadline(self):
        run = publisher(status="in_progress", conclusion=None)
        with tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp, \
             patch.object(c, "pages", return_value=[run]), patch.object(c, "api", return_value=run), \
             patch.object(c, "gh") as gh:
            with self.assertRaises(c.Pending):
                c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True, timeout=0)
            gh.assert_not_called()
            record = json.loads((Path(temp) / "publisher.json").read_text())
            self.assertEqual(record["run_id"], 42)
            self.assertFalse(record["passed"])

    def test_pending_retry_never_repeats_publish(self):
        run = publisher(conclusion="failure")
        jobs = [job("Publish PHP archive and Composer version tag"), job("Verify public Packagist installations", "failure")]
        receipt = dict(version="4.0.0", state="awaiting_availability", passed=False)
        with tempfile.TemporaryDirectory(dir=os.environ.get("TEST_TMPDIR")) as temp, \
             patch.object(c, "pages", return_value=[run]), \
             patch.object(c, "api", side_effect=[run, publisher(run_attempt=2), publisher(run_attempt=2)]), \
             patch.object(c, "collect", side_effect=[(jobs, [], [receipt]), ([job()], [], [])]), \
             patch.object(c, "gh") as gh:
            result = c.monitor("owner/repo", "php", "4.0.0", "a" * 40, Path(temp), True, sleep=lambda _: None)
            self.assertTrue(result["passed"])
            gh.assert_called_once_with("run", "rerun", "42", "--repo", "owner/repo", "--failed")


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
        self.assertEqual(common.release.coordinator(self.root, common.VERSION, "workflow_dispatch", "refs/heads/master", resume=True), common.VERSION)
        with self.assertRaises(ValueError):
            common.release.coordinator(self.root, common.VERSION, "workflow_dispatch", "refs/heads/master")
        self.git("tag", "-d", "python/v" + common.VERSION)
        with self.assertRaises(ValueError):
            common.release.coordinator(self.root, common.VERSION, "workflow_dispatch", "refs/heads/master", resume=True)
