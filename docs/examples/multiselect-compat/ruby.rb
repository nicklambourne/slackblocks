# Native constructor probes; no network operations.
require "json"
require "slackblocks"
include Slackblocks
{
  static: StaticMultiSelectElement.new(action_id: "choose", options: [Option.new(text: "One", value: "one")]),
  external: ExternalMultiSelectElement.new(action_id: "choose", min_query_length: 0),
  users: UserMultiSelectElement.new(action_id: "choose"),
  conversations: ConversationMultiSelectElement.new(action_id: "choose"),
  channels: ChannelMultiSelectElement.new(action_id: "choose")
}.each do |family, element|
  {
    actions: ActionsBlock.new(elements: [element], block_id: "choices"),
    section: SectionBlock.new(text: "Choose values", accessory: element, block_id: "choices"),
    input: InputBlock.new(label: "Choose values", element: element, block_id: "choices")
  }.each do |parent, block|
    payload = if parent == :input
      ModalView.new(title: "Multi-select probe", blocks: [block], submit: "Save")
    else
      MessagePayload.new(channel: "C0123456", text: "Choose values", blocks: [block])
    end
    puts JSON.generate(family: family, parent: parent, surface: parent == :input ? "modal" : "message", payload: payload.as_json)
  end
end
