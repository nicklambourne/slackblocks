require_relative "test_helper"

class ButtonPromptTest < Minitest::Test
  def test_fields_and_copy_validation
    button = Slackblocks::ButtonElement.new(text: "Ask", agent_prompt: "Ask Slackbot", agent_prompt_display: "x" * 80)
    assert_equal "Ask Slackbot", button.agent_prompt
    assert_equal "x" * 80, button.as_json["agent_prompt_display"]
    assert_equal "Changed", button.with(agent_prompt: "Changed").agent_prompt
    assert_equal "Ask Slackbot", button.agent_prompt
    assert_raises(Slackblocks::ValidationError) { button.with(agent_prompt: nil) }
    refute Slackblocks::ButtonElement.new(text: "Ask", agent_prompt: "Ask Slackbot").as_json.key?("agent_prompt_display")
  end

  def test_non_string_fields_are_rejected
    [:agent_prompt, :agent_prompt_display].each do |field|
      error = assert_raises(Slackblocks::ValidationError) { Slackblocks::ButtonElement.new(text: "Ask", **{field => 1}) }
      assert_equal "type-mismatch", error.category
    end
  end
end
