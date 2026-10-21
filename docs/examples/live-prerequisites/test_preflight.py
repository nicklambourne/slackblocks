import json
import unittest
from copy import deepcopy
from pathlib import Path

from preflight import evaluate


class PreflightTests(unittest.TestCase):
    def setUp(self):
        self.template = json.loads(
            (Path(__file__).parent / "template.json").read_text()
        )

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
        m["fixtures"]["remote_file"].update(
            external_id="test-external", share_evidence="test-share.json"
        )
        m["fixtures"]["video"].update(
            url="https://video.example.com/embed/test",
            configured_unfurl_domains=["video.example.com"],
            embeddable=True,
        )
        m["fixtures"]["workflow"].update(
            available_in_workspace=True,
            trigger_reference="protected-config/workflow-trigger",
        )
        m["fixtures"]["usergroup"]["id"] = "test-group"
        m["actors"] = [
            {"id": "test-a", "consent": True, "session_evidence": "session-a.json"},
            {"id": "test-b", "consent": True, "session_evidence": "session-b.json"},
        ]
        results = evaluate(m)["gates"]
        self.assertEqual(
            [g["status"] for g in results],
            ["ready_for_live_attempt"] * 7 + ["deferred"],
        )
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
        self.template["fixtures"]["interactivity"] = {
            "genuine": True,
            "evidence": "test.json",
        }
        self.assertEqual(evaluate(self.template)["gates"][0]["status"], "blocked")

    def test_video_domains_require_valid_list_and_domain_boundaries(self):
        m = deepcopy(self.template)
        m["scope_evidence"] = "test-scopes.json"
        m["effective_scopes"] = ["links.embed:write"]
        video = m["fixtures"]["video"]
        video.update(
            genuine=True,
            evidence="test-video.json",
            url="https://video.example.com/embed/test",
            embeddable=True,
        )
        for domains in (
            "evilvideo.example.com",
            None,
            42,
            {"video.example.com": True},
            [None],
            [""],
            ["com"],
            ["127.0.0.1"],
            ["https://video.example.com"],
            ["vidéo.example.com"],
        ):
            with self.subTest(domains=domains):
                video["configured_unfurl_domains"] = domains
                result = next(g for g in evaluate(m)["gates"] if g["gate"] == "video")
                self.assertEqual(result["status"], "blocked")
                self.assertIn(
                    "configured_unfurl_domains must be a list of registered domain names",
                    result["reasons"],
                )
        for domains, status in [
            (["evilvideo.example.com"], "blocked"),
            (["example.com"], "ready_for_live_attempt"),
            (["VIDEO.EXAMPLE.COM"], "ready_for_live_attempt"),
            (["sub.video.example.com"], "blocked"),
            (["video.example.com"], "ready_for_live_attempt"),
        ]:
            video["configured_unfurl_domains"] = domains
            result = next(g for g in evaluate(m)["gates"] if g["gate"] == "video")
            self.assertEqual(result["status"], status)

    def test_malformed_actor_ids_and_receipts_stay_blocked(self):
        for bad in (["actor"], {"id": "actor"}, 42, True, None, "", "   "):
            with self.subTest(value=bad):
                self.template["actors"] = [
                    {"id": bad, "consent": True, "session_evidence": "receipt-a"},
                    {"id": "actor-b", "consent": True, "session_evidence": "receipt-b"},
                ]
                gate = next(
                    g
                    for g in evaluate(self.template)["gates"]
                    if g["gate"] == "second_user_visibility"
                )
                self.assertEqual(gate["status"], "blocked")
        self.template["actors"] = [
            {"id": "actor-a", "consent": True, "session_evidence": True},
            {"id": "actor-b", "consent": True, "session_evidence": "receipt-b"},
        ]
        self.assertEqual(evaluate(self.template)["gates"][-2]["status"], "blocked")

    def test_receipt_declarations_must_be_nonempty_strings(self):
        self.template["effective_scopes"] = ["files:read"]
        self.template["fixtures"]["interactivity"] = {
            "genuine": True,
            "evidence": "receipt",
        }
        for invalid in (True, 1, {"receipt": "x"}, ["receipt"], "  "):
            with self.subTest(value=invalid):
                self.template["scope_evidence"] = invalid
                self.assertEqual(
                    evaluate(self.template)["gates"][0]["status"], "blocked"
                )
        self.template["scope_evidence"] = "receipt"
        self.template["fixtures"]["interactivity"]["evidence"] = True
        self.assertEqual(evaluate(self.template)["gates"][0]["status"], "blocked")

    def test_manifest_shape_and_malformed_video_urls(self):
        for invalid in (None, [], ["fixtures"], "manifest", 42):
            with (
                self.subTest(value=invalid),
                self.assertRaisesRegex(ValueError, "manifest must be an object"),
            ):
                evaluate(invalid)
        for invalid in (None, 42, "https://[invalid", "http://video.example.com"):
            with self.subTest(value=invalid):
                self.template["fixtures"]["video"]["url"] = invalid
                self.assertEqual(
                    evaluate(self.template)["gates"][3]["status"], "blocked"
                )

    def test_secret_fields_and_unrequested_mobile_expansion_are_rejected(self):
        for key in ("token", "webhook_url"):
            with self.assertRaises(ValueError):
                evaluate({**self.template, key: "do-not-put-secrets-here"})
        with self.assertRaises(ValueError):
            evaluate({**self.template, "mobile": "passed"})


if __name__ == "__main__":
    unittest.main()
