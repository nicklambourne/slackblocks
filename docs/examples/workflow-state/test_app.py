import json
import unittest
from concurrent.futures import ThreadPoolExecutor
from copy import deepcopy
from threading import Barrier

from app import Workflow, initial, render, transition


class WorkflowTests(unittest.TestCase):
    def setUp(self):
        self.state = initial("owner", "C_ORIGIN", "123.456")

    def event(self, **values):
        return {
            "id": "event-1",
            "actor": "owner",
            "revision": 0,
            "action": "create",
            "title": "Review release",
            **values,
        }

    def test_domain_content_and_native_controls_change(self):
        before = deepcopy(self.state)
        created = transition(self.state, self.event())
        self.assertEqual(self.state, before)
        state = created["state"]
        self.assertEqual(state["tasks"], [{"id": "1", "title": "Review release", "done": False}])
        wire = created["effects"][0]["payload"]
        self.assertEqual(wire["blocks"][1]["text"]["text"], "1 tasks; 0 completed; 1 open")
        self.assertEqual(wire["blocks"][2]["accessory"]["value"], "1:1")
        completed = transition(
            state, self.event(id="event-2", revision=1, action="complete", task_id="1")
        )
        self.assertFalse(state["tasks"][0]["done"])
        self.assertTrue(completed["state"]["tasks"][0]["done"])
        wire = completed["effects"][0]["payload"]
        self.assertEqual(wire["blocks"][1]["text"]["text"], "1 tasks; 1 completed; 0 open")
        self.assertEqual(wire["blocks"][2]["text"]["text"], "Review release — Completed")
        self.assertNotIn("accessory", wire["blocks"][2])
        reply = completed["effects"][1]
        self.assertEqual((reply["channel"], reply["thread_ts"]), ("C_ORIGIN", "123.456"))
        self.assertEqual(reply["payload"]["channel"], "C_ORIGIN")
        self.assertIn("Review release", reply["payload"]["text"])
        self.assertNotIn("thread_ts", reply["payload"])

    def test_duplicate_stale_missing_owner_and_terminal_preserve_exact_state(self):
        state = transition(self.state, self.event())["state"]
        cases = [
            (self.event(), "duplicate"),
            (self.event(id="new"), "stale_revision"),
            (self.event(id="new", revision=1, actor=None), "wrong_actor"),
            (self.event(id="new", revision=1, actor="other"), "wrong_actor"),
        ]
        for event, reason in cases:
            with self.subTest(reason=reason):
                result = transition(state, event)
                self.assertEqual(result, {"state": state, "effects": [], "error": reason})
        cancelled = transition(state, self.event(id="cancel", revision=1, action="cancel"))["state"]
        result = transition(cancelled, self.event(id="late", revision=2))
        self.assertEqual(result, {"state": cancelled, "effects": [], "error": "terminal"})
        self.assertNotIn('"accessory"', json.dumps(render(cancelled)))

    def test_invalid_events_preserve_state_and_emit_no_effects(self):
        cases = [
            (self.event(id=None), "missing_event_id"),
            (self.event(revision=False), "stale_revision"),
            (self.event(action="unknown"), "unknown_action"),
            (self.event(action="complete", task_id="missing"), "not_open"),
        ]
        cases.extend(
            (self.event(title=title), "invalid_title") for title in [None, 42, "", "   ", "x" * 201]
        )
        for event, reason in cases:
            with self.subTest(event=event):
                before = deepcopy(self.state)
                self.assertEqual(
                    transition(self.state, event), {"state": before, "effects": [], "error": reason}
                )
                self.assertEqual(self.state, before)
        state = transition(self.state, self.event())["state"]
        state = transition(
            state, self.event(id="done", revision=1, action="complete", task_id="1")
        )["state"]
        self.assertEqual(
            transition(state, self.event(id="again", revision=2, action="complete", task_id="1")),
            {"state": state, "effects": [], "error": "not_open"},
        )

    def test_source_channel_cannot_be_overridden_by_form_values(self):
        result = transition(self.state, self.event(channel="C_WRONG", message_ts="999.0"))
        self.assertEqual(result["effects"][0]["channel"], "C_ORIGIN")
        self.assertEqual(result["effects"][0]["ts"], "123.456")

    def test_user_title_is_plain_text_in_both_blocks_and_fallback(self):
        title = "<!channel> <https://example.com|click> *literal*"
        state = transition(self.state, self.event(title=title))["state"]
        completed = transition(
            state, self.event(id="done", revision=1, action="complete", task_id="1")
        )
        reply = completed["effects"][1]["payload"]
        self.assertEqual(reply["text"], f"Completed: {title}")
        self.assertIs(reply["mrkdwn"], False)
        self.assertEqual(
            reply["blocks"][0]["text"], {"type": "plain_text", "text": f"Completed: {title}"}
        )

    def test_concurrent_same_revision_does_not_lose_a_committed_change(self):
        workflow = Workflow(self.state)
        barrier = Barrier(2)

        def invoke(i):
            barrier.wait()
            return workflow.handle(self.event(id=f"event-{i}", title=f"Task {i}"))

        with ThreadPoolExecutor(max_workers=2) as pool:
            results = list(pool.map(invoke, [1, 2]))
        self.assertEqual(
            sorted(r["error"] or "accepted" for r in results), ["accepted", "stale_revision"]
        )
        self.assertEqual(len(results[-1]["state"]["tasks"]), 1)

    def test_sparse_boundary_and_invalid_have_domain_oracles(self):
        self.assertEqual(render(self.state)["blocks"][2]["text"]["text"], "No tasks yet")
        state = self.state
        for i in range(20):
            state = transition(state, self.event(id=str(i), revision=i, title="x" * 200))["state"]
        self.assertEqual(len(state["tasks"]), 20)
        self.assertEqual(len(render(state)["blocks"]), 22)
        result = transition(state, self.event(id="overflow", revision=20))
        self.assertEqual(result["error"], "task_limit")
        self.assertEqual(result["state"], state)
        self.assertEqual(
            transition(self.state, self.event(title="x" * 201))["error"], "invalid_title"
        )


if __name__ == "__main__":
    unittest.main()
