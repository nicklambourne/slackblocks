defmodule Slackblocks.Internal.Builder do
  @moduledoc false
  alias Slackblocks.Internal.{Codec, Schema, Value}

  @payloads [
    Slackblocks.Message,
    Slackblocks.MessageResponse,
    Slackblocks.WebhookMessage,
    Slackblocks.Views.Modal,
    Slackblocks.Views.Home
  ]

  def url(value, options) do
    Value.capture(fn ->
      Value.options(options, [:team_id])
      team = Keyword.get(options, :team_id, "")
      team = if is_nil(team), do: "", else: team
      Codec.string!(team, ["team_id"])
      json = value |> payload() |> Jason.encode!()
      "https://app.slack.com/block-kit-builder/" <> escape(team) <> "#" <> escape(json)
    end)
  end

  defp payload(%{__struct__: module} = value) do
    cond do
      module in Schema.roles()["Block"] -> payload([value])
      module in @payloads -> Slackblocks.to_map!(value)
      true -> Value.fail(:type_mismatch, [], "expected a block or message/view payload")
    end
  end

  defp payload(values) when is_list(values) do
    Value.list!(values, ["blocks"])

    blocks =
      for {value, index} <- Enum.with_index(values) do
        unless is_struct(value) and value.__struct__ in Schema.roles()["Block"],
          do: Value.fail(:type_mismatch, ["blocks", index], "expected a block")

        Codec.wire(value, ["blocks", index], :complete)
      end

    %{"blocks" => blocks}
  end

  defp payload(value) when is_map(value), do: Codec.json(value, [])
  defp payload(_), do: Value.fail(:type_mismatch, [], "expected preview payload")
  defp escape(value), do: URI.encode(value, &URI.char_unreserved?/1)
end
