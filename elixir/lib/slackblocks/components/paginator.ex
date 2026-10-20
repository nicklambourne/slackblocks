defmodule Slackblocks.Components.Paginator do
  @moduledoc "Pure pagination into ordinary blocks. Interaction routing and state belong to your application."
  alias Slackblocks.Blocks.{Actions, Context}
  alias Slackblocks.Elements.Button
  alias Slackblocks.Internal.{Codec, Schema, Value}
  alias Slackblocks.Objects.MarkdownText

  @doc """
  Selects a page from a nonempty block list and appends navigation when needed.

  Requires `action_id_prefix`. Options are `page` (1), `page_size` (5),
  `previous_text` ("Previous"), `next_text` ("Next"), `show_page_indicator` (true)
  and the navigation `block_id` (nil). Buttons use `.previous`/`.next` suffixes
  and decimal page numbers as values. Page numbers start at one.

  Controls are additional blocks. Put the expanded list in a message or view to
  validate that surface's total block count. No IDs are generated implicitly.
  """
  @spec to_blocks([Slackblocks.Types.block()], keyword()) ::
          {:ok, [Slackblocks.Types.block()]} | {:error, Slackblocks.ValidationError.t()}
  def to_blocks(blocks, options) do
    Value.capture(fn ->
      Value.options(
        options,
        ~w(action_id_prefix page page_size previous_text next_text show_page_indicator block_id)a
      )

      Value.list!(blocks, ["blocks"])
      if blocks == [], do: Value.fail(:missing_required, ["blocks"], "expected content blocks")

      blocks |> Enum.with_index() |> Enum.each(&check_block/1)

      prefix = Keyword.get(options, :action_id_prefix)

      if prefix == nil or prefix == "",
        do: Value.fail(:missing_required, ["action_id_prefix"], "expected a nonempty prefix")

      Codec.string!(prefix, ["action_id_prefix"])
      page = positive(Keyword.get(options, :page, 1), "page")
      size = positive(Keyword.get(options, :page_size, 5), "page_size")
      count = div(length(blocks) - 1, size) + 1
      if page > count, do: Value.fail(:out_of_range, ["page"], "page exceeds page count")
      indicator = Keyword.get(options, :show_page_indicator, true)

      unless is_boolean(indicator),
        do: Value.fail(:type_mismatch, ["show_page_indicator"], "expected boolean")

      content = Enum.slice(blocks, (page - 1) * size, size)

      if count == 1,
        do: content,
        else: content ++ navigation(page, count, prefix, indicator, options)
    end)
  end

  defp check_block({block, index}) do
    unless is_struct(block) and block.__struct__ in Schema.roles()["Block"],
      do: Value.fail(:type_mismatch, ["blocks", index], "expected a block")

    Codec.wire(block, ["blocks", index], :complete)
  end

  defp positive(value, key) do
    unless is_integer(value), do: Value.fail(:type_mismatch, [key], "expected integer")
    if value < 1, do: Value.fail(:out_of_range, [key], "expected positive integer")
    value
  end

  defp navigation(page, count, prefix, indicator, options) do
    previous =
      if page > 1,
        do: [
          Button.new!(
            text: Keyword.get(options, :previous_text, "Previous"),
            action_id: prefix <> ".previous",
            value: Integer.to_string(page - 1)
          )
        ],
        else: []

    next =
      if page < count,
        do: [
          Button.new!(
            text: Keyword.get(options, :next_text, "Next"),
            action_id: prefix <> ".next",
            value: Integer.to_string(page + 1)
          )
        ],
        else: []

    context =
      if indicator,
        do: [Context.new!(elements: [MarkdownText.new!(text: "Page #{page} of #{count}")])],
        else: []

    context ++ [Actions.new!(elements: previous ++ next, block_id: options[:block_id])]
  end

  @doc "Like `to_blocks/2`, raising on invalid input."
  @spec to_blocks!([Slackblocks.Types.block()], keyword()) :: [Slackblocks.Types.block()]
  def to_blocks!(blocks, options), do: Value.unwrap(to_blocks(blocks, options))
end
