package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.google.gson.Gson;
import com.slack.api.model.block.ActionsBlock;
import com.slack.api.model.block.LayoutBlock;
import com.slack.api.model.block.SectionBlock;
import com.slack.api.model.block.composition.TextObject;
import com.slack.api.model.block.element.BlockElement;
import com.slack.api.util.json.GsonFactory;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.element.StaticSelectElement;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import io.github.nicklambourne.slackblocks.object.Option;
import io.github.nicklambourne.slackblocks.object.PlainText;
import java.util.List;
import org.junit.jupiter.api.Test;

/** slackblocks elements and text objects work inside the official SDK's own block models. */
final class SlackSdkInteropTest {
  private static final Gson SDK = GsonFactory.createSnakeCase();

  @Test
  void elementsAreSdkBlockElements() {
    BlockElement button = ButtonElement.builder("Approve", "approve").build();
    BlockElement select =
        StaticSelectElement.builder()
            .actionId("size")
            .options(Option.builder("Large", "l").build())
            .build();

    LayoutBlock actions = ActionsBlock.builder().elements(List.of(button, select)).build();

    assertEquals(
        "{\"type\":\"actions\",\"elements\":[{\"type\":\"button\",\"text\":{\"type\":\"plain_text\","
            + "\"text\":\"Approve\"},\"action_id\":\"approve\"},{\"type\":\"static_select\","
            + "\"action_id\":\"size\",\"options\":[{\"text\":{\"type\":\"plain_text\","
            + "\"text\":\"Large\"},\"value\":\"l\"}]}]}",
        SDK.toJson(actions));
  }

  @Test
  void textObjectsAreSdkTextObjects() {
    TextObject plain = PlainText.builder().text("Plain").emoji(true).build();
    TextObject markdown = MarkdownText.of("*Bold*");

    LayoutBlock section =
        SectionBlock.builder()
            .text(markdown)
            .fields(List.of(plain))
            .accessory(ButtonElement.builder("Open", "open").build())
            .build();

    assertEquals("plain_text", plain.getType());
    assertEquals("*Bold*", markdown.getText());
    assertEquals(
        "{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"*Bold*\"},"
            + "\"fields\":[{\"type\":\"plain_text\",\"text\":\"Plain\",\"emoji\":true}],"
            + "\"accessory\":{\"type\":\"button\",\"text\":{\"type\":\"plain_text\",\"text\":\"Open\"},"
            + "\"action_id\":\"open\"}}",
        SDK.toJson(section));
  }
}
