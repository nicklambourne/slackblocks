defmodule Slackblocks do
  @moduledoc """
  Constructs and validates Slack Block Kit values without sending HTTP requests.

  Use model constructors for text conveniences and `to_map/1` for a complete
  validated Slack payload. Every output boundary rechecks edited structs.
  """
  alias Slackblocks.Internal.{Codec, Value}
  @version Mix.Project.config()[:version]
  @spec version() :: String.t()
  def version, do: @version
  @spec spec_version() :: String.t()
  def spec_version, do: "1.2.0"

  @doc "Checks the complete value, including receiving context and payload totals."
  @spec validate(struct()) :: :ok | {:error, Slackblocks.ValidationError.t()}
  def validate(value) do
    case to_map(value) do
      {:ok, _} -> :ok
      error -> error
    end
  end

  @doc "Returns a validated wire map with string keys and omitted optional nil fields."
  @spec to_map(struct()) :: {:ok, map()} | {:error, Slackblocks.ValidationError.t()}
  def to_map(value), do: Value.capture(fn -> Codec.wire(value, [], :complete) end)
  @doc "Like `to_map/1`, raising on invalid values."
  @spec to_map!(struct()) :: map()
  def to_map!(value), do: Value.unwrap(to_map(value))

  @doc "Validates before encoding. Direct Jason encoding may raise ValidationError."
  @spec encode(struct()) :: {:ok, binary()} | {:error, Exception.t()}
  def encode(value) do
    with {:ok, wire} <- to_map(value), do: Jason.encode(wire)
  end

  @doc "Like `encode/1`, raising on failure."
  @spec encode!(struct()) :: binary()
  def encode!(value), do: Value.unwrap(encode(value))

  @doc "Reconstructs a known model or role from wire maps; optional defaults stay omitted."
  @spec from_map(module() | atom(), map()) ::
          {:ok, struct()} | {:error, Slackblocks.ValidationError.t()}
  def from_map(selector, wire) do
    Value.capture(fn ->
      unless is_atom(selector),
        do: Value.fail(:invalid_usage, [], "expected a model module or role atom")

      value = Codec.parse(selector, wire, [])
      Codec.wire(value, [], :complete)
      value
    end)
  end

  @doc "Like `from_map/2`, raising on failure."
  @spec from_map!(module() | atom(), map()) :: struct()
  def from_map!(selector, wire), do: Value.unwrap(from_map(selector, wire))

  @doc "Decodes JSON without atomizing keys; rejects duplicate keys and lossy numbers."
  @spec decode(module() | atom(), binary()) :: {:ok, struct()} | {:error, Exception.t()}
  def decode(selector, json) when is_binary(json) do
    with {:ok, raw} <-
           Jason.decode(json,
             keys: :strings,
             strings: :copy,
             objects: :ordered_objects,
             floats: :decimals
           ),
         {:ok, wire} <- Value.capture(fn -> Codec.json(raw, [], true) end) do
      from_map(selector, wire)
    end
  end

  def decode(_selector, _json),
    do:
      {:error,
       %Slackblocks.ValidationError{
         category: :type_mismatch,
         path: [],
         message: "expected JSON binary"
       }}

  @doc "Like `decode/2`, raising on failure."
  @spec decode!(module() | atom(), binary()) :: struct()
  def decode!(selector, json), do: Value.unwrap(decode(selector, json))
end
