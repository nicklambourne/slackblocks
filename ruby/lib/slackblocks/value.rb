require "json"

module Slackblocks
  module KeywordOnly
    def new(*args, **keywords)
      raise ArgumentError, "use keyword arguments" unless args.empty?

      super(**keywords)
    end

    def [](*args, **keywords)
      new(*args, **keywords)
    end
  end

  module Value
    def self.deep_freeze(value)
      case value
      when String
        value.freeze
      when Array
        value.each { |item| deep_freeze(item) }
        value.freeze
      when Hash
        value.each { |key, item|
          deep_freeze(key)
          deep_freeze(item)
        }
        value.freeze
      else
        value
      end
    end

    def self.fail!(category, path, message)
      raise ValidationError.new(category, path, message)
    end

    def self.string(value, path)
      fail!("type-mismatch", path, "expected a String") unless value.is_a?(String)
      fail!("type-mismatch", path, "invalid string encoding") unless value.valid_encoding?

      value.encode(Encoding::UTF_8).freeze
    rescue Encoding::UndefinedConversionError, Encoding::InvalidByteSequenceError
      fail!("type-mismatch", path, "cannot transcode to UTF-8")
    end

    def self.json_value(value, path, seen = {})
      case value
      when nil, true, false, Integer
        value
      when String
        string(value, path)
      when Float
        fail!("out-of-range", path, "number must be finite") unless value.finite?

        value
      when Value
        value
      when Array
        fail!("type-mismatch", path, "cyclic array") if seen[value.object_id]

        seen[value.object_id] = true
        copy = value.each_with_index.map { |item, index| json_value(item, "#{path}[#{index}]", seen) }.freeze
        seen.delete(value.object_id)
        copy
      when Hash
        fail!("type-mismatch", path, "cyclic map") if seen[value.object_id]

        seen[value.object_id] = true
        # @type var copy: Hash[String, untyped]
        copy = {}
        value.each do |key, item|
          fail!("type-mismatch", path, "map key must be a string or symbol") unless key.is_a?(String) || key.is_a?(Symbol)

          normalized = string(key.to_s, path)
          fail!("invalid-usage", "#{path}.#{normalized}", "duplicate key") if copy.key?(normalized)

          copy[normalized] = json_value(item, "#{path}.#{normalized}", seen)
        end
        seen.delete(value.object_id)
        copy.freeze
      else
        fail!("type-mismatch", path, "value cannot be represented as JSON")
      end
    end

    def self.normalize_field(field, value, path)
      kind = field.fetch("kind")
      target = field["target"] && Slackblocks.const_get(field["target"], false)
      case kind
      when "string"
        string(value, path)
      when "boolean"
        fail!("type-mismatch", path, "expected a boolean") unless value == true || value == false

        value
      when "int", "long"
        fail!("type-mismatch", path, "expected an Integer") unless value.is_a?(Integer)

        value
      when "double", "number"
        fail!("type-mismatch", path, "expected an Integer or Float") unless value.is_a?(Integer) || value.instance_of?(Float)
        fail!("out-of-range", path, "number must be finite") if value.is_a?(Float) && !value.finite?

        value
      when "enum"
        allowed = target.constants(false).map { |name| target.const_get(name, false) }
        fail!("type-mismatch", path, "unknown enum value") unless value.is_a?(Symbol) && allowed.include?(value)

        value
      when "map"
        fail!("type-mismatch", path, "expected a Hash") unless value.is_a?(Hash)

        json_value(value, path)
      when "style"
        fail!("type-mismatch", path, "expected RichTextStyle") unless value.is_a?(RichTextStyle)

        used = value.to_h.select { |name, flag| name != :additional_fields && !flag.nil? }.keys.map(&:to_s) + value.additional_fields.keys
        fail!("type-mismatch", path, "unsupported style flag") unless (used - field.fetch("flags")).empty?

        value
      when "object"
        fail!("type-mismatch", path, "expected #{target}") unless value.is_a?(target)

        value
      when "text"
        return Slackblocks.const_get((field["coerce"] == "plain_text") ? "PlainText" : "MarkdownText", false).new(text: value) if value.is_a?(String)

        fail!("type-mismatch", path, "expected text") unless value.is_a?(target)

        value
      when "list", "rows", "textList", "stringList"
        fail!("type-mismatch", path, "expected an Array") unless value.is_a?(Array)

        value.each_with_index.map do |item, index|
          item_path = "#{path}[#{index}]"
          if kind == "rows"
            fail!("type-mismatch", item_path, "expected a row Array") unless item.is_a?(Array)

            item.each_with_index.map do |cell, column|
              fail!("type-mismatch", "#{item_path}[#{column}]", "wrong cell type") unless cell.is_a?(target)

              cell
            end.freeze
          elsif kind == "textList"
            normalize_field({"kind" => "text", "target" => field["target"], "coerce" => field["coerce"]}, item, item_path)
          elsif kind == "stringList"
            string(item, item_path)
          else
            fail!("type-mismatch", item_path, "wrong item type") unless item.is_a?(target)

            item
          end
        end.freeze
      else
        raise "unhandled model field kind: #{kind}"
      end
    end

    def self.construct(klass, keywords)
      fields = klass::FIELD_SPECS
      names = fields.map { |field| field.fetch("name").to_sym }
      unknown = keywords.keys - names - [:additional_fields]
      raise ArgumentError, "unknown keywords: #{unknown.inspect}" unless unknown.empty?

      # @type var values: Hash[Symbol, untyped]
      values = {}
      fields.each do |field|
        name = field.fetch("name")
        symbol = name.to_sym
        path = "#{klass.name.split("::").last}.#{field.fetch("wire")}"
        value = if keywords.key?(symbol)
          keywords[symbol]
        else
          default = klass::DEFAULTS.fetch(name, nil)
          (field["kind"] == "enum" && default) ? default.to_sym : default
        end
        if value.nil?
          fail!("missing-required", path, "required field is missing") if field["required"]
          values[symbol] = nil
        else
          values[symbol] = normalize_field(field, value, path)
        end
      end
      raw_extra = keywords.fetch(:additional_fields, nil)
      if raw_extra.nil?
        values[:additional_fields] = deep_freeze({})
      else
        fail!("type-mismatch", "#{klass.name}.additional_fields", "expected a Hash") unless raw_extra.is_a?(Hash)

        normalized = json_value(raw_extra, "#{klass.name}.additional_fields")
        reserved = fields.map { |field| field.fetch("wire") } + ["type"]
        normalized.each_key do |key|
          fail!("invalid-usage", "#{klass.name}.#{key}", "field is reserved") if key.empty? || reserved.include?(key)
        end
        values[:additional_fields] = normalized
      end
      Validator.validate(klass, values)
      values
    end

    def self.detach(value)
      case value
      when Value
        value.as_json
      when Array
        value.map { |item| detach(item) }
      when Hash
        value.to_h { |key, item| [key.dup, detach(item)] }
      when String
        value.dup
      when Symbol
        value.to_s
      else
        value
      end
    end

    def self.wire(klass, values)
      # @type var wire: Hash[String, untyped]
      wire = {}
      wire["type"] = klass::WIRE_TYPE.dup unless klass::WIRE_TYPE.empty?
      klass::FIELD_SPECS.each do |field|
        value = values[field.fetch("name").to_sym]
        wire[field.fetch("wire").dup] = detach(value) unless value.nil?
      end
      values.fetch(:additional_fields).each { |key, value| wire[key.dup] = detach(value) }
      if klass.name == "Slackblocks::PlanBlock" && wire.key?("tasks")
        wire["tasks"].each { |task| task.delete("type") }
      elsif klass.name == "Slackblocks::Attachment" && wire["color"].is_a?(String)
        color = wire["color"]
        wire["color"] = "##{color}" if color.match?(/\A[0-9a-fA-F]{6}\z/)
      end
      wire
    end

    def initialize(**keywords)
      Value.construct(self.class, keywords).each do |name, value|
        instance_variable_set("@#{name}", value)
      end
      freeze
    end

    def to_h
      self.class.const_get(:FIELD_SPECS).to_h do |field|
        name = field.fetch("name").to_sym
        [name, public_send(name)]
      end.merge(additional_fields: additional_fields)
    end

    def with(**changes)
      changes.empty? ? self : self.class.public_send(:new, **to_h.merge(changes))
    end

    def ==(other)
      other.instance_of?(self.class) && to_h == other.to_h
    end

    alias_method :eql?, :==

    def hash
      [self.class, to_h].hash
    end

    def as_json(_options = nil)
      Value.wire(self.class, to_h)
    end

    def to_json(options = nil)
      options.nil? ? JSON.generate(as_json) : JSON.generate(as_json, options)
    end
  end
end
