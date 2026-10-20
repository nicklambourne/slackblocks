Slackblocks.Message.new!(
  channel: "C0123456",
  blocks: [Slackblocks.Blocks.Section.new!(text: "Hello from slackblocks!", block_id: "hello")]
)
|> Slackblocks.to_map!()
