defmodule Slackblocks.FoundationTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Blocks.{DataTable, Plan, RichText, Section, Table, TaskCard}
  alias Slackblocks.Components.{Accordion, AccordionSection}
  alias Slackblocks.Elements.Button
  alias Slackblocks.Objects.{MarkdownText, PlainText, RawNumber, RawText}

  alias Slackblocks.RichText.Section, as: RichSection

  alias Slackblocks.RichText.Text, as: RichTextValue

  alias Slackblocks.{Message, ValidationError}

  test "native construction, inspection, checked editing and complete message" do
    button = Button.new!(text: "View build", action_id: "view", style: :primary)
    section = Section.new!(text: "*Build complete*", accessory: button)
    edited = Section.change!(section, text: "*Updated*", block_id: "stable")
    assert section.text.text == "*Build complete*"
    assert edited.text.text == "*Updated*"
    message = Message.new!(channel: "C123", blocks: [edited])

    assert %{
             "channel" => "C123",
             "mrkdwn" => true,
             "text" => "",
             "blocks" => [%{"block_id" => "stable", "accessory" => %{"style" => "primary"}}]
           } = Slackblocks.to_map!(message)

    assert Jason.decode!(Jason.encode!(message)) === Slackblocks.to_map!(message)
    assert Slackblocks.decode!(Message, Slackblocks.encode!(message)) === message
  end

  test "local task context is resolved by a complete plan" do
    pending = TaskCard.new!(task_id: "task_1", title: "Queued", status: :pending)

    assert {:error, %ValidationError{category: :type_mismatch, path: ["status"]}} =
             Slackblocks.to_map(pending)

    plan = Plan.new!(title: "Work", tasks: [pending])
    wire = Slackblocks.to_map!(plan)
    refute Map.has_key?(hd(wire["tasks"]), "type")
    assert pending.__struct__ == TaskCard
    assert Slackblocks.decode!(Plan, Slackblocks.encode!(plan)) == plan

    assert {:error, %ValidationError{category: :invalid_usage}} =
             Plan.new(title: "Work", tasks: [pending, pending])
  end

  test "tables have distinct rectangular and header constraints" do
    a = RawText.new!(text: "A")
    b = RawText.new!(text: "B")
    assert {:ok, _} = Table.new(rows: [[a, b], [a]])

    assert {:error, %ValidationError{category: :invalid_usage, path: ["rows", 1]}} =
             DataTable.new(rows: [[a, b], [a]], caption: "Report")

    rich =
      RichText.new!(
        elements: [
          RichSection.new!(elements: [RichTextValue.new!(text: "Hello")])
        ]
      )

    assert {:ok, _} = DataTable.new(rows: [[a], [rich]], caption: "Report")

    assert {:error, %ValidationError{category: :type_mismatch}} =
             DataTable.new(rows: [[rich], [a]], caption: "Report")
  end

  test "helper expansion retains explicit IDs and open/closed state" do
    body = Section.new!(text: "Details")
    closed = AccordionSection.new!(title: "Summary", blocks: [body], block_id: "summary")
    opened = AccordionSection.new!(title: "More", blocks: [body], expanded: true)
    assert closed.default_collapsed
    refute opened.default_collapsed
    assert Accordion.to_blocks!([closed, opened]) == [closed, opened]
    assert {:error, %ValidationError{category: :missing_required}} = Accordion.to_blocks([])
    assert {:error, %ValidationError{category: :type_mismatch}} = Accordion.to_blocks([body])
  end

  test "every output boundary rejects forged or directly edited structs" do
    valid = Section.new!(text: "Hello")

    for invalid <- [
          %{valid | text: "not normalized"},
          Map.put(valid, :extra, 1),
          Map.delete(valid, :fields)
        ] do
      assert {:error, %ValidationError{category: :type_mismatch}} = Slackblocks.to_map(invalid)
      assert {:error, %ValidationError{}} = Slackblocks.encode(invalid)
      assert_raise ValidationError, fn -> Jason.encode!(invalid) end
    end

    invalid = %{valid | text: %{valid.text | text: String.duplicate("x", 3001)}}

    assert {:error, %ValidationError{category: :length_exceeded, path: ["text", "text"]}} =
             Slackblocks.to_map(invalid)

    assert_raise ValidationError, fn -> Jason.encode!(%{payload: invalid}) end
  end

  test "keyword, required field, text and role failures stay categorized" do
    assert {:error, %ValidationError{category: :invalid_usage}} =
             Section.new(text: "a", text: "b")

    assert {:error, %ValidationError{category: :invalid_usage}} = Section.new(unknown: true)
    assert {:error, %ValidationError{category: :type_mismatch}} = PlainText.new(text: ~c"hello")
    assert {:error, %ValidationError{category: :type_mismatch}} = PlainText.new(%{text: "hello"})
    assert {:error, %ValidationError{category: :missing_required}} = PlainText.new([])
    assert {:error, %ValidationError{category: :missing_required}} = PlainText.new(text: nil)

    assert {:error, %ValidationError{category: :type_mismatch}} =
             Button.new(text: MarkdownText.new!(text: "hello"))

    assert {:error, %ValidationError{category: :type_mismatch}} = PlainText.new(text: <<255>>)

    assert {:error, %ValidationError{category: :invalid_usage}} =
             Slackblocks.from_map(:unknown, %{})
  end

  test "parse preserves optional omissions instead of restoring constructor defaults" do
    value = Slackblocks.from_map!(Message, %{"channel" => "C123"})
    assert value.text == nil and value.mrkdwn == nil
    assert Slackblocks.to_map!(value) == %{"channel" => "C123"}
    created = Message.new!(channel: "C123") |> Message.change!(text: nil, mrkdwn: nil)
    assert created == value
    assert Slackblocks.decode!(Message, Slackblocks.encode!(created)) == value

    assert {:error, %ValidationError{category: :invalid_usage}} =
             Slackblocks.from_map(Message, %{"channel" => "C123", "surprise" => 1})
  end

  test "JSON preserves arbitrary integers and checks original decimal precision" do
    integer = 9_007_199_254_740_993
    raw = RawNumber.new!(text: "large", value: integer)
    assert Slackblocks.decode!(RawNumber, Slackblocks.encode!(raw)).value === integer

    assert Slackblocks.decode!(RawNumber, ~s({"type":"raw_number","text":"n","value":0.1})).value ===
             0.1

    for number <- ["0.123456789012345678901", "1e999", "1e-999"] do
      json = ~s({"type":"raw_number","text":"n","value":#{number}})

      assert {:error, %ValidationError{category: :out_of_range}} =
               Slackblocks.decode(RawNumber, json)
    end

    Decimal.Context.with(%Decimal.Context{precision: 1}, fn ->
      assert Slackblocks.decode!(RawNumber, ~s({"type":"raw_number","text":"n","value":1.23})).value ===
               1.23
    end)

    assert {:error, %ValidationError{category: :type_mismatch}} =
             Slackblocks.decode(
               DataTable,
               ~s({"type":"data_table","caption":"c","rows":[[{"type":"raw_text","text":"h"}],[{"type":"raw_text","text":"v"}]],"page_size":1.0})
             )
  end

  test "duplicate objects, syntax errors, roles and codepoint limits" do
    assert {:error, %ValidationError{category: :invalid_usage, path: ["text"]}} =
             Slackblocks.decode(PlainText, ~s({"type":"plain_text","text":"a","text":"b"}))

    assert {:error, %Jason.DecodeError{}} = Slackblocks.decode(PlainText, "{")

    assert {:ok, %PlainText{}} =
             Slackblocks.from_map(:text, %{"type" => "plain_text", "text" => "ok"})

    assert {:ok, _} = PlainText.new(text: String.duplicate("😀", 3000))

    assert {:error, %ValidationError{category: :length_exceeded}} =
             PlainText.new(text: String.duplicate("e\u0301", 1501))
  end

  test "opaque metadata is JSON data and cannot imitate modeled edges" do
    metadata = %{
      "type" => "markdown",
      "text" => String.duplicate("x", 12_001),
      "rows" => %{},
      "tasks" => [],
      "nil" => nil,
      "false" => false,
      "zero" => 0,
      "chars" => [97, 98]
    }

    message = Message.new!(channel: "C123", metadata: metadata)
    assert Slackblocks.to_map!(message)["metadata"] === metadata
    assert Slackblocks.decode!(Message, Slackblocks.encode!(message)).metadata === metadata

    for bad <- [
          %{key: "atom"},
          %{"value" => self()},
          %{"value" => Decimal.new("1.2")},
          %{"value" => [1 | 2]}
        ] do
      assert {:error, %ValidationError{category: :type_mismatch}} =
               Message.new(channel: "C123", metadata: bad)
    end

    assert {:error, %ValidationError{category: :type_mismatch}} = Section.new(fields: ["a" | "b"])

    assert {:error, %ValidationError{category: :type_mismatch}} =
             Section.new([{:text, "ok"} | :bad])

    assert {:error, %ValidationError{category: :invalid_usage, path: ["metadata", "same"]}} =
             Slackblocks.decode(Message, ~s({"channel":"C123","metadata":{"same":1,"same":2}}))
  end
end
