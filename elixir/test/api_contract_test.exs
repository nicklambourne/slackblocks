defmodule Slackblocks.ApiContractTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Internal.Schema
  alias Slackblocks.Test.{ApiAudit, Samples}
  alias Slackblocks.ValidationError
  @root Path.expand("../..", __DIR__)
  @contract Jason.decode!(File.read!(Path.join(@root, "spec/model.json")))
  @names Jason.decode!(File.read!(Path.join(@root, "elixir/design/parity.json")))["models"]

  test "compiled production modules and every public function have explicit dispositions" do
    assert ApiAudit.check!() == :ok

    for {name, entry} <- ApiAudit.inventory()["modules"],
        entry["kind"] in ~w(model helper boundary) do
      module = String.to_existing_atom("Elixir." <> name)
      assert {:docs_v1, _, _, _, %{"en" => _}, _, docs} = Code.fetch_docs(module)

      functions =
        module.__info__(:functions)
        |> Enum.reject(fn {name, _} -> String.starts_with?(Atom.to_string(name), "__") end)

      for {function, arity} <- functions do
        assert Enum.any?(docs, fn {{kind, n, a}, _, _, doc, metadata} ->
                 kind == :function and n == function and
                   arity in (a - Map.get(metadata, :defaults, 0))..a and doc != :none
               end),
               "missing docs for #{name}.#{function}/#{arity}"
      end
    end
  end

  test "every model producer maps to a live capability exercised by its native fixtures" do
    capabilities =
      Jason.decode!(File.read!(Path.join(@root, "spec/coverage.json")))["capabilities"]

    fixtures = Samples.fixture_modules()

    entries =
      ApiAudit.inventory()["modules"]
      |> Enum.filter(fn {_, entry} -> entry["kind"] == "model" end)

    assert MapSet.new(Enum.map(entries, fn {_, entry} -> entry["capability"] end)) ==
             MapSet.new(Map.keys(capabilities))

    for {name, entry} <- entries do
      assert Enum.any?(
               Map.fetch!(capabilities, entry["capability"]),
               &MapSet.member?(Map.fetch!(fixtures, &1), name)
             ),
             "no independent native evidence for #{name}"
    end
  end

  test "all model fields, defaults, requiredness and type declarations match the contract independently" do
    samples = Samples.all()

    assert MapSet.new(Map.keys(samples)) ==
             MapSet.new(Enum.map(Map.values(@names), &String.to_existing_atom("Elixir." <> &1)))

    for model <- @contract["types"] do
      module = String.to_existing_atom("Elixir." <> @names[model["name"]])
      value = Map.fetch!(samples, module)
      fields = model["fields"]
      native_fields = Enum.map(fields, &String.to_existing_atom(&1["wire"]))
      assert Map.keys(Map.from_struct(value)) |> Enum.sort() == Enum.sort(native_fields)
      defaults = struct(module)
      assert {:ok, types} = Code.Typespec.fetch_types(module)
      assert Enum.any?(types, fn {kind, {name, _, _}} -> kind == :type and name == :t end)

      for field <- fields do
        key = String.to_existing_atom(field["wire"])
        default = model["defaults"][field["wire"]]

        default =
          if default != nil and field["kind"] == "enum",
            do: String.to_existing_atom(default),
            else: default

        assert Map.fetch!(defaults, key) === default

        options_without_key = value |> Map.from_struct() |> Map.delete(key) |> Map.to_list()

        if field["required"] == true and not Map.has_key?(model["defaults"], field["wire"]) do
          assert_raise ArgumentError, fn -> struct!(module, options_without_key) end
        else
          assert is_struct(struct!(module, options_without_key), module)
        end

        assert {:error, %ValidationError{category: :type_mismatch, path: [wire | _]}} =
                 module.change(value, [{key, self()}])

        assert wire == field["wire"]

        if field["required"] == true do
          assert {:error, %ValidationError{category: :missing_required, path: [^wire]}} =
                   module.change(value, [{key, nil}])
        end

        if field["kind"] == "text" and field["coerce"] in ["plain_text", "mrkdwn"] do
          updated = module.change!(value, [{key, "native text"}])
          assert Map.fetch!(updated, key).text == "native text"
          assert Schema.model(Map.fetch!(updated, key).__struct__).wire == field["coerce"]
        end
      end

      assert {:ok, ^value} = module.change(value, [])
      assert module.new!(Map.to_list(Map.from_struct(value))) == value
      if function_exported?(module, :new, 0), do: assert({:ok, _} = module.new())
    end
  end

  test "receiving roles and enum types are exact and every member is exercised natively" do
    roles =
      Map.new(@contract["interfaces"], fn role ->
        members =
          @contract["types"]
          |> Enum.filter(fn model ->
            role["name"] in model["implements"] or
              (model["package"] in ["block", "element"] and
                 String.capitalize(model["package"]) == role["name"])
          end)
          |> Enum.map(&String.to_existing_atom("Elixir." <> @names[&1["name"]]))

        {role["name"], members}
      end)

    assert Map.new(Schema.roles(), fn {name, members} -> {name, Enum.sort(members)} end) ==
             Map.new(roles, fn {name, members} -> {name, Enum.sort(members)} end)

    enums =
      Map.new(@contract["enums"], fn enum ->
        {enum["name"],
         Map.new(enum["constants"], &{String.to_existing_atom(&1["wire"]), &1["wire"]})}
      end)

    assert Schema.enums() == enums
    all = Samples.all()

    for {role, members} <- roles, module <- members do
      selector = role |> Macro.underscore() |> String.to_existing_atom()
      value = all[module]
      assert Slackblocks.from_map!(selector, Slackblocks.to_map!(value)).__struct__ == module
    end
  end

  test "declared optional defaults are absent on wire ingress and can be explicitly cleared" do
    for model <- @contract["types"], map_size(model["defaults"]) > 0 do
      module = String.to_existing_atom("Elixir." <> @names[model["name"]])
      value = Map.fetch!(Samples.all(), module)
      wire = Slackblocks.to_map!(value)

      for {field, _default} <- model["defaults"] do
        f = Enum.find(model["fields"], &(&1["wire"] == field))
        parsed = Slackblocks.from_map!(module, Map.delete(wire, field))

        if f["required"] != true do
          assert Map.fetch!(parsed, String.to_existing_atom(field)) == nil
          cleared = module.change!(value, [{String.to_existing_atom(field), nil}])

          assert Slackblocks.decode!(module, Slackblocks.encode!(cleared))
                 |> Slackblocks.to_map!() == Slackblocks.to_map!(cleared)
        else
          assert Map.fetch!(parsed, String.to_existing_atom(field)) ==
                   Map.fetch!(struct(module), String.to_existing_atom(field))
        end
      end
    end
  end
end
