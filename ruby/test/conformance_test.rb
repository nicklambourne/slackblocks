require_relative "test_helper"
require_relative "invalid_cases"
require_relative "valid_constructions"

class FixtureDriver
  ROOT_TYPES = {
    "attachments/" => "Attachment",
    "messages/message_response" => "MessageResponse",
    "messages/webhook_message" => "WebhookMessage",
    "messages/message_" => "MessagePayload",
    "objects/confirmation" => "Confirmation",
    "objects/conversation_filter" => "ConversationFilter",
    "objects/dispatch_action_configuration" => "DispatchActionConfiguration",
    "objects/input_parameter" => "InputParameter",
    "objects/option_group" => "OptionGroup",
    "objects/option_" => "Option",
    "objects/slack_file" => "SlackFile",
    "objects/trigger" => "Trigger",
    "objects/workflow" => "Workflow"
  }.freeze

  def initialize
    model = JSON.parse(File.read(File.expand_path("../../spec/model.json", __dir__)))
    @types = model.fetch("types").to_h { |entry| [entry.fetch("name"), entry] }
  end

  def root_type(id)
    ROOT_TYPES.find { |prefix, _| id.start_with?(prefix) }&.last
  end

  def build(json, expected = nil, path = "")
    return json unless json.is_a?(Hash)

    type_name = resolve(json, expected, path)
    raise "Unmapped Slack value at #{path}" unless type_name

    spec = @types.fetch(type_name)
    fields = spec.fetch("fields").to_h { |field| [field.fetch("wire"), field] }
    kwargs = {}
    json.each do |wire, value|
      next if wire == "type" && value == spec.fetch("wireType")

      field = fields[wire]
      raise "Unmapped field #{type_name}.#{wire} at #{path}" unless field

      kwargs[ruby_name(field.fetch("method"))] = argument(field, value, "#{type_name}.#{wire}")
    end
    Slackblocks.const_get(type_name).new(**kwargs)
  end

  private

  def ruby_name(method)
    method.gsub(/([A-Z])/, '_\\1').delete_prefix("_").downcase.to_sym
  end

  def resolve(json, expected, path)
    return expected if expected && @types.key?(expected)

    wire = json["type"]
    candidates = @types.values.select do |entry|
      next false unless entry.fetch("wireType") == wire

      expected.nil? || Slackblocks.const_get(entry.fetch("name")).ancestors.include?(Slackblocks.const_get(expected))
    end.map { |entry| entry.fetch("name") }
    if candidates.empty? && expected
      candidates = @types.values.select { |entry| entry.fetch("wireType") == wire }.map { |entry| entry.fetch("name") }
    end
    if candidates.sort == %w[ImageBlock ImageElement]
      candidates = [(path.include?("elements") || path.start_with?("ImageElement")) ? "ImageElement" : "ImageBlock"]
    end
    raise "Ambiguous #{wire} at #{path}: #{candidates.inspect}" unless candidates.length == 1

    candidates.first
  end

  def argument(field, value, path)
    return nil if value.nil?

    kind = field.fetch("kind")
    target = field["type"]
    case kind
    when "string", "boolean", "int", "long", "double", "number", "stringList", "map"
      value
    when "enum"
      value.to_sym
    when "style"
      Slackblocks::RichTextStyle.new(**value.transform_keys(&:to_sym))
    when "object"
      build(value, target, path)
    when "list", "textList"
      value.each_with_index.map { |item, index| element(field, item, "#{path}[#{index}]") }
    when "rows"
      value.each_with_index.map do |row, index|
        row.each_with_index.map { |item, column| build(item, target, "#{path}[#{index}][#{column}]") }
      end
    when "text"
      element(field, value, path)
    else
      raise "Unhandled #{kind} at #{path}"
    end
  end

  def element(field, value, path)
    return value unless value.is_a?(Hash)

    if %w[text textList].include?(field.fetch("kind")) &&
        value.keys.sort == %w[text type] && value["type"] == field["coerce"]
      return value.fetch("text")
    end
    build(value, field["type"], path)
  end
end

class ConformanceTest < Minitest::Test
  ROOT = File.expand_path("../..", __dir__)
  MANIFEST = JSON.parse(File.read(File.join(ROOT, "spec/manifest.json")))
  IDS = MANIFEST.fetch("fixtures").map { |entry| entry.fetch("id") }.freeze

  def test_spec_version
    assert_equal MANIFEST.fetch("spec_version"), Slackblocks::SPEC_VERSION
  end

  def test_manifest_matches_disk
    files = Dir.glob(File.join(ROOT, "spec/fixtures/valid/**/*.json"))
      .map { |path| path.delete_prefix(File.join(ROOT, "spec/fixtures/valid/")).delete_suffix(".json") }
    assert_equal IDS.sort, files.sort
  end

  def test_skiplist_empty
    entries = File.readlines(File.join(ROOT, "ruby/conformance/skiplist.txt")).reject do |line|
      line.strip.empty? || line.lstrip.start_with?("#")
    end
    assert_empty entries
  end

  def test_independent_constructions_cover_manifest
    assert_equal IDS.sort, ValidConstructions::CONSTRUCTIONS.keys.sort
  end

  IDS.each do |id|
    define_method("test_valid_#{id.tr("/", "_")}") do
      expected = JSON.parse(File.read(File.join(ROOT, "spec/fixtures/valid", "#{id}.json")))
      driver = FixtureDriver.new
      built = driver.build(expected, driver.root_type(id), id)
      assert_equal expected, JSON.parse(built.to_json)
    end

    define_method("test_independent_#{id.tr("/", "_")}") do
      expected = JSON.parse(File.read(File.join(ROOT, "spec/fixtures/valid", "#{id}.json")))
      built = ValidConstructions::CONSTRUCTIONS.fetch(id).call
      assert_equal expected, JSON.parse(built.to_json)
    end
  end
end

class InvalidConformanceTest < Minitest::Test
  ROOT = ConformanceTest::ROOT
  MANIFEST = JSON.parse(File.read(File.join(ROOT, "spec/fixtures/invalid/manifest.json")))
  IDS = InvalidCases::IDS
  TYPES = {
    "Button" => "ButtonElement",
    "Checkboxes" => "CheckboxesElement",
    "FeedbackButtons" => "FeedbackButtonsElement",
    "FileInput" => "FileInputElement",
    "HomeTab" => "HomeTabView",
    "IconButton" => "IconButtonElement",
    "Message" => "MessagePayload",
    "Modal" => "ModalView",
    "NumberInput" => "NumberInputElement",
    "Overflow" => "OverflowElement",
    "PlainTextInput" => "PlainTextInputElement",
    "RadioButtons" => "RadioButtonsElement",
    "StaticSelect" => "StaticSelectElement",
    "URLSource" => "UrlSource"
  }.freeze

  def test_manifest_and_inputs_match
    assert_equal MANIFEST.fetch("cases").map { |entry| entry.fetch("id") }.sort, IDS.sort
  end

  MANIFEST.fetch("cases").each do |entry|
    id = entry.fetch("id")
    define_method("test_invalid_#{id.tr("-", "_")}") do
      input = InvalidCases.for(id)
      name = TYPES.fetch(input.fetch("name"), input.fetch("name"))
      error = assert_raises(Slackblocks::ValidationError) do
        FixtureDriver.new.build(input.fetch("value"), name, name)
      end
      assert_equal entry.fetch("category"), error.category
      refute_empty error.path
    end
  end
end
