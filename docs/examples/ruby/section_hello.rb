require "slackblocks"

payload = Slackblocks::MessagePayload.new(
  channel: "C0123456",
  blocks: [Slackblocks::SectionBlock.new(text: "Hello from slackblocks!", block_id: "hello")]
)
puts payload.to_json
