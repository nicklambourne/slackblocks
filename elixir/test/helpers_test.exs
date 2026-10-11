defmodule Slackblocks.HelpersTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Blocks.{Actions, Context, Divider, Section}
  alias Slackblocks.Components.{Accordion, AccordionSection, Paginator}
  alias Slackblocks.Objects.{InputParameter, Workflow}
  alias Slackblocks.RichText.Style

  alias Slackblocks.{Color, Message, ValidationError}

  test "pagination selects stable pages and explicit controls" do
    blocks = for i <- 1..11, do: Section.new!(text: "Row #{i}")
    first = Paginator.to_blocks!(blocks, action_id_prefix: "results")
    assert Enum.take(first, 5) == Enum.take(blocks, 5)
    assert %Context{elements: [%{text: "Page 1 of 3"}]} = Enum.at(first, 5)
    assert %Actions{elements: [%{action_id: "results.next", value: "2"}]} = List.last(first)

    middle =
      Paginator.to_blocks!(blocks,
        action_id_prefix: "results",
        page: 2,
        show_page_indicator: false,
        previous_text: "Back",
        next_text: "Forward",
        block_id: "navigation"
      )

    assert %Actions{block_id: "navigation", elements: [previous, next]} = List.last(middle)

    assert {previous.text.text, previous.value, next.text.text, next.value} ==
             {"Back", "1", "Forward", "3"}

    last = Paginator.to_blocks!(blocks, action_id_prefix: "results", page: 3)
    assert hd(last) == List.last(blocks)
    assert [%{action_id: "results.previous", value: "2"}] = List.last(last).elements
    assert Paginator.to_blocks!([Divider.new!()], action_id_prefix: "one") == [Divider.new!()]
    assert length(blocks) == 11
  end

  test "pagination validates bounds, roles, options and expanded payload limits" do
    block = Divider.new!()

    for {blocks, options, category} <- [
          {[], [action_id_prefix: "x"], :missing_required},
          {[block | :bad], [action_id_prefix: "x"], :type_mismatch},
          {[InputParameter.new!(name: "x", value: "y")], [action_id_prefix: "x"], :type_mismatch},
          {[block], [], :missing_required},
          {[block], [action_id_prefix: ""], :missing_required},
          {[block], [action_id_prefix: false], :type_mismatch},
          {[block], [action_id_prefix: "x", page: 0], :out_of_range},
          {[block], [action_id_prefix: "x", page: 2], :out_of_range},
          {[block], [action_id_prefix: "x", page_size: 1.0], :type_mismatch},
          {[block], [action_id_prefix: "x", show_page_indicator: nil], :type_mismatch},
          {[block], [action_id_prefix: "x", unknown: 1], :invalid_usage}
        ] do
      assert {:error, %ValidationError{category: ^category}} =
               Paginator.to_blocks(blocks, options)
    end

    assert_raise ValidationError, fn ->
      Paginator.to_blocks!([block, block],
        action_id_prefix: String.duplicate("x", 255),
        page_size: 1
      )
    end

    expanded =
      Paginator.to_blocks!(List.duplicate(block, 51), action_id_prefix: "x", page_size: 49)

    assert {:error, %ValidationError{category: :length_exceeded}} =
             Message.new(channel: "C123", blocks: expanded)
  end

  test "colors retain existing names, semantic values and precise hex behavior" do
    for {name, expected} <- [
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
        ] do
      assert Color.named!(name) == expected
    end

    assert Color.hex!("A0B1c2") == "#A0B1c2"
    assert Color.hex!("#abcdef") == "#abcdef"
    assert Color.rgb!(0, 128, 255) == "#0080ff"
    assert {:error, %ValidationError{category: :type_mismatch}} = Color.named("purple")

    for value <- ["abc", "##abcdef", "abcdef\n", "gggggg", <<255>>, nil] do
      assert {:error, %ValidationError{}} = Color.hex(value)
    end

    assert {:error, %ValidationError{category: :out_of_range, path: ["color", 1]}} =
             Color.rgb(0, 256, 0)

    assert {:error, %ValidationError{category: :type_mismatch}} = Color.rgb(0.0, 0, 0)
    assert_raise ValidationError, fn -> Color.named!(:unknown) end
  end

  test "workflow URL shortcut produces a normal checked trigger" do
    url = "https://slack.com/shortcuts/Fn123"
    assert Slackblocks.to_map!(Workflow.from_url!(url)) == %{"trigger" => %{"url" => url}}
    value = Workflow.from_url!(url, [InputParameter.new!(name: "ticket", value: "42")])

    assert Slackblocks.to_map!(value)["trigger"]["customizable_input_parameters"] == [
             %{"name" => "ticket", "value" => "42"}
           ]

    assert {:error, %ValidationError{}} = Workflow.from_url(url, [false])
    assert {:error, %ValidationError{}} = Workflow.from_url(url, [1 | 2])
    assert {:error, %ValidationError{}} = Workflow.from_url(nil)
  end

  test "builder URLs wrap blocks, preserve wire values and encode each component once" do
    section = Section.new!(text: "Hello 👋 #?%/ +")
    url = Slackblocks.block_kit_builder_url!(section, team_id: "T /?#")
    assert String.starts_with?(url, "https://app.slack.com/block-kit-builder/T%20%2F%3F%23#")
    assert preview(url) == %{"blocks" => [Slackblocks.to_map!(section)]}
    message = Message.new!(channel: "C123", blocks: [section])
    assert preview(Slackblocks.block_kit_builder_url!(message)) == Slackblocks.to_map!(message)

    assert preview(Slackblocks.block_kit_builder_url!([section, Divider.new!()]))["blocks"]
           |> length() == 2

    data = %{"a" => %{}, "b" => [], "c" => nil, "d" => false, "e" => 0}
    assert preview(Slackblocks.block_kit_builder_url!(data)) === data

    for invalid <- [nil, Style.new!(), [Style.new!()], [section | 1], %{bad: true}] do
      assert {:error, %ValidationError{}} = Slackblocks.block_kit_builder_url(invalid)
    end

    assert {:error, %ValidationError{}} =
             Slackblocks.block_kit_builder_url(section, team_id: false)

    assert_raise ValidationError, fn ->
      Slackblocks.block_kit_builder_url!([%{section | text: "raw"}])
    end
  end

  test "style and accordion malformed inputs retain tagged errors and complete paths" do
    assert {:error, %ValidationError{}} = Style.new([{:bold, true} | :bad])
    assert {:error, %ValidationError{}} = AccordionSection.new([{:title, "x"} | :bad])

    assert {:error, %ValidationError{}} =
             AccordionSection.new(title: "x", blocks: [], expanded: 1)

    assert {:error, %ValidationError{}} = Accordion.to_blocks([Divider.new!()])

    assert {:error, %ValidationError{path: ["style", "bold"]}} =
             Slackblocks.from_map(Slackblocks.RichText.Text, %{
               "type" => "text",
               "text" => "x",
               "style" => %{"bold" => "yes"}
             })

    for flag <- [:bold, :italic, :strike, :code, :highlight, :client_highlight, :unlink] do
      assert Jason.encode!(Style.new!([{flag, false}])) |> Jason.decode!() == %{
               Atom.to_string(flag) => false
             }
    end
  end

  defp preview(url),
    do: url |> URI.parse() |> Map.fetch!(:fragment) |> URI.decode() |> Jason.decode!()
end
