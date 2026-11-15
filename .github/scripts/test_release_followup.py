import datetime
import unittest

import release_followup as followup


class FollowupTests(unittest.TestCase):
    def setUp(self):
        self.now = datetime.datetime(2026, 11, 15, 12, tzinfo=followup.UTC)
        self.records = [dict(language=lang, version="4.0.0", source="a" * 40, coordinator_run="123",
                             state="publisher_verified", passed=True, release_started_at="2026-11-15T10:00:00Z") for lang in followup.PREFIXES]

    def eligible(self, consumers=None, identity=None):
        return followup.eligible(self.records, consumers or [], identity, 123, self.now)

    def test_only_pending_complete_identity_can_resume(self):
        self.assertIsNone(self.eligible())
        self.records[-1]["state"] = "awaiting_availability"
        self.records[-1]["passed"] = False
        self.assertEqual(self.eligible(), "4.0.0")
        for field, value in (("state", "blocked"), ("source", "b" * 40), ("version", "3.2.0"),
                             ("coordinator_run", "124"), ("release_started_at", "2026-11-14T10:00:00Z")):
            original = self.records[0][field]
            self.records[0][field] = value
            with self.subTest(field=field):
                self.assertIsNone(self.eligible())
            self.records[0][field] = original
        self.records.pop()
        self.assertIsNone(self.eligible())

    def test_pending_public_consumer_is_retryable_but_compile_failure_is_not(self):
        identity = dict(version="4.0.0", source="a" * 40, state="verified")
        consumers = [{**r, "state": "verified", "run_id": "123"} for r in self.records]
        consumers[-1]["state"] = "awaiting_availability"
        self.assertEqual(self.eligible(consumers, identity), "4.0.0")
        consumers[-1]["state"] = "failed"
        self.assertIsNone(self.eligible(consumers, identity))
        identity["state"] = "awaiting_availability"
        self.assertEqual(self.eligible([], identity), "4.0.0")
