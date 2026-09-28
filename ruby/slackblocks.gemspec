require_relative "lib/slackblocks/version"

Gem::Specification.new do |spec|
  spec.name = "slackblocks"
  spec.version = Slackblocks::VERSION
  spec.authors = ["Nicholas Lambourne"]
  spec.email = ["dev@ndl.au"]
  spec.summary = "Validated Slack Block Kit values and payloads"
  spec.description = "Build Slack Block Kit payloads with Ruby values validated against the shared slackblocks specification."
  spec.homepage = "https://github.com/nicklambourne/slackblocks"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.3"
  spec.require_paths = ["lib"]
  spec.files = Dir.glob("{lib,sig}/**/*", File::FNM_DOTMATCH).select { |path| File.file?(path) } +
    %w[README.md CHANGELOG.md LICENSE LICENSE.BSD-3-Clause]
  spec.metadata = {
    "homepage_uri" => spec.homepage,
    "source_uri" => spec.homepage,
    "changelog_uri" => "#{spec.homepage}/blob/master/ruby/CHANGELOG.md",
    "rubygems_mfa_required" => "true"
  }
end
