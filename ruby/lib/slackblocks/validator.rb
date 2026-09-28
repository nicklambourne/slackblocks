module Slackblocks
  module Validator
    def self.validate(klass, values)
      path = klass.name.split("::").last
      wire = Value.wire(klass, values)
      check_limits(wire["text"], "text", "#{path}.text") if klass == PlainText || klass == MarkdownText
      klass::FIELD_SPECS.each do |field|
        next if values[field.fetch("name").to_sym].nil?

        check_limits(wire[field.fetch("wire")], field["limits"], "#{path}.#{field.fetch("wire")}")
      end
      special(klass, wire, path)
    end

    def self.limit(key)
      Limits::VALUES.fetch(key)
    end

    def self.fail!(category, path, message)
      Value.fail!(category, path, message)
    end

    def self.string_limit(value, key, path)
      return unless value

      fail!("length-exceeded", path, "exceeds #{key}") if value.length > limit(key)
    end

    def self.text_limit(value, key, path)
      return unless value

      string_limit(value.fetch("text"), key, path)
    end

    def self.max_items(items, key, path)
      return unless items

      fail!("length-exceeded", path, "exceeds #{key}") if items.length > limit(key)
    end

    def self.special(klass, wire, path)
      case klass.name.split("::").last
      when "SlackIcon"
        unless Vocabulary::ICON_NAMES.include?(wire["name"])
          fail!("type-mismatch", "#{path}.name", "unknown Slack icon")
        end
      when "Attachment"
        if (color = wire["color"]) && !%w[good warning danger].include?(color) &&
            !color.match?(/\A#[0-9a-fA-F]{6}\z/)
          fail!("type-mismatch", "#{path}.color", "expected a six-digit hex color or alias")
        end
      when "SectionBlock"
        fields = wire["fields"] || []
        fail!("missing-required", path, "expected text or fields") if !wire.key?("text") && fields.empty?
        fail!("missing-required", "#{path}.fields", "fields cannot be empty") if wire.key?("fields") && fields.empty?
        fields.each_with_index do |field, index|
          text_limit(field, "section.fields.item_max_length", "#{path}.fields[#{index}].text")
        end
      when "StaticSelectElement", "MultiStaticSelectElement"
        if wire.key?("options") && wire.key?("option_groups")
          fail!("mutually-exclusive", path, "options and option_groups cannot both be present")
        end
      when "CardBlock"
        if !%w[hero_image title actions body].any? { |key| wire.key?(key) }
          fail!("missing-required", path, "card needs content")
        end
        if wire.key?("icon") && wire.key?("slack_icon")
          fail!("mutually-exclusive", path, "icon and slack_icon cannot both be present")
        end
      when "ContainerBlock"
        unless wire.key?("title") || wire.key?("rich_text_title")
          fail!("missing-required", path, "expected title")
        end
        if wire["default_collapsed"] && !wire["is_collapsible"]
          fail!("invalid-usage", "#{path}.default_collapsed", "requires is_collapsible")
        end
        if wire["has_header_divider"] && wire["is_collapsible"]
          fail!("invalid-usage", "#{path}.has_header_divider", "requires non-collapsible container")
        end
      when "ImageBlock", "ImageElement"
        has_url, has_file = wire.key?("image_url"), wire.key?("slack_file")
        fail!("missing-required", path, "expected an image source") unless has_url || has_file
        fail!("mutually-exclusive", path, "expected one image source") if has_url && has_file
      when "SlackFile"
        if wire.key?("id") == wire.key?("url")
          fail!("mutually-exclusive", path, "expected exactly one of id or url")
        end
        if wire["id"] && !wire["id"].match?(/\AF[A-Z0-9]{8,}\z/)
          fail!("type-mismatch", "#{path}.id", "invalid Slack file ID")
        end
      when "ConversationFilter"
        if wire.empty?
          fail!("missing-required", path, "expected at least one filter field")
        end
        (wire["include"] || []).each_with_index do |item, index|
          unless %w[im mpim private public].include?(item)
            fail!("type-mismatch", "#{path}.include[#{index}]", "unknown conversation type")
          end
        end
      when "DispatchActionConfiguration"
        fail!("missing-required", "#{path}.trigger_actions_on", "required") unless wire.key?("trigger_actions_on")
      when "NumberInputElement"
        if wire["min_value"] && wire["max_value"] && wire["min_value"] > wire["max_value"]
          fail!("out-of-range", path, "min_value exceeds max_value")
        end
      when "DataTableBlock", "TableBlock"
        table(wire, path, klass == DataTableBlock)
      when "AxisConfig"
        (wire["categories"] || []).each_with_index do |category, index|
          string_limit(category, "data_visualization.category_label.max_length", "#{path}.categories[#{index}]")
        end
        if wire["categories"] && wire["categories"].uniq.length != wire["categories"].length
          fail!("invalid-usage", "#{path}.categories", "duplicate category")
        end
      when "ChartSegment"
        if wire["value"] <= limit("data_visualization.segment.value.exclusive_min")
          fail!("out-of-range", "#{path}.value", "expected positive value")
        end
      when "LineChart", "BarChart", "AreaChart"
        chart(wire, path)
      when "PlanBlock"
        ids = wire.fetch("tasks").map { |task| task["task_id"] }
        fail!("invalid-usage", "#{path}.tasks", "duplicate task ID") if ids.uniq.length != ids.length
      when "MessagePayload", "MessageResponse", "WebhookMessage"
        message(wire, path)
      when "ModalView", "HomeTabView"
        surface(wire.fetch("blocks"), (klass == ModalView) ? "modal" : "home", "#{path}.blocks")
        if klass == ModalView && !wire.key?("submit") && wire.fetch("blocks").any? { |block| block["type"] == "input" }
          fail!("missing-required", "#{path}.submit", "required when modal contains an input")
        end
      end
    end

    def self.surface(blocks, name, path)
      allowed = Vocabulary::SURFACE_BLOCK_TYPES.fetch(name)
      blocks.each_with_index do |block, index|
        type = block["type"]
        fail!("type-mismatch", "#{path}[#{index}].type", "unsupported #{name} block") unless allowed.include?(type)
        standalone_task(block, "#{path}[#{index}]")
      end
    end

    def self.standalone_task(value, path)
      case value
      when Hash
        if value["type"] == "task_card" && value["status"] == "pending"
          fail!("type-mismatch", "#{path}.status", "pending is plan-only")
        end
        value.each do |key, nested|
          next if key == "tasks" && value["type"] == "plan"

          standalone_task(nested, "#{path}.#{key}")
        end
      when Array
        value.each_with_index { |item, index| standalone_task(item, "#{path}[#{index}]") }
      end
    end

    def self.message(wire, path)
      surface(wire["blocks"] || [], "message", "#{path}.blocks")
      totals = {"markdown" => 0, "data_table" => 0}
      tally(wire, totals)
      if totals["markdown"] > limit("markdown.total_text.max_length")
        fail!("length-exceeded", path, "message markdown total exceeded")
      end
      if totals["data_table"] > limit("data_table.total_content.max_length")
        fail!("length-exceeded", path, "message data table total exceeded")
      end
    end

    def self.tally(node, totals)
      case node
      when Hash
        if node["type"] == "markdown"
          totals["markdown"] += node.fetch("text").length
        elsif node["type"] == "data_table"
          totals["data_table"] += characters(node.fetch("rows"))
        else
          node.each_value { |child| tally(child, totals) }
        end
      when Array
        node.each { |child| tally(child, totals) }
      end
    end

    def self.characters(node)
      case node
      when Array
        node.sum { |child| characters(child) }
      when Hash
        node.sum do |key, child|
          (key == "text" && child.is_a?(String)) ? child.length : characters(child)
        end
      else
        0
      end
    end

    def self.table(wire, path, data_table)
      rows = wire.fetch("rows")
      maximum = data_table ? "data_table.columns.max_items" : "table.columns.max_items"
      rows.each_with_index do |row, index|
        row_path = "#{path}.rows[#{index}]"
        max_items(row, maximum, row_path)
        if data_table
          fail!("length-exceeded", row_path, "row cannot be empty") if row.empty?
          if row.length != rows.first.length
            fail!("invalid-usage", row_path, "data table rows must be rectangular")
          end
          row.each_with_index do |cell, column|
            cell_path = "#{row_path}[#{column}]"
            if index.zero? && cell["type"] == "rich_text"
              fail!("type-mismatch", cell_path, "header must be raw")
            end
            if %w[raw_text raw_number].include?(cell["type"]) && cell.fetch("text").empty?
              fail!("length-exceeded", "#{cell_path}.text", "empty cell")
            end
          end
        end
      end
      if data_table
        fail!("invalid-usage", "#{path}.column_settings", "unsupported") if wire.key?("column_settings")
        if characters(rows) > limit("data_table.content.max_length")
          fail!("length-exceeded", "#{path}.rows", "table content exceeded")
        end
      end
    end

    def self.chart(wire, path)
      axis = wire.fetch("axis_config").fetch("categories")
      # @type var names: Array[String]
      names = []
      wire.fetch("series").each_with_index do |series, index|
        item_path = "#{path}.series[#{index}]"
        names << series["name"]
        labels = series.fetch("data").map { |point| point["label"] }
        unless labels.length == axis.length && labels.sort == axis.sort
          fail!("invalid-usage", "#{item_path}.data", "points must cover every category exactly once")
        end
      end
      fail!("invalid-usage", "#{path}.series", "duplicate series name") if names.uniq.length != names.length
    end

    def self.check_limits(value, prefix, path)
      return unless prefix

      limits = Limits::VALUES
      length = if value.is_a?(String)
        value.length
      elsif value.is_a?(Array)
        value.length
      elsif value.is_a?(Hash) && value["text"].is_a?(String)
        value["text"].length
      end
      suffixes = value.is_a?(Array) ? %w[min_items max_items] : %w[min_length max_length]
      if length
        suffixes.each do |suffix|
          bound = limits["#{prefix}.#{suffix}"]
          next unless bound

          if suffix.start_with?("min") && length < bound
            fail!("length-exceeded", path, "expected at least #{bound}")
          elsif suffix.start_with?("max") && length > bound
            fail!("length-exceeded", path, "expected at most #{bound}")
          end
        end
      end
      if value.is_a?(Numeric)
        %w[min max exclusive_min exclusive_max].each do |suffix|
          bound = limits["#{prefix}.#{suffix}"]
          next unless bound

          invalid = case suffix
          when "min" then value < bound
          when "max" then value > bound
          when "exclusive_min" then value <= bound
          when "exclusive_max" then value >= bound
          end
          fail!("out-of-range", path, "outside #{suffix} #{bound}") if invalid
        end
      end
    end
  end
end
