package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;

import com.slack.api.model.block.LayoutBlock;
import com.slack.api.util.json.GsonFactory;
import io.github.nicklambourne.slackblocks.block.ActionsBlock;
import io.github.nicklambourne.slackblocks.block.HeaderBlock;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import java.util.List;
import org.junit.jupiter.api.Test;

final class ModelApiTest {
  @Test
  void buildsAReadableImmutableMessageLayout() {
    List<LayoutBlock> blocks =
        List.of(
            HeaderBlock.builder("Build #482 passed :white_check_mark:").build(),
            SectionBlock.builder()
                .markdownFields("*Branch*\n\u0060main\u0060", "*Tests*\n1,247 passed")
                .build(),
            ActionsBlock.builder()
                .elements(
                    ButtonElement.builder("View build", "view")
                        .url("https://ci.example.com/482")
                        .build())
                .build());

    assertEquals("header", blocks.get(0).getType());
    assertEquals(
        "[{\"type\":\"header\",\"text\":{\"type\":\"plain_text\",\"text\":"
            + "\"Build #482 passed :white_check_mark:\"}},{\"type\":\"section\",\"fields\":["
            + "{\"type\":\"mrkdwn\",\"text\":\"*Branch*\\n`main`\"},"
            + "{\"type\":\"mrkdwn\",\"text\":\"*Tests*\\n1,247 passed\"}]},"
            + "{\"type\":\"actions\",\"elements\":[{\"type\":\"button\","
            + "\"text\":{\"type\":\"plain_text\",\"text\":\"View build\"},"
            + "\"action_id\":\"view\",\"url\":\"https://ci.example.com/482\"}]}]",
        SlackblocksJson.write(blocks));
  }

  @Test
  void officialSlackSdkSerializesSlackblocksLayoutBlocksDirectly() {
    List<LayoutBlock> blocks =
        List.of(SectionBlock.builder().text(MarkdownText.of("*Ready*")).build());

    String json = GsonFactory.createSnakeCase().toJson(blocks);

    assertEquals(
        "[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"*Ready*\"}}]",
        json);
    assertInstanceOf(LayoutBlock.class, blocks.get(0));
  }

  @Test
  void aBuiltValueIsUnaffectedByLaterBuilderChanges() {
    SectionBlock.Builder builder = SectionBlock.builder().markdownText("first");
    SectionBlock first = builder.build();

    builder.markdownText("second");

    assertEquals("first", ((java.util.Map<?, ?>) first.toMap().get("text")).get("text"));
  }
}
