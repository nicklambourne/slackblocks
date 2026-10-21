require_relative "test_helper"

class TextBoundsTest < Minitest::Test
  def test_satisfiable_and_omitted_bounds
    [[nil, nil], [20, nil], [nil, 10], [0, 1], [10, 10], [3000, 3000]].each do |minimum, maximum|
      value = Slackblocks::PlainTextInputElement.new(min_length: minimum, max_length: maximum)
      assert_equal minimum, value.min_length unless minimum.nil?
      assert_equal maximum, value.max_length unless maximum.nil?
    end
    error = assert_raises(Slackblocks::ValidationError) { Slackblocks::PlainTextInputElement.new(min_length: 20, max_length: 10) }
    assert_equal "out-of-range", error.category
    assert_equal "PlainTextInputElement.min_length", error.path
  end

  def test_structural_controls_remain_accepted
    assert_equal 0, Slackblocks::ExternalSelectElement.new(min_query_length: 0).min_query_length
    assert_equal 0, Slackblocks::ExternalMultiSelectElement.new(min_query_length: 0).min_query_length
    ["2028-02-29", "2028-02-30"].each { |date| assert_equal date, Slackblocks::DatePickerElement.new(initial_date: date).initial_date }
    ["00:00", "23:59"].each { |time| assert_equal time, Slackblocks::TimePickerElement.new(initial_time: time).initial_time }
    assert_equal "", Slackblocks::ImageElement.new(image_url: "https://example.com/image.png", alt_text: "").alt_text
  end
end
