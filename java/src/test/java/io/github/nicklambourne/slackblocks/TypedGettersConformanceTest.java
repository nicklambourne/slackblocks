package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.OptionalDouble;
import java.util.OptionalInt;
import java.util.OptionalLong;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

/** Every typed getter, on every value built from the shared corpus, agrees with the wire JSON. */
final class TypedGettersConformanceTest {
  private static final Gson GSON = new Gson();
  private static final AtomicInteger CHECKED = new AtomicInteger();

  @TestFactory
  Stream<DynamicTest> gettersAgreeWithTheWireFormForEveryValidFixture() throws Exception {
    JsonObject manifest =
        JsonParser.parseString(Files.readString(Path.of("..", "spec", "manifest.json")))
            .getAsJsonObject();
    FluentDriver driver = new FluentDriver();
    Map<String, JsonObject> types = new LinkedHashMap<>();
    JsonObject model =
        JsonParser.parseString(Files.readString(Path.of("generator", "model.json")))
            .getAsJsonObject();
    for (JsonElement type : model.getAsJsonArray("types")) {
      types.put(type.getAsJsonObject().get("name").getAsString(), type.getAsJsonObject());
    }
    return manifest.getAsJsonArray("fixtures").asList().stream()
        .map(entry -> entry.getAsJsonObject().get("id").getAsString())
        .map(
            id ->
                DynamicTest.dynamicTest(
                    id,
                    () -> {
                      JsonElement fixture =
                          JsonParser.parseString(
                              Files.readString(
                                  Path.of("..", "spec", "fixtures", "valid", id + ".json")));
                      Object built =
                          driver.build(fixture, ConformanceTest.rootType(driver, id), id);
                      int before = CHECKED.get();
                      assertGetters((SlackObject) built, types);
                      assertTrue(CHECKED.get() > before, "no getters were checked for " + id);
                    }));
  }

  @Test
  void aRawWireFieldTheGetterCannotRepresentFailsClearly() {
    SectionBlock section =
        SectionBlock.builder()
            .text("Deploy")
            .wireField("accessory", Map.of("type", "future_element"))
            .wireField("fields", List.of(Map.of("type", "mrkdwn", "text", "*Raw*")))
            .build();

    assertEquals(List.of(MarkdownText.of("*Raw*")), section.getFields());
    IllegalStateException error = assertThrows(IllegalStateException.class, section::getAccessory);
    assertTrue(
        error.getMessage().startsWith("SectionBlock.accessory holds a "), error.getMessage());
    assertEquals(Map.of("type", "future_element"), section.toMap().get("accessory"));
  }

  private static void assertGetters(SlackObject value, Map<String, JsonObject> types)
      throws Exception {
    JsonObject spec = types.get(value.getClass().getSimpleName());
    if (spec == null) {
      return;
    }
    Map<String, Object> wire = value.toMap();
    for (JsonElement fieldElement : spec.getAsJsonArray("fields")) {
      JsonObject field = fieldElement.getAsJsonObject();
      String key = field.get("wire").getAsString();
      Method getter = value.getClass().getMethod(getterName(field.get("method").getAsString()));
      Object result = unwrap(getter.invoke(value));
      String label = spec.get("name").getAsString() + "." + getter.getName() + "()";
      if (!wire.containsKey(key)) {
        assertTrue(result == null || (result instanceof List<?> list && list.isEmpty()), label);
        continue;
      }
      Object actual = toWire(result, types);
      if (spec.get("name").getAsString().equals("PlanBlock") && key.equals("tasks")) {
        actual = ((List<?>) actual).stream().map(TypedGettersConformanceTest::withoutType).toList();
      }
      assertEquals(
          FluentDriver.canonical(GSON.toJsonTree(wire.get(key))),
          FluentDriver.canonical(GSON.toJsonTree(actual)),
          label);
      CHECKED.incrementAndGet();
    }
  }

  private static Object unwrap(Object result) {
    if (result instanceof Optional<?> optional) {
      return optional.orElse(null);
    }
    if (result instanceof OptionalInt optional) {
      return optional.isPresent() ? optional.getAsInt() : null;
    }
    if (result instanceof OptionalLong optional) {
      return optional.isPresent() ? optional.getAsLong() : null;
    }
    if (result instanceof OptionalDouble optional) {
      return optional.isPresent() ? optional.getAsDouble() : null;
    }
    return result;
  }

  private static Object toWire(Object value, Map<String, JsonObject> types) throws Exception {
    if (value instanceof SlackObject slackObject) {
      assertGetters(slackObject, types);
      return slackObject.toMap();
    }
    if (value instanceof Enum<?> constant) {
      return constant.getDeclaringClass().getMethod("wireValue").invoke(constant);
    }
    if (value instanceof List<?> list) {
      List<Object> result = new ArrayList<>();
      for (Object item : list) {
        result.add(toWire(item, types));
      }
      return result;
    }
    return value;
  }

  private static Object withoutType(Object item) {
    Map<Object, Object> copy = new LinkedHashMap<>((Map<?, ?>) item);
    copy.remove("type");
    return copy;
  }

  private static String getterName(String method) {
    String name =
        method.startsWith("is") && Character.isUpperCase(method.charAt(2))
            ? method.substring(2)
            : method;
    return "get" + Character.toUpperCase(name.charAt(0)) + name.substring(1);
  }
}
