defmodule Slackblocks.ConformanceTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Internal.Schema
  alias Slackblocks.Test.{InvalidConstructions, ValidConstructions}
  alias Slackblocks.ValidationError
  @spec_root Path.expand("../../spec", __DIR__)

  for {id, _} <- ValidConstructions.all() do
    test "valid native construction: #{id}" do
      id = unquote(id)
      build = Map.fetch!(Map.new(ValidConstructions.all()), id)
      value = build.()
      expected = File.read!(Path.join(@spec_root, "fixtures/valid/#{id}.json")) |> Jason.decode!()
      actual = value |> Slackblocks.encode!() |> Jason.decode!()
      assert actual === expected
      assert Slackblocks.from_map!(value.__struct__, actual) |> Slackblocks.to_map!() === actual

      assert Slackblocks.decode!(value.__struct__, Slackblocks.encode!(value))
             |> Slackblocks.to_map!() === actual

      assert value.__struct__.change!(value, []) === value
    end
  end

  @invalid_manifest File.read!(Path.join(@spec_root, "fixtures/invalid/manifest.json"))
                    |> Jason.decode!()
  @categories Map.new(@invalid_manifest["cases"], &{&1["id"], &1["category"]})
  for {id, _} <- InvalidConstructions.all() do
    test "invalid native attempt: #{id}" do
      id = unquote(id)
      build = Map.fetch!(Map.new(InvalidConstructions.all()), id)
      error = assert_raise ValidationError, fn -> build.() |> Slackblocks.encode!() end

      assert error.category |> Atom.to_string() |> String.replace("_", "-") ==
               Map.fetch!(@categories, id)

      assert is_list(error.path)
    end
  end

  test "registries are exact, skip list is empty, and every scalar limit is covered" do
    manifest = read("manifest.json")
    valid_ids = Enum.map(manifest["fixtures"], & &1["id"])
    invalid_ids = Enum.map(@invalid_manifest["cases"], & &1["id"])
    assert_ids(ValidConstructions.all(), valid_ids)
    assert_ids(InvalidConstructions.all(), invalid_ids)
    assert File.read!(Path.expand("../conformance/skiplist.txt", __DIR__)) |> String.trim() == ""
    assert Slackblocks.spec_version() == manifest["spec_version"]
    assert Schema.limits() == flatten(read("limits.json"))
    assert Schema.vocabulary() == Map.delete(read("vocabulary.json"), "$comment")

    constraints =
      @invalid_manifest["cases"]
      |> Enum.map(& &1["constraint"])
      |> Enum.reject(&is_nil/1)
      |> MapSet.new()

    assert MapSet.subset?(MapSet.new(Map.keys(Schema.limits())), constraints)

    for {_capability, fixtures} <- read("coverage.json")["capabilities"], id <- fixtures do
      assert id in valid_ids
    end
  end

  defp assert_ids(registry, expected) do
    actual = Enum.map(registry, &elem(&1, 0))
    assert length(actual) == length(Enum.uniq(actual))
    assert Enum.sort(actual) == Enum.sort(expected)
  end

  defp read(file), do: File.read!(Path.join(@spec_root, file)) |> Jason.decode!()

  defp flatten(map, prefix \\ "") do
    Enum.reduce(map, %{}, fn {key, value}, acc ->
      path = if prefix == "", do: key, else: prefix <> "." <> key
      if is_map(value), do: Map.merge(acc, flatten(value, path)), else: Map.put(acc, path, value)
    end)
  end
end
