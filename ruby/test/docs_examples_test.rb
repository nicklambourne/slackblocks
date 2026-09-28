require "open3"
require "rbconfig"
require_relative "test_helper"

class DocsExamplesTest < Minitest::Test
  ROOT = File.expand_path("../..", __dir__)

  def test_section_hello_matches_shared_json
    example = File.join(ROOT, "docs/examples/ruby/section_hello.rb")
    stdout, stderr, status = Open3.capture3(RbConfig.ruby, "-I", File.join(ROOT, "ruby/lib"), example)
    assert status.success?, stderr

    expected = JSON.parse(File.read(File.join(ROOT, "docs/examples/section_hello.json")))
    assert_equal expected, JSON.parse(stdout)
  end
end
