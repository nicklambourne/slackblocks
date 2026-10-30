require_relative "test_helper"

class LayoutTest < Minitest::Test
  def grid(**options)
    Slackblocks::LayoutBlock.new(label: "Grid", rows: [Slackblocks::LayoutRow.new(items: [Slackblocks::LayoutItem.new(block: Slackblocks::DividerBlock.new)])], **options)
  end

  def test_surface_boundaries_and_replacement_fallback
    Slackblocks::MessagePayload.new(channel: "C123", blocks: Array.new(25) { grid })
    Slackblocks::HomeTabView.new(blocks: Array.new(50) { grid })
    Slackblocks::MessagePayload.new(channel: "C123", blocks: [grid(fallback: Array.new(50) { Slackblocks::DividerBlock.new })])
    Slackblocks::MessagePayload.new(channel: "C123", blocks: Array.new(50) { Slackblocks::ContainerBlock.new(title: "Existing", child_blocks: Array.new(10) { Slackblocks::DividerBlock.new }) })
    assert_equal 25, Slackblocks::MessagePayload.new(channel: "C123", blocks: Array.new(25) { grid }).blocks.length
  end

  def test_checked_immutable_editing_and_enums
    original = grid
    changed = original.with(is_label_visible: false, grid_lines: Slackblocks::LayoutGridLines::ROWS, fallback: [])
    assert_nil original.is_label_visible
    assert_equal false, changed.is_label_visible
    assert_equal [], changed.fallback
    assert_equal :end, Slackblocks::LayoutAlignment::END_
    assert_raises(FrozenError) { original.rows.clear }
    assert_raises(Slackblocks::ValidationError) { Slackblocks::LayoutItem.new(block: Slackblocks::DividerBlock.new, width: :full) }
  end
end
