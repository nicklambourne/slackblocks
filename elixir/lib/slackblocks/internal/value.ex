defmodule Slackblocks.Internal.Value do
  @moduledoc false
  alias Slackblocks.Internal.{Codec, Schema}
  alias Slackblocks.ValidationError

  def capture(fun) do
    {:ok, fun.()}
  rescue
    error in ValidationError -> {:error, error}
  end

  def list!([], _path), do: :ok
  def list!([_ | tail], path), do: list!(tail, path)
  def list!(_, path), do: fail(:type_mismatch, path, "expected a proper list")

  def unwrap({:ok, value}), do: value
  def unwrap({:error, error}), do: raise(error)

  def fail(category, path, message),
    do: raise(ValidationError, category: category, path: path, message: message)

  def new(module, options) do
    capture(fn ->
      schema = Schema.model(module)
      values = keywords(options, schema.fields)
      value = struct(module, normalize(values, schema.fields))
      Codec.wire(value, [], :local)
      value
    end)
  end

  def change(module, value, options) do
    capture(fn ->
      schema = Schema.model(module)
      Codec.shape!(value, module, schema, [])
      values = keywords(options, schema.fields)
      result = struct(value, normalize(values, schema.fields))
      Codec.wire(result, [], :local)
      result
    end)
  end

  def options(options, allowed), do: keywords(options, Enum.map(allowed, &%{key: &1}))

  defp keywords(options, fields) do
    list!(options, [])

    unless Keyword.keyword?(options),
      do: fail(:type_mismatch, [], "expected keyword options")

    keys = Keyword.keys(options)
    if length(keys) != length(Enum.uniq(keys)), do: fail(:invalid_usage, [], "duplicate keyword")
    allowed = Enum.map(fields, & &1.key)

    case Enum.find(keys, &(&1 not in allowed)) do
      nil -> options
      key -> fail(:invalid_usage, [Atom.to_string(key)], "unknown keyword")
    end
  end

  defp normalize(values, fields) do
    Enum.map(values, fn {key, value} ->
      field = Enum.find(fields, &(&1.key == key))
      {key, coerce(value, field)}
    end)
  end

  defp coerce(value, %{kind: :text, coerce: kind})
       when is_binary(value) and kind in ["plain_text", "mrkdwn"] do
    module =
      if kind == "plain_text",
        do: Slackblocks.Objects.PlainText,
        else: Slackblocks.Objects.MarkdownText

    struct(module, text: value)
  end

  defp coerce(values, %{kind: :textList} = field) when is_list(values),
    do:
      (
        list!(values, [])
        Enum.map(values, &coerce(&1, %{field | kind: :text}))
      )

  defp coerce(value, _field), do: value
end
