defmodule Slackblocks.NumberBoundsTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Elements.NumberInput
  alias Slackblocks.ValidationError

  test "native numeric conveniences store strings, preserve zero and leave omitted bounds absent" do
    for value <- [0, -2, 1.25, -0.0, 9_007_199_254_740_993, "-1.50", "10.250"] do
      input = NumberInput.new!(is_decimal_allowed: true, min_value: value, max_value: value)
      assert input.min_value === to_string(value)
      assert input.max_value === to_string(value)
      assert Slackblocks.to_map!(input)["min_value"] === to_string(value)
      assert Slackblocks.decode!(NumberInput, Slackblocks.encode!(input)) === input
    end

    input = NumberInput.new!(is_decimal_allowed: false, min_value: 0)
    refute Map.has_key?(Slackblocks.to_map!(input), "max_value")
    cleared = NumberInput.change!(input, min_value: nil, max_value: 10)
    refute Map.has_key?(Slackblocks.to_map!(cleared), "min_value")
    assert cleared.max_value === "10"
    assert input.min_value === "0"
  end

  test "invalid spellings and nonnumeric native values produce structured field errors" do
    for value <- [
          true,
          [],
          %{},
          "",
          "NaN",
          "Infinity",
          "0x10",
          "1_000",
          " 1",
          "1 ",
          "1\n",
          "1\r\n",
          "１",
          "1e",
          "1e+",
          "1e2e3",
          "--1",
          "."
        ] do
      assert {:error, %ValidationError{category: :type_mismatch, path: ["min_value"]}} =
               NumberInput.new(is_decimal_allowed: true, min_value: value)

      assert {:error, %ValidationError{category: :type_mismatch, path: ["max_value"]}} =
               NumberInput.new(is_decimal_allowed: true, max_value: value)
    end
  end

  test "exact ordering covers signs, ranks and coefficient precision without float rounding" do
    for {minimum, maximum} <- [
          {"9007199254740993", "9007199254740992"},
          {"0.10000000000000001", "0.1"},
          {"-9007199254740992", "-9007199254740993"},
          {"-0.1", "-0.10000000000000001"},
          {"1e-1000", "9e-1001"},
          {"1", "-1"},
          {"0", "-0.1"},
          {"1", "0"},
          {"-1e309", "-2e309"}
        ] do
      assert {:error, %ValidationError{category: :out_of_range}} =
               NumberInput.new(is_decimal_allowed: true, min_value: minimum, max_value: maximum)

      input = NumberInput.new!(is_decimal_allowed: true, min_value: maximum, max_value: minimum)
      assert input.min_value === maximum

      assert {:error, %ValidationError{category: :out_of_range}} =
               NumberInput.change(input, min_value: minimum, max_value: maximum)

      assert input.min_value === maximum
    end
  end

  test "equivalent decimals, signed zeros and huge exponents retain their exact spellings" do
    exponent = String.duplicate("9", 5_000)

    for {minimum, maximum} <- [
          {"0.1000", "1e-1"},
          {"1e-1", "0.1000"},
          {"-0e999", "+0.000"},
          {"+0.000", "-0e999"},
          {".5", "+0.50"},
          {"1.", "01"},
          {"1e309", "2e309"},
          {"1e#{exponent}", "2e#{exponent}"},
          {"1e-#{exponent}", "2e-#{exponent}"}
        ] do
      input = NumberInput.new!(is_decimal_allowed: true, min_value: minimum, max_value: maximum)
      assert input.min_value === minimum
      assert input.max_value === maximum
    end
  end

  test "wire ingress and direct struct edits must already contain valid string bounds" do
    input = NumberInput.new!(is_decimal_allowed: true, min_value: "0", max_value: "1")
    wire = Slackblocks.to_map!(input)
    assert Slackblocks.from_map!(NumberInput, wire) === input

    for invalid <- [0, true, "NaN"] do
      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.from_map(NumberInput, Map.put(wire, "min_value", invalid))

      assert {:error, %ValidationError{category: :type_mismatch}} =
               Slackblocks.to_map(%{input | min_value: invalid})
    end

    assert {:error, %ValidationError{category: :out_of_range}} =
             Slackblocks.from_map(NumberInput, Map.put(wire, "min_value", "2"))

    assert {:error, %ValidationError{category: :out_of_range}} =
             Slackblocks.to_map(%{input | min_value: "2"})
  end
end
