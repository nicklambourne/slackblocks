defmodule Slackblocks.Components.Accordion do
  @moduledoc "Expands collapsible sections into ordinary blocks in display order."
  alias Slackblocks.Blocks.Container
  alias Slackblocks.Internal.Value

  @doc "Checks a nonempty list of collapsible containers before returning its blocks."
  @spec to_blocks([Container.t()]) ::
          {:ok, [Container.t()]} | {:error, Slackblocks.ValidationError.t()}
  def to_blocks(sections) do
    Value.capture(fn ->
      Value.list!(sections, ["sections"])

      if sections == [],
        do: Value.fail(:missing_required, ["sections"], "expected at least one section")

      sections |> Enum.with_index() |> Enum.each(&check_section/1)

      sections
    end)
  end

  defp check_section({section, index}) do
    unless is_struct(section, Container) and section.is_collapsible == true,
      do: Value.fail(:type_mismatch, ["sections", index], "expected a collapsible container")

    Slackblocks.to_map!(section)
  end

  @doc "Like `to_blocks/1`, raising on failure."
  @spec to_blocks!([Container.t()]) :: [Container.t()]
  def to_blocks!(sections), do: Value.unwrap(to_blocks(sections))
end
