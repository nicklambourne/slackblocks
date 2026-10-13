"""A native task workflow. No Slack calls, authentication, or durable storage."""
from copy import deepcopy
from threading import Lock
import slackblocks as s


def initial(owner, channel, message_ts):
    # The receiver supplies this origin from the authenticated message/modal.
    return {"owner": owner, "origin": {"channel": channel, "ts": message_ts},
            "revision": 0, "seen": [], "tasks": [], "next_id": 1, "cancelled": False}


def render(state):
    revision = state["revision"]
    tasks = state["tasks"]
    complete = sum(t["done"] for t in tasks)
    blocks = [s.HeaderBlock("Team tasks", block_id=f"header-{revision}"),
              s.SectionBlock(f"{len(tasks)} tasks; {complete} completed; {len(tasks)-complete} open",
                             block_id=f"totals-{revision}")]
    if not tasks:
        blocks.append(s.SectionBlock("No tasks yet", block_id=f"empty-{revision}"))
    for task in tasks:
        # PlainText prevents a task title from becoming mention/link markup.
        status = "Completed" if task["done"] else "Open"
        button = None if task["done"] or state["cancelled"] else s.Button(
            "Complete", action_id="task-complete", value=f"{revision}:{task['id']}")
        blocks.append(s.SectionBlock(s.PlainText(f"{task['title']} — {status}"),
                                     accessory=button, block_id=f"task-{revision}-{task['id']}"))
    if state["cancelled"]:
        blocks.append(s.SectionBlock("Cancelled; no more changes", block_id=f"closed-{revision}"))
    return s.Message(state["origin"]["channel"], text=f"Team tasks: {complete}/{len(tasks)} completed",
                     blocks=blocks).to_dict()


def transition(before, event):
    """Normalize authenticated callbacks into id/actor/revision/action/values first."""
    def reject(reason):
        return {"state": deepcopy(before), "effects": [], "error": reason}

    if not event.get("id"):
        return reject("missing_event_id")
    if event["id"] in before["seen"]:
        return reject("duplicate")
    if not event.get("actor") or event["actor"] != before["owner"]:
        return reject("wrong_actor")
    if type(event.get("revision")) is not int or event["revision"] != before["revision"]:
        return reject("stale_revision")
    if before["cancelled"]:
        return reject("terminal")
    state = deepcopy(before)
    action = event.get("action")
    if action == "create":
        title = event.get("title", "").strip()
        if not title or len(title) > 200:
            return reject("invalid_title")
        if len(state["tasks"]) >= 20:
            return reject("task_limit")
        state["tasks"].append({"id": str(state["next_id"]), "title": title, "done": False})
        state["next_id"] += 1
    elif action == "complete":
        task = next((t for t in state["tasks"] if t["id"] == event.get("task_id")), None)
        if task is None or task["done"]:
            return reject("not_open")
        task["done"] = True
    elif action == "cancel":
        state["cancelled"] = True
    else:
        return reject("unknown_action")
    state["seen"].append(event["id"])
    state["revision"] += 1
    origin = state["origin"]
    effects = [{"operation_id": f"{event['id']}:update", "method": "chat.update",
                "channel": origin["channel"], "ts": origin["ts"], "payload": render(state)}]
    if action == "complete":
        # thread_ts is transport metadata, not a claim about Message's native fields.
        reply = s.Message(origin["channel"], text=f"Completed: {task['title']}", mrkdwn=False,
                          blocks=[s.SectionBlock(s.PlainText(f"Completed: {task['title']}"),
                                                 block_id=f"reply-{state['revision']}")])
        effects.append({"operation_id": f"{event['id']}:reply", "method": "chat.postMessage",
                        "channel": origin["channel"], "thread_ts": origin["ts"],
                        "payload": reply.to_dict()})
    return {"state": state, "effects": effects, "error": None}


class Workflow:
    """Single-process serialization example; use transactional persistence in production."""
    def __init__(self, state):
        self._state = deepcopy(state)
        self._lock = Lock()

    def handle(self, event):
        with self._lock:
            result = transition(self._state, event)
            self._state = deepcopy(result["state"])
            return result
