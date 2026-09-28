# Inputs for every case in spec/fixtures/invalid/manifest.json.
# Each case is constructed through the public Ruby API by FixtureDriver.
module InvalidCases
  IDS = %w[
    text-empty
    text-too-long
    button-action-id-too-long
    button-text-too-long
    button-url-too-long
    button-value-too-long
    confirmation-title-too-long
    confirmation-text-too-long
    confirmation-confirm-too-long
    confirmation-deny-too-long
    option-text-too-long
    option-value-too-long
    option-url-too-long
    option-description-too-long
    option-group-label-too-long
    option-group-empty
    option-group-too-many-options
    select-placeholder-too-long
    plain-text-input-placeholder-too-long
    email-input-placeholder-too-long
    url-input-placeholder-too-long
    number-input-placeholder-too-long
    date-picker-placeholder-too-long
    time-picker-placeholder-too-long
    rich-text-input-placeholder-too-long
    select-too-many-options
    select-too-many-option-groups
    overflow-empty
    overflow-too-many-options
    checkboxes-empty
    checkboxes-too-many-options
    radio-buttons-empty
    radio-buttons-too-many-options
    table-too-many-rows
    table-too-many-columns
    file-input-max-files-too-small
    file-input-max-files-too-large
    dispatch-action-no-triggers
    dispatch-action-missing-triggers
    dispatch-action-too-many-triggers
    plain-text-input-max-length-too-large
    actions-too-many-elements
    context-too-many-elements
    header-text-too-long
    image-url-too-long
    image-alt-text-too-long
    input-label-too-long
    input-hint-too-long
    markdown-too-long
    section-text-too-long
    section-too-many-fields
    section-field-too-long
    video-alt-text-too-long
    video-title-too-long
    video-author-name-too-long
    video-description-too-long
    video-provider-name-too-long
    message-channel-empty
    message-too-many-blocks
    message-too-many-attachments
    message-invalid-block-surface
    modal-invalid-block-surface
    home-invalid-block-surface
    modal-input-requires-submit
    view-too-many-blocks
    view-private-metadata-too-long
    view-callback-id-too-long
    view-title-too-long
    view-close-too-long
    view-submit-too-long
    section-missing-content
    section-empty-fields
    static-select-options-and-groups
    image-url-and-slack-file
    number-input-inverted-range
    context-invalid-element
    input-invalid-element
    block-id-too-long
    button-accessibility-label-too-long
    workflow-button-text-too-long
    workflow-button-accessibility-label-too-long
    alert-text-too-long
    card-title-too-long
    card-subtitle-too-long
    card-body-too-long
    card-too-many-actions
    card-subtext-too-long
    carousel-empty
    carousel-too-many-cards
    container-title-too-long
    container-subtitle-too-long
    container-too-many-child-blocks
    context-actions-too-many-elements
    feedback-button-text-too-long
    feedback-button-value-too-long
    feedback-button-accessibility-label-too-long
    icon-button-too-many-visible-users
    icon-button-value-too-long
    icon-button-accessibility-label-too-long
    data-table-too-few-rows
    data-table-too-many-rows
    data-table-too-few-columns
    data-table-too-many-columns
    data-table-page-size-too-small
    data-table-page-size-too-large
    data-table-cell-text-empty
    data-table-content-too-long
    data-visualization-title-too-long
    pie-chart-empty
    pie-chart-too-many-segments
    chart-segment-label-too-long
    chart-segment-value-not-positive
    chart-series-empty
    chart-duplicate-point-labels
    chart-too-many-series
    data-series-name-too-long
    data-series-empty
    data-series-too-many-points
    data-point-label-too-long
    axis-categories-empty
    axis-too-many-categories
    axis-category-label-too-long
    axis-label-too-long
    plain-text-input-min-length-negative
    plain-text-input-min-length-too-large
    plain-text-input-max-length-too-small
    rich-text-input-min-lines-too-small
    rich-text-input-min-lines-too-large
    rich-text-input-max-lines-too-small
    rich-text-input-max-lines-too-large
    multi-select-max-selected-items-too-small
    conversation-filter-include-empty
    conversation-filter-unknown-include
    workflow-button-missing-action-id
    number-input-missing-decimal-flag
    slack-icon-missing-name
    slack-file-id-malformed
    image-element-url-too-long
    image-element-alt-text-too-long
    image-block-title-too-long
    image-block-url-and-slack-file
    image-block-missing-source
    container-no-child-blocks
    table-too-many-column-settings
    data-table-row-header-index-negative
    data-table-total-content-too-long
    markdown-total-too-long
    plan-missing-tasks
    plan-too-many-tasks
    plan-duplicate-task-ids
    task-card-missing-status
    task-card-pending-status
    rich-text-list-indent-negative
    rich-text-list-indent-too-large
    rich-text-list-offset-negative
    rich-text-list-border-negative
    rich-text-list-border-too-large
    rich-text-quote-border-negative
    rich-text-quote-border-too-large
    rich-text-preformatted-border-negative
    rich-text-preformatted-border-too-large
    video-thumbnail-url-too-long
    video-url-too-long
    video-title-url-too-long
    video-provider-icon-url-too-long
    view-external-id-too-long
    attachment-missing-blocks
  ].freeze

  module_function

  def for(id)
    case id
    when "text-empty" then invalid_case("PlainText", text_object("plain_text", ""))
    when "text-too-long" then invalid_case("PlainText", text_object("plain_text", repeated("x", 3001)))
    when "button-action-id-too-long" then typed("Button", "button", "text", plain("A"), "action_id", repeated("x", 256))
    when "button-text-too-long" then typed("Button", "button", "text", plain(repeated("x", 76)), "action_id", "a")
    when "button-url-too-long" then typed("Button", "button", "text", plain("A"), "action_id", "a", "url", repeated("x", 3001))
    when "button-value-too-long" then typed("Button", "button", "text", plain("A"), "action_id", "a", "value", repeated("x", 2001))
    when "confirmation-title-too-long" then confirmation(repeated("x", 101), "Text", "Yes", "No")
    when "confirmation-text-too-long" then confirmation("Title", repeated("x", 301), "Yes", "No")
    when "confirmation-confirm-too-long" then confirmation("Title", "Text", repeated("x", 31), "No")
    when "confirmation-deny-too-long" then confirmation("Title", "Text", "Yes", repeated("x", 31))
    when "option-text-too-long" then invalid_case("Option", option(repeated("x", 76), "a"))
    when "option-value-too-long" then invalid_case("Option", option("A", repeated("x", 151)))
    when "option-url-too-long" then invalid_case("Option", map("text", plain("A"), "value", "a", "url", repeated("🙂", 3001)))
    when "option-description-too-long" then invalid_case("Option", map("text", plain("A"), "value", "a", "description", plain(repeated("x", 76))))
    when "option-group-label-too-long" then option_group_case(repeated("x", 76), items(option("A", "a")))
    when "option-group-empty" then option_group_case("Group", items)
    when "option-group-too-many-options" then option_group_case("Group", copies(101, ->(index) { option("A", "a") }))
    when "select-placeholder-too-long" then typed("StaticSelect", "static_select", "action_id", "a", "options", items(option("A", "a")), "placeholder", plain(repeated("x", 151)))
    when "plain-text-input-placeholder-too-long" then typed("PlainTextInput", "plain_text_input", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "email-input-placeholder-too-long" then typed("EmailInputElement", "email_text_input", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "url-input-placeholder-too-long" then typed("UrlInputElement", "url_text_input", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "number-input-placeholder-too-long" then typed("NumberInput", "number_input", "action_id", "a", "is_decimal_allowed", false, "placeholder", plain(repeated("x", 151)))
    when "date-picker-placeholder-too-long" then typed("DatePickerElement", "datepicker", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "time-picker-placeholder-too-long" then typed("TimePickerElement", "timepicker", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "rich-text-input-placeholder-too-long" then typed("RichTextInputElement", "rich_text_input", "action_id", "a", "placeholder", plain(repeated("x", 151)))
    when "select-too-many-options" then typed("StaticSelect", "static_select", "action_id", "a", "options", copies(101, ->(index) { option("A", "a") }))
    when "select-too-many-option-groups" then typed("StaticSelect", "static_select", "action_id", "a", "option_groups", copies(101, ->(index) { map("label", plain("Group"), "options", items(option("A", "a"))) }))
    when "overflow-empty" then typed("Overflow", "overflow", "action_id", "a", "options", items)
    when "overflow-too-many-options" then typed("Overflow", "overflow", "action_id", "a", "options", copies(6, ->(index) { option("A", "a") }))
    when "checkboxes-empty" then typed("Checkboxes", "checkboxes", "action_id", "a", "options", items)
    when "checkboxes-too-many-options" then typed("Checkboxes", "checkboxes", "action_id", "a", "options", copies(11, ->(index) { option("A", "a") }))
    when "radio-buttons-empty" then typed("RadioButtons", "radio_buttons", "action_id", "a", "options", items)
    when "radio-buttons-too-many-options" then typed("RadioButtons", "radio_buttons", "action_id", "a", "options", copies(11, ->(index) { option("A", "a") }))
    when "table-too-many-rows" then typed("TableBlock", "table", "rows", copies(101, ->(index) { items(raw_text("A")) }))
    when "table-too-many-columns" then typed("TableBlock", "table", "rows", items(copies(21, ->(index) { raw_text("A") })))
    when "file-input-max-files-too-small" then typed("FileInput", "file_input", "action_id", "a", "max_files", 0)
    when "file-input-max-files-too-large" then typed("FileInput", "file_input", "action_id", "a", "max_files", 11)
    when "dispatch-action-no-triggers" then invalid_case("DispatchActionConfiguration", map("trigger_actions_on", items))
    when "dispatch-action-missing-triggers" then invalid_case("DispatchActionConfiguration", map)
    when "dispatch-action-too-many-triggers" then invalid_case("DispatchActionConfiguration", map("trigger_actions_on", items("on_enter_pressed", "on_character_entered", "on_enter_pressed")))
    when "plain-text-input-max-length-too-large" then typed("PlainTextInput", "plain_text_input", "action_id", "a", "max_length", 3001)
    when "actions-too-many-elements" then typed("ActionsBlock", "actions", "elements", copies(26, ->(index) { button }))
    when "context-too-many-elements" then typed("ContextBlock", "context", "elements", copies(11, ->(index) { mrkdwn("A") }))
    when "header-text-too-long" then typed("HeaderBlock", "header", "text", plain(repeated("x", 151)))
    when "image-url-too-long" then typed("ImageBlock", "image", "image_url", repeated("x", 3001), "alt_text", "Alt")
    when "image-alt-text-too-long" then typed("ImageBlock", "image", "image_url", "https://example.com/image.png", "alt_text", repeated("x", 2001))
    when "input-label-too-long" then typed("InputBlock", "input", "label", plain(repeated("x", 2001)), "element", input_element)
    when "input-hint-too-long" then typed("InputBlock", "input", "label", plain("Label"), "hint", plain(repeated("x", 2001)), "element", input_element)
    when "markdown-too-long" then typed("MarkdownBlock", "markdown", "text", repeated("x", 12001))
    when "section-text-too-long" then typed("SectionBlock", "section", "text", mrkdwn(repeated("x", 3001)))
    when "section-too-many-fields" then typed("SectionBlock", "section", "fields", copies(11, ->(index) { mrkdwn("x") }))
    when "section-field-too-long" then typed("SectionBlock", "section", "fields", items(mrkdwn(repeated("x", 2001))))
    when "video-alt-text-too-long" then invalid_video("alt_text", repeated("x", limit("video.alt_text.max_length") + 1))
    when "video-title-too-long" then invalid_video("title", plain(repeated("x", 201)))
    when "video-author-name-too-long" then invalid_video("author_name", repeated("x", 51))
    when "video-description-too-long" then invalid_video("description", plain(repeated("x", 201)))
    when "video-provider-name-too-long" then invalid_video("provider_name", repeated("x", 51))
    when "message-channel-empty" then invalid_case("Message", map("channel", ""))
    when "message-too-many-blocks" then invalid_case("Message", map("channel", "C123", "blocks", copies(51, ->(index) { divider })))
    when "message-too-many-attachments" then invalid_case("Message", map("channel", "C123", "attachments", copies(101, ->(index) { map("blocks", items(divider)) })))
    when "message-invalid-block-surface" then invalid_case("Message", map("channel", "C123", "blocks", items(obj("alert", "text", plain("Modal only")))))
    when "modal-invalid-block-surface" then typed("Modal", "modal", "title", plain("Invalid"), "blocks", items(obj("markdown", "text", "Message only")))
    when "home-invalid-block-surface" then typed("HomeTab", "home", "blocks", items(obj("alert", "text", plain("Modal only"))))
    when "modal-input-requires-submit" then typed("Modal", "modal", "title", plain("Missing submit"), "blocks", items(input_block))
    when "view-too-many-blocks" then typed("HomeTab", "home", "blocks", copies(101, ->(index) { divider }))
    when "view-private-metadata-too-long" then typed("HomeTab", "home", "blocks", items(divider), "private_metadata", repeated("x", 3001))
    when "view-callback-id-too-long" then typed("HomeTab", "home", "blocks", items(divider), "callback_id", repeated("x", 256))
    when "view-title-too-long" then typed("Modal", "modal", "title", plain(repeated("x", 25)), "blocks", items(divider))
    when "view-close-too-long" then typed("Modal", "modal", "title", plain("Title"), "close", plain(repeated("x", 25)), "blocks", items(divider))
    when "view-submit-too-long" then typed("Modal", "modal", "title", plain("Title"), "submit", plain(repeated("x", 25)), "blocks", items(divider))
    when "section-missing-content" then typed("SectionBlock", "section")
    when "section-empty-fields" then typed("SectionBlock", "section", "fields", items)
    when "static-select-options-and-groups" then typed("StaticSelect", "static_select", "action_id", "a", "options", items(option("A", "a")), "option_groups", items(map("label", plain("A"), "options", items(option("B", "b")))))
    when "image-url-and-slack-file" then typed("ImageElement", "image", "alt_text", "image", "image_url", "https://example.com/image.png", "slack_file", map("id", "F0123ABC456"))
    when "number-input-inverted-range" then typed("NumberInput", "number_input", "action_id", "a", "is_decimal_allowed", true, "min_value", 2, "max_value", 1)
    when "context-invalid-element" then typed("ContextBlock", "context", "elements", items(divider))
    when "input-invalid-element" then typed("InputBlock", "input", "label", plain("Label"), "element", button)
    when "block-id-too-long" then typed("DividerBlock", "divider", "block_id", repeated("x", 256))
    when "button-accessibility-label-too-long" then typed("Button", "button", "text", plain("A"), "action_id", "a", "accessibility_label", repeated("x", 76))
    when "workflow-button-text-too-long" then typed("WorkflowButtonElement", "workflow_button", "text", plain(repeated("x", 76)), "workflow", workflow, "action_id", "a")
    when "workflow-button-accessibility-label-too-long" then typed("WorkflowButtonElement", "workflow_button", "text", plain("Run"), "workflow", workflow, "action_id", "a", "accessibility_label", repeated("x", 76))
    when "alert-text-too-long" then typed("AlertBlock", "alert", "text", plain(repeated("x", 201)))
    when "card-title-too-long" then typed("CardBlock", "card", "title", plain(repeated("x", 151)))
    when "card-subtitle-too-long" then typed("CardBlock", "card", "title", plain("Card"), "subtitle", plain(repeated("x", 151)))
    when "card-body-too-long" then typed("CardBlock", "card", "body", plain(repeated("x", 201)))
    when "card-too-many-actions" then typed("CardBlock", "card", "title", plain("Card"), "actions", copies(4, ->(index) { button }))
    when "card-subtext-too-long" then typed("CardBlock", "card", "title", plain("Card"), "subtext", plain(repeated("x", 201)))
    when "carousel-empty" then typed("CarouselBlock", "carousel", "elements", items)
    when "carousel-too-many-cards" then typed("CarouselBlock", "carousel", "elements", copies(11, ->(index) { card }))
    when "container-title-too-long" then typed("ContainerBlock", "container", "title", plain(repeated("x", 151)), "child_blocks", items(divider))
    when "container-subtitle-too-long" then typed("ContainerBlock", "container", "title", plain("Container"), "subtitle", plain(repeated("x", 151)), "child_blocks", items(divider))
    when "container-too-many-child-blocks" then typed("ContainerBlock", "container", "title", plain("Container"), "child_blocks", copies(11, ->(index) { divider }))
    when "context-actions-too-many-elements" then typed("ContextActionsBlock", "context_actions", "elements", copies(6, ->(index) { icon_button }))
    when "feedback-button-text-too-long" then typed("FeedbackButtons", "feedback_buttons", "positive_button", feedback(repeated("x", 76), "good"), "negative_button", feedback("Bad", "bad"))
    when "feedback-button-value-too-long" then typed("FeedbackButtons", "feedback_buttons", "positive_button", feedback("Good", repeated("x", 2001)), "negative_button", feedback("Bad", "bad"))
    when "feedback-button-accessibility-label-too-long" then typed("FeedbackButtons", "feedback_buttons", "positive_button", map("text", plain("Good"), "value", "good", "accessibility_label", repeated("x", 76)), "negative_button", feedback("Bad", "bad"))
    when "icon-button-too-many-visible-users" then typed("IconButton", "icon_button", "text", plain("Delete"), "icon", "trash", "visible_to_user_ids", copies(11, ->(index) { "U" + index.to_s }))
    when "icon-button-value-too-long" then typed("IconButton", "icon_button", "text", plain("Delete"), "icon", "trash", "value", repeated("x", 2001))
    when "icon-button-accessibility-label-too-long" then typed("IconButton", "icon_button", "text", plain("Delete"), "icon", "trash", "accessibility_label", repeated("x", 76))
    when "data-table-too-few-rows" then data_table_case(items(items(raw_text("Name"))), "Names")
    when "data-table-too-many-rows" then data_table_case(copies(202, ->(index) { items(raw_text("A")) }), "Names")
    when "data-table-too-few-columns" then data_table_case(items(items, items), "Empty")
    when "data-table-too-many-columns" then data_table_case(copies(2, ->(row) { copies(21, ->(column) { raw_text("A") }) }), "Wide")
    when "data-table-page-size-too-small" then data_table_case(valid_rows, "Names", "page_size", 0)
    when "data-table-page-size-too-large" then data_table_case(valid_rows, "Names", "page_size", 101)
    when "data-table-cell-text-empty" then data_table_case(items(items(raw_text("Name")), items(raw_text(""))), "Names")
    when "data-table-content-too-long" then data_table_case(items(items(raw_text("Name")), items(raw_text(repeated("x", 20000)))), "Names")
    when "data-visualization-title-too-long" then typed("DataVisualizationBlock", "data_visualization", "title", repeated("x", 51), "chart", obj("pie", "segments", items(segment("A", 1))))
    when "pie-chart-empty" then typed("PieChart", "pie", "segments", items)
    when "pie-chart-too-many-segments" then typed("PieChart", "pie", "segments", copies(13, ->(index) { segment("S", 1) }))
    when "chart-segment-label-too-long" then typed("ChartSegment", "", "label", repeated("x", 21), "value", 1)
    when "chart-segment-value-not-positive" then typed("ChartSegment", "", "label", "A", "value", 0)
    when "chart-series-empty" then typed("LineChart", "line", "axis_config", axis("A"), "series", items)
    when "chart-duplicate-point-labels" then typed("LineChart", "line", "axis_config", axis("A", "B"), "series", items(series("Series", point("A", 1), point("A", 2))))
    when "chart-too-many-series" then typed("LineChart", "line", "series", copies(13, ->(index) { series("S" + index.to_s, point("A", 1)) }), "axis_config", axis("A"))
    when "data-series-name-too-long" then invalid_case("DataSeries", series(repeated("x", 21), point("A", 1)))
    when "data-series-empty" then invalid_case("DataSeries", map("name", "Series", "data", items))
    when "data-series-too-many-points" then invalid_case("DataSeries", series("Series", copies(21, ->(index) { point("P" + index.to_s, index) })))
    when "data-point-label-too-long" then invalid_case("DataPoint", point(repeated("x", 21), 1))
    when "axis-categories-empty" then invalid_case("AxisConfig", map("categories", items))
    when "axis-too-many-categories" then invalid_case("AxisConfig", map("categories", copies(21, ->(index) { "X" + index.to_s })))
    when "axis-category-label-too-long" then invalid_case("AxisConfig", axis(repeated("x", 21)))
    when "axis-label-too-long" then invalid_case("AxisConfig", map("categories", items("A"), "x_label", repeated("x", 51)))
    when "plain-text-input-min-length-negative" then typed("PlainTextInput", "plain_text_input", "action_id", "a", "min_length", limit("plain_text_input.min_length.min") - 1)
    when "plain-text-input-min-length-too-large" then typed("PlainTextInput", "plain_text_input", "action_id", "a", "min_length", limit("plain_text_input.min_length.max") + 1)
    when "plain-text-input-max-length-too-small" then typed("PlainTextInput", "plain_text_input", "action_id", "a", "max_length", limit("plain_text_input.max_length.min") - 1)
    when "rich-text-input-min-lines-too-small" then rich_text_input_case("min_lines", limit("rich_text_input.min_lines.min") - 1)
    when "rich-text-input-min-lines-too-large" then rich_text_input_case("min_lines", limit("rich_text_input.min_lines.max") + 1)
    when "rich-text-input-max-lines-too-small" then rich_text_input_case("max_lines", limit("rich_text_input.max_lines.min") - 1)
    when "rich-text-input-max-lines-too-large" then rich_text_input_case("max_lines", limit("rich_text_input.max_lines.max") + 1)
    when "multi-select-max-selected-items-too-small" then typed("UserMultiSelectElement", "multi_users_select", "action_id", "a", "max_selected_items", limit("multi_select.max_selected_items.min") - 1)
    when "conversation-filter-include-empty" then invalid_case("ConversationFilter", map("include", items))
    when "conversation-filter-unknown-include" then invalid_case("ConversationFilter", map("include", items("channel")))
    when "workflow-button-missing-action-id" then typed("WorkflowButtonElement", "workflow_button", "text", plain("Run"), "workflow", workflow)
    when "number-input-missing-decimal-flag" then typed("NumberInput", "number_input", "action_id", "a")
    when "slack-icon-missing-name" then typed("SlackIcon", "icon")
    when "slack-file-id-malformed" then invalid_case("SlackFile", map("id", "F0123456"))
    when "image-element-url-too-long" then typed("ImageElement", "image", "image_url", repeated("x", limit("image_element.image_url.max_length") + 1), "alt_text", "Alt")
    when "image-element-alt-text-too-long" then typed("ImageElement", "image", "image_url", "https://example.com/image.png", "alt_text", repeated("x", limit("image_element.alt_text.max_length") + 1))
    when "image-block-title-too-long" then typed("ImageBlock", "image", "image_url", "https://example.com/image.png", "alt_text", "Alt", "title", plain(repeated("x", limit("image.title.max_length") + 1)))
    when "image-block-url-and-slack-file" then typed("ImageBlock", "image", "image_url", "https://example.com/image.png", "slack_file", map("id", "F0123ABC456"), "alt_text", "Alt")
    when "image-block-missing-source" then typed("ImageBlock", "image", "alt_text", "Alt")
    when "container-no-child-blocks" then typed("ContainerBlock", "container", "title", plain("Container"), "child_blocks", items)
    when "table-too-many-column-settings" then typed("TableBlock", "table", "rows", items(items(raw_text("A"))), "column_settings", copies(limit("table.column_settings.max_items") + 1, ->(index) { map("is_wrapped", true) }))
    when "data-table-row-header-index-negative" then data_table_case(valid_rows, "Names", "row_header_column_index", limit("data_table.row_header_column_index.min") - 1)
    when "data-table-total-content-too-long" then two_data_tables_over_message_total
    when "markdown-total-too-long" then two_markdown_blocks_over_message_total
    when "plan-missing-tasks" then typed("PlanBlock", "plan", "title", "Plan")
    when "plan-too-many-tasks" then typed("PlanBlock", "plan", "title", "Plan", "tasks", copies(limit("plan.tasks.max_items") + 1, ->(index) { plan_task("task_" + index.to_s) }))
    when "plan-duplicate-task-ids" then typed("PlanBlock", "plan", "title", "Plan", "tasks", items(plan_task("task"), plan_task("task")))
    when "task-card-missing-status" then typed("TaskCardBlock", "task_card", "task_id", "task", "title", "Task")
    when "task-card-pending-status" then invalid_case("Message", map("channel", "C123", "blocks", items(obj("task_card", "task_id", "task", "title", "Task", "status", "pending"))))
    when "rich-text-list-indent-negative" then rich_text_list_case("indent", limit("rich_text_list.indent.min") - 1)
    when "rich-text-list-indent-too-large" then rich_text_list_case("indent", limit("rich_text_list.indent.max") + 1)
    when "rich-text-list-offset-negative" then rich_text_list_case("offset", limit("rich_text_list.offset.min") - 1)
    when "rich-text-list-border-negative" then rich_text_list_case("border", limit("rich_text_list.border.min") - 1)
    when "rich-text-list-border-too-large" then rich_text_list_case("border", limit("rich_text_list.border.max") + 1)
    when "rich-text-quote-border-negative" then rich_text_container("RichTextQuote", "rich_text_quote", limit("rich_text_quote.border.min") - 1)
    when "rich-text-quote-border-too-large" then rich_text_container("RichTextQuote", "rich_text_quote", limit("rich_text_quote.border.max") + 1)
    when "rich-text-preformatted-border-negative" then rich_text_container("RichTextCodeBlock", "rich_text_preformatted", limit("rich_text_preformatted.border.min") - 1)
    when "rich-text-preformatted-border-too-large" then rich_text_container("RichTextCodeBlock", "rich_text_preformatted", limit("rich_text_preformatted.border.max") + 1)
    when "video-thumbnail-url-too-long" then invalid_video("thumbnail_url", repeated("x", limit("video.thumbnail_url.max_length") + 1))
    when "video-url-too-long" then invalid_video("video_url", repeated("x", limit("video.video_url.max_length") + 1))
    when "video-title-url-too-long" then invalid_video("title_url", repeated("x", limit("video.title_url.max_length") + 1))
    when "video-provider-icon-url-too-long" then invalid_video("provider_icon_url", repeated("x", limit("video.provider_icon_url.max_length") + 1))
    when "view-external-id-too-long" then typed("HomeTab", "home", "blocks", items(divider), "external_id", repeated("x", limit("view.external_id.max_length") + 1))
    when "attachment-missing-blocks" then invalid_case("Attachment", map("fallback", "Summary"))
    else
      raise KeyError, "Unmapped invalid case: #{id}"
    end
  end

  def limit(path) = Slackblocks::Limits[path]
  def repeated(value, count) = value * count
  def copies(count, factory) = Array.new(count) { |index| factory.call(index) }
  def items(*values) = values
  def obj(type, *fields) = (type.empty? ? {} : {"type" => type}).merge(Hash[*fields])
  def map(*fields) = Hash[*fields]
  def invalid_case(name, value) = {"name" => name, "value" => value}
  def typed(name, type, *fields) = invalid_case(name, obj(type, *fields))
  def text_object(type, value) = obj(type, "text", value)
  def plain(value) = text_object("plain_text", value)
  def mrkdwn(value) = text_object("mrkdwn", value)

  def confirmation(title, text, confirm, deny) =
    invalid_case("Confirmation", map("title", plain(title), "text", mrkdwn(text), "confirm", plain(confirm), "deny", plain(deny)))

  def option(text, value) = map("text", plain(text), "value", value)
  def option_group_case(label, options) = invalid_case("OptionGroup", map("label", plain(label), "options", options))
  def feedback(text, value) = map("text", plain(text), "value", value)
  def button = obj("button", "text", plain("A"), "action_id", "a")
  def icon_button = obj("icon_button", "text", plain("Delete"), "icon", "trash")
  def workflow = map("trigger", map("url", "https://slack.com/shortcuts/Ft0/abc"))
  def input_element = obj("plain_text_input", "action_id", "a")
  def input_block = obj("input", "label", plain("Name"), "element", input_element)
  def divider = obj("divider")
  def card = obj("card", "title", plain("Card"))
  def raw_text(value) = obj("raw_text", "text", value)
  def valid_rows = items(items(raw_text("Name")), items(raw_text("Alice")))
  def segment(label, value) = map("label", label, "value", value)
  def point(label, value) = map("label", label, "value", value)

  def series(name, *points) =
    map("name", name, "data", ((points.length == 1 && points.first.is_a?(Array)) ? points.first : points))

  def axis(*categories) = map("categories", categories)
  def rich_text(value) = obj("text", "text", value)
  def plan_task(id) = map("task_id", id, "title", "Task", "status", "complete")

  def data_table_case(rows, caption, *fields) =
    invalid_case("DataTableBlock", obj("data_table", "rows", rows, "caption", caption, *fields))

  def valid_video =
    obj("video", "alt_text", "Video", "thumbnail_url", "https://example.com/thumbnail.png", "title", plain("Title"), "video_url", "https://example.com/video.mp4")

  def invalid_video(field, value) =
    invalid_case("VideoBlock", valid_video.merge(field => value))

  def rich_text_input_case(field, value) =
    typed("RichTextInputElement", "rich_text_input", "action_id", "a", field, value)

  def rich_text_list_case(field, value) =
    typed("RichTextList", "rich_text_list", "style", "bullet", "elements", items(obj("rich_text_section", "elements", items(rich_text("Item")))), field, value)

  def rich_text_container(name, type, border) =
    typed(name, type, "elements", items(rich_text("Text")), "border", border)

  def two_data_tables_over_message_total
    each = (limit("data_table.total_content.max_length") / 2) + 1
    table = obj("data_table", "rows", items(items(raw_text("Name")), items(raw_text(repeated("x", each - 4)))), "caption", "Names")
    invalid_case("Message", map("channel", "C123", "blocks", items(table, table)))
  end

  def two_markdown_blocks_over_message_total
    markdown = obj("markdown", "text", repeated("x", (limit("markdown.total_text.max_length") / 2) + 1))
    invalid_case("Message", map("channel", "C123", "blocks", items(markdown, markdown)))
  end
end
