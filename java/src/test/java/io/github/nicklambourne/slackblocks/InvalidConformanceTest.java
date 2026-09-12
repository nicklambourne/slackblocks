package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import io.github.nicklambourne.slackblocks.internal.Validator;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.function.IntFunction;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.TestFactory;

final class InvalidConformanceTest {
  private record InvalidValue(String name, Map<String, Object> value) {}

  @TestFactory
  Stream<DynamicTest> everySharedInvalidCaseHasTheRequiredCategory() throws Exception {
    Path manifestPath = Path.of("..", "spec", "fixtures", "invalid", "manifest.json");
    JsonObject manifest = JsonParser.parseString(Files.readString(manifestPath)).getAsJsonObject();
    assertEquals(Slackblocks.SPEC_VERSION, manifest.get("spec_version").getAsString());
    assertEquals(115, manifest.getAsJsonArray("cases").size());
    return manifest.getAsJsonArray("cases").asList().stream()
        .map(JsonElement::getAsJsonObject)
        .map(entry -> DynamicTest.dynamicTest(entry.get("id").getAsString(), () -> {
          String id = entry.get("id").getAsString();
          InvalidValue invalid = invalidValue(id);
          ValidationException error = assertThrows(
              ValidationException.class, () -> Validator.validate(invalid.name(), invalid.value()));
          assertEquals(entry.get("category").getAsString(), error.getCategory().wireValue());
        }));
  }

  private static InvalidValue invalidValue(String id) {
    return switch (id) {
      case "text-empty" -> invalid("PlainText", text("plain_text", ""));
      case "text-too-long" -> invalid("PlainText", text("plain_text", repeated("x", 3001)));
      case "button-action-id-too-long" -> typed("Button", "button", "text", plain("A"), "action_id", repeated("x", 256));
      case "button-text-too-long" -> typed("Button", "button", "text", plain(repeated("x", 76)), "action_id", "a");
      case "button-url-too-long" -> typed("Button", "button", "text", plain("A"), "action_id", "a", "url", repeated("x", 3001));
      case "button-value-too-long" -> typed("Button", "button", "text", plain("A"), "action_id", "a", "value", repeated("x", 2001));
      case "confirmation-title-too-long" -> confirmation(repeated("x", 101), "Text", "Yes", "No");
      case "confirmation-text-too-long" -> confirmation("Title", repeated("x", 301), "Yes", "No");
      case "confirmation-confirm-too-long" -> confirmation("Title", "Text", repeated("x", 31), "No");
      case "confirmation-deny-too-long" -> confirmation("Title", "Text", "Yes", repeated("x", 31));
      case "option-text-too-long" -> invalid("Option", option(repeated("x", 76), "a"));
      case "option-value-too-long" -> invalid("Option", option("A", repeated("x", 151)));
      case "option-url-too-long" -> invalid("Option", map("text", plain("A"), "value", "a", "url", repeated("🙂", 3001)));
      case "option-description-too-long" -> invalid("Option", map("text", plain("A"), "value", "a", "description", plain(repeated("x", 76))));
      case "option-group-label-too-long" -> optionGroup(repeated("x", 76), List.of(option("A", "a")));
      case "option-group-empty" -> optionGroup("Group", List.of());
      case "option-group-too-many-options" -> optionGroup("Group", copies(101, index -> option("A", "a")));
      case "select-placeholder-too-long" -> typed("StaticSelect", "static_select", "action_id", "a", "options", List.of(option("A", "a")), "placeholder", plain(repeated("x", 151)));
      case "select-too-many-options" -> typed("StaticSelect", "static_select", "action_id", "a", "options", copies(101, index -> option("A", "a")));
      case "select-too-many-option-groups" -> typed("StaticSelect", "static_select", "action_id", "a", "option_groups", copies(101, index -> map("label", plain("Group"), "options", List.of(option("A", "a")))));
      case "overflow-empty" -> typed("Overflow", "overflow", "action_id", "a", "options", List.of());
      case "overflow-too-many-options" -> typed("Overflow", "overflow", "action_id", "a", "options", copies(6, index -> option("A", "a")));
      case "checkboxes-empty" -> typed("Checkboxes", "checkboxes", "action_id", "a", "options", List.of());
      case "checkboxes-too-many-options" -> typed("Checkboxes", "checkboxes", "action_id", "a", "options", copies(11, index -> option("A", "a")));
      case "radio-buttons-empty" -> typed("RadioButtons", "radio_buttons", "action_id", "a", "options", List.of());
      case "radio-buttons-too-many-options" -> typed("RadioButtons", "radio_buttons", "action_id", "a", "options", copies(11, index -> option("A", "a")));
      case "url-source-url-empty" -> typed("URLSource", "url", "url", "", "text", "text");
      case "url-source-url-too-long" -> typed("URLSource", "url", "url", repeated("x", 3001), "text", "text");
      case "table-too-many-rows" -> typed("TableBlock", "table", "rows", copies(101, index -> List.of(rawText("A"))));
      case "table-ragged-rows" -> typed("TableBlock", "table", "rows", List.of(List.of(rawText("A"), rawText("B")), List.of(rawText("C"))));
      case "table-column-settings-mismatch" -> typed("TableBlock", "table", "rows", List.of(List.of(rawText("A"), rawText("B"))), "column_settings", List.of(map("is_wrapped", true)));
      case "file-input-max-files-too-small" -> typed("FileInput", "file_input", "action_id", "a", "max_files", 0);
      case "file-input-max-files-too-large" -> typed("FileInput", "file_input", "action_id", "a", "max_files", 11);
      case "plain-text-input-max-length-too-large" -> typed("PlainTextInput", "plain_text_input", "action_id", "a", "max_length", 3001);
      case "actions-too-many-elements" -> typed("ActionsBlock", "actions", "elements", copies(26, index -> button()));
      case "context-too-many-elements" -> typed("ContextBlock", "context", "elements", copies(11, index -> mrkdwn("A")));
      case "header-text-too-long" -> typed("HeaderBlock", "header", "text", plain(repeated("x", 151)));
      case "image-url-too-long" -> typed("ImageBlock", "image", "image_url", repeated("x", 3001), "alt_text", "Alt");
      case "image-alt-text-too-long" -> typed("ImageBlock", "image", "image_url", "https://example.com/image.png", "alt_text", repeated("x", 2001));
      case "input-label-too-long" -> typed("InputBlock", "input", "label", plain(repeated("x", 2001)), "element", inputElement());
      case "input-hint-too-long" -> typed("InputBlock", "input", "label", plain("Label"), "hint", plain(repeated("x", 2001)), "element", inputElement());
      case "markdown-empty" -> typed("MarkdownBlock", "markdown", "text", "");
      case "markdown-too-long" -> typed("MarkdownBlock", "markdown", "text", repeated("x", 12001));
      case "section-text-too-long" -> typed("SectionBlock", "section", "text", mrkdwn(repeated("x", 3001)));
      case "section-too-many-fields" -> typed("SectionBlock", "section", "fields", copies(11, index -> mrkdwn("x")));
      case "section-field-too-long" -> typed("SectionBlock", "section", "fields", List.of(mrkdwn(repeated("x", 2001))));
      case "video-alt-text-empty" -> invalidVideo("alt_text", "");
      case "video-alt-text-too-long" -> invalidVideo("alt_text", repeated("x", 201));
      case "video-title-too-long" -> invalidVideo("title", plain(repeated("x", 201)));
      case "video-author-name-too-long" -> invalidVideo("author_name", repeated("x", 51));
      case "video-description-too-long" -> invalidVideo("description", plain(repeated("x", 201)));
      case "video-provider-name-too-long" -> invalidVideo("provider_name", repeated("x", 51));
      case "message-channel-empty" -> invalid("Message", map("channel", ""));
      case "message-too-many-blocks" -> invalid("Message", map("channel", "C123", "blocks", copies(51, index -> divider())));
      case "message-too-many-attachments" -> invalid("Message", map("channel", "C123", "attachments", copies(101, index -> map("blocks", List.of(divider())))));
      case "message-invalid-block-surface" -> invalid("Message", map("channel", "C123", "blocks", List.of(obj("alert", "text", plain("Modal only")))));
      case "modal-invalid-block-surface" -> typed("Modal", "modal", "title", plain("Invalid"), "blocks", List.of(obj("markdown", "text", "Message only")));
      case "home-invalid-block-surface" -> typed("HomeTab", "home", "blocks", List.of(obj("alert", "text", plain("Modal only"))));
      case "modal-input-requires-submit" -> typed("Modal", "modal", "title", plain("Missing submit"), "blocks", List.of(inputBlock()));
      case "view-missing-blocks" -> typed("HomeTab", "home", "blocks", List.of());
      case "view-too-many-blocks" -> typed("HomeTab", "home", "blocks", copies(101, index -> divider()));
      case "view-private-metadata-too-long" -> typed("HomeTab", "home", "blocks", List.of(divider()), "private_metadata", repeated("x", 3001));
      case "view-callback-id-too-long" -> typed("HomeTab", "home", "blocks", List.of(divider()), "callback_id", repeated("x", 256));
      case "view-title-too-long" -> typed("Modal", "modal", "title", plain(repeated("x", 25)), "blocks", List.of(divider()));
      case "view-close-too-long" -> typed("Modal", "modal", "title", plain("Title"), "close", plain(repeated("x", 25)), "blocks", List.of(divider()));
      case "view-submit-too-long" -> typed("Modal", "modal", "title", plain("Title"), "submit", plain(repeated("x", 25)), "blocks", List.of(divider()));
      case "section-missing-content" -> typed("SectionBlock", "section");
      case "section-empty-fields" -> typed("SectionBlock", "section", "fields", List.of());
      case "static-select-options-and-groups" -> typed("StaticSelect", "static_select", "action_id", "a", "options", List.of(option("A", "a")), "option_groups", List.of(map("label", plain("A"), "options", List.of(option("B", "b")))));
      case "image-url-and-slack-file" -> typed("ImageElement", "image", "alt_text", "image", "image_url", "https://example.com/image.png", "slack_file", map("id", "F123"));
      case "number-input-inverted-range" -> typed("NumberInput", "number_input", "action_id", "a", "is_decimal_allowed", true, "min_value", 2, "max_value", 1);
      case "context-invalid-element" -> typed("ContextBlock", "context", "elements", List.of(divider()));
      case "input-invalid-element" -> typed("InputBlock", "input", "label", plain("Label"), "element", button());
      case "block-id-too-long" -> typed("DividerBlock", "divider", "block_id", repeated("x", 256));
      case "button-accessibility-label-too-long" -> typed("Button", "button", "text", plain("A"), "action_id", "a", "accessibility_label", repeated("x", 76));
      case "alert-text-too-long" -> typed("AlertBlock", "alert", "text", plain(repeated("x", 201)));
      case "card-title-too-long" -> typed("CardBlock", "card", "title", plain(repeated("x", 151)));
      case "card-subtitle-too-long" -> typed("CardBlock", "card", "title", plain("Card"), "subtitle", plain(repeated("x", 151)));
      case "card-body-too-long" -> typed("CardBlock", "card", "body", plain(repeated("x", 201)));
      case "card-too-many-actions" -> typed("CardBlock", "card", "title", plain("Card"), "actions", copies(4, index -> button()));
      case "card-subtext-too-long" -> typed("CardBlock", "card", "title", plain("Card"), "subtext", plain(repeated("x", 201)));
      case "carousel-empty" -> typed("CarouselBlock", "carousel", "elements", List.of());
      case "carousel-too-many-cards" -> typed("CarouselBlock", "carousel", "elements", copies(11, index -> card()));
      case "container-title-too-long" -> typed("ContainerBlock", "container", "title", plain(repeated("x", 151)), "child_blocks", List.of(divider()));
      case "container-subtitle-too-long" -> typed("ContainerBlock", "container", "title", plain("Container"), "subtitle", plain(repeated("x", 151)), "child_blocks", List.of(divider()));
      case "container-too-many-child-blocks" -> typed("ContainerBlock", "container", "title", plain("Container"), "child_blocks", copies(11, index -> divider()));
      case "context-actions-too-many-elements" -> typed("ContextActionsBlock", "context_actions", "elements", copies(6, index -> iconButton()));
      case "feedback-button-text-too-long" -> typed("FeedbackButtons", "feedback_buttons", "positive_button", feedback(repeated("x", 76), "good"), "negative_button", feedback("Bad", "bad"));
      case "feedback-button-value-too-long" -> typed("FeedbackButtons", "feedback_buttons", "positive_button", feedback("Good", repeated("x", 2001)), "negative_button", feedback("Bad", "bad"));
      case "feedback-button-accessibility-label-too-long" -> typed("FeedbackButtons", "feedback_buttons", "positive_button", map("text", plain("Good"), "value", "good", "accessibility_label", repeated("x", 76)), "negative_button", feedback("Bad", "bad"));
      case "icon-button-too-many-visible-users" -> typed("IconButton", "icon_button", "text", plain("Delete"), "icon", "trash", "visible_to_user_ids", copies(11, index -> "U" + index));
      case "data-table-too-few-rows" -> dataTable(List.of(List.of(rawText("Name"))), "Names");
      case "data-table-too-many-rows" -> dataTable(copies(202, index -> List.of(rawText("A"))), "Names");
      case "data-table-too-few-columns" -> dataTable(List.of(List.of(), List.of()), "Empty");
      case "data-table-too-many-columns" -> dataTable(copies(2, row -> copies(21, column -> rawText("A"))), "Wide");
      case "data-table-page-size-too-small" -> dataTable(validRows(), "Names", "page_size", 0);
      case "data-table-page-size-too-large" -> dataTable(validRows(), "Names", "page_size", 101);
      case "data-table-cell-text-empty" -> dataTable(List.of(List.of(rawText("Name")), List.of(rawText(""))), "Names");
      case "data-table-content-too-long" -> dataTable(List.of(List.of(rawText("Name")), List.of(rawText(repeated("x", 20000)))), "Names");
      case "data-visualization-title-too-long" -> typed("DataVisualizationBlock", "data_visualization", "title", repeated("x", 51), "chart", obj("pie", "segments", List.of(segment("A", 1))));
      case "pie-chart-empty" -> typed("PieChart", "pie", "segments", List.of());
      case "pie-chart-too-many-segments" -> typed("PieChart", "pie", "segments", copies(13, index -> segment("S", 1)));
      case "chart-segment-label-too-long" -> typed("ChartSegment", "", "label", repeated("x", 21), "value", 1);
      case "chart-segment-value-not-positive" -> typed("ChartSegment", "", "label", "A", "value", 0);
      case "chart-series-empty" -> typed("LineChart", "line", "axis_config", axis("A"), "series", List.of());
      case "chart-duplicate-point-labels" -> typed("LineChart", "line", "axis_config", axis("A", "B"), "series", List.of(series("Series", point("A", 1), point("A", 2))));
      case "chart-too-many-series" -> typed("LineChart", "line", "series", copies(13, index -> series("S" + index, point("A", 1))), "axis_config", axis("A"));
      case "data-series-name-too-long" -> invalid("DataSeries", series(repeated("x", 21), point("A", 1)));
      case "data-series-empty" -> invalid("DataSeries", map("name", "Series", "data", List.of()));
      case "data-series-too-many-points" -> invalid("DataSeries", series("Series", copies(21, index -> point("P" + index, index)).toArray()));
      case "data-point-label-too-long" -> invalid("DataPoint", point(repeated("x", 21), 1));
      case "axis-categories-empty" -> invalid("AxisConfig", map("categories", List.of()));
      case "axis-too-many-categories" -> invalid("AxisConfig", map("categories", copies(21, index -> "X" + index)));
      case "axis-category-label-too-long" -> invalid("AxisConfig", axis(repeated("x", 21)));
      case "axis-label-too-long" -> invalid("AxisConfig", map("categories", List.of("A"), "x_label", repeated("x", 51)));
      default -> throw new AssertionError("No Java invalid construction registered for " + id);
    };
  }

  private static InvalidValue invalid(String name, Map<String, Object> value) {
    return new InvalidValue(name, value);
  }

  private static InvalidValue typed(String name, String type, Object... fields) {
    return invalid(name, obj(type, fields));
  }

  private static InvalidValue confirmation(String title, String text, String confirm, String deny) {
    return invalid("Confirmation", map("title", plain(title), "text", mrkdwn(text),
        "confirm", plain(confirm), "deny", plain(deny)));
  }

  private static InvalidValue optionGroup(String label, List<?> options) {
    return invalid("OptionGroup", map("label", plain(label), "options", options));
  }

  private static InvalidValue invalidVideo(String field, Object value) {
    Map<String, Object> video = validVideo();
    video.put(field, value);
    return invalid("VideoBlock", video);
  }

  private static InvalidValue dataTable(List<?> rows, String caption, Object... fields) {
    Map<String, Object> value = obj("data_table", "rows", rows, "caption", caption);
    put(value, fields);
    return invalid("DataTableBlock", value);
  }

  private static Map<String, Object> validVideo() {
    return obj("video", "alt_text", "Video", "thumbnail_url", "https://example.com/thumbnail.png",
        "title", plain("Title"), "video_url", "https://example.com/video.mp4");
  }

  private static Map<String, Object> option(String text, String value) {
    return map("text", plain(text), "value", value);
  }

  private static Map<String, Object> feedback(String text, String value) {
    return map("text", plain(text), "value", value);
  }

  private static Map<String, Object> button() {
    return obj("button", "text", plain("A"), "action_id", "a");
  }

  private static Map<String, Object> iconButton() {
    return obj("icon_button", "text", plain("Delete"), "icon", "trash");
  }

  private static Map<String, Object> inputElement() {
    return obj("plain_text_input", "action_id", "a");
  }

  private static Map<String, Object> inputBlock() {
    return obj("input", "label", plain("Name"), "element", inputElement());
  }

  private static Map<String, Object> divider() {
    return obj("divider");
  }

  private static Map<String, Object> card() {
    return obj("card", "title", plain("Card"));
  }

  private static Map<String, Object> rawText(String value) {
    return obj("raw_text", "text", value);
  }

  private static List<List<Map<String, Object>>> validRows() {
    return List.of(List.of(rawText("Name")), List.of(rawText("Alice")));
  }

  private static Map<String, Object> segment(String label, double value) {
    return map("label", label, "value", value);
  }

  private static Map<String, Object> point(String label, double value) {
    return map("label", label, "value", value);
  }

  private static Map<String, Object> series(String name, Object... points) {
    return map("name", name, "data", List.of(points));
  }

  private static Map<String, Object> axis(String... categories) {
    return map("categories", List.of(categories));
  }

  private static Map<String, Object> plain(String value) {
    return text("plain_text", value);
  }

  private static Map<String, Object> mrkdwn(String value) {
    return text("mrkdwn", value);
  }

  private static Map<String, Object> text(String type, String value) {
    return obj(type, "text", value);
  }

  private static Map<String, Object> obj(String type, Object... fields) {
    Map<String, Object> result = new LinkedHashMap<>();
    if (!type.isEmpty()) {
      result.put("type", type);
    }
    put(result, fields);
    return result;
  }

  private static Map<String, Object> map(Object... fields) {
    return obj("", fields);
  }

  private static void put(Map<String, Object> target, Object... fields) {
    if (fields.length % 2 != 0) {
      throw new IllegalArgumentException("fields must be key/value pairs");
    }
    for (int index = 0; index < fields.length; index += 2) {
      target.put(String.valueOf(fields[index]), fields[index + 1]);
    }
  }

  private static List<Object> copies(int count, IntFunction<Object> factory) {
    List<Object> result = new ArrayList<>(count);
    for (int index = 0; index < count; index++) {
      result.add(factory.apply(index));
    }
    return result;
  }

  private static String repeated(String value, int count) {
    return value.repeat(count);
  }
}
