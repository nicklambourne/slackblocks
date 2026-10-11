defmodule Slackblocks.RichText.Style do
  @moduledoc "Optional rich-text flags. Unsupported flags are rejected by their receiving field."
  alias Slackblocks.Internal.Value
  defstruct [:bold, :italic, :strike, :code, :highlight, :client_highlight, :unlink]

  @type t :: %__MODULE__{
          bold: boolean() | nil,
          italic: boolean() | nil,
          strike: boolean() | nil,
          code: boolean() | nil,
          highlight: boolean() | nil,
          client_highlight: boolean() | nil,
          unlink: boolean() | nil
        }
  @flags ~w(bold italic strike code highlight client_highlight unlink)a

  @doc "Constructs boolean style flags; nil omits a flag and false is preserved."
  @spec new(keyword()) :: {:ok, t()} | {:error, Slackblocks.ValidationError.t()}
  def new(options \\ []) do
    Value.capture(fn ->
      unless is_list(options) and Keyword.keyword?(options),
        do: Value.fail(:type_mismatch, [], "expected keyword options")

      keys = Keyword.keys(options)

      unless Enum.all?(keys, &(&1 in @flags)) and length(keys) == length(Enum.uniq(keys)),
        do: Value.fail(:invalid_usage, [], "unknown or duplicate style flag")

      value = struct(__MODULE__, options)
      wire(value, Enum.map(@flags, &Atom.to_string/1), [])
      value
    end)
  end

  @doc "Like `new/1`, raising on failure."
  @spec new!(keyword()) :: t()
  def new!(options \\ []), do: Value.unwrap(new(options))

  @doc false
  def wire(%__MODULE__{} = value, allowed, path) do
    unless Enum.sort(Map.keys(value)) == Enum.sort([:__struct__ | @flags]),
      do: Value.fail(:type_mismatch, path, "invalid style struct shape")

    Enum.reduce(@flags, %{}, fn flag, acc ->
      item = Map.fetch!(value, flag)
      key = Atom.to_string(flag)

      cond do
        is_nil(item) ->
          acc

        key not in allowed or not is_boolean(item) ->
          Value.fail(:type_mismatch, path ++ [key], "unsupported style flag")

        true ->
          Map.put(acc, key, item)
      end
    end)
  end

  def wire(_value, _allowed, path),
    do: Value.fail(:type_mismatch, path, "expected RichText.Style")

  @doc false
  def parse(value, path) when is_map(value) and not is_struct(value) do
    known = Map.new(@flags, &{Atom.to_string(&1), &1})

    options =
      Enum.map(value, fn {key, v} ->
        flag = Map.get(known, key) || Value.fail(:invalid_usage, path, "unknown style key")
        {flag, v}
      end)

    Value.unwrap(new(options))
  end

  def parse(_value, path), do: Value.fail(:type_mismatch, path, "expected style object")
end

defimpl Jason.Encoder, for: Slackblocks.RichText.Style do
  def encode(value, options) do
    flags = ~w(bold italic strike code highlight client_highlight unlink)
    Jason.Encode.map(Slackblocks.RichText.Style.wire(value, flags, []), options)
  end
end
