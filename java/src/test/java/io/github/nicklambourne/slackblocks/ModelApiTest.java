package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.slack.api.model.block.LayoutBlock;
import com.slack.api.util.json.GsonFactory;
import io.github.nicklambourne.slackblocks.block.ActionsBlock;
import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.ContainerBlock;
import io.github.nicklambourne.slackblocks.block.ContainerWidth;
import io.github.nicklambourne.slackblocks.block.DataTableBlock;
import io.github.nicklambourne.slackblocks.block.DividerBlock;
import io.github.nicklambourne.slackblocks.block.HeaderBlock;
import io.github.nicklambourne.slackblocks.block.PlanBlock;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.block.TaskCardBlock;
import io.github.nicklambourne.slackblocks.block.TaskStatus;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.element.ButtonStyle;
import io.github.nicklambourne.slackblocks.object.ChartSegment;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import io.github.nicklambourne.slackblocks.object.PlainText;
import io.github.nicklambourne.slackblocks.object.RawNumber;
import io.github.nicklambourne.slackblocks.object.RawText;
import io.github.nicklambourne.slackblocks.object.RichTextStyle;
import io.github.nicklambourne.slackblocks.object.RichTextText;
import io.github.nicklambourne.slackblocks.payload.MessagePayload;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

final class ModelApiTest {
  @Test
  void buildsAReadableImmutableMessageLayout() {
    List<Block> blocks =
        List.of(
            HeaderBlock.builder("Build #482 passed :white_check_mark:").build(),
            SectionBlock.builder()
                .markdownFields("*Branch*\n`main`", "*Tests*\n1,247 passed")
                .build(),
            ActionsBlock.builder()
                .elements(
                    ButtonElement.builder("View build", "view")
                        .url("https://ci.example.com/482")
                        .style(ButtonStyle.PRIMARY)
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
            + "\"action_id\":\"view\",\"url\":\"https://ci.example.com/482\","
            + "\"style\":\"primary\"}]}]",
        SlackblocksJson.write(blocks));
  }

  @Test
  void officialSlackSdkSerializesSlackblocksLayoutBlocksDirectly() {
    List<LayoutBlock> blocks =
        List.of(SectionBlock.builder().text(MarkdownText.of("*Ready*")).build());

    String json = GsonFactory.createSnakeCase().toJson(blocks);

    assertEquals(
        "[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"*Ready*\"}}]", json);
    assertInstanceOf(LayoutBlock.class, blocks.get(0));
  }

  @Test
  void aBuiltValueIsUnaffectedByLaterBuilderChanges() {
    SectionBlock.Builder builder = SectionBlock.builder().markdownText("first");
    SectionBlock first = builder.build();

    builder.markdownText("second");

    assertEquals(Map.of("type", "mrkdwn", "text", "first"), first.toMap().get("text"));
  }

  @Test
  void planTasksOmitTheirTaskCardTypeOnTheWire() {
    PlanBlock plan =
        PlanBlock.builder()
            .title("Release plan")
            .tasks(
                TaskCardBlock.builder()
                    .taskId("test")
                    .title("Run the test suite")
                    .status(TaskStatus.COMPLETE)
                    .build())
            .build();

    assertEquals(
        "{\"type\":\"plan\",\"title\":\"Release plan\",\"tasks\":[{\"task_id\":\"test\","
            + "\"title\":\"Run the test suite\",\"status\":\"complete\"}]}",
        plan.toJson());
  }

  @Test
  void wholeNumbersAreSerializedWithoutADecimalPoint() {
    assertEquals(
        "{\"type\":\"raw_number\",\"value\":42,\"text\":\"42\"}",
        RawNumber.builder().value(42).text("42").build().toJson());
    assertEquals(
        "{\"label\":\"High\",\"value\":3.5}",
        ChartSegment.builder().label("High").value(3.5).build().toJson());
  }

  @Test
  void stringsForTextFieldsAreCoercedToTheFieldsTextObjectType() {
    assertEquals(
        Map.of("type", "plain_text", "text", "Header"),
        HeaderBlock.builder("Header").build().toMap().get("text"));
    assertEquals(
        Map.of("type", "mrkdwn", "text", "*Body*"),
        SectionBlock.builder().text("*Body*").build().toMap().get("text"));
    assertEquals(
        Map.of("type", "plain_text", "text", "Plain", "emoji", true),
        SectionBlock.builder()
            .text(PlainText.builder().text("Plain").emoji(true).build())
            .build()
            .toMap()
            .get("text"));
  }

  @Test
  void enumsWriteSlackWireValues() {
    ContainerBlock container =
        ContainerBlock.builder()
            .title("Details")
            .childBlocks(DividerBlock.create())
            .width(ContainerWidth.WIDE)
            .build();

    assertEquals("wide", container.toMap().get("width"));
  }

  @Test
  void richTextStyleFlagsMergeAndAStyleValueReplacesThem() {
    RichTextText flagged = RichTextText.builder().text("Deploy").bold(true).italic(false).build();
    RichTextText styled =
        RichTextText.builder()
            .text("Deploy")
            .code(true)
            .style(RichTextStyle.builder().bold(true).italic(false).build())
            .build();

    assertEquals(Map.of("bold", true, "italic", false), flagged.toMap().get("style"));
    assertEquals(flagged, styled);
  }

  @Test
  void tableRowsAcceptOnlyCellValues() {
    DataTableBlock table =
        DataTableBlock.builder()
            .caption("Scores")
            .rows(List.of(RawText.of("Name"), RawText.of("Score")))
            .rows(List.of(RawText.of("Alice"), RawNumber.builder().value(42).text("42").build()))
            .build();

    assertEquals(2, ((List<?>) table.toMap().get("rows")).size());
  }

  @Test
  void typedGettersReturnTheConfiguredValues() {
    ButtonElement approve =
        ButtonElement.builder("Approve", "approve").style(ButtonStyle.PRIMARY).build();
    SectionBlock section =
        SectionBlock.builder()
            .text("*Deploy* ready")
            .fields(PlainText.of("Branch"))
            .markdownFields("`main`")
            .accessory(approve)
            .blockId("deploy")
            .build();

    assertEquals(MarkdownText.of("*Deploy* ready"), section.getText().orElseThrow());
    assertEquals(List.of(PlainText.of("Branch"), MarkdownText.of("`main`")), section.getFields());
    assertEquals(approve, section.getAccessory().orElseThrow());
    assertEquals("deploy", section.getBlockId());
    assertEquals(PlainText.of("Approve"), approve.getText());
    assertEquals("approve", approve.getActionId());
    assertEquals(ButtonStyle.PRIMARY, approve.getStyle().orElseThrow());
    assertEquals(java.util.Optional.empty(), approve.getUrl());
  }

  @Test
  void gettersExposeNestedTypesNumbersRowsAndStyles() {
    TaskCardBlock task =
        TaskCardBlock.builder().taskId("test").title("Run").status(TaskStatus.COMPLETE).build();
    PlanBlock plan = PlanBlock.builder().title("Release").tasks(task).build();
    RawNumber score = RawNumber.builder().value(42).text("42").build();
    DataTableBlock table =
        DataTableBlock.builder()
            .caption("Scores")
            .rows(List.of(RawText.of("Name"), RawText.of("Score")))
            .rows(List.of(RawText.of("Alice"), score))
            .build();
    RichTextText text = RichTextText.builder().text("Deploy").bold(true).italic(false).build();

    assertEquals(List.of(task), plan.getTasks());
    assertEquals(TaskStatus.COMPLETE, plan.getTasks().get(0).getStatus().orElseThrow());
    assertEquals(42L, score.getValue());
    assertEquals(score, table.getRows().get(1).get(1));
    assertEquals(java.util.OptionalInt.of(5), table.getPageSize());
    assertEquals(
        RichTextStyle.builder().bold(true).italic(false).build(), text.getStyle().orElseThrow());
    assertEquals(List.of(), SectionBlock.builder().text("x").build().getFields());
  }

  @Test
  void valuesHaveValueSemantics() {
    DividerBlock first = DividerBlock.builder().blockId("divider").build();
    DividerBlock second = DividerBlock.builder().blockId("divider").build();

    assertEquals(first, second);
    assertEquals(first.hashCode(), second.hashCode());
    assertNotEquals(first, DividerBlock.create());
    assertEquals("{\"type\":\"divider\",\"block_id\":\"divider\"}", first.toString());
  }

  @Test
  void metadataMustBeJsonCompatible() {
    MessagePayload.Builder builder = MessagePayload.builder().channel("C123");

    IllegalArgumentException error =
        assertThrows(
            IllegalArgumentException.class,
            () ->
                builder.metadata(
                    Map.of("event_payload", Map.of("thread", Thread.currentThread()))));
    assertEquals(
        "metadata.event_payload.thread: java.lang.Thread cannot be written as Slack JSON; use a"
            + " string, number, boolean, list, map, or slackblocks value",
        error.getMessage());
  }

  @Test
  void nullCollectionItemsAreRejected() {
    ActionsBlock.Builder builder = ActionsBlock.builder();

    assertThrows(
        NullPointerException.class,
        () -> builder.elements(ButtonElement.builder("A", "a").build(), null));
  }
}
