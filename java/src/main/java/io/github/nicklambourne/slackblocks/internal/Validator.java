package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.ErrorCategory;
import io.github.nicklambourne.slackblocks.ValidationException;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.regex.Pattern;

/** Central, language-neutral Slack wire validation. */
public final class Validator {
  private static final Map<String, List<String>> REQUIRED_FIELDS =
      Map.ofEntries(
          entry("actions", "elements"),
          entry("alert", "text"),
          entry("area", "series", "axis_config"),
          entry("bar", "series", "axis_config"),
          entry("button", "text", "action_id"),
          entry("carousel", "elements"),
          entry("channel", "channel_id"),
          entry("channels_select", "action_id"),
          entry("checkboxes", "action_id", "options"),
          entry("container", "child_blocks"),
          entry("context", "elements"),
          entry("context_actions", "elements"),
          entry("conversations_select", "action_id"),
          entry("data_table", "rows", "caption"),
          entry("data_visualization", "title", "chart"),
          entry("datepicker", "action_id"),
          entry("datetimepicker", "action_id"),
          entry("email_text_input", "action_id"),
          entry("emoji", "name"),
          entry("external_select", "action_id"),
          entry("feedback_buttons", "positive_button", "negative_button"),
          entry("file", "external_id"),
          entry("file_input", "action_id"),
          entry("header", "text"),
          entry("home", "blocks"),
          entry("icon_button", "text"),
          entry("image", "alt_text"),
          entry("input", "label", "element"),
          entry("line", "series", "axis_config"),
          entry("link", "url"),
          entry("markdown", "text"),
          entry("modal", "title", "blocks"),
          entry("multi_channels_select", "action_id"),
          entry("multi_conversations_select", "action_id"),
          entry("multi_external_select", "action_id"),
          entry("multi_static_select", "action_id"),
          entry("multi_users_select", "action_id"),
          entry("number_input", "action_id"),
          entry("overflow", "action_id", "options"),
          entry("pie", "segments"),
          entry("plain_text_input", "action_id"),
          entry("plan", "title"),
          entry("radio_buttons", "action_id", "options"),
          entry("raw_number", "value", "text"),
          entry("raw_text", "text"),
          entry("rich_text", "elements"),
          entry("rich_text_input", "action_id"),
          entry("rich_text_list", "style", "elements"),
          entry("rich_text_preformatted", "elements"),
          entry("rich_text_quote", "elements"),
          entry("rich_text_section", "elements"),
          entry("static_select", "action_id"),
          entry("table", "rows"),
          entry("task_card", "task_id", "title"),
          entry("text", "text"),
          entry("timepicker", "action_id"),
          entry("url", "url", "text"),
          entry("url_text_input", "action_id"),
          entry("user", "user_id"),
          entry("usergroup", "usergroup_id"),
          entry("users_select", "action_id"),
          entry("video", "alt_text", "thumbnail_url", "title", "video_url"),
          entry("workflow_button", "text", "workflow"));

  private static final Set<String> INPUT_ELEMENT_TYPES =
      Set.of(
          "plain_text_input", "number_input", "checkboxes", "radio_buttons", "datepicker",
          "datetimepicker", "timepicker", "channels_select", "multi_channels_select",
          "conversations_select", "multi_conversations_select", "external_select",
          "multi_external_select", "static_select", "multi_static_select", "users_select",
          "multi_users_select", "rich_text_input", "email_text_input", "url_text_input",
          "file_input");

  private static final Set<String> CONFIRM_TYPES =
      Set.of(
          "button", "channels_select", "checkboxes", "conversations_select", "datepicker",
          "datetimepicker", "external_select", "icon_button", "multi_channels_select",
          "multi_conversations_select", "multi_external_select", "multi_static_select",
          "multi_users_select", "overflow", "radio_buttons", "static_select", "timepicker",
          "users_select", "workflow_button");

  private static final Set<String> SLACK_ICON_NAMES =
      Set.of(
          "archive", "book", "bookmark", "bot", "bug", "calendar", "call", "caret-left",
          "caret-right", "check", "clipboard", "code", "comment", "compass", "copy", "cube",
          "download", "edit", "email", "eye-closed", "eye-open", "file", "flag", "folder",
          "gear", "globe", "heart", "help", "image", "info", "key", "lightbulb", "link",
          "map", "mobile", "new-window", "pin", "plus", "refine", "refresh", "rocket",
          "save", "screen", "share", "sparkle", "star", "star-filled", "tag", "thumbs-down",
          "thumbs-up", "trash", "upload", "user", "warning");

  private static final Set<String> CONTEXT_ELEMENT_TYPES = Set.of("plain_text", "mrkdwn", "image");
  private static final Set<String> ALERT_LEVELS = Set.of("default", "info", "warning", "error", "success");
  private static final Set<String> CONTAINER_WIDTHS = Set.of("narrow", "standard", "wide", "full");
  private static final Set<String> TASK_STATUSES = Set.of("pending", "in_progress", "complete", "error");
  private static final Set<String> TABLE_CELL_TYPES = Set.of("raw_text", "rich_text");
  private static final Set<String> DATA_TABLE_CELL_TYPES = Set.of("raw_text", "rich_text", "raw_number");

  private static final Map<String, Set<String>> SURFACE_BLOCKS =
      Map.of(
          "message",
              Set.of("actions", "card", "carousel", "container", "context", "context_actions",
                  "data_table", "data_visualization", "divider", "file", "header", "image",
                  "markdown", "plan", "rich_text", "section", "table", "task_card", "video"),
          "modal",
              Set.of("actions", "alert", "card", "context", "divider", "header", "image",
                  "input", "rich_text", "section", "video"),
          "home",
              Set.of("actions", "card", "carousel", "container", "context", "data_table",
                  "divider", "header", "image", "input", "rich_text", "section", "table",
                  "video"));

  private static final Pattern ATTACHMENT_COLOR = Pattern.compile("^#[0-9a-fA-F]{6}$");

  private Validator() {}

  /** Validates one fully materialized Slack object at build time. */
  public static void validate(String name, Map<String, Object> value) {
    if (name.isEmpty()) {
      throw new IllegalArgumentException("builder name must not be empty");
    }
    if (value.isEmpty()) {
      throw new IllegalArgumentException("Slack object must not be empty");
    }
    validateBuilder(name, value);
    validateObject(value, "");
  }

  private static Map.Entry<String, List<String>> entry(String type, String... fields) {
    return Map.entry(type, List.of(fields));
  }

  private static void validateBuilder(String name, Map<String, Object> value) {
    switch (name) {
      case "Confirmation" -> validateConfirmation(value, name);
      case "Option" -> validateOption(value, name);
      case "OptionGroup" -> validateOptionGroup(value, name);
      case "ConversationFilter" -> {
        if (!value.containsKey("include")
            && !value.containsKey("exclude_external_shared_channels")
            && !value.containsKey("exclude_bot_users")) {
          fail(ErrorCategory.MISSING_REQUIRED, name, "expected at least one filter field");
        }
      }
      case "InputParameter" -> {
        require(value, "name", name);
        require(value, "value", name);
      }
      case "Trigger" -> require(value, "url", name);
      case "Workflow" -> require(value, "trigger", name);
      case "SlackFile" -> {
        if (value.containsKey("id") == value.containsKey("url")) {
          fail(ErrorCategory.MUTUALLY_EXCLUSIVE, name, "expected exactly one of id or url");
        }
      }
      case "ChartSegment" -> validateLabelValue(value, name, 20, true);
      case "DataPoint" -> validateLabelValue(value, name, 20, false);
      case "DataSeries" -> validateDataSeries(value, name);
      case "AxisConfig" -> validateAxisConfig(value, name);
      case "FeedbackButton" -> validateFeedbackButton(value, name);
      case "Attachment" -> {
        Object raw = value.get("color");
        if (raw instanceof String color
            && !Set.of("good", "warning", "danger").contains(color)
            && !ATTACHMENT_COLOR.matcher(color).matches()) {
          fail(ErrorCategory.TYPE_MISMATCH, child(name, "color"),
              "expected a six-digit hex color or Slack alias");
        }
      }
      case "Message" -> {
        require(value, "channel", name);
        Object raw = value.get("channel");
        if (!(raw instanceof String channel)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(name, "channel"), "expected a string");
        } else {
          stringLength(channel, child(name, "channel"), 1, 0);
        }
        validateMessageCollections(value, name);
      }
      case "MessageResponse", "WebhookMessage" -> validateMessageCollections(value, name);
      default -> {
        // Most builders are completely described by their wire discriminator.
      }
    }
  }

  private static void validateObject(Map<String, Object> value, String path) {
    String type = value.get("type") instanceof String text ? text : "";
    for (String field : REQUIRED_FIELDS.getOrDefault(type, List.of())) {
      require(value, field, path);
    }
    if (value.get("block_id") instanceof String blockId) {
      stringLength(blockId, child(path, "block_id"), 0, 255);
    }
    if (value.get("action_id") instanceof String actionId) {
      stringLength(actionId, child(path, "action_id"), 0, 255);
    }
    if (CONFIRM_TYPES.contains(type) && value.containsKey("confirm")) {
      validateConfirmation(objectAt(value.get("confirm"), child(path, "confirm")), child(path, "confirm"));
    }

    switch (type) {
      case "plain_text", "mrkdwn" -> textLength(value, child(path, "text"), 1, 3000);
      case "icon" -> {
        if (!(value.get("name") instanceof String name) || !SLACK_ICON_NAMES.contains(name)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(path, "name"), "unknown Slack icon");
        }
      }
      case "section" -> validateSection(value, path);
      case "header" -> textLength(value.get("text"), child(path, "text.text"), 0, 150);
      case "button", "workflow_button" -> validateButton(value, path);
      case "icon_button" -> validateIconButton(value, path);
      case "feedback_buttons" -> {
        for (String field : List.of("positive_button", "negative_button")) {
          validateFeedbackButton(objectAt(value.get(field), child(path, field)), child(path, field));
        }
      }
      case "file_input" -> {
        Double number = number(value.get("max_files"));
        if (number != null && (number < 1 || number > 10)) {
          fail(ErrorCategory.OUT_OF_RANGE, child(path, "max_files"), "expected a value between 1 and 10");
        }
      }
      case "plain_text_input" -> {
        Double number = number(value.get("max_length"));
        if (number != null && number > 3000) {
          fail(ErrorCategory.OUT_OF_RANGE, child(path, "max_length"), "exceeds maximum 3000");
        }
        if (value.containsKey("placeholder")) {
          textLength(value.get("placeholder"), child(path, "placeholder.text"), 0, 150);
        }
      }
      case "overflow", "checkboxes", "radio_buttons" -> {
        List<?> options = listAt(value.get("options"), child(path, "options"));
        sliceLength(options, child(path, "options"), 1, type.equals("overflow") ? 5 : 10);
        validateOptions(options, child(path, "options"));
      }
      case "url" -> stringLength(stringAt(value.get("url"), child(path, "url")), child(path, "url"), 1, 3000);
      case "static_select", "multi_static_select" -> validateStaticSelect(value, path);
      case "number_input" -> {
        Double minimum = number(value.get("min_value"));
        Double maximum = number(value.get("max_value"));
        if (minimum != null && maximum != null && minimum > maximum) {
          fail(ErrorCategory.OUT_OF_RANGE, path, "min_value cannot exceed max_value");
        }
      }
      case "image" -> validateImage(value, path);
      case "context" -> validateContext(value, path);
      case "actions" -> sliceLength(listAt(value.get("elements"), child(path, "elements")), child(path, "elements"), 0, 25);
      case "alert" -> {
        textLength(value.get("text"), child(path, "text.text"), 0, 200);
        if (value.get("level") instanceof String level && !ALERT_LEVELS.contains(level)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(path, "level"), "unknown alert level");
        }
      }
      case "card" -> validateCard(value, path);
      case "carousel" -> validateCarousel(value, path);
      case "container" -> validateContainer(value, path);
      case "context_actions" -> sliceLength(listAt(value.get("elements"), child(path, "elements")), child(path, "elements"), 1, 5);
      case "data_table" -> validateTable(value, path, true);
      case "table" -> validateTable(value, path, false);
      case "data_visualization" -> {
        if (value.get("title") instanceof String title) {
          stringLength(title, child(path, "title"), 0, 50);
        }
        objectAt(value.get("chart"), child(path, "chart"));
      }
      case "pie" -> validatePie(value, path);
      case "bar", "area", "line" -> validateSeriesChart(value, path);
      case "task_card" -> {
        if (value.get("status") instanceof String status && !TASK_STATUSES.contains(status)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(path, "status"), "unknown task status");
        }
      }
      case "input" -> validateInput(value, path);
      case "markdown" -> stringLength(stringAt(value.get("text"), child(path, "text")), child(path, "text"), 1, 12000);
      case "video" -> validateVideo(value, path);
      case "modal", "home" -> validateView(value, path, type);
      default -> {
        // Generic required-field and nested checks still apply.
      }
    }

    for (Map.Entry<String, Object> field : value.entrySet()) {
      if (!field.getKey().equals("type") && !field.getKey().equals("event_payload")) {
        validateNested(field.getValue(), child(path, field.getKey()));
      }
    }
  }

  private static void validateSection(Map<String, Object> value, String path) {
    boolean hasText = value.containsKey("text");
    List<?> fields = value.containsKey("fields") ? listAt(value.get("fields"), child(path, "fields")) : List.of();
    if (!hasText && fields.isEmpty()) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected text, fields, or both");
    }
    if (hasText) {
      textLength(value.get("text"), child(path, "text.text"), 0, 3000);
    }
    if (value.containsKey("fields")) {
      sliceLength(fields, child(path, "fields"), 0, 10);
      for (int index = 0; index < fields.size(); index++) {
        textLength(fields.get(index), child(path, "fields") + "[" + index + "].text", 0, 2000);
      }
    }
  }

  private static void validateButton(Map<String, Object> value, String path) {
    textLength(value.get("text"), child(path, "text.text"), 0, 75);
    checkOptionalString(value, "url", path, 3000);
    checkOptionalString(value, "value", path, 2000);
    checkOptionalString(value, "accessibility_label", path, 75);
  }

  private static void validateIconButton(Map<String, Object> value, String path) {
    if (!"trash".equals(value.get("icon"))) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "icon"), "expected trash");
    }
    checkOptionalString(value, "value", path, 2000);
    checkOptionalString(value, "accessibility_label", path, 75);
    if (value.get("visible_to_user_ids") instanceof List<?> users) {
      sliceLength(users, child(path, "visible_to_user_ids"), 0, 10);
    }
  }

  private static void validateStaticSelect(Map<String, Object> value, String path) {
    boolean hasOptions = value.containsKey("options");
    boolean hasGroups = value.containsKey("option_groups");
    if (hasOptions && hasGroups) {
      fail(ErrorCategory.MUTUALLY_EXCLUSIVE, path, "options and option_groups cannot be provided together");
    }
    if (hasOptions) {
      List<?> options = listAt(value.get("options"), child(path, "options"));
      sliceLength(options, child(path, "options"), 0, 100);
      validateOptions(options, child(path, "options"));
    }
    if (hasGroups) {
      List<?> groups = listAt(value.get("option_groups"), child(path, "option_groups"));
      sliceLength(groups, child(path, "option_groups"), 0, 100);
      for (int index = 0; index < groups.size(); index++) {
        validateOptionGroup(objectAt(groups.get(index), child(path, "option_groups") + "[" + index + "]"),
            child(path, "option_groups") + "[" + index + "]");
      }
    }
    if (value.containsKey("placeholder")) {
      textLength(value.get("placeholder"), child(path, "placeholder.text"), 0, 150);
    }
  }

  private static void validateImage(Map<String, Object> value, String path) {
    boolean hasUrl = value.containsKey("image_url");
    boolean hasSlackFile = value.containsKey("slack_file");
    if (!hasUrl && !hasSlackFile) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected image_url or slack_file");
    }
    if (hasUrl && hasSlackFile) {
      fail(ErrorCategory.MUTUALLY_EXCLUSIVE, path, "image_url and slack_file cannot be provided together");
    }
    checkOptionalString(value, "image_url", path, 3000);
    checkOptionalString(value, "alt_text", path, 2000);
  }

  private static void validateContext(Map<String, Object> value, String path) {
    List<?> elements = listAt(value.get("elements"), child(path, "elements"));
    sliceLength(elements, child(path, "elements"), 0, 10);
    for (int index = 0; index < elements.size(); index++) {
      String elementPath = child(path, "elements") + "[" + index + "]";
      if (!CONTEXT_ELEMENT_TYPES.contains(objectType(objectAt(elements.get(index), elementPath)))) {
        fail(ErrorCategory.TYPE_MISMATCH, elementPath, "expected text or image element");
      }
    }
  }

  private static void validateCard(Map<String, Object> value, String path) {
    if (!value.containsKey("hero_image") && !value.containsKey("title")
        && !value.containsKey("actions") && !value.containsKey("body")) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected hero_image, title, actions, or body");
    }
    if (value.containsKey("icon") && value.containsKey("slack_icon")) {
      fail(ErrorCategory.MUTUALLY_EXCLUSIVE, path, "icon and slack_icon cannot be provided together");
    }
    checkOptionalText(value, "title", path, 150);
    checkOptionalText(value, "subtitle", path, 150);
    checkOptionalText(value, "body", path, 200);
    checkOptionalText(value, "subtext", path, 200);
    if (value.get("actions") instanceof List<?> actions) {
      sliceLength(actions, child(path, "actions"), 0, 3);
    }
  }

  private static void validateCarousel(Map<String, Object> value, String path) {
    List<?> cards = listAt(value.get("elements"), child(path, "elements"));
    sliceLength(cards, child(path, "elements"), 1, 10);
    for (int index = 0; index < cards.size(); index++) {
      String cardPath = child(path, "elements") + "[" + index + "]";
      if (!"card".equals(objectType(objectAt(cards.get(index), cardPath)))) {
        fail(ErrorCategory.TYPE_MISMATCH, cardPath, "expected a card");
      }
    }
  }

  private static void validateContainer(Map<String, Object> value, String path) {
    if (!value.containsKey("title") && !value.containsKey("rich_text_title")) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected title or rich_text_title");
    }
    checkOptionalText(value, "title", path, 150);
    checkOptionalText(value, "subtitle", path, 150);
    List<?> blocks = listAt(value.get("child_blocks"), child(path, "child_blocks"));
    sliceLength(blocks, child(path, "child_blocks"), 1, 10);
    if (value.get("width") instanceof String width && !CONTAINER_WIDTHS.contains(width)) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "width"), "unknown container width");
    }
    if (Boolean.TRUE.equals(value.get("default_collapsed"))
        && !Boolean.TRUE.equals(value.get("is_collapsible"))) {
      fail(ErrorCategory.INVALID_USAGE, child(path, "default_collapsed"), "requires is_collapsible");
    }
    if (Boolean.TRUE.equals(value.get("has_header_divider"))
        && Boolean.TRUE.equals(value.get("is_collapsible"))) {
      fail(ErrorCategory.INVALID_USAGE, child(path, "has_header_divider"),
          "requires a non-collapsible container");
    }
  }

  private static void validatePie(Map<String, Object> value, String path) {
    List<?> segments = listAt(value.get("segments"), child(path, "segments"));
    sliceLength(segments, child(path, "segments"), 1, 12);
    for (int index = 0; index < segments.size(); index++) {
      String segmentPath = child(path, "segments") + "[" + index + "]";
      validateLabelValue(objectAt(segments.get(index), segmentPath), segmentPath, 20, true);
    }
  }

  private static void validateInput(Map<String, Object> value, String path) {
    textLength(value.get("label"), child(path, "label.text"), 0, 2000);
    if (value.containsKey("hint")) {
      textLength(value.get("hint"), child(path, "hint.text"), 0, 2000);
    }
    Map<String, Object> element = objectAt(value.get("element"), child(path, "element"));
    if (!INPUT_ELEMENT_TYPES.contains(objectType(element))) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "element"), "expected an input-compatible element");
    }
  }

  private static void validateVideo(Map<String, Object> value, String path) {
    if (value.get("alt_text") instanceof String altText) {
      stringLength(altText, child(path, "alt_text"), 1, 200);
    }
    textLength(value.get("title"), child(path, "title.text"), 0, 200);
    checkOptionalString(value, "author_name", path, 50);
    checkOptionalString(value, "provider_name", path, 50);
    checkOptionalText(value, "description", path, 200);
  }

  private static void validateView(Map<String, Object> value, String path, String type) {
    List<?> blocks = listAt(value.get("blocks"), child(path, "blocks"));
    sliceLength(blocks, child(path, "blocks"), 1, 100);
    validateSurface(blocks, type, child(path, "blocks"));
    checkOptionalString(value, "private_metadata", path, 3000);
    checkOptionalString(value, "callback_id", path, 255);
    if (type.equals("modal")) {
      if (!value.containsKey("submit")) {
        for (Object raw : blocks) {
          if (raw instanceof Map<?, ?> && "input".equals(objectType(objectAt(raw, child(path, "blocks"))))) {
            fail(ErrorCategory.MISSING_REQUIRED, child(path, "submit"),
                "required when the modal contains an input block");
          }
        }
      }
      checkOptionalText(value, "title", path, 24);
      checkOptionalText(value, "close", path, 24);
      checkOptionalText(value, "submit", path, 24);
    }
  }

  private static void validateOptionGroup(Map<String, Object> value, String path) {
    require(value, "label", path);
    textLength(value.get("label"), child(path, "label.text"), 0, 75);
    List<?> options = listAt(value.get("options"), child(path, "options"));
    sliceLength(options, child(path, "options"), 1, 100);
    validateOptions(options, child(path, "options"));
  }

  private static void validateFeedbackButton(Map<String, Object> value, String path) {
    require(value, "text", path);
    require(value, "value", path);
    textLength(value.get("text"), child(path, "text.text"), 0, 75);
    checkOptionalString(value, "value", path, 2000);
    checkOptionalString(value, "accessibility_label", path, 75);
  }

  private static void validateDataSeries(Map<String, Object> value, String path) {
    String name = stringAt(value.get("name"), child(path, "name"));
    stringLength(name, child(path, "name"), 0, 20);
    List<?> data = listAt(value.get("data"), child(path, "data"));
    sliceLength(data, child(path, "data"), 1, 20);
  }

  private static void validateAxisConfig(Map<String, Object> value, String path) {
    List<?> categories = listAt(value.get("categories"), child(path, "categories"));
    sliceLength(categories, child(path, "categories"), 1, 20);
    Set<String> seen = new java.util.HashSet<>();
    for (int index = 0; index < categories.size(); index++) {
      String itemPath = child(path, "categories") + "[" + index + "]";
      String category = stringAt(categories.get(index), itemPath);
      stringLength(category, itemPath, 0, 20);
      if (!seen.add(category)) {
        fail(ErrorCategory.INVALID_USAGE, child(path, "categories"), "expected unique labels");
      }
    }
    checkOptionalString(value, "x_label", path, 50);
    checkOptionalString(value, "y_label", path, 50);
  }

  private static void validateConfirmation(Map<String, Object> value, String path) {
    Map<String, Integer> limits = Map.of("title", 100, "text", 300, "confirm", 30, "deny", 30);
    for (Map.Entry<String, Integer> field : limits.entrySet()) {
      require(value, field.getKey(), path);
      textLength(value.get(field.getKey()), child(path, field.getKey() + ".text"), 0, field.getValue());
    }
  }

  private static void validateOption(Map<String, Object> value, String path) {
    require(value, "text", path);
    require(value, "value", path);
    textLength(value.get("text"), child(path, "text.text"), 0, 75);
    checkOptionalString(value, "value", path, 150);
    checkOptionalText(value, "description", path, 75);
    checkOptionalString(value, "url", path, 3000);
  }

  private static void validateOptions(List<?> values, String path) {
    for (int index = 0; index < values.size(); index++) {
      String optionPath = path + "[" + index + "]";
      validateOption(objectAt(values.get(index), optionPath), optionPath);
    }
  }

  private static void validateTable(Map<String, Object> value, String path, boolean dataTable) {
    List<?> rows = listAt(value.get("rows"), child(path, "rows"));
    int minimumRows = dataTable ? 2 : 1;
    int maximumRows = dataTable ? 201 : 100;
    int minimumColumns = dataTable ? 1 : 0;
    sliceLength(rows, child(path, "rows"), minimumRows, maximumRows);
    int columns = -1;
    int contentLength = 0;
    for (int rowIndex = 0; rowIndex < rows.size(); rowIndex++) {
      String rowPath = child(path, "rows") + "[" + rowIndex + "]";
      List<?> row = listAt(rows.get(rowIndex), rowPath);
      sliceLength(row, rowPath, minimumColumns, 20);
      if (columns < 0) {
        columns = row.size();
      } else if (row.size() != columns) {
        fail(ErrorCategory.INVALID_USAGE, rowPath, "column count differs");
      }
      for (int cellIndex = 0; cellIndex < row.size(); cellIndex++) {
        String cellPath = rowPath + "[" + cellIndex + "]";
        Map<String, Object> cell = objectAt(row.get(cellIndex), cellPath);
        Set<String> allowed = dataTable ? DATA_TABLE_CELL_TYPES : TABLE_CELL_TYPES;
        if (!allowed.contains(objectType(cell))) {
          fail(ErrorCategory.TYPE_MISMATCH, cellPath, "unsupported table cell");
        }
        if (dataTable && rowIndex == 0 && "rich_text".equals(objectType(cell))) {
          fail(ErrorCategory.TYPE_MISMATCH, cellPath, "header cells cannot contain rich text");
        }
        contentLength += textCharacterCount(cell);
        if (dataTable && Set.of("raw_text", "raw_number").contains(objectType(cell))) {
          stringLength(stringAt(cell.get("text"), child(cellPath, "text")), child(cellPath, "text"), 1, 0);
        }
      }
    }
    if (value.containsKey("column_settings")) {
      List<?> settings = listAt(value.get("column_settings"), child(path, "column_settings"));
      sliceLength(settings, child(path, "column_settings"), 0, 20);
      if (settings.size() != columns) {
        fail(ErrorCategory.INVALID_USAGE, child(path, "column_settings"),
            "expected one entry for every column");
      }
    }
    if (dataTable) {
      Double pageSize = number(value.get("page_size"));
      if (pageSize != null && (pageSize < 1 || pageSize > 100)) {
        fail(ErrorCategory.OUT_OF_RANGE, child(path, "page_size"), "expected a value between 1 and 100");
      }
      String caption = stringAt(value.get("caption"), child(path, "caption"));
      stringLength(caption, child(path, "caption"), 1, 0);
      if (contentLength > 20000) {
        fail(ErrorCategory.LENGTH_EXCEEDED, child(path, "rows"), "content exceeds maximum 20000");
      }
    }
  }

  private static void validateSeriesChart(Map<String, Object> value, String path) {
    List<?> series = listAt(value.get("series"), child(path, "series"));
    sliceLength(series, child(path, "series"), 1, 12);
    Map<String, Object> axis = objectAt(value.get("axis_config"), child(path, "axis_config"));
    validateAxisConfig(axis, child(path, "axis_config"));
    List<?> categories = listAt(axis.get("categories"), child(path, "axis_config.categories"));
    Set<String> categorySet = new java.util.HashSet<>();
    for (Object raw : categories) {
      categorySet.add(stringAt(raw, child(path, "axis_config.categories")));
    }
    Set<String> names = new java.util.HashSet<>();
    for (int index = 0; index < series.size(); index++) {
      String itemPath = child(path, "series") + "[" + index + "]";
      Map<String, Object> item = objectAt(series.get(index), itemPath);
      validateDataSeries(item, itemPath);
      String name = stringAt(item.get("name"), child(itemPath, "name"));
      if (!names.add(name)) {
        fail(ErrorCategory.INVALID_USAGE, child(path, "series"), "series names must be unique");
      }
      List<?> points = listAt(item.get("data"), child(itemPath, "data"));
      Set<String> seen = new java.util.HashSet<>();
      for (int pointIndex = 0; pointIndex < points.size(); pointIndex++) {
        String pointPath = child(itemPath, "data") + "[" + pointIndex + "]";
        Map<String, Object> point = objectAt(points.get(pointIndex), pointPath);
        validateLabelValue(point, pointPath, 20, false);
        seen.add(stringAt(point.get("label"), child(pointPath, "label")));
      }
      if (points.size() != categorySet.size() || !seen.equals(categorySet)) {
        fail(ErrorCategory.INVALID_USAGE, child(itemPath, "data"),
            "expected exactly one point for every axis category");
      }
    }
  }

  private static void validateLabelValue(
      Map<String, Object> value, String path, int labelMaximum, boolean positive) {
    require(value, "label", path);
    String label = stringAt(value.get("label"), child(path, "label"));
    stringLength(label, child(path, "label"), 0, labelMaximum);
    Double number = number(value.get("value"));
    if (number == null || !Double.isFinite(number)) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "value"), "expected a finite number");
    }
    if (positive && number <= 0) {
      fail(ErrorCategory.OUT_OF_RANGE, child(path, "value"), "expected a value greater than 0");
    }
  }

  private static void validateMessageCollections(Map<String, Object> value, String path) {
    if (value.containsKey("blocks")) {
      List<?> blocks = listAt(value.get("blocks"), child(path, "blocks"));
      sliceLength(blocks, child(path, "blocks"), 0, 50);
      validateSurface(blocks, "message", child(path, "blocks"));
    }
    if (value.containsKey("attachments")) {
      sliceLength(listAt(value.get("attachments"), child(path, "attachments")),
          child(path, "attachments"), 0, 100);
    }
  }

  private static void validateSurface(List<?> blocks, String surface, String path) {
    Set<String> allowed = SURFACE_BLOCKS.get(surface);
    for (int index = 0; index < blocks.size(); index++) {
      String blockPath = path + "[" + index + "]";
      String type = objectType(objectAt(blocks.get(index), blockPath));
      if (!allowed.contains(type)) {
        fail(ErrorCategory.TYPE_MISMATCH, blockPath + ".type",
            "block type " + type + " is not supported on " + surface + " surfaces");
      }
    }
  }

  private static void validateNested(Object value, String path) {
    if (value instanceof Map<?, ?>) {
      validateObject(objectAt(value, path), path);
    } else if (value instanceof List<?> list) {
      for (int index = 0; index < list.size(); index++) {
        validateNested(list.get(index), path + "[" + index + "]");
      }
    } else if (value instanceof Double number && !Double.isFinite(number)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected a finite number");
    } else if (value instanceof Float number && !Float.isFinite(number)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected a finite number");
    }
  }

  private static Map<String, Object> objectAt(Object value, String path) {
    if (!(value instanceof Map<?, ?> map)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected an object");
      throw new AssertionError("unreachable");
    }
    Map<String, Object> result = new LinkedHashMap<>();
    for (Map.Entry<?, ?> entry : map.entrySet()) {
      if (!(entry.getKey() instanceof String key)) {
        fail(ErrorCategory.TYPE_MISMATCH, path, "expected string object keys");
      } else {
        result.put(key, entry.getValue());
      }
    }
    return result;
  }

  private static List<?> listAt(Object value, String path) {
    if (!(value instanceof List<?> list)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected an array");
      throw new AssertionError("unreachable");
    }
    return list;
  }

  private static String stringAt(Object value, String path) {
    if (!(value instanceof String text)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected a string");
      throw new AssertionError("unreachable");
    }
    return text;
  }

  private static String objectType(Map<String, Object> value) {
    return value.get("type") instanceof String type ? type : "";
  }

  private static void textLength(Object value, String path, int minimum, int maximum) {
    String text;
    if (value instanceof String direct) {
      text = direct;
    } else if (value instanceof Map<?, ?>) {
      text = stringAt(objectAt(value, path).get("text"), path);
    } else {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected text");
      throw new AssertionError("unreachable");
    }
    stringLength(text, path, minimum, maximum);
  }

  private static void stringLength(String value, String path, int minimum, int maximum) {
    int size = value.codePointCount(0, value.length());
    if (minimum > 0 && size < minimum) {
      fail(ErrorCategory.LENGTH_EXCEEDED, path, size + " is less than minimum " + minimum);
    }
    if (maximum > 0 && size > maximum) {
      fail(ErrorCategory.LENGTH_EXCEEDED, path, size + " exceeds maximum " + maximum);
    }
  }

  private static void sliceLength(List<?> value, String path, int minimum, int maximum) {
    if (minimum > 0 && value.size() < minimum) {
      fail(ErrorCategory.LENGTH_EXCEEDED, path, value.size() + " is less than minimum " + minimum);
    }
    if (maximum > 0 && value.size() > maximum) {
      fail(ErrorCategory.LENGTH_EXCEEDED, path, value.size() + " exceeds maximum " + maximum);
    }
  }

  private static Double number(Object value) {
    return value instanceof Number number ? number.doubleValue() : null;
  }

  private static int textCharacterCount(Object value) {
    if (value instanceof Map<?, ?>) {
      int total = 0;
      for (Map.Entry<String, Object> entry : objectAt(value, "").entrySet()) {
        if (entry.getKey().equals("text") && entry.getValue() instanceof String text) {
          total += text.codePointCount(0, text.length());
        } else {
          total += textCharacterCount(entry.getValue());
        }
      }
      return total;
    }
    if (value instanceof List<?> list) {
      int total = 0;
      for (Object item : list) {
        total += textCharacterCount(item);
      }
      return total;
    }
    return 0;
  }

  private static void checkOptionalString(
      Map<String, Object> value, String field, String path, int maximum) {
    if (value.get(field) instanceof String text) {
      stringLength(text, child(path, field), 0, maximum);
    }
  }

  private static void checkOptionalText(
      Map<String, Object> value, String field, String path, int maximum) {
    if (value.containsKey(field)) {
      textLength(value.get(field), child(path, field + ".text"), 0, maximum);
    }
  }

  private static void require(Map<String, Object> value, String field, String path) {
    if (!value.containsKey(field)) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected " + field);
    }
  }

  private static String child(String path, String field) {
    return path.isEmpty() ? field : path + "." + field;
  }

  private static void fail(ErrorCategory category, String path, String message) {
    throw new ValidationException(category, path, message);
  }
}
