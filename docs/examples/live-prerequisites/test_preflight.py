from copy import deepcopy
import json
from pathlib import Path
import unittest
from preflight import evaluate


class PreflightTests(unittest.TestCase):
    def setUp(self):
        self.template = json.loads((Path(__file__).parent / "template.json").read_text())

    def test_empty_setup_blocks_live_gates_and_defers_mobile(self):
        results = evaluate(self.template)["gates"]
        self.assertEqual([g["status"] for g in results], ["blocked"] * 7 + ["deferred"])
        self.assertTrue(all(g["live_api"] == "not_checked" for g in results))

    def test_genuine_declarations_are_only_readiness_and_not_success(self):
        m = deepcopy(self.template)
        m["scope_evidence"] = "private-receipts/scopes.json"
        m["effective_scopes"] = ["files:read", "remote_files:read", "links.embed:write"]
        for fixture in m["fixtures"].values():
            fixture.update(genuine=True, evidence="unit-test-declaration-only.json")
        m["fixtures"]["slack_file"].update(file_id="test-file", accessible_to_app=True)
        m["fixtures"]["remote_file"].update(external_id="test-external", share_evidence="test-share.json")
        m["fixtures"]["video"].update(url="https://video.example.com/embed/test", configured_unfurl_domains=["video.example.com"], embeddable=True)
        m["fixtures"]["workflow"].update(available_in_workspace=True, trigger_reference="protected-config/workflow-trigger")
        m["fixtures"]["usergroup"]["id"] = "test-group"
        m["actors"] = [{"id": "test-a", "consent": True, "session_evidence": "session-a.json"}, {"id": "test-b", "consent": True, "session_evidence": "session-b.json"}]
        results = evaluate(m)["gates"]
        self.assertEqual([g["status"] for g in results], ["ready_for_live_attempt"] * 7 + ["deferred"])
        self.assertTrue(all(g["visible_result"] == "not_checked" for g in results))
        # A duplicate actor, inaccessible file or synthetic URL cannot close its gate.
        m["actors"][1]["id"] = "test-a"
        m["fixtures"]["slack_file"]["accessible_to_app"] = False
        m["fixtures"]["workflow"]["genuine"] = False
        by_name = {g["gate"]: g for g in evaluate(m)["gates"]}
        for name in ("second_user_visibility", "slack_file_image", "workflow"):
            self.assertEqual(by_name[name]["status"], "blocked")

    def test_scope_name_without_receipt_does_not_establish_installation(self):
        self.template["effective_scopes"] = ["files:read"]
        self.template["fixtures"]["interactivity"] = {"genuine": True, "evidence": "test.json"}
        self.assertEqual(evaluate(self.template)["gates"][0]["status"], "blocked")

    def test_secret_fields_and_unrequested_mobile_expansion_are_rejected(self):
        for key in ("token", "webhook_url"):
            with self.assertRaises(ValueError):
                evaluate({**self.template, key: "do-not-put-secrets-here"})
        with self.assertRaises(ValueError):
            evaluate({**self.template, "mobile": "passed"})


if __name__ == "__main__":
    unittest.main()
