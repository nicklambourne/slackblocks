package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.google.gson.annotations.JsonAdapter;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import io.github.nicklambourne.slackblocks.internal.WireObjects;
import java.util.Map;
import org.junit.jupiter.api.Test;

final class FoundationTest {
  @JsonAdapter(SlackObjectJsonAdapter.class)
  private record TestValue(Map<String, Object> values) implements SlackObject {
    @Override
    public Map<String, Object> toMap() {
      return WireObjects.materialize(values);
    }
  }

  @Test
  void serializesAsSlackWireObjectRatherThanAnImplementationWrapper() {
    BuilderState state = new BuilderState("Test", "section");
    state.set("block_id", "summary");

    TestValue value = state.build(TestValue::new);

    assertEquals("{\"type\":\"section\",\"block_id\":\"summary\"}", value.toJson());
  }

  @Test
  void builtValuesAreDefensiveSnapshots() {
    BuilderState state = new BuilderState("Test", "section");
    state.set("block_id", "first");
    TestValue first = state.build(TestValue::new);
    state.set("block_id", "second");

    assertEquals("first", first.toMap().get("block_id"));
  }

  @Test
  void rejectsStructurallyEmptyObjects() {
    BuilderState state = new BuilderState("Test", "");

    assertThrows(IllegalArgumentException.class, () -> state.build(TestValue::new));
  }
}
