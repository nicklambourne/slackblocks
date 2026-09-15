package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

/** Builds every shared valid fixture through the typed fluent API and compares the exact JSON. */
final class ConformanceTest {
  private static final Path SPEC_ROOT = Path.of("..", "spec");
  private static final Path VALID = SPEC_ROOT.resolve("fixtures/valid");

  /**
   * Java types for fixtures whose root object has no Slack {@code type}, matched by id prefix in
   * order, most specific first.
   */
  private static final List<Map.Entry<String, String>> UNTYPED_ROOTS =
      List.of(
          Map.entry("attachments/", "Attachment"),
          Map.entry("messages/message_response", "MessageResponse"),
          Map.entry("messages/webhook_message", "WebhookMessage"),
          Map.entry("messages/message_", "MessagePayload"),
          Map.entry("objects/confirmation", "Confirmation"),
          Map.entry("objects/conversation_filter", "ConversationFilter"),
          Map.entry("objects/dispatch_action_configuration", "DispatchActionConfiguration"),
          Map.entry("objects/input_parameter", "InputParameter"),
          Map.entry("objects/option_group", "OptionGroup"),
          Map.entry("objects/option_", "Option"),
          Map.entry("objects/slack_file", "SlackFile"),
          Map.entry("objects/trigger", "Trigger"),
          Map.entry("objects/workflow", "Workflow"));

  @TestFactory
  Stream<DynamicTest> everySharedValidFixtureBuildsThroughNamedFluentMethods() throws Exception {
    JsonObject manifest = manifest();
    assertEquals(Slackblocks.SPEC_VERSION, manifest.get("spec_version").getAsString());
    FluentDriver driver = new FluentDriver();
    return manifest.getAsJsonArray("fixtures").asList().stream()
        .map(entry -> entry.getAsJsonObject().get("id").getAsString())
        .map(id -> DynamicTest.dynamicTest(id, () -> assertFixture(driver, id)));
  }

  @Test
  void everyValidFixtureFileIsListedInTheManifest() throws Exception {
    Set<String> listed =
        manifest().getAsJsonArray("fixtures").asList().stream()
            .map(entry -> entry.getAsJsonObject().get("id").getAsString())
            .collect(Collectors.toCollection(TreeSet::new));
    Set<String> onDisk;
    try (Stream<Path> files = Files.walk(VALID)) {
      onDisk =
          files
              .filter(file -> file.toString().endsWith(".json"))
              .map(
                  file ->
                      VALID
                          .relativize(file)
                          .toString()
                          .replace('\\', '/')
                          .replaceAll("\\.json$", ""))
              .collect(Collectors.toCollection(TreeSet::new));
    }
    assertEquals(onDisk, listed);
  }

  @Test
  void releasedJavaConformanceSkipListIsEmpty() throws Exception {
    for (String line : Files.readAllLines(Path.of("conformance", "skiplist.txt"))) {
      assertTrue(
          line.isBlank() || line.stripLeading().startsWith("#"),
          () -> "Java conformance skip list contains " + line);
    }
  }

  private static void assertFixture(FluentDriver driver, String id) throws Exception {
    JsonElement expected = JsonParser.parseString(Files.readString(VALID.resolve(id + ".json")));
    int fallbacksBefore = driver.fallbacks().size();
    Object built = driver.build(expected, rootType(driver, id), id);
    List<String> fallbacks = driver.fallbacks().subList(fallbacksBefore, driver.fallbacks().size());

    assertEquals(List.of(), List.copyOf(fallbacks), "fields without a usable named method");
    SlackObject value = (SlackObject) built;
    JsonElement actual = JsonParser.parseString(value.toJson());
    assertEquals(FluentDriver.canonical(expected), FluentDriver.canonical(actual));
  }

  static Class<?> rootType(FluentDriver driver, String id) throws Exception {
    for (Map.Entry<String, String> root : UNTYPED_ROOTS) {
      if (id.startsWith(root.getKey())) {
        return driver.type(root.getValue());
      }
    }
    return Object.class;
  }

  private static JsonObject manifest() throws Exception {
    return JsonParser.parseString(Files.readString(SPEC_ROOT.resolve("manifest.json")))
        .getAsJsonObject();
  }
}
