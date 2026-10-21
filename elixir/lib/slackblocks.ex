defmodule Slackblocks do
  @moduledoc """
  Constructs and validates Slack Block Kit values without sending HTTP requests.

  Use model constructors for text conveniences and `to_map/1` for a complete
  validated Slack payload. Every output boundary rechecks edited structs.

  ## Examples

      iex> Slackblocks.Blocks.Divider.new!() |> Slackblocks.to_map!()
      %{"type" => "divider"}

      iex> {:error, error} = Slackblocks.Blocks.Section.new([])
      iex> error.category
      :missing_required
  """
  alias Slackblocks.Internal.{Builder, Codec, Value}
  @version Mix.Project.config()[:version]
  @doc "The version of the installed package."
  @spec version() :: String.t()
  def version, do: @version
  @doc "The shared Slack Block Kit contract version implemented by this package."
  @spec spec_version() :: String.t()
  def spec_version, do: "1.3.0"

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

  @doc """
  Decodes JSON without atomizing keys; rejects duplicate keys and lossy numbers.

  An application that compiled Jason before installing Decimal must run
  `mix deps.compile jason --force` in each build environment, then rebuild its
  release. Missing compile-time Decimal support returns an actionable
  `ArgumentError` in the tagged result; `decode!/2` raises it.
  """
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
  rescue
    error in ArgumentError ->
      if error.message == "decimal library not found, :decimals option not available" do
        {:error,
         ArgumentError.exception(
           "Jason was compiled without Decimal support. Run `mix deps.compile jason --force` " <>
             "in this MIX_ENV, then rebuild the application/release before using Slackblocks.decode/2."
         )}
      else
        reraise error, __STACKTRACE__
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

  @doc """
  Returns a Block Kit Builder URL without making network requests.

  Accepts a block, list of blocks, message/view payload, or explicit string-keyed
  JSON map. Blocks are wrapped in `blocks`; maps are validated as JSON data only.
  Optional `team_id` is encoded as one URL path segment. The fragment preserves
  list order and Unicode; JSON object key order is unspecified.
  """
  @spec block_kit_builder_url(struct() | list() | map(), keyword()) ::
          {:ok, binary()} | {:error, Slackblocks.ValidationError.t()}
  def block_kit_builder_url(value, options \\ []),
    do: Builder.url(value, options)

  @doc "Like `block_kit_builder_url/2`, raising on invalid input."
  @spec block_kit_builder_url!(struct() | list() | map(), keyword()) :: binary()
  def block_kit_builder_url!(value, options \\ []),
    do: Value.unwrap(block_kit_builder_url(value, options))
end
