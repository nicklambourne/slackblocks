"""Native constructor probes; prints JSONL and never contacts Slack."""
import json
import slackblocks as s

for family, element in {
    "static": s.StaticMultiSelectMenu("choose", options=[s.Option(s.PlainText("One"), "one")]),
    "external": s.ExternalMultiSelectMenu("choose", min_query_length=0),
    "users": s.UserMultiSelectMenu("choose"),
    "conversations": s.ConversationMultiSelectMenu("choose"),
    "channels": s.ChannelMultiSelectMenu("choose"),
}.items():
    for parent, block in {
        "actions": s.ActionsBlock([element], block_id="choices"),
        "section": s.SectionBlock("Choose values", accessory=element, block_id="choices"),
        "input": s.InputBlock(label="Choose values", element=element, block_id="choices"),
    }.items():
        payload = (s.ModalView("Multi-select probe", [block], submit="Save", submit_disabled=None)
                   if parent == "input" else s.Message("C0123456", text="Choose values", blocks=[block]))
        print(json.dumps({"family": family, "parent": parent,
                          "surface": "modal" if parent == "input" else "message",
                          "payload": payload.to_dict()}))
