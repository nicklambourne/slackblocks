module Slackblocks
  class RichTextStyle
    include Value

    attr_reader(*StyleFlags::NAMES.map(&:to_sym), :additional_fields)
  end
  RichTextStyle.const_set(:WIRE_TYPE, "")
  RichTextStyle.const_set(:FIELD_SPECS, Value.deep_freeze(StyleFlags::NAMES.map do |name|
    {"name" => name, "wire" => name, "kind" => "boolean", "required" => false}
  end))
  RichTextStyle.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextStyle.singleton_class.prepend(KeywordOnly)
end
