package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import io.github.nicklambourne.slackblocks.block.CardBlock;
import io.github.nicklambourne.slackblocks.block.DividerBlock;
import io.github.nicklambourne.slackblocks.block.RichTextBlock;
import io.github.nicklambourne.slackblocks.block.TaskCardBlock;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.element.IconButtonElement;
import io.github.nicklambourne.slackblocks.element.ImageElement;
import io.github.nicklambourne.slackblocks.element.PlainTextInputElement;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.object.AxisConfig;
import io.github.nicklambourne.slackblocks.object.ChartSegment;
import io.github.nicklambourne.slackblocks.object.ColumnSettings;
import io.github.nicklambourne.slackblocks.object.Confirmation;
import io.github.nicklambourne.slackblocks.object.ConversationFilter;
import io.github.nicklambourne.slackblocks.object.DataPoint;
import io.github.nicklambourne.slackblocks.object.DataSeries;
import io.github.nicklambourne.slackblocks.object.DispatchActionConfiguration;
import io.github.nicklambourne.slackblocks.object.FeedbackButton;
import io.github.nicklambourne.slackblocks.object.InputParameter;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import io.github.nicklambourne.slackblocks.object.Option;
import io.github.nicklambourne.slackblocks.object.OptionGroup;
import io.github.nicklambourne.slackblocks.object.PieChart;
import io.github.nicklambourne.slackblocks.object.PlainText;
import io.github.nicklambourne.slackblocks.object.RawText;
import io.github.nicklambourne.slackblocks.object.RichTextSection;
import io.github.nicklambourne.slackblocks.object.RichTextStyle;
import io.github.nicklambourne.slackblocks.object.RichTextText;
import io.github.nicklambourne.slackblocks.object.SlackFile;
import io.github.nicklambourne.slackblocks.object.SlackIcon;
import io.github.nicklambourne.slackblocks.object.Trigger;
import io.github.nicklambourne.slackblocks.object.UrlSource;
import io.github.nicklambourne.slackblocks.object.Workflow;
import io.github.nicklambourne.slackblocks.payload.Attachment;
import java.lang.reflect.Array;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.function.Supplier;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.TestFactory;

/** Calls every named builder method described in model.json and checks the wire field it sets. */
final class BuilderMethodsTest {
  private static final String BASE = "io.github.nicklambourne.slackblocks.";
  private static final Map<String, Supplier<Object>> SAMPLES = new LinkedHashMap<>();

  static {
    Supplier<Object> text = () -> MarkdownText.of("x");
    Supplier<Object> richText = () -> RichTextText.builder().text("x").build();
    Supplier<Object> section =
        () -> RichTextSection.builder().elements(RichTextText.builder().text("x").build()).build();
    Supplier<Object> input = () -> PlainTextInputElement.builder().actionId("a").build();
    sample("PlainText", () -> PlainText.of("x"));
    sample("MarkdownText", text);
    sample("Text", text);
    sample("ContextElement", text);
    sample("Element", input);
    sample("InputElement", input);
    sample(
        "ImageElement",
        () -> ImageElement.builder().imageUrl("https://example.com/x.png").altText("x").build());
    sample("ContextActionsElement", () -> IconButtonElement.builder().text("x").build());
    sample("ButtonElement", () -> ButtonElement.builder("x", "a").build());
    sample("Block", DividerBlock::create);
    sample("CardBlock", () -> CardBlock.builder().title("x").build());
    sample("TaskCardBlock", () -> TaskCardBlock.builder().taskId("t").title("x").build());
    sample(
        "RichTextBlock",
        () ->
            RichTextBlock.builder()
                .elements(
                    RichTextSection.builder()
                        .elements(RichTextText.builder().text("x").build())
                        .build())
                .build());
    sample("RichTextSection", section);
    sample("RichTextBlockElement", section);
    sample("RichTextText", richText);
    sample("RichTextSectionElement", richText);
    sample("TableCell", () -> RawText.of("x"));
    sample("DataTableCell", () -> RawText.of("x"));
    sample("ChartSegment", () -> ChartSegment.builder().label("x").value(1).build());
    sample(
        "Chart",
        () ->
            PieChart.builder()
                .segments(ChartSegment.builder().label("x").value(1).build())
                .build());
    sample("DataPoint", () -> DataPoint.builder().label("x").value(1).build());
    sample(
        "DataSeries",
        () ->
            DataSeries.builder()
                .name("x")
                .data(DataPoint.builder().label("x").value(1).build())
                .build());
    sample("AxisConfig", () -> AxisConfig.builder().categories("x").build());
    sample("ColumnSettings", () -> ColumnSettings.builder().isWrapped(true).build());
    sample(
        "Confirmation",
        () -> Confirmation.builder().title("x").text("x").confirm("x").deny("x").build());
    sample("Option", () -> Option.builder("x", "v").build());
    sample(
        "OptionGroup",
        () -> OptionGroup.builder().label("x").options(Option.builder("x", "v").build()).build());
    sample("ConversationFilter", () -> ConversationFilter.builder().include("im").build());
    sample(
        "DispatchActionConfiguration",
        () -> DispatchActionConfiguration.builder().triggerActionsOn("on_enter_pressed").build());
    sample("FeedbackButton", () -> FeedbackButton.builder().text("x").value("v").build());
    sample("SlackFile", () -> SlackFile.builder().id("F123").build());
    sample("SlackIcon", () -> SlackIcon.builder().name("rocket").build());
    sample("UrlSource", () -> UrlSource.builder().url("https://example.com").text("x").build());
    sample("Trigger", () -> Trigger.builder().url("https://example.com").build());
    sample(
        "Workflow",
        () ->
            Workflow.builder()
                .trigger(Trigger.builder().url("https://example.com").build())
                .build());
    sample("InputParameter", () -> InputParameter.builder().name("x").value("v").build());
    sample("Attachment", () -> Attachment.builder().blocks(DividerBlock.create()).build());
    sample("RichTextStyle", () -> RichTextStyle.builder().bold(true).build());
  }

  private static void sample(String type, Supplier<Object> supplier) {
    SAMPLES.put(type, supplier);
  }

  @TestFactory
  Stream<DynamicTest> everyNamedBuilderMethodSetsItsWireField() throws Exception {
    JsonObject model =
        JsonParser.parseString(Files.readString(Path.of("generator", "model.json")))
            .getAsJsonObject();
    Map<String, String> packages = new LinkedHashMap<>();
    for (String group : List.of("interfaces", "enums", "types")) {
      for (JsonElement item : model.getAsJsonArray(group)) {
        packages.put(
            item.getAsJsonObject().get("name").getAsString(),
            item.getAsJsonObject().get("package").getAsString());
      }
    }
    packages.put("RichTextStyle", "object");
    List<DynamicTest> tests = new ArrayList<>();
    for (JsonElement element : model.getAsJsonArray("types")) {
      JsonObject type = element.getAsJsonObject();
      String name = type.get("name").getAsString();
      tests.add(DynamicTest.dynamicTest(name, () -> assertType(type, packages)));
    }
    return tests.stream();
  }

  private static void assertType(JsonObject type, Map<String, String> packages) throws Exception {
    Class<?> valueClass =
        Class.forName(
            BASE + type.get("package").getAsString() + "." + type.get("name").getAsString());
    Class<?> builderClass = Class.forName(valueClass.getName() + "$Builder");
    Set<Method> covered = new HashSet<>();
    for (JsonElement fieldElement : type.getAsJsonArray("fields")) {
      JsonObject field = fieldElement.getAsJsonObject();
      String method = field.get("method").getAsString();
      String wire = field.get("wire").getAsString();
      String kind = field.get("kind").getAsString();
      Class<?> target =
          field.has("type")
              ? Class.forName(
                  BASE
                      + packages.get(field.get("type").getAsString())
                      + "."
                      + field.get("type").getAsString())
              : Object.class;
      switch (kind) {
        case "string" -> check(builderClass, covered, method, String.class, "x", wire, "x");
        case "boolean" -> check(builderClass, covered, method, boolean.class, true, wire, true);
        case "int" -> check(builderClass, covered, method, int.class, 1, wire, 1);
        case "long" -> check(builderClass, covered, method, long.class, 1L, wire, 1L);
        case "double" -> check(builderClass, covered, method, double.class, 1.5, wire, 1.5);
        case "number" -> {
          check(builderClass, covered, method, long.class, 2L, wire, 2L);
          check(builderClass, covered, method, double.class, 2.5, wire, 2.5);
        }
        case "stringList" ->
            check(
                builderClass,
                covered,
                method,
                String[].class,
                new String[] {"x"},
                wire,
                List.of("x"));
        case "map" ->
            check(
                builderClass, covered, method, Map.class, Map.of("k", "v"), wire, Map.of("k", "v"));
        case "enum" -> {
          Object constant = target.getEnumConstants()[0];
          Object wireValue = target.getMethod("wireValue").invoke(constant);
          check(builderClass, covered, method, target, constant, wire, wireValue);
        }
        case "object" -> {
          Object value = sampleFor(field.get("type").getAsString());
          check(builderClass, covered, method, target, value, wire, value);
        }
        case "list" -> {
          Object value = sampleFor(field.get("type").getAsString());
          Object array = Array.newInstance(target, 1);
          Array.set(array, 0, value);
          check(builderClass, covered, method, array.getClass(), array, wire, List.of(value));
        }
        case "rows" -> {
          List<Object> row = List.of(sampleFor(field.get("type").getAsString()));
          check(
              builderClass, covered, method, List[].class, new List<?>[] {row}, wire, List.of(row));
        }
        case "text" -> {
          Map<String, Object> coerced =
              Map.of("type", field.get("coerce").getAsString(), "text", "x");
          check(builderClass, covered, method, String.class, "x", wire, coerced);
          Object value = sampleFor(field.get("type").getAsString());
          check(builderClass, covered, method, target, value, wire, value);
          if (method.equals("text")) {
            String alias =
                field.get("coerce").getAsString().equals("plain_text")
                    ? "plainText"
                    : "markdownText";
            check(builderClass, covered, alias, String.class, "x", wire, coerced);
          }
        }
        case "textList" -> {
          Object value = sampleFor(field.get("type").getAsString());
          Object array = Array.newInstance(target, 1);
          Array.set(array, 0, value);
          check(builderClass, covered, method, array.getClass(), array, wire, List.of(value));
          String markdown =
              "markdown" + Character.toUpperCase(method.charAt(0)) + method.substring(1);
          check(
              builderClass,
              covered,
              markdown,
              String[].class,
              new String[] {"x"},
              wire,
              List.of(Map.of("type", "mrkdwn", "text", "x")));
        }
        case "style" -> {
          Object value = sampleFor("RichTextStyle");
          check(
              builderClass,
              covered,
              method,
              Class.forName(BASE + "object.RichTextStyle"),
              value,
              wire,
              value);
          for (JsonElement flag : field.getAsJsonArray("flags")) {
            String wireFlag = flag.getAsString();
            check(
                builderClass,
                covered,
                camel(wireFlag),
                boolean.class,
                true,
                wire,
                Map.of(wireFlag, true));
          }
        }
        default -> throw new AssertionError("Unknown field kind " + kind);
      }
    }
    for (JsonElement fieldElement : type.getAsJsonArray("fields")) {
      JsonObject field = fieldElement.getAsJsonObject();
      String wire = field.get("wire").getAsString();
      boolean inherited =
          (type.get("package").getAsString().equals("block") && wire.equals("block_id"))
              || (Set.of("PlainText", "MarkdownText").contains(type.get("name").getAsString())
                  && wire.equals("text"));
      Method getter = valueClass.getMethod(getterName(field.get("method").getAsString()));
      assertEquals(
          true,
          Modifier.isPublic(getter.getModifiers()) && getter.getReturnType() != void.class,
          valueClass.getSimpleName() + " needs a typed getter for " + wire);
      if (!inherited) {
        assertEquals(valueClass, getter.getDeclaringClass(), getter.toString());
      }
    }
    List<String> untested =
        Arrays.stream(builderClass.getDeclaredMethods())
            .filter(
                candidate ->
                    Modifier.isPublic(candidate.getModifiers()) && !candidate.isSynthetic())
            .filter(
                candidate ->
                    !candidate.getName().equals("build")
                        && !candidate.getName().equals("wireField"))
            .filter(candidate -> !covered.contains(candidate))
            .map(Method::toString)
            .toList();
    assertEquals(List.of(), untested, "Builder methods missing from model.json");
  }

  private static void check(
      Class<?> builderClass,
      Set<Method> covered,
      String name,
      Class<?> parameter,
      Object argument,
      String wire,
      Object expected)
      throws Exception {
    Method method = builderClass.getMethod(name, parameter);
    covered.add(method);
    Object builder = newBuilder(builderClass);
    Object returned = method.invoke(builder, argument);
    assertSame(builder, returned, name + " must return the same builder");
    Field stateField = builderClass.getDeclaredField("state");
    stateField.setAccessible(true);
    BuilderState state = (BuilderState) stateField.get(builder);
    assertEquals(
        expected,
        normalize(state.get(wire)),
        builderClass.getName() + "." + name + "(" + parameter.getSimpleName() + ")");
  }

  private static Object normalize(Object value) {
    if (value instanceof RichTextStyle style) {
      return style;
    }
    if (value instanceof Map<?, ?> map) {
      Map<Object, Object> copy = new LinkedHashMap<>();
      map.forEach((key, nested) -> copy.put(key, normalize(nested)));
      return copy;
    }
    if (value instanceof List<?> list) {
      return list.stream().map(BuilderMethodsTest::normalize).toList();
    }
    return value;
  }

  private static Object newBuilder(Class<?> builderClass) throws Exception {
    Method factory = builderClass.getEnclosingClass().getMethod("builder");
    return factory.invoke(null);
  }

  private static Object sampleFor(String type) {
    Supplier<Object> supplier = SAMPLES.get(type);
    if (supplier == null) {
      throw new AssertionError("Add a sample value for " + type + " to BuilderMethodsTest");
    }
    return supplier.get();
  }

  private static String getterName(String method) {
    String name =
        method.startsWith("is") && Character.isUpperCase(method.charAt(2))
            ? method.substring(2)
            : method;
    return "get" + Character.toUpperCase(name.charAt(0)) + name.substring(1);
  }

  private static String camel(String snake) {
    StringBuilder result = new StringBuilder();
    boolean upper = false;
    for (int index = 0; index < snake.length(); index++) {
      char character = snake.charAt(index);
      if (character == '_') {
        upper = true;
      } else {
        result.append(upper ? Character.toUpperCase(character) : character);
        upper = false;
      }
    }
    return result.toString();
  }
}
