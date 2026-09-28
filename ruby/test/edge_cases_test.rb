require_relative "test_helper"

class EdgeCasesTest < Minitest::Test
  def assert_category(category)
    error = assert_raises(Slackblocks::ValidationError) { yield }
    assert_equal category, error.category
    refute_empty error.path
  end

  def test_invalid_public_value_types
    assert_category("type-mismatch") { Slackblocks::PlainText.new(text: 42) }
    assert_category("type-mismatch") { Slackblocks::NumberInputElement.new(is_decimal_allowed: "false") }
    assert_category("type-mismatch") { Slackblocks::RichTextText.new(text: "Hi", style: {}) }
    assert_category("type-mismatch") { Slackblocks::Option.new(text: 42, value: "one") }
    assert_category("type-mismatch") { Slackblocks::AxisConfig.new(categories: "one") }
  end

  def test_additional_fields_reject_non_json_and_cyclic_values
    assert_category("type-mismatch") { Slackblocks::DividerBlock.new(additional_fields: []) }
    assert_category("type-mismatch") { Slackblocks::DividerBlock.new(additional_fields: {42 => "value"}) }
    assert_category("out-of-range") { Slackblocks::DividerBlock.new(additional_fields: {"number" => Float::INFINITY}) }

    array = []
    array << array
    assert_category("type-mismatch") { Slackblocks::DividerBlock.new(additional_fields: {"cycle" => array}) }

    hash = {}
    hash["self"] = hash
    assert_category("type-mismatch") { Slackblocks::DividerBlock.new(additional_fields: {"cycle" => hash}) }
  end

  def test_structural_validation_edges
    assert_category("missing-required") { Slackblocks::SectionBlock.new(fields: []) }
    assert_category("missing-required") { Slackblocks::CardBlock.new }
    assert_category("missing-required") { Slackblocks::ContainerBlock.new }
    assert_category("mutually-exclusive") { Slackblocks::SlackFile.new }
    assert_category("invalid-usage") { Slackblocks::AxisConfig.new(categories: ["same", "same"]) }
  end

  def test_component_input_types_and_single_page
    block = Slackblocks::DividerBlock.new
    assert_category("type-mismatch") { Slackblocks::Accordion.create(sections: block) }
    assert_category("type-mismatch") { Slackblocks::Paginator.create(action_id_prefix: "items", blocks: block) }
    assert_category("type-mismatch") { Slackblocks::Paginator.create(action_id_prefix: 5, blocks: [block]) }
    assert_category("type-mismatch") { Slackblocks::Paginator.create(action_id_prefix: "items", blocks: [block], page: "1") }
    assert_equal [block], Slackblocks::Paginator.create(action_id_prefix: "items", blocks: [block])
  end

  def test_validation_error_rejects_unknown_category
    assert_raises(ArgumentError) { Slackblocks::ValidationError.new("unknown", "field", "message") }
  end
end
