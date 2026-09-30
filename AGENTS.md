# Repository instructions

This open source repository uses GitHub-provided runners for GitHub Actions. The repository owner has authorized standard hosted runner labels for its workflows. Follow the existing workflow conventions, keep matrix checks stable on unrelated pull requests, and verify the actual runner metadata after a new workflow runs.

## Block Kit changes

When adding or changing a Block Kit type, field, vocabulary, validation rule, or
composition helper, follow [Adding or updating Block Kit support](docs/docs/maintaining-block-kit.mdx).
Start with official Slack evidence and the shared contract; update every
maintained implementation using its native API, regenerate owned outputs, and
verify conformance, exports, documentation and package checks. Do not treat raw
JSON escape hatches or nonempty release skip lists as completed support. Extend
the guide's language and verification lists whenever a new implementation ships.
