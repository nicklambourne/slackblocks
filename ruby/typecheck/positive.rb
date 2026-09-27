require "slackblocks"

text = Slackblocks::PlainText.new(text: "Hello")
button = Slackblocks::ButtonElement.new(text: text, action_id: "open")
block = Slackblocks::ActionsBlock.new(elements: [button])
Slackblocks::MessagePayload.new(channel: "C123", blocks: [block]).to_json
