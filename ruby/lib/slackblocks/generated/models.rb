# Generated from spec/model.json. Do not edit by hand.
module Slackblocks
  module Block
  end

  module ContextElement
  end

  module ContextActionsElement
  end

  module TableCell
  end

  module DataTableCell
  end

  module Element
  end

  module InputElement
  end

  module Text
  end

  module Chart
  end

  module RichTextBlockElement
  end

  module RichTextSectionElement
  end

  module AlertLevel
    DEFAULT = :default
    INFO = :info
    WARNING = :warning
    ERROR = :error
    SUCCESS = :success
  end

  module ContainerWidth
    NARROW = :narrow
    STANDARD = :standard
    WIDE = :wide
    FULL = :full
  end

  module TaskStatus
    PENDING = :pending
    IN_PROGRESS = :in_progress
    COMPLETE = :complete
    ERROR = :error
  end

  module ButtonStyle
    PRIMARY = :primary
    DANGER = :danger
  end

  module IconButtonIcon
    TRASH = :trash
  end

  module ColumnAlign
    LEFT = :left
    CENTER = :center
    RIGHT = :right
  end

  module RichTextListStyle
    BULLET = :bullet
    ORDERED = :ordered
  end

  module ResponseType
    IN_CHANNEL = :in_channel
    EPHEMERAL = :ephemeral
  end

  class ActionsBlock
    include Value
    include Block
    attr_reader :elements, :block_id, :additional_fields
  end
  ActionsBlock.const_set(:WIRE_TYPE, "actions")
  ActionsBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "Element", "coerce" => nil, "flags" => nil, "limits" => "actions.elements", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  ActionsBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  ActionsBlock.singleton_class.prepend(KeywordOnly)

  class AlertBlock
    include Value
    include Block
    attr_reader :text, :level, :block_id, :additional_fields
  end
  AlertBlock.const_set(:WIRE_TYPE, "alert")
  AlertBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "alert.text", "required" => true}, {"name" => "level", "wire" => "level", "kind" => "enum", "target" => "AlertLevel", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  AlertBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  AlertBlock.singleton_class.prepend(KeywordOnly)

  class CardBlock
    include Value
    include Block
    attr_reader :hero_image, :icon, :title, :subtitle, :body, :actions, :slack_icon, :subtext, :block_id, :additional_fields
  end
  CardBlock.const_set(:WIRE_TYPE, "card")
  CardBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "hero_image", "wire" => "hero_image", "kind" => "object", "target" => "ImageElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "icon", "wire" => "icon", "kind" => "object", "target" => "ImageElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "title", "wire" => "title", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "card.title", "required" => false}, {"name" => "subtitle", "wire" => "subtitle", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "card.subtitle", "required" => false}, {"name" => "body", "wire" => "body", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "card.body", "required" => false}, {"name" => "actions", "wire" => "actions", "kind" => "list", "target" => "ButtonElement", "coerce" => nil, "flags" => nil, "limits" => "card.actions", "required" => false}, {"name" => "slack_icon", "wire" => "slack_icon", "kind" => "object", "target" => "SlackIcon", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "subtext", "wire" => "subtext", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "card.subtext", "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  CardBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  CardBlock.singleton_class.prepend(KeywordOnly)

  class CarouselBlock
    include Value
    include Block
    attr_reader :elements, :block_id, :additional_fields
  end
  CarouselBlock.const_set(:WIRE_TYPE, "carousel")
  CarouselBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "CardBlock", "coerce" => nil, "flags" => nil, "limits" => "carousel.elements", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  CarouselBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  CarouselBlock.singleton_class.prepend(KeywordOnly)

  class ContainerBlock
    include Value
    include Block
    attr_reader :child_blocks, :title, :rich_text_title, :subtitle, :width, :icon, :is_collapsible, :default_collapsed, :has_header_divider, :block_id, :additional_fields
  end
  ContainerBlock.const_set(:WIRE_TYPE, "container")
  ContainerBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "child_blocks", "wire" => "child_blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "container.child_blocks", "required" => true}, {"name" => "title", "wire" => "title", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "container.title", "required" => false}, {"name" => "rich_text_title", "wire" => "rich_text_title", "kind" => "object", "target" => "RichTextBlock", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "subtitle", "wire" => "subtitle", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "container.subtitle", "required" => false}, {"name" => "width", "wire" => "width", "kind" => "enum", "target" => "ContainerWidth", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "icon", "wire" => "icon", "kind" => "object", "target" => "ImageElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "is_collapsible", "wire" => "is_collapsible", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "default_collapsed", "wire" => "default_collapsed", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "has_header_divider", "wire" => "has_header_divider", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  ContainerBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  ContainerBlock.singleton_class.prepend(KeywordOnly)

  class ContextActionsBlock
    include Value
    include Block
    attr_reader :elements, :block_id, :additional_fields
  end
  ContextActionsBlock.const_set(:WIRE_TYPE, "context_actions")
  ContextActionsBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "ContextActionsElement", "coerce" => nil, "flags" => nil, "limits" => "context_actions.elements", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  ContextActionsBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  ContextActionsBlock.singleton_class.prepend(KeywordOnly)

  class ContextBlock
    include Value
    include Block
    attr_reader :elements, :block_id, :additional_fields
  end
  ContextBlock.const_set(:WIRE_TYPE, "context")
  ContextBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "ContextElement", "coerce" => nil, "flags" => nil, "limits" => "context.elements", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  ContextBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  ContextBlock.singleton_class.prepend(KeywordOnly)

  class DataTableBlock
    include Value
    include Block
    attr_reader :rows, :caption, :page_size, :row_header_column_index, :block_id, :additional_fields
  end
  DataTableBlock.const_set(:WIRE_TYPE, "data_table")
  DataTableBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "rows", "wire" => "rows", "kind" => "rows", "target" => "DataTableCell", "coerce" => nil, "flags" => nil, "limits" => "data_table.rows", "required" => true}, {"name" => "caption", "wire" => "caption", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "page_size", "wire" => "page_size", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_table.page_size", "required" => false}, {"name" => "row_header_column_index", "wire" => "row_header_column_index", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_table.row_header_column_index", "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  DataTableBlock.const_set(:DEFAULTS, Value.deep_freeze({"page_size" => 5, "row_header_column_index" => 0}))
  DataTableBlock.singleton_class.prepend(KeywordOnly)

  class DataVisualizationBlock
    include Value
    include Block
    attr_reader :title, :chart, :block_id, :additional_fields
  end
  DataVisualizationBlock.const_set(:WIRE_TYPE, "data_visualization")
  DataVisualizationBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "title", "wire" => "title", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.title", "required" => true}, {"name" => "chart", "wire" => "chart", "kind" => "object", "target" => "Chart", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  DataVisualizationBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  DataVisualizationBlock.singleton_class.prepend(KeywordOnly)

  class DividerBlock
    include Value
    include Block
    attr_reader :block_id, :additional_fields
  end
  DividerBlock.const_set(:WIRE_TYPE, "divider")
  DividerBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  DividerBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  DividerBlock.singleton_class.prepend(KeywordOnly)

  class FileBlock
    include Value
    include Block
    attr_reader :external_id, :source, :block_id, :additional_fields
  end
  FileBlock.const_set(:WIRE_TYPE, "file")
  FileBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "external_id", "wire" => "external_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "source", "wire" => "source", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  FileBlock.const_set(:DEFAULTS, Value.deep_freeze({"source" => "remote"}))
  FileBlock.singleton_class.prepend(KeywordOnly)

  class HeaderBlock
    include Value
    include Block
    attr_reader :text, :block_id, :additional_fields
  end
  HeaderBlock.const_set(:WIRE_TYPE, "header")
  HeaderBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "header.text", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  HeaderBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  HeaderBlock.singleton_class.prepend(KeywordOnly)

  class ImageBlock
    include Value
    include Block
    attr_reader :image_url, :slack_file, :alt_text, :title, :block_id, :additional_fields
  end
  ImageBlock.const_set(:WIRE_TYPE, "image")
  ImageBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "image_url", "wire" => "image_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "image.image_url", "required" => false}, {"name" => "slack_file", "wire" => "slack_file", "kind" => "object", "target" => "SlackFile", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "alt_text", "wire" => "alt_text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "image.alt_text", "required" => true}, {"name" => "title", "wire" => "title", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "image.title", "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  ImageBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  ImageBlock.singleton_class.prepend(KeywordOnly)

  class InputBlock
    include Value
    include Block
    attr_reader :label, :element, :dispatch_action, :block_id, :hint, :optional, :additional_fields
  end
  InputBlock.const_set(:WIRE_TYPE, "input")
  InputBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "label", "wire" => "label", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "input.label", "required" => true}, {"name" => "element", "wire" => "element", "kind" => "object", "target" => "InputElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "dispatch_action", "wire" => "dispatch_action", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}, {"name" => "hint", "wire" => "hint", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "input.hint", "required" => false}, {"name" => "optional", "wire" => "optional", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  InputBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  InputBlock.singleton_class.prepend(KeywordOnly)

  class MarkdownBlock
    include Value
    include Block
    attr_reader :text, :block_id, :additional_fields
  end
  MarkdownBlock.const_set(:WIRE_TYPE, "markdown")
  MarkdownBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "markdown.text", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  MarkdownBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  MarkdownBlock.singleton_class.prepend(KeywordOnly)

  class PlanBlock
    include Value
    include Block
    attr_reader :title, :tasks, :block_id, :additional_fields
  end
  PlanBlock.const_set(:WIRE_TYPE, "plan")
  PlanBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "title", "wire" => "title", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "tasks", "wire" => "tasks", "kind" => "list", "target" => "TaskCardBlock", "coerce" => nil, "flags" => nil, "limits" => "plan.tasks", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  PlanBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  PlanBlock.singleton_class.prepend(KeywordOnly)

  class RichTextBlock
    include Value
    include Block
    include DataTableCell
    include TableCell
    attr_reader :elements, :block_id, :additional_fields
  end
  RichTextBlock.const_set(:WIRE_TYPE, "rich_text")
  RichTextBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "RichTextBlockElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  RichTextBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextBlock.singleton_class.prepend(KeywordOnly)

  class SectionBlock
    include Value
    include Block
    attr_reader :text, :fields, :accessory, :block_id, :additional_fields
  end
  SectionBlock.const_set(:WIRE_TYPE, "section")
  SectionBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "section.text", "required" => false}, {"name" => "fields", "wire" => "fields", "kind" => "textList", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "section.fields", "required" => false}, {"name" => "accessory", "wire" => "accessory", "kind" => "object", "target" => "Element", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  SectionBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  SectionBlock.singleton_class.prepend(KeywordOnly)

  class TableBlock
    include Value
    include Block
    attr_reader :rows, :column_settings, :block_id, :additional_fields
  end
  TableBlock.const_set(:WIRE_TYPE, "table")
  TableBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "rows", "wire" => "rows", "kind" => "rows", "target" => "TableCell", "coerce" => nil, "flags" => nil, "limits" => "table.rows", "required" => true}, {"name" => "column_settings", "wire" => "column_settings", "kind" => "list", "target" => "ColumnSettings", "coerce" => nil, "flags" => nil, "limits" => "table.column_settings", "required" => false}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  TableBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  TableBlock.singleton_class.prepend(KeywordOnly)

  class TaskCardBlock
    include Value
    include Block
    attr_reader :task_id, :title, :details, :output, :sources, :status, :block_id, :additional_fields
  end
  TaskCardBlock.const_set(:WIRE_TYPE, "task_card")
  TaskCardBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "task_id", "wire" => "task_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "title", "wire" => "title", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "details", "wire" => "details", "kind" => "object", "target" => "RichTextBlock", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "output", "wire" => "output", "kind" => "object", "target" => "RichTextBlock", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "sources", "wire" => "sources", "kind" => "list", "target" => "UrlSource", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "status", "wire" => "status", "kind" => "enum", "target" => "TaskStatus", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}]))
  TaskCardBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  TaskCardBlock.singleton_class.prepend(KeywordOnly)

  class VideoBlock
    include Value
    include Block
    attr_reader :alt_text, :thumbnail_url, :title, :video_url, :block_id, :author_name, :description, :provider_icon_url, :provider_name, :title_url, :additional_fields
  end
  VideoBlock.const_set(:WIRE_TYPE, "video")
  VideoBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "alt_text", "wire" => "alt_text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.alt_text", "required" => true}, {"name" => "thumbnail_url", "wire" => "thumbnail_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.thumbnail_url", "required" => true}, {"name" => "title", "wire" => "title", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "video.title", "required" => true}, {"name" => "video_url", "wire" => "video_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.video_url", "required" => true}, {"name" => "block_id", "wire" => "block_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "block_id", "required" => false}, {"name" => "author_name", "wire" => "author_name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.author_name", "required" => false}, {"name" => "description", "wire" => "description", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "video.description", "required" => false}, {"name" => "provider_icon_url", "wire" => "provider_icon_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.provider_icon_url", "required" => false}, {"name" => "provider_name", "wire" => "provider_name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.provider_name", "required" => false}, {"name" => "title_url", "wire" => "title_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "video.title_url", "required" => false}]))
  VideoBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  VideoBlock.singleton_class.prepend(KeywordOnly)

  class ButtonElement
    include Value
    include Element
    attr_reader :text, :action_id, :url, :value, :style, :confirm, :accessibility_label, :additional_fields
  end
  ButtonElement.const_set(:WIRE_TYPE, "button")
  ButtonElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "button.text", "required" => true}, {"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "button.url", "required" => false}, {"name" => "value", "wire" => "value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "button.value", "required" => false}, {"name" => "style", "wire" => "style", "kind" => "enum", "target" => "ButtonStyle", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "accessibility_label", "wire" => "accessibility_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "button.accessibility_label", "required" => false}]))
  ButtonElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ButtonElement.singleton_class.prepend(KeywordOnly)

  class ChannelMultiSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_channels, :confirm, :max_selected_items, :focus_on_load, :placeholder, :additional_fields
  end
  ChannelMultiSelectElement.const_set(:WIRE_TYPE, "multi_channels_select")
  ChannelMultiSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_channels", "wire" => "initial_channels", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_selected_items", "wire" => "max_selected_items", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "multi_select.max_selected_items", "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ChannelMultiSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ChannelMultiSelectElement.singleton_class.prepend(KeywordOnly)

  class ChannelSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_channel, :response_url_enabled, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  ChannelSelectElement.const_set(:WIRE_TYPE, "channels_select")
  ChannelSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_channel", "wire" => "initial_channel", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "response_url_enabled", "wire" => "response_url_enabled", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ChannelSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ChannelSelectElement.singleton_class.prepend(KeywordOnly)

  class CheckboxesElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :options, :initial_options, :confirm, :focus_on_load, :additional_fields
  end
  CheckboxesElement.const_set(:WIRE_TYPE, "checkboxes")
  CheckboxesElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "checkboxes.options", "required" => true}, {"name" => "initial_options", "wire" => "initial_options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  CheckboxesElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  CheckboxesElement.singleton_class.prepend(KeywordOnly)

  class ConversationMultiSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_conversations, :default_to_current_conversation, :filter, :confirm, :max_selected_items, :focus_on_load, :placeholder, :additional_fields
  end
  ConversationMultiSelectElement.const_set(:WIRE_TYPE, "multi_conversations_select")
  ConversationMultiSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_conversations", "wire" => "initial_conversations", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "default_to_current_conversation", "wire" => "default_to_current_conversation", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "filter", "wire" => "filter", "kind" => "object", "target" => "ConversationFilter", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_selected_items", "wire" => "max_selected_items", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "multi_select.max_selected_items", "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ConversationMultiSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ConversationMultiSelectElement.singleton_class.prepend(KeywordOnly)

  class ConversationSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_conversation, :default_to_current_conversation, :filter, :response_url_enabled, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  ConversationSelectElement.const_set(:WIRE_TYPE, "conversations_select")
  ConversationSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_conversation", "wire" => "initial_conversation", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "default_to_current_conversation", "wire" => "default_to_current_conversation", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "filter", "wire" => "filter", "kind" => "object", "target" => "ConversationFilter", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "response_url_enabled", "wire" => "response_url_enabled", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ConversationSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ConversationSelectElement.singleton_class.prepend(KeywordOnly)

  class DatePickerElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_date, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  DatePickerElement.const_set(:WIRE_TYPE, "datepicker")
  DatePickerElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_date", "wire" => "initial_date", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "date_picker.placeholder", "required" => false}]))
  DatePickerElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  DatePickerElement.singleton_class.prepend(KeywordOnly)

  class DateTimePickerElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_date_time, :confirm, :focus_on_load, :additional_fields
  end
  DateTimePickerElement.const_set(:WIRE_TYPE, "datetimepicker")
  DateTimePickerElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_date_time", "wire" => "initial_date_time", "kind" => "long", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  DateTimePickerElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  DateTimePickerElement.singleton_class.prepend(KeywordOnly)

  class EmailInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_value, :dispatch_action_config, :focus_on_load, :placeholder, :additional_fields
  end
  EmailInputElement.const_set(:WIRE_TYPE, "email_text_input")
  EmailInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_value", "wire" => "initial_value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "dispatch_action_config", "wire" => "dispatch_action_config", "kind" => "object", "target" => "DispatchActionConfiguration", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "email_input.placeholder", "required" => false}]))
  EmailInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  EmailInputElement.singleton_class.prepend(KeywordOnly)

  class ExternalMultiSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :min_query_length, :initial_options, :confirm, :max_selected_items, :focus_on_load, :placeholder, :additional_fields
  end
  ExternalMultiSelectElement.const_set(:WIRE_TYPE, "multi_external_select")
  ExternalMultiSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "min_query_length", "wire" => "min_query_length", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "initial_options", "wire" => "initial_options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_selected_items", "wire" => "max_selected_items", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "multi_select.max_selected_items", "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ExternalMultiSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ExternalMultiSelectElement.singleton_class.prepend(KeywordOnly)

  class ExternalSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :min_query_length, :initial_option, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  ExternalSelectElement.const_set(:WIRE_TYPE, "external_select")
  ExternalSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "min_query_length", "wire" => "min_query_length", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "initial_option", "wire" => "initial_option", "kind" => "object", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  ExternalSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ExternalSelectElement.singleton_class.prepend(KeywordOnly)

  class FeedbackButtonsElement
    include Value
    include ContextActionsElement
    include Element
    attr_reader :positive_button, :negative_button, :action_id, :additional_fields
  end
  FeedbackButtonsElement.const_set(:WIRE_TYPE, "feedback_buttons")
  FeedbackButtonsElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "positive_button", "wire" => "positive_button", "kind" => "object", "target" => "FeedbackButton", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "negative_button", "wire" => "negative_button", "kind" => "object", "target" => "FeedbackButton", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}]))
  FeedbackButtonsElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  FeedbackButtonsElement.singleton_class.prepend(KeywordOnly)

  class FileInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :filetypes, :max_files, :additional_fields
  end
  FileInputElement.const_set(:WIRE_TYPE, "file_input")
  FileInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "filetypes", "wire" => "filetypes", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_files", "wire" => "max_files", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "file_input.max_files", "required" => false}]))
  FileInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  FileInputElement.singleton_class.prepend(KeywordOnly)

  class IconButtonElement
    include Value
    include ContextActionsElement
    include Element
    attr_reader :text, :icon, :action_id, :value, :confirm, :accessibility_label, :visible_to_user_ids, :additional_fields
  end
  IconButtonElement.const_set(:WIRE_TYPE, "icon_button")
  IconButtonElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => nil, "required" => true}, {"name" => "icon", "wire" => "icon", "kind" => "enum", "target" => "IconButtonIcon", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "value", "wire" => "value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "icon_button.value", "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "accessibility_label", "wire" => "accessibility_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "icon_button.accessibility_label", "required" => false}, {"name" => "visible_to_user_ids", "wire" => "visible_to_user_ids", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "icon_button.visible_to_user_ids", "required" => false}]))
  IconButtonElement.const_set(:DEFAULTS, Value.deep_freeze({"icon" => "trash"}))
  IconButtonElement.singleton_class.prepend(KeywordOnly)

  class ImageElement
    include Value
    include ContextElement
    include Element
    attr_reader :alt_text, :image_url, :slack_file, :additional_fields
  end
  ImageElement.const_set(:WIRE_TYPE, "image")
  ImageElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "alt_text", "wire" => "alt_text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "image_element.alt_text", "required" => true}, {"name" => "image_url", "wire" => "image_url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "image_element.image_url", "required" => false}, {"name" => "slack_file", "wire" => "slack_file", "kind" => "object", "target" => "SlackFile", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  ImageElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  ImageElement.singleton_class.prepend(KeywordOnly)

  class NumberInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :is_decimal_allowed, :initial_value, :min_value, :max_value, :dispatch_action_config, :focus_on_load, :placeholder, :additional_fields
  end
  NumberInputElement.const_set(:WIRE_TYPE, "number_input")
  NumberInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "is_decimal_allowed", "wire" => "is_decimal_allowed", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "initial_value", "wire" => "initial_value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "min_value", "wire" => "min_value", "kind" => "double", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_value", "wire" => "max_value", "kind" => "double", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "dispatch_action_config", "wire" => "dispatch_action_config", "kind" => "object", "target" => "DispatchActionConfiguration", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "number_input.placeholder", "required" => false}]))
  NumberInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  NumberInputElement.singleton_class.prepend(KeywordOnly)

  class OverflowElement
    include Value
    include Element
    attr_reader :action_id, :options, :confirm, :additional_fields
  end
  OverflowElement.const_set(:WIRE_TYPE, "overflow")
  OverflowElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "overflow.options", "required" => true}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  OverflowElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  OverflowElement.singleton_class.prepend(KeywordOnly)

  class PlainTextInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_value, :multiline, :min_length, :max_length, :dispatch_action_config, :focus_on_load, :placeholder, :additional_fields
  end
  PlainTextInputElement.const_set(:WIRE_TYPE, "plain_text_input")
  PlainTextInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_value", "wire" => "initial_value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "multiline", "wire" => "multiline", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "min_length", "wire" => "min_length", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "plain_text_input.min_length", "required" => false}, {"name" => "max_length", "wire" => "max_length", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "plain_text_input.max_length", "required" => false}, {"name" => "dispatch_action_config", "wire" => "dispatch_action_config", "kind" => "object", "target" => "DispatchActionConfiguration", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "plain_text_input.placeholder", "required" => false}]))
  PlainTextInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  PlainTextInputElement.singleton_class.prepend(KeywordOnly)

  class RadioButtonsElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :options, :initial_option, :confirm, :focus_on_load, :additional_fields
  end
  RadioButtonsElement.const_set(:WIRE_TYPE, "radio_buttons")
  RadioButtonsElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "radio_buttons.options", "required" => true}, {"name" => "initial_option", "wire" => "initial_option", "kind" => "object", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  RadioButtonsElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  RadioButtonsElement.singleton_class.prepend(KeywordOnly)

  class RichTextInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_value, :dispatch_action_config, :focus_on_load, :placeholder, :min_lines, :max_lines, :additional_fields
  end
  RichTextInputElement.const_set(:WIRE_TYPE, "rich_text_input")
  RichTextInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => true}, {"name" => "initial_value", "wire" => "initial_value", "kind" => "object", "target" => "RichTextBlock", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "dispatch_action_config", "wire" => "dispatch_action_config", "kind" => "object", "target" => "DispatchActionConfiguration", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "rich_text_input.placeholder", "required" => false}, {"name" => "min_lines", "wire" => "min_lines", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_input.min_lines", "required" => false}, {"name" => "max_lines", "wire" => "max_lines", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_input.max_lines", "required" => false}]))
  RichTextInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextInputElement.singleton_class.prepend(KeywordOnly)

  class StaticMultiSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :options, :option_groups, :initial_options, :confirm, :max_selected_items, :focus_on_load, :placeholder, :additional_fields
  end
  StaticMultiSelectElement.const_set(:WIRE_TYPE, "multi_static_select")
  StaticMultiSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "select.options", "required" => false}, {"name" => "option_groups", "wire" => "option_groups", "kind" => "list", "target" => "OptionGroup", "coerce" => nil, "flags" => nil, "limits" => "select.option_groups", "required" => false}, {"name" => "initial_options", "wire" => "initial_options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_selected_items", "wire" => "max_selected_items", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "multi_select.max_selected_items", "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  StaticMultiSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  StaticMultiSelectElement.singleton_class.prepend(KeywordOnly)

  class StaticSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :options, :option_groups, :initial_option, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  StaticSelectElement.const_set(:WIRE_TYPE, "static_select")
  StaticSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "select.options", "required" => false}, {"name" => "option_groups", "wire" => "option_groups", "kind" => "list", "target" => "OptionGroup", "coerce" => nil, "flags" => nil, "limits" => "select.option_groups", "required" => false}, {"name" => "initial_option", "wire" => "initial_option", "kind" => "object", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  StaticSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  StaticSelectElement.singleton_class.prepend(KeywordOnly)

  class TimePickerElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_time, :timezone, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  TimePickerElement.const_set(:WIRE_TYPE, "timepicker")
  TimePickerElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_time", "wire" => "initial_time", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "timezone", "wire" => "timezone", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "time_picker.placeholder", "required" => false}]))
  TimePickerElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  TimePickerElement.singleton_class.prepend(KeywordOnly)

  class UrlInputElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_value, :dispatch_action_config, :focus_on_load, :placeholder, :additional_fields
  end
  UrlInputElement.const_set(:WIRE_TYPE, "url_text_input")
  UrlInputElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_value", "wire" => "initial_value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "dispatch_action_config", "wire" => "dispatch_action_config", "kind" => "object", "target" => "DispatchActionConfiguration", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "url_input.placeholder", "required" => false}]))
  UrlInputElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  UrlInputElement.singleton_class.prepend(KeywordOnly)

  class UserMultiSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_users, :confirm, :max_selected_items, :focus_on_load, :placeholder, :additional_fields
  end
  UserMultiSelectElement.const_set(:WIRE_TYPE, "multi_users_select")
  UserMultiSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_users", "wire" => "initial_users", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "max_selected_items", "wire" => "max_selected_items", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "multi_select.max_selected_items", "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  UserMultiSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  UserMultiSelectElement.singleton_class.prepend(KeywordOnly)

  class UserSelectElement
    include Value
    include Element
    include InputElement
    attr_reader :action_id, :initial_user, :confirm, :focus_on_load, :placeholder, :additional_fields
  end
  UserSelectElement.const_set(:WIRE_TYPE, "users_select")
  UserSelectElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => false}, {"name" => "initial_user", "wire" => "initial_user", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "focus_on_load", "wire" => "focus_on_load", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "placeholder", "wire" => "placeholder", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "select.placeholder", "required" => false}]))
  UserSelectElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  UserSelectElement.singleton_class.prepend(KeywordOnly)

  class WorkflowButtonElement
    include Value
    include Element
    attr_reader :text, :workflow, :action_id, :confirm, :style, :accessibility_label, :additional_fields
  end
  WorkflowButtonElement.const_set(:WIRE_TYPE, "workflow_button")
  WorkflowButtonElement.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "workflow_button.text", "required" => true}, {"name" => "workflow", "wire" => "workflow", "kind" => "object", "target" => "Workflow", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "action_id", "wire" => "action_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "action_id", "required" => true}, {"name" => "confirm", "wire" => "confirm", "kind" => "object", "target" => "Confirmation", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "style", "wire" => "style", "kind" => "enum", "target" => "ButtonStyle", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "accessibility_label", "wire" => "accessibility_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "workflow_button.accessibility_label", "required" => false}]))
  WorkflowButtonElement.const_set(:DEFAULTS, Value.deep_freeze({}))
  WorkflowButtonElement.singleton_class.prepend(KeywordOnly)

  class AreaChart
    include Value
    include Chart
    attr_reader :series, :axis_config, :additional_fields
  end
  AreaChart.const_set(:WIRE_TYPE, "area")
  AreaChart.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "series", "wire" => "series", "kind" => "list", "target" => "DataSeries", "coerce" => nil, "flags" => nil, "limits" => "data_visualization.series", "required" => true}, {"name" => "axis_config", "wire" => "axis_config", "kind" => "object", "target" => "AxisConfig", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  AreaChart.const_set(:DEFAULTS, Value.deep_freeze({}))
  AreaChart.singleton_class.prepend(KeywordOnly)

  class AxisConfig
    include Value
    attr_reader :categories, :x_label, :y_label, :additional_fields
  end
  AxisConfig.const_set(:WIRE_TYPE, "")
  AxisConfig.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "categories", "wire" => "categories", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.categories", "required" => true}, {"name" => "x_label", "wire" => "x_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.axis_label", "required" => false}, {"name" => "y_label", "wire" => "y_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.axis_label", "required" => false}]))
  AxisConfig.const_set(:DEFAULTS, Value.deep_freeze({}))
  AxisConfig.singleton_class.prepend(KeywordOnly)

  class BarChart
    include Value
    include Chart
    attr_reader :series, :axis_config, :additional_fields
  end
  BarChart.const_set(:WIRE_TYPE, "bar")
  BarChart.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "series", "wire" => "series", "kind" => "list", "target" => "DataSeries", "coerce" => nil, "flags" => nil, "limits" => "data_visualization.series", "required" => true}, {"name" => "axis_config", "wire" => "axis_config", "kind" => "object", "target" => "AxisConfig", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  BarChart.const_set(:DEFAULTS, Value.deep_freeze({}))
  BarChart.singleton_class.prepend(KeywordOnly)

  class ChartSegment
    include Value
    attr_reader :label, :value, :additional_fields
  end
  ChartSegment.const_set(:WIRE_TYPE, "")
  ChartSegment.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "label", "wire" => "label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.segment.label", "required" => true}, {"name" => "value", "wire" => "value", "kind" => "number", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  ChartSegment.const_set(:DEFAULTS, Value.deep_freeze({}))
  ChartSegment.singleton_class.prepend(KeywordOnly)

  class ColumnSettings
    include Value
    attr_reader :align, :is_wrapped, :additional_fields
  end
  ColumnSettings.const_set(:WIRE_TYPE, "")
  ColumnSettings.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "align", "wire" => "align", "kind" => "enum", "target" => "ColumnAlign", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "is_wrapped", "wire" => "is_wrapped", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  ColumnSettings.const_set(:DEFAULTS, Value.deep_freeze({}))
  ColumnSettings.singleton_class.prepend(KeywordOnly)

  class Confirmation
    include Value
    attr_reader :title, :text, :confirm, :deny, :style, :additional_fields
  end
  Confirmation.const_set(:WIRE_TYPE, "")
  Confirmation.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "title", "wire" => "title", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "confirmation.title", "required" => true}, {"name" => "text", "wire" => "text", "kind" => "text", "target" => "Text", "coerce" => "mrkdwn", "flags" => nil, "limits" => "confirmation.text", "required" => true}, {"name" => "confirm", "wire" => "confirm", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "confirmation.confirm", "required" => true}, {"name" => "deny", "wire" => "deny", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "confirmation.deny", "required" => true}, {"name" => "style", "wire" => "style", "kind" => "enum", "target" => "ButtonStyle", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  Confirmation.const_set(:DEFAULTS, Value.deep_freeze({}))
  Confirmation.singleton_class.prepend(KeywordOnly)

  class ConversationFilter
    include Value
    attr_reader :include, :exclude_external_shared_channels, :exclude_bot_users, :additional_fields
  end
  ConversationFilter.const_set(:WIRE_TYPE, "")
  ConversationFilter.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "include", "wire" => "include", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "conversation_filter.include", "required" => false}, {"name" => "exclude_external_shared_channels", "wire" => "exclude_external_shared_channels", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "exclude_bot_users", "wire" => "exclude_bot_users", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  ConversationFilter.const_set(:DEFAULTS, Value.deep_freeze({}))
  ConversationFilter.singleton_class.prepend(KeywordOnly)

  class DataPoint
    include Value
    attr_reader :label, :value, :additional_fields
  end
  DataPoint.const_set(:WIRE_TYPE, "")
  DataPoint.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "label", "wire" => "label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.point_label", "required" => true}, {"name" => "value", "wire" => "value", "kind" => "number", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  DataPoint.const_set(:DEFAULTS, Value.deep_freeze({}))
  DataPoint.singleton_class.prepend(KeywordOnly)

  class DataSeries
    include Value
    attr_reader :name, :data, :additional_fields
  end
  DataSeries.const_set(:WIRE_TYPE, "")
  DataSeries.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "name", "wire" => "name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "data_visualization.series_name", "required" => true}, {"name" => "data", "wire" => "data", "kind" => "list", "target" => "DataPoint", "coerce" => nil, "flags" => nil, "limits" => "data_visualization.data", "required" => true}]))
  DataSeries.const_set(:DEFAULTS, Value.deep_freeze({}))
  DataSeries.singleton_class.prepend(KeywordOnly)

  class DispatchActionConfiguration
    include Value
    attr_reader :trigger_actions_on, :additional_fields
  end
  DispatchActionConfiguration.const_set(:WIRE_TYPE, "")
  DispatchActionConfiguration.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "trigger_actions_on", "wire" => "trigger_actions_on", "kind" => "stringList", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "dispatch_action_configuration.trigger_actions_on", "required" => false}]))
  DispatchActionConfiguration.const_set(:DEFAULTS, Value.deep_freeze({}))
  DispatchActionConfiguration.singleton_class.prepend(KeywordOnly)

  class FeedbackButton
    include Value
    attr_reader :text, :value, :accessibility_label, :additional_fields
  end
  FeedbackButton.const_set(:WIRE_TYPE, "")
  FeedbackButton.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "feedback_button.text", "required" => true}, {"name" => "value", "wire" => "value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "feedback_button.value", "required" => true}, {"name" => "accessibility_label", "wire" => "accessibility_label", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "feedback_button.accessibility_label", "required" => false}]))
  FeedbackButton.const_set(:DEFAULTS, Value.deep_freeze({}))
  FeedbackButton.singleton_class.prepend(KeywordOnly)

  class InputParameter
    include Value
    attr_reader :name, :value, :additional_fields
  end
  InputParameter.const_set(:WIRE_TYPE, "")
  InputParameter.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "name", "wire" => "name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "value", "wire" => "value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  InputParameter.const_set(:DEFAULTS, Value.deep_freeze({}))
  InputParameter.singleton_class.prepend(KeywordOnly)

  class LineChart
    include Value
    include Chart
    attr_reader :series, :axis_config, :additional_fields
  end
  LineChart.const_set(:WIRE_TYPE, "line")
  LineChart.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "series", "wire" => "series", "kind" => "list", "target" => "DataSeries", "coerce" => nil, "flags" => nil, "limits" => "data_visualization.series", "required" => true}, {"name" => "axis_config", "wire" => "axis_config", "kind" => "object", "target" => "AxisConfig", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  LineChart.const_set(:DEFAULTS, Value.deep_freeze({}))
  LineChart.singleton_class.prepend(KeywordOnly)

  class MarkdownText
    include Value
    include ContextElement
    include Text
    attr_reader :text, :verbatim, :additional_fields
  end
  MarkdownText.const_set(:WIRE_TYPE, "mrkdwn")
  MarkdownText.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "verbatim", "wire" => "verbatim", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  MarkdownText.const_set(:DEFAULTS, Value.deep_freeze({}))
  MarkdownText.singleton_class.prepend(KeywordOnly)

  class Option
    include Value
    attr_reader :text, :value, :description, :url, :additional_fields
  end
  Option.const_set(:WIRE_TYPE, "")
  Option.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "text", "target" => "Text", "coerce" => "plain_text", "flags" => nil, "limits" => "option.text", "required" => true}, {"name" => "value", "wire" => "value", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "option.value", "required" => true}, {"name" => "description", "wire" => "description", "kind" => "text", "target" => "Text", "coerce" => "plain_text", "flags" => nil, "limits" => "option.description", "required" => false}, {"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "option.url", "required" => false}]))
  Option.const_set(:DEFAULTS, Value.deep_freeze({}))
  Option.singleton_class.prepend(KeywordOnly)

  class OptionGroup
    include Value
    attr_reader :label, :options, :additional_fields
  end
  OptionGroup.const_set(:WIRE_TYPE, "")
  OptionGroup.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "label", "wire" => "label", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "option_group.label", "required" => true}, {"name" => "options", "wire" => "options", "kind" => "list", "target" => "Option", "coerce" => nil, "flags" => nil, "limits" => "option_group.options", "required" => true}]))
  OptionGroup.const_set(:DEFAULTS, Value.deep_freeze({}))
  OptionGroup.singleton_class.prepend(KeywordOnly)

  class PieChart
    include Value
    include Chart
    attr_reader :segments, :additional_fields
  end
  PieChart.const_set(:WIRE_TYPE, "pie")
  PieChart.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "segments", "wire" => "segments", "kind" => "list", "target" => "ChartSegment", "coerce" => nil, "flags" => nil, "limits" => "data_visualization.segments", "required" => true}]))
  PieChart.const_set(:DEFAULTS, Value.deep_freeze({}))
  PieChart.singleton_class.prepend(KeywordOnly)

  class PlainText
    include Value
    include ContextElement
    include Text
    attr_reader :text, :emoji, :additional_fields
  end
  PlainText.const_set(:WIRE_TYPE, "plain_text")
  PlainText.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "emoji", "wire" => "emoji", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  PlainText.const_set(:DEFAULTS, Value.deep_freeze({}))
  PlainText.singleton_class.prepend(KeywordOnly)

  class RawNumber
    include Value
    include DataTableCell
    attr_reader :value, :text, :additional_fields
  end
  RawNumber.const_set(:WIRE_TYPE, "raw_number")
  RawNumber.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "value", "wire" => "value", "kind" => "number", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  RawNumber.const_set(:DEFAULTS, Value.deep_freeze({}))
  RawNumber.singleton_class.prepend(KeywordOnly)

  class RawText
    include Value
    include DataTableCell
    include TableCell
    attr_reader :text, :additional_fields
  end
  RawText.const_set(:WIRE_TYPE, "raw_text")
  RawText.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  RawText.const_set(:DEFAULTS, Value.deep_freeze({}))
  RawText.singleton_class.prepend(KeywordOnly)

  class RichTextChannel
    include Value
    include RichTextSectionElement
    attr_reader :channel_id, :style, :additional_fields
  end
  RichTextChannel.const_set(:WIRE_TYPE, "channel")
  RichTextChannel.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "channel_id", "wire" => "channel_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "style", "wire" => "style", "kind" => "style", "target" => nil, "coerce" => nil, "flags" => ["bold", "italic", "strike", "highlight", "client_highlight", "unlink"], "limits" => nil, "required" => false}]))
  RichTextChannel.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextChannel.singleton_class.prepend(KeywordOnly)

  class RichTextCodeBlock
    include Value
    include RichTextBlockElement
    attr_reader :elements, :border, :additional_fields
  end
  RichTextCodeBlock.const_set(:WIRE_TYPE, "rich_text_preformatted")
  RichTextCodeBlock.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "RichTextSectionElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "border", "wire" => "border", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_preformatted.border", "required" => false}]))
  RichTextCodeBlock.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextCodeBlock.singleton_class.prepend(KeywordOnly)

  class RichTextEmoji
    include Value
    include RichTextSectionElement
    attr_reader :name, :skin_tone, :additional_fields
  end
  RichTextEmoji.const_set(:WIRE_TYPE, "emoji")
  RichTextEmoji.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "name", "wire" => "name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "skin_tone", "wire" => "skin_tone", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  RichTextEmoji.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextEmoji.singleton_class.prepend(KeywordOnly)

  class RichTextLink
    include Value
    include RichTextSectionElement
    attr_reader :url, :text, :style, :unsafe, :additional_fields
  end
  RichTextLink.const_set(:WIRE_TYPE, "link")
  RichTextLink.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "style", "wire" => "style", "kind" => "style", "target" => nil, "coerce" => nil, "flags" => ["bold", "italic", "strike", "code"], "limits" => nil, "required" => false}, {"name" => "unsafe", "wire" => "unsafe", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  RichTextLink.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextLink.singleton_class.prepend(KeywordOnly)

  class RichTextList
    include Value
    include RichTextBlockElement
    attr_reader :style, :elements, :indent, :offset, :border, :additional_fields
  end
  RichTextList.const_set(:WIRE_TYPE, "rich_text_list")
  RichTextList.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "style", "wire" => "style", "kind" => "enum", "target" => "RichTextListStyle", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "RichTextSection", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "indent", "wire" => "indent", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_list.indent", "required" => false}, {"name" => "offset", "wire" => "offset", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_list.offset", "required" => false}, {"name" => "border", "wire" => "border", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_list.border", "required" => false}]))
  RichTextList.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextList.singleton_class.prepend(KeywordOnly)

  class RichTextQuote
    include Value
    include RichTextBlockElement
    attr_reader :elements, :border, :additional_fields
  end
  RichTextQuote.const_set(:WIRE_TYPE, "rich_text_quote")
  RichTextQuote.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "RichTextSectionElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "border", "wire" => "border", "kind" => "int", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "rich_text_quote.border", "required" => false}]))
  RichTextQuote.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextQuote.singleton_class.prepend(KeywordOnly)

  class RichTextSection
    include Value
    include RichTextBlockElement
    attr_reader :elements, :additional_fields
  end
  RichTextSection.const_set(:WIRE_TYPE, "rich_text_section")
  RichTextSection.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "elements", "wire" => "elements", "kind" => "list", "target" => "RichTextSectionElement", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  RichTextSection.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextSection.singleton_class.prepend(KeywordOnly)

  class RichTextText
    include Value
    include RichTextSectionElement
    attr_reader :text, :style, :additional_fields
  end
  RichTextText.const_set(:WIRE_TYPE, "text")
  RichTextText.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "style", "wire" => "style", "kind" => "style", "target" => nil, "coerce" => nil, "flags" => ["bold", "italic", "strike", "code"], "limits" => nil, "required" => false}]))
  RichTextText.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextText.singleton_class.prepend(KeywordOnly)

  class RichTextUser
    include Value
    include RichTextSectionElement
    attr_reader :user_id, :style, :additional_fields
  end
  RichTextUser.const_set(:WIRE_TYPE, "user")
  RichTextUser.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "user_id", "wire" => "user_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "style", "wire" => "style", "kind" => "style", "target" => nil, "coerce" => nil, "flags" => ["bold", "italic", "strike", "highlight", "client_highlight", "unlink"], "limits" => nil, "required" => false}]))
  RichTextUser.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextUser.singleton_class.prepend(KeywordOnly)

  class RichTextUserGroup
    include Value
    include RichTextSectionElement
    attr_reader :user_group_id, :style, :additional_fields
  end
  RichTextUserGroup.const_set(:WIRE_TYPE, "usergroup")
  RichTextUserGroup.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "user_group_id", "wire" => "usergroup_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "style", "wire" => "style", "kind" => "style", "target" => nil, "coerce" => nil, "flags" => ["bold", "italic", "strike", "highlight", "client_highlight", "unlink"], "limits" => nil, "required" => false}]))
  RichTextUserGroup.const_set(:DEFAULTS, Value.deep_freeze({}))
  RichTextUserGroup.singleton_class.prepend(KeywordOnly)

  class SlackFile
    include Value
    attr_reader :id, :url, :additional_fields
  end
  SlackFile.const_set(:WIRE_TYPE, "")
  SlackFile.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "id", "wire" => "id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  SlackFile.const_set(:DEFAULTS, Value.deep_freeze({}))
  SlackFile.singleton_class.prepend(KeywordOnly)

  class SlackIcon
    include Value
    attr_reader :name, :additional_fields
  end
  SlackIcon.const_set(:WIRE_TYPE, "icon")
  SlackIcon.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "name", "wire" => "name", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  SlackIcon.const_set(:DEFAULTS, Value.deep_freeze({}))
  SlackIcon.singleton_class.prepend(KeywordOnly)

  class Trigger
    include Value
    attr_reader :url, :customizable_input_parameters, :additional_fields
  end
  Trigger.const_set(:WIRE_TYPE, "")
  Trigger.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "customizable_input_parameters", "wire" => "customizable_input_parameters", "kind" => "list", "target" => "InputParameter", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  Trigger.const_set(:DEFAULTS, Value.deep_freeze({}))
  Trigger.singleton_class.prepend(KeywordOnly)

  class UrlSource
    include Value
    attr_reader :url, :text, :additional_fields
  end
  UrlSource.const_set(:WIRE_TYPE, "url")
  UrlSource.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "url", "wire" => "url", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  UrlSource.const_set(:DEFAULTS, Value.deep_freeze({}))
  UrlSource.singleton_class.prepend(KeywordOnly)

  class Workflow
    include Value
    attr_reader :trigger, :additional_fields
  end
  Workflow.const_set(:WIRE_TYPE, "")
  Workflow.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "trigger", "wire" => "trigger", "kind" => "object", "target" => "Trigger", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}]))
  Workflow.const_set(:DEFAULTS, Value.deep_freeze({}))
  Workflow.singleton_class.prepend(KeywordOnly)

  class Attachment
    include Value
    attr_reader :blocks, :color, :fallback, :additional_fields
  end
  Attachment.const_set(:WIRE_TYPE, "")
  Attachment.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => true}, {"name" => "color", "wire" => "color", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "fallback", "wire" => "fallback", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  Attachment.const_set(:DEFAULTS, Value.deep_freeze({}))
  Attachment.singleton_class.prepend(KeywordOnly)

  class HomeTabView
    include Value
    attr_reader :blocks, :private_metadata, :callback_id, :external_id, :additional_fields
  end
  HomeTabView.const_set(:WIRE_TYPE, "home")
  HomeTabView.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "view.blocks", "required" => true}, {"name" => "private_metadata", "wire" => "private_metadata", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.private_metadata", "required" => false}, {"name" => "callback_id", "wire" => "callback_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.callback_id", "required" => false}, {"name" => "external_id", "wire" => "external_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.external_id", "required" => false}]))
  HomeTabView.const_set(:DEFAULTS, Value.deep_freeze({}))
  HomeTabView.singleton_class.prepend(KeywordOnly)

  class MessagePayload
    include Value
    attr_reader :channel, :blocks, :attachments, :text, :mrkdwn, :unfurl_links, :unfurl_media, :metadata, :additional_fields
  end
  MessagePayload.const_set(:WIRE_TYPE, "")
  MessagePayload.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "channel", "wire" => "channel", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "message.channel", "required" => true}, {"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "message.blocks", "required" => false}, {"name" => "attachments", "wire" => "attachments", "kind" => "list", "target" => "Attachment", "coerce" => nil, "flags" => nil, "limits" => "message.attachments", "required" => false}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "mrkdwn", "wire" => "mrkdwn", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "unfurl_links", "wire" => "unfurl_links", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "unfurl_media", "wire" => "unfurl_media", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "metadata", "wire" => "metadata", "kind" => "map", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  MessagePayload.const_set(:DEFAULTS, Value.deep_freeze({"mrkdwn" => true, "text" => ""}))
  MessagePayload.singleton_class.prepend(KeywordOnly)

  class MessageResponse
    include Value
    attr_reader :blocks, :attachments, :text, :mrkdwn, :replace_original, :response_type, :additional_fields
  end
  MessageResponse.const_set(:WIRE_TYPE, "")
  MessageResponse.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "message.blocks", "required" => false}, {"name" => "attachments", "wire" => "attachments", "kind" => "list", "target" => "Attachment", "coerce" => nil, "flags" => nil, "limits" => "message.attachments", "required" => false}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "mrkdwn", "wire" => "mrkdwn", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "replace_original", "wire" => "replace_original", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "response_type", "wire" => "response_type", "kind" => "enum", "target" => "ResponseType", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  MessageResponse.const_set(:DEFAULTS, Value.deep_freeze({"mrkdwn" => true, "text" => "", "replace_original" => false, "response_type" => "in_channel"}))
  MessageResponse.singleton_class.prepend(KeywordOnly)

  class ModalView
    include Value
    attr_reader :title, :blocks, :close, :submit, :private_metadata, :callback_id, :clear_on_close, :notify_on_close, :external_id, :submit_disabled, :additional_fields
  end
  ModalView.const_set(:WIRE_TYPE, "modal")
  ModalView.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "title", "wire" => "title", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "view.title", "required" => true}, {"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "view.blocks", "required" => true}, {"name" => "close", "wire" => "close", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "view.close", "required" => false}, {"name" => "submit", "wire" => "submit", "kind" => "text", "target" => "PlainText", "coerce" => "plain_text", "flags" => nil, "limits" => "view.submit", "required" => false}, {"name" => "private_metadata", "wire" => "private_metadata", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.private_metadata", "required" => false}, {"name" => "callback_id", "wire" => "callback_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.callback_id", "required" => false}, {"name" => "clear_on_close", "wire" => "clear_on_close", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "notify_on_close", "wire" => "notify_on_close", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "external_id", "wire" => "external_id", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => "view.external_id", "required" => false}, {"name" => "submit_disabled", "wire" => "submit_disabled", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  ModalView.const_set(:DEFAULTS, Value.deep_freeze({}))
  ModalView.singleton_class.prepend(KeywordOnly)

  class WebhookMessage
    include Value
    attr_reader :blocks, :attachments, :text, :response_type, :replace_original, :delete_original, :unfurl_links, :unfurl_media, :metadata, :additional_fields
  end
  WebhookMessage.const_set(:WIRE_TYPE, "")
  WebhookMessage.const_set(:FIELD_SPECS, Value.deep_freeze([{"name" => "blocks", "wire" => "blocks", "kind" => "list", "target" => "Block", "coerce" => nil, "flags" => nil, "limits" => "message.blocks", "required" => false}, {"name" => "attachments", "wire" => "attachments", "kind" => "list", "target" => "Attachment", "coerce" => nil, "flags" => nil, "limits" => "message.attachments", "required" => false}, {"name" => "text", "wire" => "text", "kind" => "string", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "response_type", "wire" => "response_type", "kind" => "enum", "target" => "ResponseType", "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "replace_original", "wire" => "replace_original", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "delete_original", "wire" => "delete_original", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "unfurl_links", "wire" => "unfurl_links", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "unfurl_media", "wire" => "unfurl_media", "kind" => "boolean", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}, {"name" => "metadata", "wire" => "metadata", "kind" => "map", "target" => nil, "coerce" => nil, "flags" => nil, "limits" => nil, "required" => false}]))
  WebhookMessage.const_set(:DEFAULTS, Value.deep_freeze({}))
  WebhookMessage.singleton_class.prepend(KeywordOnly)

end
