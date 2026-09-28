require_relative "test_helper"

class PropertyTest < Minitest::Test
  def test_unicode_text_round_trips_without_sharing_input
    random = Random.new(0x51AC)
    alphabet = ["a", "é", "🙂", "\n", "\"", "\\"]

    100.times do |iteration|
      input = Array.new(random.rand(1..80)) { alphabet.sample(random: random) }.join
      text = Slackblocks::PlainText.new(text: input)
      expected = {"type" => "plain_text", "text" => input.dup}
      input << "changed"

      assert_equal expected, text.as_json, "case #{iteration}"
      assert_equal expected, JSON.parse(text.to_json), "case #{iteration}"
      assert_equal expected, JSON.parse(JSON.generate(text)), "case #{iteration}"
    end
  end

  def test_text_length_boundary_counts_codepoints
    maximum = Slackblocks::Limits["text.max_length"]
    ["a", "é", "🙂"].each do |character|
      assert_equal maximum, Slackblocks::PlainText.new(text: character * maximum).text.length
      error = assert_raises(Slackblocks::ValidationError) do
        Slackblocks::PlainText.new(text: character * (maximum + 1))
      end
      assert_equal "length-exceeded", error.category
    end
  end

  def test_nested_payloads_remain_detached_across_random_sizes
    random = Random.new(0xB10C)

    40.times do |iteration|
      count = random.rand(1..15)
      blocks = Array.new(count) do |index|
        Slackblocks::SectionBlock.new(text: "item #{index} 🙂")
      end
      message = Slackblocks::MessagePayload.new(channel: "C123", blocks: blocks)
      expected = JSON.parse(message.to_json)
      blocks.clear
      output = message.as_json
      output["blocks"].first["text"]["text"] = "changed"

      assert_equal count, message.blocks.length, "case #{iteration}"
      assert_equal expected, JSON.parse(message.to_json), "case #{iteration}"
      assert_equal expected, JSON.parse(JSON.generate({message: message})).fetch("message"), "case #{iteration}"
    end
  end

  def test_paginator_selects_each_page_without_losing_blocks
    random = Random.new(0xFACE)

    30.times do |iteration|
      count = random.rand(1..20)
      page_size = random.rand(1..8)
      blocks = Array.new(count) { |index| Slackblocks::SectionBlock.new(text: "item #{index}") }
      page_count = (count + page_size - 1) / page_size
      selected = (1..page_count).flat_map do |page|
        output = Slackblocks::Paginator.create(action_id_prefix: "items", blocks: blocks, page: page, page_size: page_size)
        output.select { |item| item.is_a?(Slackblocks::SectionBlock) }
      end

      assert_equal blocks, selected, "case #{iteration}"
    end
  end
end
