# Native constructor probes; prints JSONL and never contacts Slack.
alias Slackblocks.{Blocks, Elements, Objects, Views}

for {family, element} <- [
      static:
        Elements.StaticMultiSelect.new!(
          action_id: "choose",
          options: [Objects.Option.new!(text: "One", value: "one")]
        ),
      external: Elements.ExternalMultiSelect.new!(action_id: "choose", min_query_length: 0),
      users: Elements.UserMultiSelect.new!(action_id: "choose"),
      conversations: Elements.ConversationMultiSelect.new!(action_id: "choose"),
      channels: Elements.ChannelMultiSelect.new!(action_id: "choose")
    ],
    {parent, block} <- [
      actions: Blocks.Actions.new!(elements: [element], block_id: "choices"),
      section:
        Blocks.Section.new!(text: "Choose values", accessory: element, block_id: "choices"),
      input: Blocks.Input.new!(label: "Choose values", element: element, block_id: "choices")
    ] do
  payload =
    if parent == :input do
      Views.Modal.new!(title: "Multi-select probe", blocks: [block], submit: "Save")
    else
      Slackblocks.Message.new!(channel: "C0123456", text: "Choose values", blocks: [block])
    end

  IO.puts(
    Jason.encode!(%{
      family: family,
      parent: parent,
      surface: if(parent == :input, do: "modal", else: "message"),
      payload: Slackblocks.to_map!(payload)
    })
  )
end
