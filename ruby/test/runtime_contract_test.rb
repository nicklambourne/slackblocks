require_relative "test_helper"

class RuntimeContractTest < Minitest::Test
  def assert_category(category)
    error = assert_raises(Slackblocks::ValidationError) { yield }
    assert_equal category, error.category
    refute_empty error.path
  end

  def test_frozen_values_and_structural_identity
    source = {"nested" => ["first"]}
    first = Slackblocks::DividerBlock.new(additional_fields: {payload: source})
    equal = Slackblocks::DividerBlock.new(additional_fields: {payload: {"nested" => ["first"]}})
    original_hash = first.hash
    source["nested"] << "second"
    assert_equal equal, first
    assert first.eql?(equal)
    assert_equal original_hash, first.hash
    assert_raises(FrozenError) { first.additional_fields["payload"]["nested"] << "third" }
    assert_equal({payload: {"nested" => ["first"]}}, first.to_h[:additional_fields].transform_keys(&:to_sym))
    assert_same first, first.with
    refute_equal first, first.with(additional_fields: {payload: {"nested" => ["new"]}})
  end

  def test_json_output_is_detached_and_nested_json_protocol
    payload = Slackblocks::MessagePayload.new(
      channel: "C123",
      blocks: [Slackblocks::SectionBlock.new(text: "Hello")]
    )
    detached = payload.as_json
    detached["blocks"][0]["text"]["text"].replace("Changed")
    assert_equal "Hello", payload.as_json["blocks"][0]["text"]["text"]
    expected = JSON.parse(payload.to_json)
    assert_equal expected, JSON.parse(JSON.generate(payload))
    assert_equal expected, JSON.parse(JSON.generate([payload])).first
    assert_equal expected, JSON.parse(JSON.generate({payload: payload})).fetch("payload")
  end

  def test_required_default_and_explicit_nil
    assert_equal "in_channel", Slackblocks::MessageResponse.new.as_json.fetch("response_type")
    assert_equal "remote", Slackblocks::FileBlock.new(external_id: "f").as_json.fetch("source")
    assert_category("missing-required") { Slackblocks::FileBlock.new(external_id: "f", source: nil) }
    refute Slackblocks::MessagePayload.new(channel: "C123", text: nil).as_json.key?("text")
    assert_equal false, Slackblocks::NumberInputElement.new(is_decimal_allowed: false).as_json.fetch("is_decimal_allowed")
  end

  def test_numeric_types_and_finite_values
    assert_category("type-mismatch") { Slackblocks::ChartSegment.new(label: "A", value: Rational(1, 2)) }
    assert_category("type-mismatch") { Slackblocks::ChartSegment.new(label: "A", value: Complex(1, 0)) }
    assert_category("out-of-range") { Slackblocks::ChartSegment.new(label: "A", value: Float::INFINITY) }
    assert_category("out-of-range") { Slackblocks::ChartSegment.new(label: "A", value: Float::NAN) }
    assert_category("type-mismatch") { Slackblocks::DataTableBlock.new(rows: [], caption: "A", page_size: 2.5) }
  end

  def test_encoding_and_codepoint_boundaries
    converted = "Résumé".encode("UTF-16LE")
    assert_equal "Résumé", Slackblocks::PlainText.new(text: converted).text
    assert_category("type-mismatch") { Slackblocks::PlainText.new(text: "\xFF".b) }
    bound = Slackblocks::Limits["header.text.max_length"]
    assert_equal bound, Slackblocks::HeaderBlock.new(text: "🙂" * bound).text.text.length
    assert_category("length-exceeded") { Slackblocks::HeaderBlock.new(text: "🙂" * (bound + 1)) }
  end

  def test_enums_and_text_coercion
    assert_category("type-mismatch") { Slackblocks::MessageResponse.new(response_type: "ephemeral") }
    assert_category("type-mismatch") { Slackblocks::MessageResponse.new(response_type: :unknown) }
    assert_equal "mrkdwn", Slackblocks::SectionBlock.new(text: "Hi").text.as_json.fetch("type")
    assert_equal "plain_text", Slackblocks::HeaderBlock.new(text: "Hi").text.as_json.fetch("type")
  end

  def test_style_membership_and_extension_keys
    assert_category("type-mismatch") do
      Slackblocks::RichTextLink.new(url: "https://example.com", style: Slackblocks::RichTextStyle.new(highlight: true))
    end
    assert_category("type-mismatch") do
      Slackblocks::RichTextText.new(text: "A", style: Slackblocks::RichTextStyle.new(additional_fields: {future: true}))
    end
    assert_category("invalid-usage") { Slackblocks::DividerBlock.new(additional_fields: {"block_id" => "override"}) }
    assert_category("invalid-usage") { Slackblocks::DividerBlock.new(additional_fields: {"future" => 1, :future => 2}) }
    assert_equal({"future" => nil}, Slackblocks::DividerBlock.new(additional_fields: {future: nil}).as_json.slice("future"))
  end

  def test_whole_string_patterns_and_image_identity
    assert_category("type-mismatch") { Slackblocks::SlackFile.new(id: "F12345678\njunk") }
    assert_category("type-mismatch") { Slackblocks::SlackFile.new(id: "F12345678\n") }
    assert_category("type-mismatch") do
      Slackblocks::Attachment.new(blocks: [], color: "#abcdef\n")
    end
    image = Slackblocks::ImageElement.new(image_url: "https://example.com/x", alt_text: "A")
    assert_instance_of Slackblocks::ImageElement, image
    assert_category("mutually-exclusive") do
      Slackblocks::ImageBlock.new(image_url: "https://example.com/x", slack_file: Slackblocks::SlackFile.new(id: "F12345678"), alt_text: "A")
    end
  end

  def test_contextual_tasks_and_data_table_extensions
    task = Slackblocks::TaskCardBlock.new(task_id: "t", title: "Task", status: :pending)
    plan = Slackblocks::PlanBlock.new(title: "Plan", tasks: [task])
    refute plan.as_json["tasks"].first.key?("type")
    assert_equal "task_card", task.as_json.fetch("type")
    assert_category("type-mismatch") { Slackblocks::MessagePayload.new(channel: "C123", blocks: [task]) }

    rows = [
      [Slackblocks::RawText.new(text: "Name")],
      [Slackblocks::RawText.new(text: "Alice")]
    ]
    assert_category("invalid-usage") do
      Slackblocks::DataTableBlock.new(rows: rows, caption: "Names", additional_fields: {column_settings: []})
    end
    assert_category("invalid-usage") do
      Slackblocks::DataTableBlock.new(rows: [rows[0], [*rows[1], Slackblocks::RawText.new(text: "Extra")]], caption: "Names")
    end
  end

  def test_relaxed_rules
    button = Slackblocks::ButtonElement.new(text: "No action")
    refute button.as_json.key?("action_id")
    assert_equal "", Slackblocks::MarkdownBlock.new(text: "").text
    assert_equal [], Slackblocks::HomeTabView.new(blocks: []).blocks
    assert_equal [], Slackblocks::ModalView.new(title: "Empty", blocks: []).blocks
    assert_equal "https://example.com/" + ("x" * 5000),
      Slackblocks::UrlSource.new(url: "https://example.com/" + ("x" * 5000), text: "A").url
  end
end
