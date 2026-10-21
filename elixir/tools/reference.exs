# Export native compiled documentation and typespecs, independent of generator output.
Application.load(:slackblocks)
{:ok, modules} = :application.get_key(:slackblocks, :modules)
inventory = Jason.decode!(File.read!("design/public_api.json"))["modules"]

english = fn
  %{"en" => text} -> text
  _ -> ""
end

reference =
  modules
  |> Enum.reject(fn module ->
    name = inspect(module)

    String.starts_with?(name, "Jason.Encoder.") or
      String.starts_with?(name, "Slackblocks.Internal.")
  end)
  |> Enum.sort()
  |> Enum.map(fn module ->
    name = inspect(module)
    classification = Map.fetch!(inventory, name)
    {:docs_v1, _, :elixir, "text/markdown", module_doc, _, entries} = Code.fetch_docs(module)
    {:ok, specs} = Code.Typespec.fetch_specs(module)
    {:ok, types} = Code.Typespec.fetch_types(module)

    functions =
      for {{:function, function, arity}, _, signatures, doc, meta} <- entries,
          doc != :hidden do
        specification =
          for {{^function, ^arity}, definitions} <- specs,
              definition <- definitions do
            "@spec " <> Macro.to_string(Code.Typespec.spec_to_quoted(function, definition))
          end

        %{
          name: "#{function}/#{arity}",
          signatures: signatures,
          spec: specification,
          doc: english.(doc),
          arities: Enum.to_list((arity - Map.get(meta, :defaults, 0))..arity)
        }
      end

    types =
      for {kind, type} <- types, kind in [:type, :opaque] do
        "@#{kind} " <> Macro.to_string(Code.Typespec.type_to_quoted(type))
      end

    %{
      module: name,
      kind: classification["kind"],
      doc: english.(module_doc),
      functions: functions,
      types: types,
      exports: Enum.map(module.__info__(:functions), fn {fun, arity} -> "#{fun}/#{arity}" end)
    }
  end)

IO.puts(Jason.encode!(%{version: Slackblocks.version(), modules: reference}))
