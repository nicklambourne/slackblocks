defmodule Slackblocks.MessageChartsTest do
  use ExUnit.Case, async: true
  alias Slackblocks.{Attachment, Message, MessageResponse, ValidationError, WebhookMessage}
  alias Slackblocks.Blocks.{Container, DataVisualization}
  alias Slackblocks.Objects.{ChartSegment, PieChart}
  alias Slackblocks.Views.Home

  test "all message constructors accept two charts across blocks, attachments and containers" do
    [first, second] = charts(2)

    for module <- [Message, MessageResponse, WebhookMessage],
        fields <- [
          [blocks: [first, second]],
          [blocks: [first], attachments: [Attachment.new!(blocks: [second])]],
          [attachments: [Attachment.new!(blocks: [first, second])]],
          [blocks: [Container.new!(title: "Charts", child_blocks: [first, second])]]
        ] do
      value = module.new!(options(module) ++ fields)
      assert Slackblocks.decode!(module, Slackblocks.encode!(value)) == value
    end
  end

  test "all message constructors and edits reject three charts across every collection" do
    [first, second, third] = charts(3)

    for module <- [Message, MessageResponse, WebhookMessage],
        invalid <- [
          [blocks: [first, second, third]],
          [blocks: [first, second], attachments: [Attachment.new!(blocks: [third])]],
          [blocks: [first], attachments: [Attachment.new!(blocks: [second, third])]],
          [attachments: [Attachment.new!(blocks: [first, second, third])]],
          [blocks: [Container.new!(title: "Charts", child_blocks: [first, second, third])]],
          [
            blocks: [Container.new!(title: "Charts", child_blocks: [first, second])],
            attachments: [Attachment.new!(blocks: [third])]
          ]
        ] do
      assert {:error, %ValidationError{category: :length_exceeded}} =
               module.new(options(module) ++ invalid)

      assert {:error, %ValidationError{category: :length_exceeded}} =
               module.change(module.new!(options(module)), invalid)
    end
  end

  test "parsed input and forged output preserve the same aggregate rule" do
    for module <- [Message, MessageResponse, WebhookMessage] do
      valid = module.new!(options(module) ++ [blocks: charts(2)])

      wire =
        Slackblocks.to_map!(valid)
        |> Map.put("blocks", Enum.map(charts(3), &Slackblocks.to_map!/1))

      for result <- [
            Slackblocks.from_map(module, wire),
            Slackblocks.decode(module, Jason.encode!(wire)),
            Slackblocks.to_map(%{valid | blocks: charts(3)})
          ] do
        assert {:error, %ValidationError{category: :length_exceeded}} = result
      end
    end
  end

  test "Home views, standalone attachments and opaque metadata are not messages" do
    assert Slackblocks.validate(Home.new!(blocks: charts(3))) == :ok
    assert Slackblocks.validate(Attachment.new!(blocks: charts(3))) == :ok

    metadata = %{
      "event_type" => "charts",
      "event_payload" => %{"blocks" => Enum.map(charts(3), &Slackblocks.to_map!/1)}
    }

    for module <- [Message, WebhookMessage] do
      value = module.new!(options(module) ++ [blocks: charts(2), metadata: metadata])
      assert Slackblocks.to_map!(value)["metadata"] == metadata
    end
  end

  defp options(Message), do: [channel: "C123"]
  defp options(_), do: []

  defp charts(count) do
    for index <- 1..count do
      DataVisualization.new!(
        block_id: "chart_#{index}",
        title: "Chart",
        chart: PieChart.new!(segments: [ChartSegment.new!(label: "A", value: 1)])
      )
    end
  end
end
