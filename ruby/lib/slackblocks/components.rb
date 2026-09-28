module Slackblocks
  module AccordionSection
    def self.create(title:, blocks:, subtitle: nil, icon: nil, expanded: false, width: ContainerWidth::STANDARD, has_header_divider: false, block_id: nil)
      ContainerBlock.new(
        title: title,
        child_blocks: blocks,
        subtitle: subtitle,
        icon: icon,
        width: width,
        is_collapsible: true,
        default_collapsed: !expanded,
        has_header_divider: has_header_divider,
        block_id: block_id
      )
    end
  end

  module Accordion
    def self.create(sections:)
      Value.fail!("type-mismatch", "Accordion.sections", "expected an Array") unless sections.is_a?(Array)
      Value.fail!("missing-required", "Accordion.sections", "expected at least one section") if sections.empty?
      sections.each_with_index do |section, index|
        unless section.is_a?(ContainerBlock) && section.is_collapsible
          Value.fail!("type-mismatch", "Accordion.sections[#{index}]", "expected a collapsible container")
        end
      end
      sections.dup.freeze
    end
  end

  module Paginator
    def self.create(action_id_prefix:, blocks:, page: 1, page_size: 5, previous_text: "Previous", next_text: "Next", show_page_indicator: true, block_id: nil)
      Value.fail!("type-mismatch", "Paginator.blocks", "expected an array of blocks") unless blocks.is_a?(Array) && blocks.all? { |block| block.is_a?(Block) }
      Value.fail!("missing-required", "Paginator.blocks", "expected at least one block") if blocks.empty?
      Value.fail!("type-mismatch", "Paginator.action_id_prefix", "expected a string") unless action_id_prefix.is_a?(String)
      Value.fail!("missing-required", "Paginator.action_id_prefix", "expected a non-empty prefix") if action_id_prefix.empty?
      [[:page, page], [:page_size, page_size]].each do |field, value|
        Value.fail!("type-mismatch", "Paginator.#{field}", "expected an Integer") unless value.is_a?(Integer)
        Value.fail!("out-of-range", "Paginator.#{field}", "expected a positive integer") if value < 1
      end
      page_count = (blocks.length + page_size - 1) / page_size
      Value.fail!("out-of-range", "Paginator.page", "page exceeds page count") if page > page_count
      result = blocks.drop((page - 1) * page_size).take(page_size)
      return result.freeze if page_count == 1

      # @type var controls: Array[ButtonElement]
      controls = []
      if page > 1
        controls << ButtonElement.new(text: previous_text, action_id: "#{action_id_prefix}.previous", value: (page - 1).to_s)
      end
      if page < page_count
        controls << ButtonElement.new(text: next_text, action_id: "#{action_id_prefix}.next", value: (page + 1).to_s)
      end
      result << ContextBlock.new(elements: [MarkdownText.new(text: "Page #{page} of #{page_count}")]) if show_page_indicator
      result << ActionsBlock.new(elements: controls, block_id: block_id)
      result.freeze
    end
  end
end
