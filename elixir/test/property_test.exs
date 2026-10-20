defmodule Slackblocks.PropertyTest do
  use ExUnit.Case, async: true
  use ExUnitProperties
  alias Slackblocks.Blocks.{Container, Divider, Section}
  alias Slackblocks.Message
  alias Slackblocks.Objects.RawNumber

  property "recursive JSON metadata preserves native kinds, Unicode and exact integers" do
    check all(value <- json(), max_runs: 100) do
      original = Message.new!(channel: "C123", metadata: %{"event_payload" => value})
      parsed = Slackblocks.decode!(Message, Slackblocks.encode!(original))
      assert parsed.metadata === original.metadata
      assert Jason.encode!(original) |> Jason.decode!() === Slackblocks.to_map!(original)
    end
  end

  property "Unicode code point limits preserve text without normalization" do
    check all(text <- string(:utf8, min_length: 1, max_length: 80), max_runs: 100) do
      section = Section.new!(text: text)
      assert Slackblocks.decode!(:block, Slackblocks.encode!(section)).text.text == text
      assert length(String.to_charlist(text)) <= 80
      assert Section.change!(section, block_id: "edited").text == section.text
    end
  end

  property "bounded nested containers round-trip without changing order or original structs" do
    leaf =
      one_of([
        constant(Divider.new!()),
        map(string(:alphanumeric, min_length: 1, max_length: 10), &Section.new!(text: &1))
      ])

    block =
      tree(leaf, fn child ->
        map(
          list_of(child, min_length: 1, max_length: 3),
          &Container.new!(title: "Group", child_blocks: &1)
        )
      end)

    check all(value <- block, max_runs: 60) do
      wire = Slackblocks.to_map!(value)

      assert Slackblocks.decode!(:block, Slackblocks.encode!(value)) |> Slackblocks.to_map!() ===
               wire

      assert Task.async(fn -> Slackblocks.to_map!(value) end) |> Task.await() === wire
    end
  end

  property "arbitrary-size native integers survive checked JSON ingress exactly" do
    check all(
            number <-
              integer(
                -1_000_000_000_000_000_000_000_000_000..1_000_000_000_000_000_000_000_000_000
              ),
            max_runs: 100
          ) do
      value = RawNumber.new!(text: Integer.to_string(number), value: number)
      assert Slackblocks.decode!(RawNumber, Slackblocks.encode!(value)).value === number
    end
  end

  defp json do
    scalar =
      one_of([
        constant(nil),
        boolean(),
        integer(),
        float(min: -1_000.0, max: 1_000.0),
        string(:utf8, max_length: 12)
      ])

    tree(scalar, fn child ->
      one_of([
        list_of(child, max_length: 4),
        map_of(string(:alphanumeric, max_length: 8), child, max_length: 4)
      ])
    end)
  end
end
