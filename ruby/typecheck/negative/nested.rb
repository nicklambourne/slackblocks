require "slackblocks"
Slackblocks::ActionsBlock.new(elements: [Slackblocks::PlainText.new(text: "wrong")])

Slackblocks::LayoutItem.new(block: Slackblocks::MarkdownBlock.new(text: "No"))
Slackblocks::LayoutBlock.new(label: "Grid", rows: [], fallback: [Slackblocks::ActionsBlock.new(elements: [])])
