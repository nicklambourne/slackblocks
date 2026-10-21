defmodule Slackblocks.Test.ApiAudit do
  @moduledoc false
  @root Path.expand("../../..", __DIR__)
  @external_resource Path.join(@root, "elixir/design/public_api.json")
  @inventory Jason.decode!(File.read!(Path.join(@root, "elixir/design/public_api.json")))

  def modules do
    {:ok, modules} = :application.get_key(:slackblocks, :modules)
    Enum.reject(modules, &String.starts_with?(inspect(&1), "Slackblocks.Test."))
  end

  def check! do
    Enum.each(modules(), &check_module!/1)
    actual = MapSet.new(Enum.map(modules(), &inspect/1))
    expected = MapSet.new(Map.keys(@inventory["modules"]))

    unless actual == expected,
      do:
        raise("Public module inventory differs: #{inspect(MapSet.difference(expected, actual))}")

    :ok
  end

  def check_module!(module) do
    name = inspect(module)
    entry = @inventory["modules"][name] || raise("Unclassified production module: #{name}")

    actual =
      module.__info__(:functions)
      |> Enum.map(fn {name, arity} -> "#{name}/#{arity}" end)
      |> Enum.sort()

    unless actual == Enum.sort(entry["functions"]),
      do: raise("Unclassified public function or removed API in #{name}")

    unless is_binary(entry["reason"]) and entry["reason"] != "",
      do: raise("Missing classification reason: #{name}")

    :ok
  end

  def inventory, do: @inventory
end
