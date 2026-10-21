defmodule Slackblocks.RulesTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Blocks.{Alert, DataTable, Divider, Markdown}
  alias Slackblocks.Internal.Schema
  alias Slackblocks.Objects.{AxisConfig, PlainText, RawText, SlackIcon}
  alias Slackblocks.Test.Samples
  alias Slackblocks.ValidationError
  @root Path.expand("../..", __DIR__)
  @contract Jason.decode!(File.read!(Path.join(@root, "spec/model.json")))
  @names Jason.decode!(File.read!(Path.join(@root, "elixir/design/parity.json")))["models"]

  test "each declared contextual rule has native positive and negative evidence" do
    expected =
      for model <- @contract["types"],
          {_rule, index} <- Enum.with_index(model["rules"] || []),
          do: {model["name"], index}

    cases = cases()

    assert Enum.sort(Enum.map(cases, fn {name, index, _, _} -> {name, index} end)) ==
             Enum.sort(expected)

    for {name, _index, invalid, category} <- cases do
      assert Slackblocks.validate(sample(name)) == :ok
      error = assert_raise ValidationError, invalid
      assert error.category == category, "wrong error category for #{name}"
    end
  end

  test "string and text field maximum boundaries accept exactly the limit" do
    limits = Schema.limits()

    for model <- @contract["types"],
        field <- model["fields"],
        field["kind"] in ["string", "text"],
        field["limits"] != nil do
      max = limits[field["limits"] <> ".max_length"]

      if max && (field["kind"] == "string" || field["coerce"] in ["plain_text", "mrkdwn"]) do
        value = sample(model["name"])
        key = String.to_existing_atom(field["wire"])
        # ASCII and astral text both count as one code point per repetition.
        for char <- ["x", "🦀"] do
          assert {:ok, _} = value.__struct__.change(value, [{key, String.duplicate(char, max)}])

          assert {:error, %ValidationError{category: :length_exceeded}} =
                   value.__struct__.change(value, [{key, String.duplicate(char, max + 1)}])
        end
      end
    end
  end

  test "opaque values, empty/null boundaries and nested errors stay distinct" do
    for bad <- [self(), make_ref(), fn -> :ok end, ~c"chars"] do
      assert {:error, %ValidationError{category: :type_mismatch}} =
               PlainText.new(text: bad)
    end

    assert {:error, %ValidationError{category: :invalid_usage}} =
             Slackblocks.from_map("SectionBlock", %{})

    assert {:error, %ValidationError{}} = Slackblocks.from_map(:unknown_role, %{})
    assert {:error, %ValidationError{}} = Slackblocks.from_map(:block, %{"type" => "unknown"})
    assert {:error, %ValidationError{}} = Slackblocks.from_map(:block, %{:type => "section"})
    assert {:error, %ValidationError{}} = Slackblocks.from_map(:block, nil)
    assert {:error, %ValidationError{}} = Slackblocks.decode(:block, nil)
    assert {:error, %ValidationError{}} = Slackblocks.to_map(MapSet.new())

    assert {:error, %ValidationError{path: ["rows", 1, 0]}} =
             Slackblocks.from_map(DataTable, %{
               "type" => "data_table",
               "rows" => [[%{"type" => "raw_text", "text" => "H"}], [false]]
             })
  end

  defp cases do
    [
      {"ExternalMultiSelectElement", 0,
       fn -> change("ExternalMultiSelectElement", focus_on_load: false) end, :invalid_usage},
      {"CardBlock", 0,
       fn -> change("CardBlock", hero_image: nil, title: nil, actions: nil, body: nil) end,
       :missing_required},
      {"CardBlock", 1,
       fn ->
         change("CardBlock",
           icon: sample("ImageElement"),
           slack_icon: SlackIcon.new!(name: "bot")
         )
       end, :mutually_exclusive},
      {"ContainerBlock", 0, fn -> change("ContainerBlock", title: nil, rich_text_title: nil) end,
       :missing_required},
      {"DataTableBlock", 0,
       fn ->
         change("DataTableBlock", rows: [[sample("RichTextBlock")], [RawText.new!(text: "x")]])
       end, :type_mismatch},
      {"DataTableBlock", 1,
       fn ->
         change("DataTableBlock",
           rows: [[RawText.new!(text: "x"), RawText.new!(text: "y")], [RawText.new!(text: "z")]]
         )
       end, :invalid_usage},
      {"ImageBlock", 0,
       fn ->
         change("ImageBlock", image_url: "https://example.com/x", slack_file: sample("SlackFile"))
       end, :mutually_exclusive},
      {"PlanBlock", 0,
       fn -> change("PlanBlock", tasks: List.duplicate(hd(sample("PlanBlock").tasks), 2)) end,
       :invalid_usage},
      {"SectionBlock", 0, fn -> change("SectionBlock", text: nil, fields: nil) end,
       :missing_required},
      {"TaskCardBlock", 0,
       fn -> change("TaskCardBlock", status: :pending) |> Slackblocks.to_map!() end,
       :type_mismatch},
      {"ImageElement", 0,
       fn ->
         change("ImageElement",
           image_url: "https://example.com/x",
           slack_file: sample("SlackFile")
         )
       end, :mutually_exclusive},
      {"DatePickerElement", 0, fn -> change("DatePickerElement", initial_date: "not a date") end,
       :type_mismatch},
      {"TimePickerElement", 0, fn -> change("TimePickerElement", initial_time: "25:99") end,
       :type_mismatch},
      {"PlainTextInputElement", 0,
       fn -> change("PlainTextInputElement", min_length: 20, max_length: 10) end, :out_of_range},
      {"NumberInputElement", 0,
       fn -> change("NumberInputElement", min_value: 2, max_value: 1) end, :out_of_range},
      {"StaticMultiSelectElement", 0,
       fn ->
         change("StaticMultiSelectElement",
           options: [sample("Option")],
           option_groups: [sample("OptionGroup")]
         )
       end, :mutually_exclusive},
      {"StaticSelectElement", 0,
       fn ->
         change("StaticSelectElement",
           options: [sample("Option")],
           option_groups: [sample("OptionGroup")]
         )
       end, :mutually_exclusive},
      {"AreaChart", 0,
       fn -> change("AreaChart", axis_config: AxisConfig.new!(categories: ["different"])) end,
       :invalid_usage},
      {"BarChart", 0,
       fn -> change("BarChart", axis_config: AxisConfig.new!(categories: ["different"])) end,
       :invalid_usage},
      {"ConversationFilter", 0,
       fn ->
         change("ConversationFilter",
           include: nil,
           exclude_external_shared_channels: nil,
           exclude_bot_users: nil
         )
       end, :missing_required},
      {"ConversationFilter", 1, fn -> change("ConversationFilter", include: ["unknown"]) end,
       :type_mismatch},
      {"LineChart", 0,
       fn -> change("LineChart", axis_config: AxisConfig.new!(categories: ["different"])) end,
       :invalid_usage},
      {"SlackFile", 0,
       fn -> change("SlackFile", id: "F12345678", url: "https://example.com/x") end,
       :mutually_exclusive},
      {"SlackFile", 1, fn -> change("SlackFile", id: "invalid", url: nil) end, :type_mismatch},
      {"Attachment", 0, fn -> change("Attachment", blocks: [Alert.new!(text: "x")]) end,
       :type_mismatch},
      {"HomeTabView", 0, fn -> change("HomeTabView", blocks: [Alert.new!(text: "x")]) end,
       :type_mismatch},
      {"MessagePayload", 2,
       fn ->
         change("MessagePayload", blocks: List.duplicate(sample("DataVisualizationBlock"), 3))
       end, :length_exceeded},
      {"MessagePayload", 0, fn -> change("MessagePayload", blocks: [Alert.new!(text: "x")]) end,
       :type_mismatch},
      {"MessagePayload", 1, fn -> too_much_markdown("MessagePayload") end, :length_exceeded},
      {"MessageResponse", 2,
       fn ->
         change("MessageResponse", blocks: List.duplicate(sample("DataVisualizationBlock"), 3))
       end, :length_exceeded},
      {"MessageResponse", 0, fn -> change("MessageResponse", blocks: [Alert.new!(text: "x")]) end,
       :type_mismatch},
      {"MessageResponse", 1, fn -> too_much_markdown("MessageResponse") end, :length_exceeded},
      {"ModalView", 0, fn -> change("ModalView", blocks: [sample("FileBlock")]) end,
       :type_mismatch},
      {"ModalView", 1, fn -> change("ModalView", blocks: [sample("InputBlock")], submit: nil) end,
       :missing_required},
      {"WebhookMessage", 2,
       fn ->
         change("WebhookMessage", blocks: List.duplicate(sample("DataVisualizationBlock"), 3))
       end, :length_exceeded},
      {"WebhookMessage", 0, fn -> change("WebhookMessage", blocks: [Alert.new!(text: "x")]) end,
       :type_mismatch},
      {"WebhookMessage", 1, fn -> too_much_markdown("WebhookMessage") end, :length_exceeded}
    ]
  end

  defp too_much_markdown(name),
    do:
      change(name,
        blocks: [
          Markdown.new!(text: String.duplicate("x", 6_001)),
          Markdown.new!(text: String.duplicate("y", 6_001)),
          Divider.new!()
        ]
      )

  defp sample(name),
    do: Map.fetch!(Samples.all(), String.to_existing_atom("Elixir." <> @names[name]))

  defp change(name, options) do
    value = sample(name)
    value.__struct__.change!(value, options)
  end
end
