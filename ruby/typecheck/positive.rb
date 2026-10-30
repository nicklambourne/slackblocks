require "slackblocks"

text = Slackblocks::PlainText.new(text: "Hello")
button = Slackblocks::ButtonElement.new(text: text, action_id: "open")
block = Slackblocks::ActionsBlock.new(elements: [button])
Slackblocks::MessagePayload.new(channel: "C123", blocks: [block]).to_json

layout = Slackblocks::LayoutBlock.new(label: "Grid", rows: [Slackblocks::LayoutRow.new(items: [Slackblocks::LayoutItem.new(block: block, width: :wide, horizontal_alignment: :end)])], fallback: [Slackblocks::DividerBlock.new])
Slackblocks::MessagePayload.new(channel: "C123", blocks: [layout]).to_json
