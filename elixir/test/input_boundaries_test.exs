defmodule Slackblocks.InputBoundariesTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Elements
  alias Slackblocks.Elements.{DatePicker, PlainTextInput, TimePicker}
  alias Slackblocks.Objects.{Option, OptionGroup}
  alias Slackblocks.ValidationError

  test "plain-text bounds preserve omission, equality and zero, and reject reversed pairs" do
    for {minimum, maximum} <- [{nil, nil}, {20, nil}, {nil, 10}, {0, 1}, {10, 10}, {3000, 3000}] do
      input = PlainTextInput.new!(min_length: minimum, max_length: maximum)
      assert input.min_length === minimum
      assert input.max_length === maximum
      assert Slackblocks.decode!(PlainTextInput, Slackblocks.encode!(input)) === input
    end

    assert {:error, %ValidationError{category: :out_of_range, path: ["min_length"]}} =
             PlainTextInput.new(min_length: 20, max_length: 10)

    input = PlainTextInput.new!(min_length: 0, max_length: 10)

    assert {:error, %ValidationError{category: :out_of_range}} =
             PlainTextInput.change(input, min_length: 20)

    assert input.min_length === 0
    assert PlainTextInput.change!(input, min_length: nil).min_length === nil

    assert {:error, %ValidationError{category: :out_of_range}} =
             Slackblocks.to_map(%{input | min_length: 20})

    assert {:error, %ValidationError{category: :out_of_range}} =
             Slackblocks.from_map(
               PlainTextInput,
               Map.put(Slackblocks.to_map!(input), "min_length", 20)
             )
  end

  test "query thresholds preserve zero and reject negative values through native and wire paths" do
    for module <- [Elements.ExternalSelect, Elements.ExternalMultiSelect] do
      input = module.new!(min_query_length: 0)
      assert input.min_query_length === 0
      assert Slackblocks.to_map!(input)["min_query_length"] === 0
      refute Map.has_key?(Slackblocks.to_map!(module.new!([])), "min_query_length")

      assert {:error, %ValidationError{category: :out_of_range, path: ["min_query_length"]}} =
               module.new(min_query_length: -1)

      assert {:error, %ValidationError{category: :out_of_range}} =
               module.change(input, min_query_length: -1)

      assert {:error, %ValidationError{category: :out_of_range}} =
               Slackblocks.to_map(%{input | min_query_length: -1})

      assert {:error, %ValidationError{category: :out_of_range}} =
               Slackblocks.from_map(
                 module,
                 Map.put(Slackblocks.to_map!(input), "min_query_length", -1)
               )
    end
  end

  test "static options and groups reject supplied emptiness while preserving omitted fields" do
    option = Option.new!(text: "One", value: "one")
    group = OptionGroup.new!(label: "Group", options: [option])

    for module <- [Elements.StaticSelect, Elements.StaticMultiSelect],
        {field, item} <- [options: option, option_groups: group] do
      input = module.new!([{field, [item]}])
      assert Slackblocks.decode!(module, Slackblocks.encode!(input)) === input
      refute Map.has_key?(Slackblocks.to_map!(module.new!([])), Atom.to_string(field))

      assert {:error, %ValidationError{category: :length_exceeded, path: [key]}} =
               module.new([{field, []}])

      assert key == Atom.to_string(field)

      assert {:error, %ValidationError{category: :length_exceeded}} =
               module.change(input, [{field, []}])

      assert {:error, %ValidationError{category: :length_exceeded}} =
               Slackblocks.to_map(Map.put(input, field, []))

      assert {:error, %ValidationError{category: :length_exceeded}} =
               Slackblocks.from_map(
                 module,
                 Map.put(Slackblocks.to_map!(input), Atom.to_string(field), [])
               )
    end
  end

  test "picker formats accept documented structural boundaries without adding calendar rules" do
    for date <- ["2028-02-29", "2028-02-30"] do
      input = DatePicker.new!(initial_date: date)
      assert Slackblocks.decode!(DatePicker, Slackblocks.encode!(input)) === input
    end

    for time <- ["00:00", "23:59"] do
      input = TimePicker.new!(initial_time: time)
      assert Slackblocks.decode!(TimePicker, Slackblocks.encode!(input)) === input
    end

    assert DatePicker.new!([]).initial_date === nil
    assert TimePicker.new!([]).initial_time === nil

    assert Elements.Image.new!(image_url: "https://example.com/image.png", alt_text: "").alt_text ===
             ""
  end

  test "picker validation rejects malformed ASCII, Unicode, newline and wrong-type inputs" do
    for {module, field, valid, invalid} <- [
          {DatePicker, :initial_date, "2028-02-29",
           ["", "not a date", "2028-2-29", "2028-02-29\n", "２０２８-02-29", false, 1, []]},
          {TimePicker, :initial_time, "23:59",
           ["", "9:00", "24:00", "25:99", "23:60", "23:59\n", "２３:59", false, 1, []]}
        ],
        value <- invalid do
      input = module.new!([{field, valid}])

      assert {:error, %ValidationError{category: :type_mismatch, path: [key]}} =
               module.new([{field, value}])

      assert key == Atom.to_string(field)

      assert {:error, %ValidationError{category: :type_mismatch}} =
               module.change(input, [{field, value}])

      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.to_map(Map.put(input, field, value))

      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.from_map(
                 module,
                 Map.put(Slackblocks.to_map!(input), Atom.to_string(field), value)
               )
    end
  end
end
