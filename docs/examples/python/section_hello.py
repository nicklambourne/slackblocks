from slackblocks import Message, SectionBlock

payload = Message(
    channel="C0123456",
    blocks=SectionBlock("Hello from slackblocks!", block_id="hello"),
).to_dict()
