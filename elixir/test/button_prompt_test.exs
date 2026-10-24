defmodule Slackblocks.ButtonPromptTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Elements.Button
  alias Slackblocks.ValidationError

  test "native fields survive ingress, editing and serialization" do
    button =
      Button.new!(
        text: "Ask",
        agent_prompt: "Ask Slackbot",
        agent_prompt_display: String.duplicate("x", 80)
      )

    wire = Slackblocks.to_map!(button)
    assert {:ok, ^button} = Slackblocks.from_map(Button, wire)
    assert Button.change!(button, agent_prompt: "Changed").agent_prompt == "Changed"
    assert button.agent_prompt == "Ask Slackbot"

    assert {:error, %ValidationError{category: :missing_required}} =
             Button.change(button, agent_prompt: nil)

    refute Map.has_key?(
             Slackblocks.to_map!(Button.new!(text: "Ask", agent_prompt: "Ask Slackbot")),
             "agent_prompt_display"
           )
  end

  test "ingress and struct egress reject non-string prompt fields" do
    button = Button.new!(text: "Ask")

    for field <- [:agent_prompt, :agent_prompt_display] do
      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.to_map(Map.put(button, field, 1))

      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.from_map(
                 Button,
                 Map.put(Slackblocks.to_map!(button), Atom.to_string(field), 1)
               )
    end
  end
end
