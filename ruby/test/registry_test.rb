require_relative "test_helper"

class RegistryTest < Minitest::Test
  ROOT = File.expand_path("../..", __dir__)

  def test_every_public_value_maps_to_shared_capability
    registry = JSON.parse(File.read(File.join(ROOT, "ruby/conformance/capabilities.json")))
    values = Slackblocks.constants(false).filter_map do |name|
      constant = Slackblocks.const_get(name, false)
      name.to_s if constant.is_a?(Class) && constant.ancestors.include?(Slackblocks::Value) && name != :RichTextStyle
    end
    assert_equal registry.keys.sort, values.sort

    coverage = JSON.parse(File.read(File.join(ROOT, "spec/coverage.json"))).fetch("capabilities")
    assert_equal coverage.keys.sort, registry.values.uniq.sort
  end

  def test_every_scalar_limit_has_an_invalid_case
    registry = JSON.parse(File.read(File.join(ROOT, "spec/limits.json")))
    flatten = lambda do |node, prefix = ""|
      node.flat_map do |key, value|
        path = prefix.empty? ? key : "#{prefix}.#{key}"
        value.is_a?(Hash) ? flatten.call(value, path) : [[path, value]]
      end
    end
    leaves = flatten.call(registry).to_h
    assert_equal leaves, Slackblocks::Limits::VALUES
    manifest = JSON.parse(File.read(File.join(ROOT, "spec/fixtures/invalid/manifest.json")))
    constraints = manifest.fetch("cases").map { |entry| entry.fetch("constraint") }
    assert_empty(leaves.keys - constraints)
  end

  def test_generated_output_is_current
    command = ["python3", File.join(ROOT, "ruby/generator/generate_models.py"), "--check"]
    assert system(*command)
  end

  def test_vocabulary_matches_registry
    registry = JSON.parse(File.read(File.join(ROOT, "spec/vocabulary.json")))
    assert_equal registry.fetch("slack_icon_names"), Slackblocks::Vocabulary::ICON_NAMES
    assert_equal registry.fetch("surface_block_types"), Slackblocks::Vocabulary::SURFACE_BLOCK_TYPES
  end
end
