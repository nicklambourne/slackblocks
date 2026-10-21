defmodule Slackblocks.Test.Samples do
  @moduledoc false
  alias Slackblocks.Test.ValidConstructions

  @external_resource Path.expand("../../design/parity.json", __DIR__)
  @models Jason.decode!(File.read!(Path.expand("../../design/parity.json", __DIR__)))["models"]
          |> Map.values()

  def all do
    ValidConstructions.all()
    |> Enum.flat_map(fn {_, build} -> collect(build.()) end)
    |> Enum.reduce(%{}, fn value, acc -> Map.put_new(acc, value.__struct__, value) end)
  end

  def fixture_modules do
    Map.new(ValidConstructions.all(), fn {id, build} ->
      {id, MapSet.new(Enum.map(collect(build.()), &inspect(&1.__struct__)))}
    end)
  end

  defp collect(%{__struct__: module} = value) do
    if inspect(module) in @models, do: [value | collect(Map.from_struct(value))], else: []
  end

  defp collect(value) when is_map(value), do: value |> Map.values() |> Enum.flat_map(&collect/1)
  defp collect(value) when is_list(value), do: Enum.flat_map(value, &collect/1)
  defp collect(_), do: []
end
