# slackblocks for Ruby

Build and validate Slack Block Kit payloads with Ruby 3.3 or newer. The gem implements shared spec 1.2.0 and uses only Ruby's JSON default gem at runtime.

Install from the repository while the first Ruby release is being prepared:

    cd ruby
    gem build slackblocks.gemspec
    gem install ./slackblocks-2.5.0.gem

Use keyword arguments for every value:

    require "slackblocks"

    button = Slackblocks::ButtonElement.new(text: "Open", action_id: "open")
    blocks = [
      Slackblocks::SectionBlock.new(text: "Hello"),
      Slackblocks::ActionsBlock.new(elements: [button])
    ]
    message = Slackblocks::MessagePayload.new(channel: "C123", blocks: blocks)
    puts message.to_json

Strings in text fields become the plain text or mrkdwn object specified for that field. Named enum fields take symbols such as :in_channel, :complete, or :primary. Constructors reject invalid Slack values with Slackblocks::ValidationError, whose category and path identify the rule and field.

All values are immutable. with(**changes) returns a validated value, to_h returns structural Ruby members, and as_json returns a fresh Slack wire Hash. to_json, JSON.generate(value), and ActiveSupport::JSON.encode(value) produce the same payload. ActiveSupport is optional and is never loaded by the gem.

For unmodeled Slack fields, pass additional_fields: { "future_field" => value }. Extensions cannot replace known fields or the wire type. The gem copies and freezes owned strings and collections, including extensions.

Helpers return ordinary values that can be placed in block collections:

    section = Slackblocks::AccordionSection.create(
      title: "Details",
      blocks: [Slackblocks::SectionBlock.new(text: "More information")]
    )
    accordion_blocks = Slackblocks::Accordion.create(sections: [section])

    page_blocks = Slackblocks::Paginator.create(
      action_id_prefix: "items",
      blocks: blocks,
      page: 1,
      page_size: 5
    )

Slackblocks::VERSION is the gem version; Slackblocks::SPEC_VERSION is the shared contract version. To verify the implementation locally, run ruby/bin/check from the repository with Ruby 3.3, 3.4, and 4.0 installed.
