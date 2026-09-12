package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.Gson;
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

final class ConformanceTest {
  private static final Path SPEC_ROOT = Path.of("..", "spec");
  private static final Gson GSON = new Gson();

  private static final Map<String, String> BUILDERS =
      Map.ofEntries(
          entry("actions", "block.ActionsBlock"), entry("alert", "block.AlertBlock"),
          entry("area", "object.AreaChart"), entry("bar", "object.BarChart"),
          entry("button", "element.ButtonElement"), entry("card", "block.CardBlock"),
          entry("carousel", "block.CarouselBlock"), entry("channel", "object.RichTextChannel"),
          entry("channels_select", "element.ChannelSelectElement"),
          entry("checkboxes", "element.CheckboxesElement"), entry("container", "block.ContainerBlock"),
          entry("context", "block.ContextBlock"), entry("context_actions", "block.ContextActionsBlock"),
          entry("conversations_select", "element.ConversationSelectElement"),
          entry("data_table", "block.DataTableBlock"),
          entry("data_visualization", "block.DataVisualizationBlock"),
          entry("datepicker", "element.DatePickerElement"),
          entry("datetimepicker", "element.DateTimePickerElement"),
          entry("divider", "block.DividerBlock"),
          entry("email_text_input", "element.EmailInputElement"),
          entry("emoji", "object.RichTextEmoji"),
          entry("external_select", "element.ExternalSelectElement"),
          entry("feedback_buttons", "element.FeedbackButtonsElement"),
          entry("file", "block.FileBlock"), entry("file_input", "element.FileInputElement"),
          entry("header", "block.HeaderBlock"), entry("home", "payload.HomeTabView"),
          entry("icon", "object.SlackIcon"), entry("icon_button", "element.IconButtonElement"),
          entry("image", "block.ImageBlock"), entry("input", "block.InputBlock"),
          entry("line", "object.LineChart"),
          entry("link", "object.RichTextLink"), entry("markdown", "block.MarkdownBlock"),
          entry("modal", "payload.ModalView"), entry("mrkdwn", "object.MarkdownText"),
          entry("multi_channels_select", "element.ChannelMultiSelectElement"),
          entry("multi_conversations_select", "element.ConversationMultiSelectElement"),
          entry("multi_external_select", "element.ExternalMultiSelectElement"),
          entry("multi_static_select", "element.StaticMultiSelectElement"),
          entry("multi_users_select", "element.UserMultiSelectElement"),
          entry("number_input", "element.NumberInputElement"),
          entry("overflow", "element.OverflowElement"), entry("pie", "object.PieChart"),
          entry("plain_text", "object.PlainText"),
          entry("plain_text_input", "element.PlainTextInputElement"),
          entry("plan", "block.PlanBlock"), entry("radio_buttons", "element.RadioButtonsElement"),
          entry("raw_number", "object.RawNumber"), entry("raw_text", "object.RawText"),
          entry("rich_text", "block.RichTextBlock"),
          entry("rich_text_input", "element.RichTextInputElement"),
          entry("rich_text_list", "object.RichTextList"),
          entry("rich_text_preformatted", "object.RichTextCodeBlock"),
          entry("rich_text_quote", "object.RichTextQuote"),
          entry("rich_text_section", "object.RichTextSection"),
          entry("section", "block.SectionBlock"),
          entry("static_select", "element.StaticSelectElement"), entry("table", "block.TableBlock"),
          entry("task_card", "block.TaskCardBlock"), entry("text", "object.RichTextText"),
          entry("timepicker", "element.TimePickerElement"), entry("url", "object.UrlSource"),
          entry("url_text_input", "element.UrlInputElement"), entry("user", "object.RichTextUser"),
          entry("usergroup", "object.RichTextUserGroup"),
          entry("users_select", "element.UserSelectElement"), entry("video", "block.VideoBlock"),
          entry("workflow_button", "element.WorkflowButtonElement"));

  private static Map.Entry<String, String> entry(String type, String className) {
    return Map.entry(type, className);
  }

  @TestFactory
  Stream<DynamicTest> everySharedValidFixtureBuildsThroughThePublicApi() throws Exception {
    JsonObject manifest = JsonParser.parseString(Files.readString(SPEC_ROOT.resolve("manifest.json"))).getAsJsonObject();
    assertEquals(Slackblocks.SPEC_VERSION, manifest.get("spec_version").getAsString());
    JsonArray fixtures = manifest.getAsJsonArray("fixtures");
    assertEquals(100, fixtures.size());
    return fixtures.asList().stream().map(JsonElement::getAsJsonObject).map(entry -> {
      String id = entry.get("id").getAsString();
      return DynamicTest.dynamicTest(id, () -> assertFixture(id));
    });
  }

  @Test
  void releasedJavaConformanceSkipListIsEmpty() throws Exception {
    for (String line : Files.readAllLines(Path.of("conformance", "skiplist.txt"))) {
      assertTrue(line.isBlank() || line.stripLeading().startsWith("#"),
          () -> "Java conformance skip list contains " + line);
    }
  }

  private static void assertFixture(String id) throws Exception {
    Path fixture = SPEC_ROOT.resolve("fixtures/valid").resolve(id + ".json");
    JsonElement expected = JsonParser.parseString(Files.readString(fixture));
    Object parsed = GSON.fromJson(expected, Object.class);
    Object constructed = construct(parsed, id, true);
    JsonElement actual = JsonParser.parseString(GSON.toJson(constructed));
    assertEquals(expected, actual);
  }

  private static Object construct(Object value, String fixtureId, boolean root) throws Exception {
    if (value instanceof List<?> list) {
      List<Object> result = new ArrayList<>(list.size());
      for (Object item : list) {
        result.add(construct(item, fixtureId, false));
      }
      return result;
    }
    if (!(value instanceof Map<?, ?> source)) {
      return value;
    }
    Map<String, Object> object = new LinkedHashMap<>();
    for (Map.Entry<?, ?> field : source.entrySet()) {
      object.put(String.valueOf(field.getKey()), construct(field.getValue(), fixtureId, false));
    }
    String type = object.get("type") instanceof String text ? text : "";
    Object builder = type.isEmpty() ? rootBuilder(fixtureId, root) : typedBuilder(type, fixtureId);
    if (builder == null) {
      return object;
    }
    Method wireField = builder.getClass().getMethod("wireField", String.class, Object.class);
    for (Map.Entry<String, Object> field : object.entrySet()) {
      if (!field.getKey().equals("type")) {
        invoke(wireField, builder, field.getKey(), field.getValue());
      }
    }
    return invoke(builder.getClass().getMethod("build"), builder);
  }

  private static Object typedBuilder(String type, String fixtureId) throws Exception {
    String suffix = BUILDERS.get(type);
    if (suffix == null) {
      throw new AssertionError("No Java builder registered for Slack type " + type);
    }
    if (type.equals("image") && fixtureId.startsWith("elements/")) {
      suffix = "element.ImageElement";
    }
    return newBuilder(suffix);
  }

  private static Object rootBuilder(String fixtureId, boolean root) throws Exception {
    if (!root) {
      return null;
    }
    String suffix;
    if (fixtureId.startsWith("attachments/")) {
      suffix = "payload.Attachment";
    } else if (fixtureId.equals("messages/message_basic")) {
      suffix = "payload.MessagePayload";
    } else if (fixtureId.equals("messages/message_response")) {
      suffix = "payload.MessageResponse";
    } else if (fixtureId.equals("messages/webhook_message_basic")) {
      suffix = "payload.WebhookMessage";
    } else if (fixtureId.startsWith("objects/confirmation")) {
      suffix = "object.Confirmation";
    } else if (fixtureId.startsWith("objects/conversation_filter")) {
      suffix = "object.ConversationFilter";
    } else if (fixtureId.startsWith("objects/dispatch_action_configuration")) {
      suffix = "object.DispatchActionConfiguration";
    } else if (fixtureId.startsWith("objects/input_parameter")) {
      suffix = "object.InputParameter";
    } else if (fixtureId.startsWith("objects/option_group")) {
      suffix = "object.OptionGroup";
    } else if (fixtureId.startsWith("objects/option")) {
      suffix = "object.Option";
    } else if (fixtureId.startsWith("objects/slack_file")) {
      suffix = "object.SlackFile";
    } else if (fixtureId.startsWith("objects/trigger")) {
      suffix = "object.Trigger";
    } else if (fixtureId.startsWith("objects/workflow")) {
      suffix = "object.Workflow";
    } else {
      return null;
    }
    return newBuilder(suffix);
  }

  private static Object newBuilder(String suffix) throws Exception {
    Class<?> type = Class.forName("io.github.nicklambourne.slackblocks." + suffix);
    return invoke(type.getMethod("builder"), null);
  }

  private static Object invoke(Method method, Object receiver, Object... arguments) throws Exception {
    try {
      return method.invoke(receiver, arguments);
    } catch (InvocationTargetException error) {
      Throwable cause = error.getCause();
      if (cause instanceof Exception exception) {
        throw exception;
      }
      if (cause instanceof Error fatal) {
        throw fatal;
      }
      throw error;
    }
  }
}
