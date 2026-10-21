package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import io.github.nicklambourne.slackblocks.element.*;
import io.github.nicklambourne.slackblocks.object.Option;
import java.util.Map;
import java.util.function.Function;
import org.junit.jupiter.api.Test;

final class SelectPlaceholderTest {
  @Test
  void allNativeSelectorsEnforceUnicodeBoundaries() {
    Map<String, Function<String, Object>> factories =
        Map.ofEntries(
            Map.entry(
                "ExternalSelectElement",
                s -> ExternalSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "ExternalMultiSelectElement",
                s -> ExternalMultiSelectElement.builder().placeholder(s).build()),
            Map.entry("UserSelectElement", s -> UserSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "UserMultiSelectElement",
                s -> UserMultiSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "ChannelSelectElement", s -> ChannelSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "ChannelMultiSelectElement",
                s -> ChannelMultiSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "ConversationSelectElement",
                s -> ConversationSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "ConversationMultiSelectElement",
                s -> ConversationMultiSelectElement.builder().placeholder(s).build()),
            Map.entry(
                "StaticSelectElement",
                s ->
                    StaticSelectElement.builder()
                        .placeholder(s)
                        .options(Option.builder("A", "a").build())
                        .build()),
            Map.entry(
                "StaticMultiSelectElement",
                s ->
                    StaticMultiSelectElement.builder()
                        .placeholder(s)
                        .options(Option.builder("A", "a").build())
                        .build()));
    for (var item : factories.entrySet())
      for (String unit : new String[] {"x", "🚀"})
        for (int size : new int[] {149, 150, 151}) {
          String text = unit.repeat(size);
          if (size == 151) {
            var error =
                assertThrows(
                    ValidationException.class, () -> item.getValue().apply(text), item.getKey());
            assertEquals(ErrorCategory.LENGTH_EXCEEDED, error.getCategory());
            assertEquals(item.getKey() + ".placeholder.text", error.getPath());
          } else assertNotNull(item.getValue().apply(text));
        }
  }
}
