package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import com.slack.api.util.json.GsonFactory;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import java.util.List;
import org.junit.jupiter.api.Test;

final class ButtonPromptTest {
  @Test
  void nativeFieldsAndSdkSerializationRetainPrompts() {
    var button =
        ButtonElement.builder("Ask", "ask")
            .agentPrompt("Ask Slackbot")
            .agentPromptDisplay("x".repeat(80))
            .build();
    assertEquals("Ask Slackbot", button.getAgentPrompt().orElseThrow());
    assertEquals("x".repeat(80), button.getAgentPromptDisplay().orElseThrow());
    var actions =
        com.slack.api.model.block.ActionsBlock.builder().elements(List.of(button)).build();
    var wire =
        GsonFactory.createSnakeCase()
            .toJsonTree(actions)
            .getAsJsonObject()
            .getAsJsonArray("elements")
            .get(0)
            .getAsJsonObject();
    assertEquals("Ask Slackbot", wire.get("agent_prompt").getAsString());
    assertEquals("x".repeat(80), wire.get("agent_prompt_display").getAsString());
    assertFalse(
        ButtonElement.builder()
            .text("Ask")
            .agentPrompt("Ask Slackbot")
            .build()
            .toJson()
            .contains("agent_prompt_display"));
  }
}
