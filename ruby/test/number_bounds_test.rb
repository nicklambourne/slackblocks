require_relative "test_helper"
class NumberBoundsTest < Minitest::Test
  def test_string_wire_and_numeric_convenience
    [0, -2, 1.25, "-1.5", "10.25"].each do |value|
      wire = Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: value, max_value: value).as_json
      assert_equal value.to_s, wire["min_value"]
      assert_equal value.to_s, wire["max_value"]
    end
    refute Slackblocks::NumberInputElement.new(is_decimal_allowed: true).as_json.key?("min_value")
  end

  def test_invalid_and_inverted_bounds
    [Float::INFINITY, Float::NAN, "invalid", ""].each do |value|
      assert_raises(Slackblocks::ValidationError) { Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: value) }
    end
    assert_raises(Slackblocks::ValidationError) { Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: "10", max_value: "2") }
  end

  def test_exact_decimal_comparison
    [
      ["0.10000000000000001", "0.1"],
      ["9007199254740993", "9007199254740992"],
      ["-0.1", "-0.10000000000000001"],
      ["1e-1000", "9e-1001"],
      ["1.00000000000000001e309", "1e309"],
      [".0000000000000000001", "0"],
      ["0", "-1e-1000"],
      ["10e-999999999999999999999", "9e-999999999999999999999"],
      ["1e999999999999999999999", "9e999999999999999999998"]
    ].each do |higher, lower|
      error = assert_raises(Slackblocks::ValidationError) do
        Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: higher, max_value: lower)
      end
      assert_equal "out-of-range", error.category
      assert_equal "NumberInputElement", error.path
      wire = Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: lower, max_value: higher).as_json
      assert_equal lower, wire["min_value"]
      assert_equal higher, wire["max_value"]
    end
  end

  def test_equivalent_decimal_spellings_and_omission
    [
      ["0.1000", "1e-1"], ["-0.00", "+0e99"], ["0010.0", "+1.e1"],
      ["-.50", "-5e-1"], ["10e-999999999999999999999", "1e-999999999999999999998"],
      ["10e999999999999999999999", "1e1000000000000000000000"]
    ].each do |left, right|
      [[left, right], [right, left]].each do |minimum, maximum|
        wire = Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: minimum, max_value: maximum).as_json
        assert_equal minimum, wire["min_value"]
        assert_equal maximum, wire["max_value"]
      end
    end
    refute Slackblocks::NumberInputElement.new(is_decimal_allowed: true, max_value: 0).as_json.key?("min_value")
    refute Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: 0).as_json.key?("max_value")
  end

  def test_decimal_grammar
    ["NaN", "Infinity", "0x10", "1_0", " 1 ", "1e", ".", "1\n", true].each do |value|
      error = assert_raises(Slackblocks::ValidationError) { Slackblocks::NumberInputElement.new(is_decimal_allowed: true, min_value: value) }
      assert_equal "type-mismatch", error.category
      assert_equal "NumberInputElement.min_value", error.path
    end
  end
end
