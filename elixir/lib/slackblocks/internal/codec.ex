defmodule Slackblocks.Internal.Codec do
  @moduledoc false
  alias Slackblocks.Internal.{Rules, Schema, Value}
  alias Slackblocks.RichText.Style
  import Value, only: [fail: 3]

  @selectors %{
    block: "Block",
    element: "Element",
    text: "Text",
    input_element: "InputElement",
    context_element: "ContextElement",
    context_actions_element: "ContextActionsElement",
    table_cell: "TableCell",
    data_table_cell: "DataTableCell",
    chart: "Chart",
    rich_text_block_element: "RichTextBlockElement",
    rich_text_section_element: "RichTextSectionElement"
  }

  def shape!(value, module, schema, path) do
    expected = [:__struct__ | Enum.map(schema.fields, & &1.key)] |> Enum.sort()

    unless is_map(value) and Map.get(value, :__struct__) == module and
             Enum.sort(Map.keys(value)) == expected do
      fail(:type_mismatch, path, "expected an intact #{inspect(module)} struct")
    end
  end

  def wire(%{__struct__: module} = value, path, context) do
    schema = Schema.model(module) || fail(:type_mismatch, path, "unknown value struct")
    shape!(value, module, schema, path)

    base =
      if schema.wire == "" or (schema.name == "TaskCardBlock" and context == :plan),
        do: %{},
        else: %{"type" => schema.wire}

    wire =
      Enum.reduce(schema.fields, base, fn field, acc ->
        put_field(acc, field, Map.fetch!(value, field.key), path, schema.name)
      end)

    if schema.name in ["PlainText", "MarkdownText"],
      do: limits(wire["text"], "text", path ++ ["text"])

    Rules.validate(schema.name, wire, path, context)
    wire
  end

  def wire(_value, path, _context),
    do: fail(:type_mismatch, path, "expected a Slackblocks struct")

  defp put_field(acc, field, nil, path, _name) do
    if field.required,
      do: fail(:missing_required, path ++ [field.wire], "required field is missing")

    acc
  end

  defp put_field(acc, field, value, path, name) do
    p = path ++ [field.wire]
    context = if name == "PlanBlock" and field.key == :tasks, do: :plan, else: :complete
    rendered = field_wire(field, value, p, context)
    limits(rendered, field.limits, p)
    Map.put(acc, field.wire, rendered)
  end

  defp field_wire(%{kind: kind} = field, value, path, context)
       when kind in [:list, :rows, :textList, :stringList] do
    Value.list!(value, path)

    inner = %{
      field
      | kind: %{list: :object, rows: :list, textList: :text, stringList: :string}[kind]
    }

    value
    |> Enum.with_index()
    |> Enum.map(fn {child, i} -> field_wire(inner, child, path ++ [i], context) end)
  end

  defp field_wire(%{kind: :string}, value, path, _), do: string!(value, path)
  defp field_wire(%{kind: :boolean}, value, _path, _) when is_boolean(value), do: value

  defp field_wire(%{kind: kind}, value, _path, _)
       when kind in [:int, :long] and is_integer(value),
       do: value

  defp field_wire(%{kind: kind}, value, _path, _)
       when kind in [:number, :double] and is_number(value),
       do: value

  defp field_wire(%{kind: :enum, type: type}, value, path, _) do
    Map.get(Schema.enums()[type], value) || fail(:type_mismatch, path, "unsupported #{type} atom")
  end

  defp field_wire(%{kind: kind, type: type}, %{__struct__: module} = value, path, context)
       when kind in [:text, :object] do
    unless accepts?(type, module), do: fail(:type_mismatch, path, "expected #{type}")
    wire(value, path, context)
  end

  defp field_wire(%{kind: :map}, value, path, _) when is_map(value) and not is_struct(value),
    do: json(value, path)

  defp field_wire(%{kind: :style, flags: flags}, value, path, _) do
    Style.wire(value, flags, path)
  end

  defp field_wire(_field, _value, path, _),
    do: fail(:type_mismatch, path, "unexpected field type")

  defp accepts?(type, module) do
    case Schema.model(module) do
      nil -> false
      schema -> schema.name == type or type in schema.roles
    end
  end

  def string!(value, path) do
    unless is_binary(value) and String.valid?(value),
      do: fail(:type_mismatch, path, "expected a UTF-8 binary")

    value
  end

  def length_codepoints(value), do: value |> String.to_charlist() |> length()

  def limits(_value, nil, _path), do: :ok

  def limits(value, prefix, path) do
    size = measured_size(value)

    suffixes =
      if is_list(value), do: ["min_items", "max_items"], else: ["min_length", "max_length"]

    Enum.each(suffixes, &check_bound(size, prefix, &1, path, :length_exceeded))

    if is_number(value),
      do:
        Enum.each(
          ["min", "max", "exclusive_min", "exclusive_max"],
          &check_bound(value, prefix, &1, path, :out_of_range)
        )

    :ok
  end

  defp measured_size(value) when is_binary(value), do: length_codepoints(value)
  defp measured_size(value) when is_list(value), do: length(value)
  defp measured_size(%{"text" => text}) when is_binary(text), do: length_codepoints(text)
  defp measured_size(_), do: nil
  defp check_bound(nil, _prefix, _suffix, _path, _category), do: :ok

  defp check_bound(value, prefix, suffix, path, category) do
    bound = Schema.limits()[prefix <> "." <> suffix]

    if bound != nil and outside?(value, bound, suffix),
      do: fail(category, path, "outside #{prefix} bounds")
  end

  defp outside?(value, bound, suffix) when suffix in ["min", "min_length", "min_items"],
    do: value < bound

  defp outside?(value, bound, suffix) when suffix in ["max", "max_length", "max_items"],
    do: value > bound

  defp outside?(value, bound, "exclusive_min"), do: value <= bound
  defp outside?(value, bound, "exclusive_max"), do: value >= bound

  # Opaque JSON is never traversed as Block Kit. Ordered objects preserve duplicate evidence.
  def json(value, path, decimals \\ false)

  def json(%Jason.OrderedObject{values: pairs}, path, true) do
    Enum.reduce(pairs, %{}, fn {key, value}, acc ->
      if Map.has_key?(acc, key), do: fail(:invalid_usage, path ++ [key], "duplicate JSON key")
      Map.put(acc, key, json(value, path ++ [key], true))
    end)
  end

  def json(%Decimal{} = value, path, true), do: decimal(value, path)
  def json(value, _path, _) when is_nil(value) or is_boolean(value) or is_number(value), do: value
  def json(value, path, _) when is_binary(value), do: string!(value, path)

  def json(value, path, decimals) when is_list(value) do
    Value.list!(value, path)
    value |> Enum.with_index() |> Enum.map(fn {v, i} -> json(v, path ++ [i], decimals) end)
  end

  def json(value, path, decimals) when is_map(value) and not is_struct(value) do
    value
    |> Enum.sort_by(fn {key, _} -> key end)
    |> Map.new(fn {key, child} ->
      string!(key, path)
      {key, json(child, path ++ [key], decimals)}
    end)
  end

  def json(_value, path, _),
    do: fail(:type_mismatch, path, "expected a JSON value with string object keys")

  # OTP 26 can merge literal +0.0/-0.0 branches; parse the sign explicitly.
  defp decimal(%Decimal{coef: 0, sign: sign}, _path),
    do: String.to_float(if(sign == -1, do: "-0.0", else: "0.0"))

  defp decimal(%Decimal{coef: coef, exp: exp} = value, path) when is_integer(coef) do
    magnitude = byte_size(Integer.to_string(coef)) + exp

    if magnitude < -323 or magnitude > 309,
      do: fail(:out_of_range, path, "number exceeds float range")

    # Decimal.to_float/1 rejects representable subnormal floats. BEAM parses the
    # compact decimal rendering; the exact comparison below still rejects rounding.
    candidate =
      case value |> Decimal.to_string(:scientific) |> Float.parse() do
        {number, ""} -> number
        :error -> fail(:out_of_range, path, "number exceeds float range")
      end

    if candidate == 0.0 or Decimal.compare(value, Decimal.from_float(candidate)) != :eq,
      do: fail(:out_of_range, path, "number would lose decimal precision")

    candidate
  end

  defp decimal(_value, path), do: fail(:out_of_range, path, "nonfinite number")

  def parse(selector, input, path) do
    unless is_map(input) and not is_struct(input),
      do: fail(:type_mismatch, path, "expected wire object")

    Enum.each(Map.keys(input), &string!(&1, path))
    module = select(selector, input, path)
    schema = Schema.model(module)
    allowed = Enum.map(schema.fields, & &1.wire) ++ if(schema.wire == "", do: [], else: ["type"])

    case Enum.find(Enum.sort(Map.keys(input)), &(&1 not in allowed)) do
      nil -> :ok
      key -> fail(:invalid_usage, path ++ [key], "unknown wire field")
    end

    if schema.wire != "" and Map.get(input, "type") != schema.wire,
      do: fail(:type_mismatch, path ++ ["type"], "incorrect discriminator")

    fields =
      Enum.map(schema.fields, fn f ->
        {f.key, parsed_value(f, Map.fetch(input, f.wire), path ++ [f.wire])}
      end)

    struct(module, fields)
  end

  defp parsed_value(%{required: true, has_default: true} = field, :error, _path),
    do: field.default

  defp parsed_value(_field, :error, _path), do: nil
  defp parsed_value(_field, {:ok, nil}, _path), do: nil
  defp parsed_value(field, {:ok, value}, path), do: parse_field(field, value, path)

  defp select(selector, input, path) do
    cond do
      Schema.model(selector) != nil ->
        selector

      Map.has_key?(@selectors, selector) ->
        members = Schema.roles()[@selectors[selector]]

        Enum.find(members, &(Schema.model(&1).wire == input["type"])) ||
          fail(:type_mismatch, path ++ ["type"], "unknown role discriminator")

      is_binary(selector) ->
        Enum.find(
          Map.keys(Schema.models()),
          &(Schema.model(&1).name == selector or
              (selector in Schema.model(&1).roles and Schema.model(&1).wire == input["type"]))
        ) || fail(:type_mismatch, path, "unknown child type")

      true ->
        fail(:invalid_usage, path, "unsupported root selector")
    end
  end

  defp parse_field(%{kind: kind} = f, values, path)
       when kind in [:list, :rows, :textList, :stringList] do
    Value.list!(values, path)
    inner = %{f | kind: %{list: :object, rows: :list, textList: :text, stringList: :string}[kind]}
    values |> Enum.with_index() |> Enum.map(fn {v, i} -> parse_field(inner, v, path ++ [i]) end)
  end

  defp parse_field(%{kind: kind, type: "TaskCardBlock"}, value, path)
       when kind in [:object, :text] do
    # TaskCardBlock occurs as a concrete child only in Plan.tasks.
    unless is_map(value) and not Map.has_key?(value, "type"),
      do: fail(:invalid_usage, path, "plan tasks must omit type")

    parse("TaskCardBlock", Map.put(value, "type", "task_card"), path)
  end

  defp parse_field(%{kind: kind, type: type}, value, path) when kind in [:object, :text],
    do: parse(type, value, path)

  defp parse_field(%{kind: :enum, type: type}, value, path) do
    case Enum.find(Schema.enums()[type], fn {_key, wire} -> wire === value end) do
      {key, _} -> key
      nil -> fail(:type_mismatch, path, "unknown #{type} wire value")
    end
  end

  defp parse_field(%{kind: :style}, value, path),
    do: Style.parse(value, path)

  defp parse_field(_field, value, _path), do: value
end
