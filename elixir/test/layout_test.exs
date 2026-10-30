defmodule Slackblocks.LayoutTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Blocks.{Container, Divider, Layout}
  alias Slackblocks.Objects.{LayoutItem, LayoutRow}
  alias Slackblocks.Views.Home

  defp grid,
    do:
      Layout.new!(
        label: "Grid",
        rows: [LayoutRow.new!(items: [LayoutItem.new!(block: Divider.new!())])]
      )

  test "surface boundaries and replacement fallback preserve valid payloads" do
    assert %Slackblocks.Message{} =
             Slackblocks.Message.new!(channel: "C123", blocks: List.duplicate(grid(), 25))

    assert %Home{} =
             Home.new!(blocks: List.duplicate(grid(), 50))

    fallback = Layout.change!(grid(), fallback: List.duplicate(Divider.new!(), 50))
    assert %Slackblocks.Message{} = Slackblocks.Message.new!(channel: "C123", blocks: [fallback])
    existing = Container.new!(title: "Existing", child_blocks: List.duplicate(Divider.new!(), 10))

    assert %Slackblocks.Message{} =
             Slackblocks.Message.new!(channel: "C123", blocks: List.duplicate(existing, 50))
  end

  test "checked edits preserve omission and validate item vocabulary" do
    before = grid()
    changed = Layout.change!(before, fallback: [], is_label_visible: false, grid_lines: :none)
    assert before.fallback == nil
    assert changed.fallback == []
    assert changed.is_label_visible == false
    assert {:error, _} = LayoutItem.new(block: Divider.new!(), width: :full)
  end
end
