package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.google.gson.annotations.JsonAdapter;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

final class FoundationTest {
  @JsonAdapter(SlackObjectJsonAdapter.class)
  private record TestValue(Map<String, Object> toMap, Map<String, Object> fields)
      implements SlackObject {}

  private record TestBuilder(BuilderState state) implements Buildable<TestValue> {
    @Override
    public TestValue build() {
      return state.build(TestValue::new);
    }
  }

  @Test
  void reportsTheMavenProjectVersion() {
    assertEquals(System.getProperty("slackblocks.project.version"), Slackblocks.VERSION);
  }

  @Test
  void serializesAsSlackWireObjectRatherThanAnImplementationWrapper() {
    BuilderState state = new BuilderState("Test", "divider");
    state.set("block_id", "summary");

    TestValue value = state.build(TestValue::new);

    assertEquals("{\"type\":\"divider\",\"block_id\":\"summary\"}", value.toJson());
    assertEquals(
        "[{\"type\":\"divider\",\"block_id\":\"summary\"}]", SlackblocksJson.write(List.of(value)));
  }

  @Test
  void builtValuesAreDefensiveSnapshots() {
    BuilderState state = new BuilderState("Test", "divider");
    state.set("block_id", "first");
    TestValue first = state.build(TestValue::new);
    state.set("block_id", "second");

    assertEquals("first", first.toMap().get("block_id"));
  }

  @Test
  void nestedBuildersAreMaterializedOnceAtBuildTime() {
    BuilderState nestedState = new BuilderState("Nested", "divider");
    nestedState.set("block_id", "first");
    BuilderState parent = new BuilderState("Test", "divider");
    parent.set("nested", new TestBuilder(nestedState));

    TestValue value = parent.build(TestValue::new);
    nestedState.set("block_id", "x".repeat(300));

    assertEquals(Map.of("type", "divider", "block_id", "first"), value.toMap().get("nested"));
    assertEquals(value.toJson(), value.toJson());
  }

  @Test
  void materializedValuesCannotBeModified() {
    BuilderState state = new BuilderState("Test", "divider");
    state.append("fields", Map.of("type", "mrkdwn", "text", "a"));
    Map<String, Object> wire = state.build(TestValue::new).toMap();

    assertThrows(UnsupportedOperationException.class, () -> wire.put("block_id", "x"));
    assertThrows(UnsupportedOperationException.class, () -> ((List<?>) wire.get("fields")).clear());
  }

  @Test
  void appendingNothingLeavesTheFieldUnsetAndNullItemsAreRejected() {
    BuilderState state = new BuilderState("Test", "divider");
    state.append("fields");

    assertFalse(state.build(TestValue::new).toMap().containsKey("fields"));
    NullPointerException error =
        assertThrows(NullPointerException.class, () -> state.append("fields", "a", null));
    assertEquals("fields[1] is null", error.getMessage());
  }

  @Test
  void rejectsStructurallyEmptyObjectsWithAStructuredError() {
    BuilderState state = new BuilderState("Test", "");

    ValidationException error =
        assertThrows(ValidationException.class, () -> state.build(TestValue::new));
    assertEquals(ErrorCategory.MISSING_REQUIRED, error.getCategory());
    assertEquals("Test", error.getPath());
  }
}
