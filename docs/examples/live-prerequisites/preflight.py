"""Offline prerequisite inventory: declarations are never live-test passes."""
import argparse
import json
from urllib.parse import urlsplit
from pathlib import Path


def evaluate(manifest):
    allowed = {"scope_evidence", "effective_scopes", "fixtures", "actors", "mobile"}
    if set(manifest) - allowed:
        raise ValueError("unknown top-level fields; do not supply tokens or webhook URLs")
    scopes = manifest.get("effective_scopes", [])
    if not isinstance(scopes, list) or not all(isinstance(s, str) for s in scopes):
        raise ValueError("effective_scopes must be a list of scope names")
    fixtures = manifest.get("fixtures", {})
    actors = manifest.get("actors", [])
    if not isinstance(fixtures, dict) or not isinstance(actors, list):
        raise ValueError("fixtures must be an object and actors a list")
    results = []

    def check(name, required_scopes=(), fixture=None, extra=()):
        reasons = []
        if required_scopes and not manifest.get("scope_evidence"):
            reasons.append("effective scopes lack a retained verification receipt")
        for scope in required_scopes:
            if scope not in scopes:
                reasons.append(f"missing effective scope: {scope}")
        if fixture:
            item = fixtures.get(fixture, {})
            if not isinstance(item, dict) or item.get("genuine") is not True or not item.get("evidence"):
                reasons.append(f"{fixture}: genuine fixture and verification evidence required")
        reasons.extend(reason for passed, reason in extra if not passed)
        results.append({"gate": name, "status": "blocked" if reasons else "ready_for_live_attempt",
                        "reasons": reasons, "local_model": "not_checked", "live_api": "not_checked",
                        "visible_result": "not_checked"})

    def fixture(name):
        item = fixtures.get(name, {})
        return item if isinstance(item, dict) else {}

    check("file_input", ("files:read",), "interactivity")
    file = fixture("slack_file")
    check("slack_file_image", ("files:read",), "slack_file", [
        (bool(file.get("file_id")), "missing accessible Slack file ID"),
        (file.get("accessible_to_app") is True, "file access by the installed app is unverified")])
    remote = fixture("remote_file")
    check("remote_file_readback", ("remote_files:read",), "remote_file", [
        (bool(remote.get("external_id")), "missing registered external ID"),
        (bool(remote.get("share_evidence")), "remote file sharing is unverified; adding alone does not share")])
    video = fixture("video")
    raw_url = video.get("url")
    try:
        url = urlsplit(raw_url if isinstance(raw_url, str) else "")
    except ValueError:
        url = urlsplit("")
    check("video", ("links.embed:write",), "video", [
        (url.scheme == "https" and bool(url.hostname), "video fixture requires HTTPS"),
        (url.hostname in video.get("configured_unfurl_domains", []), "video host is absent from configured unfurl domains"),
        (video.get("embeddable") is True, "embeddable playback/access is unverified")])
    workflow = fixture("workflow")
    check("workflow", fixture="workflow", extra=[
        (workflow.get("available_in_workspace") is True, "workflow capability is unverified for this workspace/plan"),
        (bool(workflow.get("trigger_reference")), "missing genuine trigger reference; keep the trigger URL in protected configuration")])
    group = fixture("usergroup")
    check("usergroup", fixture="usergroup", extra=[
        (bool(group.get("id")), "missing genuine user-group ID")])
    ids = {a.get("id") for a in actors if isinstance(a, dict) and a.get("id")}
    check("second_user_visibility", extra=[
        (len(ids) >= 2, "two distinct consenting actor IDs required"),
        (len(actors) >= 2 and all(isinstance(a, dict) and a.get("consent") is True and a.get("session_evidence") for a in actors),
         "independent authenticated actor sessions and consent are unverified")])
    mobile = manifest.get("mobile", "deferred")
    if mobile != "deferred":
        raise ValueError("this test plan defers mobile; use a separate explicitly scoped plan")
    results.append({"gate": "mobile", "status": "deferred", "reasons": ["outside this test plan"],
                    "local_model": "not_checked", "live_api": "not_checked", "visible_result": "not_checked"})
    return {"scope": "offline declared prerequisites only; readiness is not a live pass", "gates": results}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    args = parser.parse_args()
    report = evaluate(json.loads(args.manifest.read_text()))
    print(json.dumps(report, indent=2))
    raise SystemExit(2 if any(g["status"] == "blocked" for g in report["gates"]) else 0)
