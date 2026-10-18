defmodule Slackblocks.Components.AccordionSection do
  @moduledoc "Creates a collapsible container suitable for an accordion."
  alias Slackblocks.Blocks.Container
  alias Slackblocks.Internal.Value

  @doc "Builds a section from `title`, `blocks`, and optional container presentation keywords."
  @spec new(keyword()) :: {:ok, Container.t()} | {:error, Slackblocks.ValidationError.t()}
  def new(options) do
    Value.capture(fn ->
      Value.options(options, ~w(title blocks subtitle icon expanded width block_id)a)

      expanded = Keyword.get(options, :expanded, false)

      unless is_boolean(expanded),
        do: Value.fail(:type_mismatch, ["expanded"], "expected boolean")

      Container.new!(
        title: options[:title],
        child_blocks: options[:blocks],
        subtitle: options[:subtitle],
        icon: options[:icon],
        width: Keyword.get(options, :width, :standard),
        is_collapsible: true,
        default_collapsed: not expanded,
        has_header_divider: false,
        block_id: options[:block_id]
      )
    end)
  end

  @doc "Like `new/1`, raising on failure."
  @spec new!(keyword()) :: Container.t()
  def new!(options), do: Value.unwrap(new(options))
end
