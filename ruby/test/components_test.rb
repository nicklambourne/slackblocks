require_relative "test_helper"

class ComponentsTest < Minitest::Test
  def test_accordion
    section = Slackblocks::AccordionSection.create(
      title: "Summary",
      blocks: [Slackblocks::DividerBlock.new]
    )
    assert_equal true, section.is_collapsible
    assert_equal true, section.default_collapsed
    assert_equal "Summary", section.title.text
    assert_equal [section], Slackblocks::Accordion.create(sections: [section])
    assert_raises(Slackblocks::ValidationError) { Slackblocks::Accordion.create(sections: []) }
    assert_raises(Slackblocks::ValidationError) do
      Slackblocks::Accordion.create(sections: [Slackblocks::DividerBlock.new])
    end
  end

  def test_paginator_first_middle_last
    blocks = Array.new(7) { Slackblocks::DividerBlock.new }
    first = Slackblocks::Paginator.create(action_id_prefix: "items", blocks: blocks, page_size: 3)
    assert_equal 5, first.length
    assert_equal "Page 1 of 3", first[-2].elements.first.text
    assert_equal ["items.next"], first.last.elements.map(&:action_id)
    assert_equal ["2"], first.last.elements.map(&:value)

    middle = Slackblocks::Paginator.create(action_id_prefix: "items", blocks: blocks, page: 2, page_size: 3)
    assert_equal 5, middle.length
    assert_equal ["items.previous", "items.next"], middle.last.elements.map(&:action_id)
    assert_equal ["1", "3"], middle.last.elements.map(&:value)

    last = Slackblocks::Paginator.create(action_id_prefix: "items", blocks: blocks, page: 3, page_size: 3, show_page_indicator: false)
    assert_equal 2, last.length
    assert_equal ["items.previous"], last.last.elements.map(&:action_id)
    assert_equal ["2"], last.last.elements.map(&:value)
  end

  def test_paginator_invalid_inputs
    block = Slackblocks::DividerBlock.new
    [
      {action_id_prefix: "", blocks: [block]},
      {action_id_prefix: "items", blocks: []},
      {action_id_prefix: "items", blocks: [block], page: 0},
      {action_id_prefix: "items", blocks: [block], page: 2},
      {action_id_prefix: "items", blocks: [block], page_size: 0}
    ].each do |kwargs|
      assert_raises(Slackblocks::ValidationError) { Slackblocks::Paginator.create(**kwargs) }
    end
  end
end
