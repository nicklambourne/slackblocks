module Slackblocks
  class ValidationError < StandardError
    CATEGORIES = %w[
      length-exceeded out-of-range mutually-exclusive
      type-mismatch missing-required invalid-usage
    ].freeze

    attr_reader :category, :path

    def initialize(category, path, message)
      raise ArgumentError, "unknown validation category: #{category}" unless CATEGORIES.include?(category)

      @category = category
      @path = path
      super("#{path}: #{message}")
    end
  end
end
