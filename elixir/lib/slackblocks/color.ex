defmodule Slackblocks.Color do
  @moduledoc """
  Attachment color conveniences. Returned binaries can be passed to `Slackblocks.Attachment`.

      iex> Slackblocks.Color.hex!("A0B1c2")
      "#A0B1c2"

      iex> Slackblocks.Color.rgb!(0, 128, 255)
      "#0080ff"
  """
  alias Slackblocks.Internal.Value

  @colors %{
    good: "good",
    warning: "warning",
    danger: "danger",
    red: "#ff0000",
    blue: "#0000ff",
    yellow: "#ffff00",
    green: "#00ff00",
    orange: "#ff8800",
    purple: "#8800ff",
    black: "#000000"
  }

  @doc "Resolves a semantic or named color atom: good, warning, danger, red, blue, yellow, green, orange, purple or black."
  @spec named(atom()) :: {:ok, binary()} | {:error, Slackblocks.ValidationError.t()}
  def named(name),
    do:
      Value.capture(fn ->
        Map.get(@colors, name) || Value.fail(:type_mismatch, ["color"], "unknown color atom")
      end)

  @doc "Like `named/1`, raising on invalid input."
  @spec named!(atom()) :: binary()
  def named!(name), do: Value.unwrap(named(name))

  @doc "Validates six hex digits, with an optional # prefix; preserves letter case."
  @spec hex(binary()) :: {:ok, binary()} | {:error, Slackblocks.ValidationError.t()}
  def hex(value) do
    Value.capture(fn ->
      unless is_binary(value) and String.valid?(value) and
               Regex.match?(~r/\A#?[0-9a-fA-F]{6}\z/, value),
             do: Value.fail(:type_mismatch, ["color"], "expected six hexadecimal digits")

      "#" <> String.trim_leading(value, "#")
    end)
  end

  @doc "Like `hex/1`, raising on invalid input."
  @spec hex!(binary()) :: binary()
  def hex!(value), do: Value.unwrap(hex(value))

  @doc "Converts three integer channels in 0..255 to a lowercase hexadecimal color."
  @spec rgb(integer(), integer(), integer()) ::
          {:ok, binary()} | {:error, Slackblocks.ValidationError.t()}
  def rgb(red, green, blue) do
    Value.capture(fn ->
      channels = [red, green, blue]

      channels |> Enum.with_index() |> Enum.each(&check_channel/1)

      "#" <>
        Enum.map_join(
          channels,
          &(Integer.to_string(&1, 16) |> String.downcase() |> String.pad_leading(2, "0"))
        )
    end)
  end

  defp check_channel({value, index}) do
    unless is_integer(value),
      do: Value.fail(:type_mismatch, ["color", index], "expected integer channel")

    unless value in 0..255,
      do: Value.fail(:out_of_range, ["color", index], "channel must be in 0..255")
  end

  @doc "Like `rgb/3`, raising on invalid input."
  @spec rgb!(integer(), integer(), integer()) :: binary()
  def rgb!(red, green, blue), do: Value.unwrap(rgb(red, green, blue))
end
