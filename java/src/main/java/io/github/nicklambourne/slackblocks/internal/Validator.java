package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.ErrorCategory;
import io.github.nicklambourne.slackblocks.ValidationException;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Set;
import java.util.regex.Pattern;
import org.jspecify.annotations.Nullable;

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
          "plain_text_input",
          "number_input",
          "checkboxes",
          "radio_buttons",
          "datepicker",
          "datetimepicker",
          "timepicker",
          "channels_select",
          "multi_channels_select",
          "conversations_select",
          "multi_conversations_select",
          "external_select",
          "multi_external_select",
          "static_select",
          "multi_static_select",
          "users_select",
          "multi_users_select",
          "rich_text_input",
          "email_text_input",
          "url_text_input",
          "file_input");

  private static final Set<String> CONFIRM_TYPES =
      Set.of(
          "button",
          "channels_select",
          "checkboxes",
          "conversations_select",
          "datepicker",
          "datetimepicker",
          "external_select",
          "icon_button",
          "multi_channels_select",
          "multi_conversations_select",
          "multi_external_select",
          "multi_static_select",
          "multi_users_select",
          "overflow",
          "radio_buttons",
          "static_select",
          "timepicker",
          "users_select",
          "workflow_button");

  private static final Set<String> CONTEXT_ELEMENT_TYPES = Set.of("plain_text", "mrkdwn", "image");
  private static final Set<String> ALERT_LEVELS =
      Set.of("default", "info", "warning", "error", "success");
  private static final Set<String> CONTAINER_WIDTHS = Set.of("narrow", "standard", "wide", "full");
  private static final Set<String> TASK_STATUSES =
      Set.of("pending", "in_progress", "complete", "error");
  private static final Set<String> TABLE_CELL_TYPES = Set.of("raw_text", "rich_text");
  private static final Set<String> DATA_TABLE_CELL_TYPES =
      Set.of("raw_text", "rich_text", "raw_number");

  private static final Pattern ATTACHMENT_COLOR = Pattern.compile("^#[0-9a-fA-F]{6}$");

  private Validator() {}

  /** Validates one fully materialized Slack object at build time. */
  public static void validate(String name, Map<String, Object> value) {
    if (name.isEmpty()) {
      throw new IllegalArgumentException("builder name must not be empty");
    }
    if (value.isEmpty()) {
      fail(ErrorCategory.MISSING_REQUIRED, name, "expected at least one field");
    }
    validateBuilder(name, value);
    validateObject(value, name);
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
      case "ChartSegment" ->
          validateLabelValue(
              value, name, SlackLimits.DATA_VISUALIZATION_SEGMENT_LABEL_MAX_LENGTH, true);
      case "DataPoint" ->
          validateLabelValue(
              value, name, SlackLimits.DATA_VISUALIZATION_POINT_LABEL_MAX_LENGTH, false);
      case "DataSeries" -> validateDataSeries(value, name);
      case "AxisConfig" -> validateAxisConfig(value, name);
      case "FeedbackButton" -> validateFeedbackButton(value, name);
      case "DispatchActionConfiguration" -> validateDispatchActionConfiguration(value, name);
      case "Attachment" -> {
        Object raw = value.get("color");
        if (raw instanceof String color
            && !Set.of("good", "warning", "danger").contains(color)
            && !ATTACHMENT_COLOR.matcher(color).matches()) {
          fail(
              ErrorCategory.TYPE_MISMATCH,
              child(name, "color"),
              "expected a six-digit hex color or Slack alias");
        }
      }
      case "MessagePayload" -> {
        require(value, "channel", name);
        Object raw = value.get("channel");
        if (!(raw instanceof String channel)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(name, "channel"), "expected a string");
        } else {
          stringLength(channel, child(name, "channel"), SlackLimits.MESSAGE_CHANNEL_MIN_LENGTH, 0);
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
      stringLength(blockId, child(path, "block_id"), 0, SlackLimits.BLOCK_ID_MAX_LENGTH);
    }
    if (value.get("action_id") instanceof String actionId) {
      stringLength(actionId, child(path, "action_id"), 0, SlackLimits.ACTION_ID_MAX_LENGTH);
    }
    if (CONFIRM_TYPES.contains(type) && value.containsKey("confirm")) {
      validateConfirmation(
          objectAt(value.get("confirm"), child(path, "confirm")), child(path, "confirm"));
    }
    if (value.containsKey("dispatch_action_config")) {
      validateDispatchActionConfiguration(
          objectAt(value.get("dispatch_action_config"), child(path, "dispatch_action_config")),
          child(path, "dispatch_action_config"));
    }

    switch (type) {
      case "plain_text", "mrkdwn" ->
          textLength(
              value, child(path, "text"), SlackLimits.TEXT_MIN_LENGTH, SlackLimits.TEXT_MAX_LENGTH);
      case "icon" -> {
        if (!(value.get("name") instanceof String name)
            || !SlackVocabulary.SLACK_ICON_NAMES.contains(name)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(path, "name"), "unknown Slack icon");
        }
      }
      case "section" -> validateSection(value, path);
      case "header" ->
          textLength(
              value.get("text"), child(path, "text.text"), 0, SlackLimits.HEADER_TEXT_MAX_LENGTH);
      case "button" -> validateButton(value, path);
      case "workflow_button" -> validateWorkflowButton(value, path);
      case "icon_button" -> validateIconButton(value, path);
      case "feedback_buttons" -> {
        for (String field : List.of("positive_button", "negative_button")) {
          validateFeedbackButton(
              objectAt(value.get(field), child(path, field)), child(path, field));
        }
      }
      case "file_input" -> {
        Double number = number(value.get("max_files"));
        if (number != null
            && (number < SlackLimits.FILE_INPUT_MAX_FILES_MIN
                || number > SlackLimits.FILE_INPUT_MAX_FILES_MAX)) {
          fail(
              ErrorCategory.OUT_OF_RANGE,
              child(path, "max_files"),
              "expected a value between "
                  + SlackLimits.FILE_INPUT_MAX_FILES_MIN
                  + " and "
                  + SlackLimits.FILE_INPUT_MAX_FILES_MAX);
        }
      }
      case "plain_text_input" -> {
        Double number = number(value.get("max_length"));
        if (number != null && number > SlackLimits.PLAIN_TEXT_INPUT_MAX_LENGTH_MAX) {
          fail(
              ErrorCategory.OUT_OF_RANGE,
              child(path, "max_length"),
              "exceeds maximum " + SlackLimits.PLAIN_TEXT_INPUT_MAX_LENGTH_MAX);
        }
        if (value.containsKey("placeholder")) {
          textLength(
              value.get("placeholder"),
              child(path, "placeholder.text"),
              0,
              SlackLimits.PLAIN_TEXT_INPUT_PLACEHOLDER_MAX_LENGTH);
        }
      }
      case "email_text_input" ->
          checkOptionalText(
              value, "placeholder", path, SlackLimits.EMAIL_INPUT_PLACEHOLDER_MAX_LENGTH);
      case "url_text_input" ->
          checkOptionalText(
              value, "placeholder", path, SlackLimits.URL_INPUT_PLACEHOLDER_MAX_LENGTH);
      case "datepicker" ->
          checkOptionalText(
              value, "placeholder", path, SlackLimits.DATE_PICKER_PLACEHOLDER_MAX_LENGTH);
      case "timepicker" ->
          checkOptionalText(
              value, "placeholder", path, SlackLimits.TIME_PICKER_PLACEHOLDER_MAX_LENGTH);
      case "rich_text_input" ->
          checkOptionalText(
              value, "placeholder", path, SlackLimits.RICH_TEXT_INPUT_PLACEHOLDER_MAX_LENGTH);
      case "overflow", "checkboxes", "radio_buttons" -> {
        List<?> options = listAt(value.get("options"), child(path, "options"));
        sliceLength(
            options,
            child(path, "options"),
            switch (type) {
              case "overflow" -> SlackLimits.OVERFLOW_OPTIONS_MIN_ITEMS;
              case "checkboxes" -> SlackLimits.CHECKBOXES_OPTIONS_MIN_ITEMS;
              default -> SlackLimits.RADIO_BUTTONS_OPTIONS_MIN_ITEMS;
            },
            switch (type) {
              case "overflow" -> SlackLimits.OVERFLOW_OPTIONS_MAX_ITEMS;
              case "checkboxes" -> SlackLimits.CHECKBOXES_OPTIONS_MAX_ITEMS;
              default -> SlackLimits.RADIO_BUTTONS_OPTIONS_MAX_ITEMS;
            });
        validateOptions(options, child(path, "options"));
      }
      case "url" ->
          stringLength(
              stringAt(value.get("url"), child(path, "url")),
              child(path, "url"),
              SlackLimits.URL_SOURCE_URL_MIN_LENGTH,
              SlackLimits.URL_SOURCE_URL_MAX_LENGTH);
      case "static_select", "multi_static_select" -> validateStaticSelect(value, path);
      case "number_input" -> {
        Double minimum = number(value.get("min_value"));
        Double maximum = number(value.get("max_value"));
        if (minimum != null && maximum != null && minimum > maximum) {
          fail(ErrorCategory.OUT_OF_RANGE, path, "min_value cannot exceed max_value");
        }
        checkOptionalText(
            value, "placeholder", path, SlackLimits.NUMBER_INPUT_PLACEHOLDER_MAX_LENGTH);
      }
      case "image" -> validateImage(value, path);
      case "context" -> validateContext(value, path);
      case "actions" ->
          sliceLength(
              listAt(value.get("elements"), child(path, "elements")),
              child(path, "elements"),
              0,
              SlackLimits.ACTIONS_ELEMENTS_MAX_ITEMS);
      case "alert" -> {
        textLength(
            value.get("text"), child(path, "text.text"), 0, SlackLimits.ALERT_TEXT_MAX_LENGTH);
        if (value.get("level") instanceof String level && !ALERT_LEVELS.contains(level)) {
          fail(ErrorCategory.TYPE_MISMATCH, child(path, "level"), "unknown alert level");
        }
      }
      case "card" -> validateCard(value, path);
      case "carousel" -> validateCarousel(value, path);
      case "container" -> validateContainer(value, path);
      case "context_actions" ->
          sliceLength(
              listAt(value.get("elements"), child(path, "elements")),
              child(path, "elements"),
              1,
              SlackLimits.CONTEXT_ACTIONS_ELEMENTS_MAX_ITEMS);
      case "data_table" -> validateTable(value, path, true);
      case "table" -> validateTable(value, path, false);
      case "data_visualization" -> {
        if (value.get("title") instanceof String title) {
          stringLength(
              title, child(path, "title"), 0, SlackLimits.DATA_VISUALIZATION_TITLE_MAX_LENGTH);
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
      case "markdown" ->
          stringLength(
              stringAt(value.get("text"), child(path, "text")),
              child(path, "text"),
              SlackLimits.MARKDOWN_TEXT_MIN_LENGTH,
              SlackLimits.MARKDOWN_TEXT_MAX_LENGTH);
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
    List<?> fields =
        value.containsKey("fields")
            ? listAt(value.get("fields"), child(path, "fields"))
            : List.of();
    if (!hasText && fields.isEmpty()) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected text, fields, or both");
    }
    if (hasText) {
      textLength(
          value.get("text"), child(path, "text.text"), 0, SlackLimits.SECTION_TEXT_MAX_LENGTH);
    }
    if (value.containsKey("fields")) {
      sliceLength(fields, child(path, "fields"), 0, SlackLimits.SECTION_FIELDS_MAX_ITEMS);
      for (int index = 0; index < fields.size(); index++) {
        textLength(
            fields.get(index),
            child(path, "fields") + "[" + index + "].text",
            0,
            SlackLimits.SECTION_FIELDS_ITEM_MAX_LENGTH);
      }
    }
  }

  private static void validateButton(Map<String, Object> value, String path) {
    textLength(value.get("text"), child(path, "text.text"), 0, SlackLimits.BUTTON_TEXT_MAX_LENGTH);
    checkOptionalString(value, "url", path, SlackLimits.BUTTON_URL_MAX_LENGTH);
    checkOptionalString(value, "value", path, SlackLimits.BUTTON_VALUE_MAX_LENGTH);
    checkOptionalString(
        value, "accessibility_label", path, SlackLimits.BUTTON_ACCESSIBILITY_LABEL_MAX_LENGTH);
  }

  private static void validateWorkflowButton(Map<String, Object> value, String path) {
    textLength(
        value.get("text"),
        child(path, "text.text"),
        0,
        SlackLimits.WORKFLOW_BUTTON_TEXT_MAX_LENGTH);
    checkOptionalString(
        value,
        "accessibility_label",
        path,
        SlackLimits.WORKFLOW_BUTTON_ACCESSIBILITY_LABEL_MAX_LENGTH);
  }

  private static void validateIconButton(Map<String, Object> value, String path) {
    if (!"trash".equals(value.get("icon"))) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "icon"), "expected trash");
    }
    checkOptionalString(value, "value", path, SlackLimits.ICON_BUTTON_VALUE_MAX_LENGTH);
    checkOptionalString(
        value, "accessibility_label", path, SlackLimits.ICON_BUTTON_ACCESSIBILITY_LABEL_MAX_LENGTH);
    if (value.get("visible_to_user_ids") instanceof List<?> users) {
      sliceLength(
          users,
          child(path, "visible_to_user_ids"),
          0,
          SlackLimits.ICON_BUTTON_VISIBLE_TO_USER_IDS_MAX_ITEMS);
    }
  }

  private static void validateStaticSelect(Map<String, Object> value, String path) {
    boolean hasOptions = value.containsKey("options");
    boolean hasGroups = value.containsKey("option_groups");
    if (hasOptions && hasGroups) {
      fail(
          ErrorCategory.MUTUALLY_EXCLUSIVE,
          path,
          "options and option_groups cannot be provided together");
    }
    if (hasOptions) {
      List<?> options = listAt(value.get("options"), child(path, "options"));
      sliceLength(options, child(path, "options"), 0, SlackLimits.SELECT_OPTIONS_MAX_ITEMS);
      validateOptions(options, child(path, "options"));
    }
    if (hasGroups) {
      List<?> groups = listAt(value.get("option_groups"), child(path, "option_groups"));
      sliceLength(
          groups, child(path, "option_groups"), 0, SlackLimits.SELECT_OPTION_GROUPS_MAX_ITEMS);
      for (int index = 0; index < groups.size(); index++) {
        validateOptionGroup(
            objectAt(groups.get(index), child(path, "option_groups") + "[" + index + "]"),
            child(path, "option_groups") + "[" + index + "]");
      }
    }
    if (value.containsKey("placeholder")) {
      textLength(
          value.get("placeholder"),
          child(path, "placeholder.text"),
          0,
          SlackLimits.SELECT_PLACEHOLDER_MAX_LENGTH);
    }
  }

  private static void validateImage(Map<String, Object> value, String path) {
    boolean hasUrl = value.containsKey("image_url");
    boolean hasSlackFile = value.containsKey("slack_file");
    if (!hasUrl && !hasSlackFile) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected image_url or slack_file");
    }
    if (hasUrl && hasSlackFile) {
      fail(
          ErrorCategory.MUTUALLY_EXCLUSIVE,
          path,
          "image_url and slack_file cannot be provided together");
    }
    checkOptionalString(value, "image_url", path, SlackLimits.IMAGE_IMAGE_URL_MAX_LENGTH);
    checkOptionalString(value, "alt_text", path, SlackLimits.IMAGE_ALT_TEXT_MAX_LENGTH);
  }

  private static void validateContext(Map<String, Object> value, String path) {
    List<?> elements = listAt(value.get("elements"), child(path, "elements"));
    sliceLength(elements, child(path, "elements"), 0, SlackLimits.CONTEXT_ELEMENTS_MAX_ITEMS);
    for (int index = 0; index < elements.size(); index++) {
      String elementPath = child(path, "elements") + "[" + index + "]";
      if (!CONTEXT_ELEMENT_TYPES.contains(objectType(objectAt(elements.get(index), elementPath)))) {
        fail(ErrorCategory.TYPE_MISMATCH, elementPath, "expected text or image element");
      }
    }
  }

  private static void validateCard(Map<String, Object> value, String path) {
    if (!value.containsKey("hero_image")
        && !value.containsKey("title")
        && !value.containsKey("actions")
        && !value.containsKey("body")) {
      fail(ErrorCategory.MISSING_REQUIRED, path, "expected hero_image, title, actions, or body");
    }
    if (value.containsKey("icon") && value.containsKey("slack_icon")) {
      fail(
          ErrorCategory.MUTUALLY_EXCLUSIVE,
          path,
          "icon and slack_icon cannot be provided together");
    }
    checkOptionalText(value, "title", path, SlackLimits.CARD_TITLE_MAX_LENGTH);
    checkOptionalText(value, "subtitle", path, SlackLimits.CARD_SUBTITLE_MAX_LENGTH);
    checkOptionalText(value, "body", path, SlackLimits.CARD_BODY_MAX_LENGTH);
    checkOptionalText(value, "subtext", path, SlackLimits.CARD_SUBTEXT_MAX_LENGTH);
    if (value.get("actions") instanceof List<?> actions) {
      sliceLength(actions, child(path, "actions"), 0, SlackLimits.CARD_ACTIONS_MAX_ITEMS);
    }
  }

  private static void validateCarousel(Map<String, Object> value, String path) {
    List<?> cards = listAt(value.get("elements"), child(path, "elements"));
    sliceLength(
        cards,
        child(path, "elements"),
        SlackLimits.CAROUSEL_ELEMENTS_MIN_ITEMS,
        SlackLimits.CAROUSEL_ELEMENTS_MAX_ITEMS);
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
    checkOptionalText(value, "title", path, SlackLimits.CONTAINER_TITLE_MAX_LENGTH);
    checkOptionalText(value, "subtitle", path, SlackLimits.CONTAINER_SUBTITLE_MAX_LENGTH);
    List<?> blocks = listAt(value.get("child_blocks"), child(path, "child_blocks"));
    sliceLength(
        blocks, child(path, "child_blocks"), 1, SlackLimits.CONTAINER_CHILD_BLOCKS_MAX_ITEMS);
    if (value.get("width") instanceof String width && !CONTAINER_WIDTHS.contains(width)) {
      fail(ErrorCategory.TYPE_MISMATCH, child(path, "width"), "unknown container width");
    }
    if (Boolean.TRUE.equals(value.get("default_collapsed"))
        && !Boolean.TRUE.equals(value.get("is_collapsible"))) {
      fail(
          ErrorCategory.INVALID_USAGE, child(path, "default_collapsed"), "requires is_collapsible");
    }
    if (Boolean.TRUE.equals(value.get("has_header_divider"))
        && Boolean.TRUE.equals(value.get("is_collapsible"))) {
      fail(
          ErrorCategory.INVALID_USAGE,
          child(path, "has_header_divider"),
          "requires a non-collapsible container");
    }
  }

  private static void validatePie(Map<String, Object> value, String path) {
    List<?> segments = listAt(value.get("segments"), child(path, "segments"));
    sliceLength(
        segments,
        child(path, "segments"),
        SlackLimits.DATA_VISUALIZATION_SEGMENTS_MIN_ITEMS,
        SlackLimits.DATA_VISUALIZATION_SEGMENTS_MAX_ITEMS);
    for (int index = 0; index < segments.size(); index++) {
      String segmentPath = child(path, "segments") + "[" + index + "]";
      validateLabelValue(
          objectAt(segments.get(index), segmentPath),
          segmentPath,
          SlackLimits.DATA_VISUALIZATION_SEGMENT_LABEL_MAX_LENGTH,
          true);
    }
  }

  private static void validateInput(Map<String, Object> value, String path) {
    textLength(
        value.get("label"), child(path, "label.text"), 0, SlackLimits.INPUT_LABEL_MAX_LENGTH);
    if (value.containsKey("hint")) {
      textLength(value.get("hint"), child(path, "hint.text"), 0, SlackLimits.INPUT_HINT_MAX_LENGTH);
    }
    Map<String, Object> element = objectAt(value.get("element"), child(path, "element"));
    if (!INPUT_ELEMENT_TYPES.contains(objectType(element))) {
      fail(
          ErrorCategory.TYPE_MISMATCH,
          child(path, "element"),
          "expected an input-compatible element");
    }
  }

  private static void validateVideo(Map<String, Object> value, String path) {
    if (value.get("alt_text") instanceof String altText) {
      stringLength(
          altText,
          child(path, "alt_text"),
          SlackLimits.VIDEO_ALT_TEXT_MIN_LENGTH,
          SlackLimits.VIDEO_ALT_TEXT_MAX_LENGTH);
    }
    textLength(
        value.get("title"), child(path, "title.text"), 0, SlackLimits.VIDEO_TITLE_MAX_LENGTH);
    checkOptionalString(value, "author_name", path, SlackLimits.VIDEO_AUTHOR_NAME_MAX_LENGTH);
    checkOptionalString(value, "provider_name", path, SlackLimits.VIDEO_PROVIDER_NAME_MAX_LENGTH);
    checkOptionalText(value, "description", path, SlackLimits.VIDEO_DESCRIPTION_MAX_LENGTH);
  }

  private static void validateView(Map<String, Object> value, String path, String type) {
    List<?> blocks = listAt(value.get("blocks"), child(path, "blocks"));
    sliceLength(
        blocks,
        child(path, "blocks"),
        SlackLimits.VIEW_BLOCKS_MIN_ITEMS,
        SlackLimits.VIEW_BLOCKS_MAX_ITEMS);
    validateSurface(blocks, type, child(path, "blocks"));
    checkOptionalString(
        value, "private_metadata", path, SlackLimits.VIEW_PRIVATE_METADATA_MAX_LENGTH);
    checkOptionalString(value, "callback_id", path, SlackLimits.VIEW_CALLBACK_ID_MAX_LENGTH);
    if (type.equals("modal")) {
      if (!value.containsKey("submit")) {
        for (int index = 0; index < blocks.size(); index++) {
          String blockPath = child(path, "blocks") + "[" + index + "]";
          if ("input".equals(objectType(objectAt(blocks.get(index), blockPath)))) {
            fail(
                ErrorCategory.MISSING_REQUIRED,
                child(path, "submit"),
                "required when the modal contains an input block (" + blockPath + ")");
          }
        }
      }
      checkOptionalText(value, "title", path, SlackLimits.VIEW_TITLE_MAX_LENGTH);
      checkOptionalText(value, "close", path, SlackLimits.VIEW_CLOSE_MAX_LENGTH);
      checkOptionalText(value, "submit", path, SlackLimits.VIEW_SUBMIT_MAX_LENGTH);
    }
  }

  private static void validateOptionGroup(Map<String, Object> value, String path) {
    require(value, "label", path);
    textLength(
        value.get("label"),
        child(path, "label.text"),
        0,
        SlackLimits.OPTION_GROUP_LABEL_MAX_LENGTH);
    List<?> options = listAt(value.get("options"), child(path, "options"));
    sliceLength(
        options,
        child(path, "options"),
        SlackLimits.OPTION_GROUP_OPTIONS_MIN_ITEMS,
        SlackLimits.OPTION_GROUP_OPTIONS_MAX_ITEMS);
    validateOptions(options, child(path, "options"));
  }

  private static void validateFeedbackButton(Map<String, Object> value, String path) {
    require(value, "text", path);
    require(value, "value", path);
    textLength(
        value.get("text"),
        child(path, "text.text"),
        0,
        SlackLimits.FEEDBACK_BUTTON_TEXT_MAX_LENGTH);
    checkOptionalString(value, "value", path, SlackLimits.FEEDBACK_BUTTON_VALUE_MAX_LENGTH);
    checkOptionalString(
        value,
        "accessibility_label",
        path,
        SlackLimits.FEEDBACK_BUTTON_ACCESSIBILITY_LABEL_MAX_LENGTH);
  }

  private static void validateDispatchActionConfiguration(Map<String, Object> value, String path) {
    if (value.containsKey("trigger_actions_on")) {
      sliceLength(
          listAt(value.get("trigger_actions_on"), child(path, "trigger_actions_on")),
          child(path, "trigger_actions_on"),
          SlackLimits.DISPATCH_ACTION_CONFIGURATION_TRIGGER_ACTIONS_ON_MIN_ITEMS,
          SlackLimits.DISPATCH_ACTION_CONFIGURATION_TRIGGER_ACTIONS_ON_MAX_ITEMS);
    }
  }

  private static void validateDataSeries(Map<String, Object> value, String path) {
    String name = stringAt(value.get("name"), child(path, "name"));
    stringLength(
        name, child(path, "name"), 0, SlackLimits.DATA_VISUALIZATION_SERIES_NAME_MAX_LENGTH);
    List<?> data = listAt(value.get("data"), child(path, "data"));
    sliceLength(
        data,
        child(path, "data"),
        SlackLimits.DATA_VISUALIZATION_DATA_MIN_ITEMS,
        SlackLimits.DATA_VISUALIZATION_DATA_MAX_ITEMS);
  }

  private static void validateAxisConfig(Map<String, Object> value, String path) {
    List<?> categories = listAt(value.get("categories"), child(path, "categories"));
    sliceLength(
        categories,
        child(path, "categories"),
        SlackLimits.DATA_VISUALIZATION_CATEGORIES_MIN_ITEMS,
        SlackLimits.DATA_VISUALIZATION_CATEGORIES_MAX_ITEMS);
    Set<String> seen = new java.util.HashSet<>();
    for (int index = 0; index < categories.size(); index++) {
      String itemPath = child(path, "categories") + "[" + index + "]";
      String category = stringAt(categories.get(index), itemPath);
      stringLength(category, itemPath, 0, SlackLimits.DATA_VISUALIZATION_CATEGORY_LABEL_MAX_LENGTH);
      if (!seen.add(category)) {
        fail(ErrorCategory.INVALID_USAGE, child(path, "categories"), "expected unique labels");
      }
    }
    checkOptionalString(
        value, "x_label", path, SlackLimits.DATA_VISUALIZATION_AXIS_LABEL_MAX_LENGTH);
    checkOptionalString(
        value, "y_label", path, SlackLimits.DATA_VISUALIZATION_AXIS_LABEL_MAX_LENGTH);
  }

  private static void validateConfirmation(Map<String, Object> value, String path) {
    Map<String, Integer> limits =
        Map.of(
            "title",
            SlackLimits.CONFIRMATION_TITLE_MAX_LENGTH,
            "text",
            SlackLimits.CONFIRMATION_TEXT_MAX_LENGTH,
            "confirm",
            SlackLimits.CONFIRMATION_CONFIRM_MAX_LENGTH,
            "deny",
            SlackLimits.CONFIRMATION_DENY_MAX_LENGTH);
    for (Map.Entry<String, Integer> field : limits.entrySet()) {
      require(value, field.getKey(), path);
      textLength(
          value.get(field.getKey()), child(path, field.getKey() + ".text"), 0, field.getValue());
    }
  }

  private static void validateOption(Map<String, Object> value, String path) {
    require(value, "text", path);
    require(value, "value", path);
    textLength(value.get("text"), child(path, "text.text"), 0, SlackLimits.OPTION_TEXT_MAX_LENGTH);
    checkOptionalString(value, "value", path, SlackLimits.OPTION_VALUE_MAX_LENGTH);
    checkOptionalText(value, "description", path, SlackLimits.OPTION_DESCRIPTION_MAX_LENGTH);
    checkOptionalString(value, "url", path, SlackLimits.OPTION_URL_MAX_LENGTH);
  }

  private static void validateOptions(List<?> values, String path) {
    for (int index = 0; index < values.size(); index++) {
      String optionPath = path + "[" + index + "]";
      validateOption(objectAt(values.get(index), optionPath), optionPath);
    }
  }

  private static void validateTable(Map<String, Object> value, String path, boolean dataTable) {
    List<?> rows = listAt(value.get("rows"), child(path, "rows"));
    int minimumRows = dataTable ? SlackLimits.DATA_TABLE_ROWS_MIN_ITEMS : 1;
    int maximumRows =
        dataTable ? SlackLimits.DATA_TABLE_ROWS_MAX_ITEMS : SlackLimits.TABLE_ROWS_MAX_ITEMS;
    int minimumColumns = dataTable ? SlackLimits.DATA_TABLE_COLUMNS_MIN_ITEMS : 0;
    int maximumColumns =
        dataTable ? SlackLimits.DATA_TABLE_COLUMNS_MAX_ITEMS : SlackLimits.TABLE_COLUMNS_MAX_ITEMS;
    sliceLength(rows, child(path, "rows"), minimumRows, maximumRows);
    int columns = -1;
    int contentLength = 0;
    for (int rowIndex = 0; rowIndex < rows.size(); rowIndex++) {
      String rowPath = child(path, "rows") + "[" + rowIndex + "]";
      List<?> row = listAt(rows.get(rowIndex), rowPath);
      sliceLength(row, rowPath, minimumColumns, maximumColumns);
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
          stringLength(
              stringAt(cell.get("text"), child(cellPath, "text")),
              child(cellPath, "text"),
              SlackLimits.DATA_TABLE_CELL_TEXT_MIN_LENGTH,
              0);
        }
      }
    }
    if (value.containsKey("column_settings")) {
      List<?> settings = listAt(value.get("column_settings"), child(path, "column_settings"));
      sliceLength(settings, child(path, "column_settings"), 0, 20);
      if (settings.size() != columns) {
        fail(
            ErrorCategory.INVALID_USAGE,
            child(path, "column_settings"),
            "expected one entry for every column");
      }
    }
    if (dataTable) {
      Double pageSize = number(value.get("page_size"));
      if (pageSize != null
          && (pageSize < SlackLimits.DATA_TABLE_PAGE_SIZE_MIN
              || pageSize > SlackLimits.DATA_TABLE_PAGE_SIZE_MAX)) {
        fail(
            ErrorCategory.OUT_OF_RANGE,
            child(path, "page_size"),
            "expected a value between "
                + SlackLimits.DATA_TABLE_PAGE_SIZE_MIN
                + " and "
                + SlackLimits.DATA_TABLE_PAGE_SIZE_MAX);
      }
      String caption = stringAt(value.get("caption"), child(path, "caption"));
      stringLength(caption, child(path, "caption"), 1, 0);
      if (contentLength > SlackLimits.DATA_TABLE_CONTENT_MAX_LENGTH) {
        fail(
            ErrorCategory.LENGTH_EXCEEDED,
            child(path, "rows"),
            "content exceeds maximum " + SlackLimits.DATA_TABLE_CONTENT_MAX_LENGTH);
      }
    }
  }

  private static void validateSeriesChart(Map<String, Object> value, String path) {
    List<?> series = listAt(value.get("series"), child(path, "series"));
    sliceLength(
        series,
        child(path, "series"),
        SlackLimits.DATA_VISUALIZATION_SERIES_MIN_ITEMS,
        SlackLimits.DATA_VISUALIZATION_SERIES_MAX_ITEMS);
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
        validateLabelValue(
            point, pointPath, SlackLimits.DATA_VISUALIZATION_POINT_LABEL_MAX_LENGTH, false);
        seen.add(stringAt(point.get("label"), child(pointPath, "label")));
      }
      if (points.size() != categorySet.size() || !seen.equals(categorySet)) {
        fail(
            ErrorCategory.INVALID_USAGE,
            child(itemPath, "data"),
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
      return;
    }
    if (positive && number <= SlackLimits.DATA_VISUALIZATION_SEGMENT_VALUE_EXCLUSIVE_MIN) {
      fail(
          ErrorCategory.OUT_OF_RANGE,
          child(path, "value"),
          "expected a value greater than "
              + SlackLimits.DATA_VISUALIZATION_SEGMENT_VALUE_EXCLUSIVE_MIN);
    }
  }

  private static void validateMessageCollections(Map<String, Object> value, String path) {
    if (value.containsKey("blocks")) {
      List<?> blocks = listAt(value.get("blocks"), child(path, "blocks"));
      sliceLength(blocks, child(path, "blocks"), 0, SlackLimits.MESSAGE_BLOCKS_MAX_ITEMS);
      validateSurface(blocks, "message", child(path, "blocks"));
    }
    if (value.containsKey("attachments")) {
      sliceLength(
          listAt(value.get("attachments"), child(path, "attachments")),
          child(path, "attachments"),
          0,
          SlackLimits.MESSAGE_ATTACHMENTS_MAX_ITEMS);
    }
  }

  private static void validateSurface(List<?> blocks, String surface, String path) {
    Set<String> allowed =
        Objects.requireNonNull(SlackVocabulary.SURFACE_BLOCK_TYPES.get(surface), surface);
    for (int index = 0; index < blocks.size(); index++) {
      String blockPath = path + "[" + index + "]";
      String type = objectType(objectAt(blocks.get(index), blockPath));
      if (!allowed.contains(type)) {
        fail(
            ErrorCategory.TYPE_MISMATCH,
            blockPath + ".type",
            "block type " + type + " is not supported on " + surface + " surfaces");
      }
    }
  }

  private static void validateNested(@Nullable Object value, String path) {
    if (value == null) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected a value, not null");
    } else if (value instanceof Map<?, ?>) {
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

  private static Map<String, Object> objectAt(@Nullable Object value, String path) {
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

  private static List<?> listAt(@Nullable Object value, String path) {
    if (!(value instanceof List<?> list)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected an array");
      throw new AssertionError("unreachable");
    }
    return list;
  }

  private static String stringAt(@Nullable Object value, String path) {
    if (!(value instanceof String text)) {
      fail(ErrorCategory.TYPE_MISMATCH, path, "expected a string");
      throw new AssertionError("unreachable");
    }
    return text;
  }

  private static String objectType(Map<String, Object> value) {
    return value.get("type") instanceof String type ? type : "";
  }

  private static void textLength(@Nullable Object value, String path, int minimum, int maximum) {
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

  private static @Nullable Double number(@Nullable Object value) {
    return value instanceof Number number ? number.doubleValue() : null;
  }

  private static int textCharacterCount(@Nullable Object value) {
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
