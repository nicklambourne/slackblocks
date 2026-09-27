require_relative "test_helper"

class ValueTest < Minitest::Test
  def test_keyword_only_construction
    assert_raises(ArgumentError) { Slackblocks::PlainText.new("hello") }
    assert_raises(ArgumentError) { Slackblocks::PlainText["hello"] }
    assert_raises(ArgumentError) { Slackblocks::PlainText.new(unknown: "hello") }
  end

  def test_immutability_and_with
    original = "hello"
    block = Slackblocks::PlainText.new(text: original)
    original << "!"
    assert_equal "hello", block.text
    assert_raises(FrozenError) { block.text << "!" }
    assert_equal "updated", block.with(text: "updated").text
    error = assert_raises(Slackblocks::ValidationError) { block.with(text: "") }
    assert_equal "length-exceeded", error.category
  end

  def test_nested_collections_and_wire_output_are_detached
    children = [Slackblocks::SectionBlock.new(text: "Hi")]
    payload = Slackblocks::MessagePayload.new(channel: "C123", blocks: children)
    children.clear
    assert_equal 1, payload.blocks.length
    wire = payload.as_json
    wire["blocks"][0]["text"]["text"] << "!"
    assert_equal "Hi", payload.as_json["blocks"][0]["text"]["text"]
    assert_equal JSON.parse(payload.to_json), JSON.parse(JSON.generate(payload))
    assert_equal JSON.parse(payload.to_json), JSON.parse(JSON.generate([payload])).first
    assert_equal JSON.parse(payload.to_json), JSON.parse(JSON.generate({payload: payload}))["payload"]
  end

  def test_missing_required_and_false
    error = assert_raises(Slackblocks::ValidationError) { Slackblocks::NumberInputElement.new(action_id: "n") }
    assert_equal "missing-required", error.category
    input = Slackblocks::NumberInputElement.new(is_decimal_allowed: false)
    assert_equal false, input.as_json["is_decimal_allowed"]
  end

  def test_model_default_and_explicit_nil
    file = Slackblocks::FileBlock.new(external_id: "f")
    assert_equal "remote", file.as_json["source"]
    error = assert_raises(Slackblocks::ValidationError) do
      Slackblocks::FileBlock.new(external_id: "f", source: nil)
    end
    assert_equal "missing-required", error.category
    message = Slackblocks::MessagePayload.new(channel: "C123", text: nil)
    refute message.as_json.key?("text")
  end

  def test_codepoint_limits_and_encoding
    text = Slackblocks::PlainText.new(text: "🙂")
    assert_equal "🙂", text.text
    error = assert_raises(Slackblocks::ValidationError) do
      Slackblocks::PlainText.new(text: "x" * (Slackblocks::Limits["text.max_length"] + 1))
    end
    assert_equal "length-exceeded", error.category
    error = assert_raises(Slackblocks::ValidationError) { Slackblocks::PlainText.new(text: "\xFF".b) }
    assert_equal "type-mismatch", error.category
  end

  def test_extra_fields_and_json_types
    extra = {"future" => {"enabled" => true}}
    value = Slackblocks::DividerBlock.new(additional_fields: extra)
    extra["future"]["enabled"] = false
    assert_equal true, value.as_json["future"]["enabled"]
    assert_raises(Slackblocks::ValidationError) do
      Slackblocks::DividerBlock.new(additional_fields: {"type" => "section"})
    end
    error = assert_raises(Slackblocks::ValidationError) do
      Slackblocks::DividerBlock.new(additional_fields: {"future" => Object.new})
    end
    assert_equal "type-mismatch", error.category
  end

  def test_ruby_name_and_wire_name
    user = Slackblocks::RichTextUserGroup.new(user_group_id: "S123")
    assert_equal "S123", user.as_json["usergroup_id"]
    refute user.as_json.key?("user_group_id")
  end

  def test_no_auto_generated_block_id
    block = Slackblocks::SectionBlock.new(text: "hello")
    refute block.as_json.key?("block_id")
  end

  def test_generated_registry_matches_spec
    root = File.expand_path("../..", __dir__)
    limits = JSON.parse(File.read(File.join(root, "spec/limits.json")))
    flatten = lambda do |node, prefix = ""|
      node.flat_map do |key, value|
        path = prefix.empty? ? key : "#{prefix}.#{key}"
        value.is_a?(Hash) ? flatten.call(value, path) : [[path, value]]
      end
    end
    assert_equal flatten.call(limits).to_h, Slackblocks::Limits::VALUES
    model = JSON.parse(File.read(File.join(root, "spec/manifest.json")))
    assert_equal model.fetch("spec_version"), Slackblocks::SPEC_VERSION
  end
end
