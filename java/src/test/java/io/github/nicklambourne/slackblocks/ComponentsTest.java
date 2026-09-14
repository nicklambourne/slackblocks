package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.slack.api.methods.request.chat.ChatPostMessageRequest;
import com.slack.api.util.json.GsonFactory;
import com.slack.api.model.block.LayoutBlock;
import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.ContainerWidth;
import io.github.nicklambourne.slackblocks.block.DividerBlock;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.component.Accordion;
import io.github.nicklambourne.slackblocks.component.AccordionSection;
import io.github.nicklambourne.slackblocks.component.Paginator;
import java.util.List;
import org.junit.jupiter.api.Test;

final class ComponentsTest {
  @Test
  void accordionBuildsSlackNativeContainerBlocks() {
    List<Block> blocks = Accordion.builder()
        .sections(
            AccordionSection.builder("Deployment details")
                .blocks(SectionBlock.builder().markdownText("*Build:* 482").build())
                .expanded(true)
                .width(ContainerWidth.WIDE)
                .build())
        .build();

    assertEquals("container", blocks.get(0).getType());
    assertEquals(true, blocks.get(0).toMap().get("is_collapsible"));
    assertEquals(false, blocks.get(0).toMap().get("default_collapsed"));
    assertEquals("wide", blocks.get(0).toMap().get("width"));
  }

  @Test
  void paginatorBuildsOnePageAndNavigationControls() {
    List<Block> source = List.of(
        section("one"), section("two"), section("three"), section("four"), section("five"));

    List<Block> page = Paginator.builder("results")
        .blocks(source.toArray(Block[]::new))
        .page(2)
        .pageSize(2)
        .blockId("results.controls")
        .build();

    assertEquals(List.of("section", "section", "context", "actions"),
        page.stream().map(Block::getType).toList());
    assertEquals("results.controls", page.get(3).getBlockId());
    assertEquals("1", ((java.util.Map<?, ?>) ((List<?>) page.get(3).toMap().get("elements")).get(0)).get("value"));
  }

  @Test
  void componentsReportStructuredValidationFailures() {
    ValidationException emptyAccordion = assertThrows(
        ValidationException.class, () -> Accordion.builder().build());
    assertEquals(ErrorCategory.MISSING_REQUIRED, emptyAccordion.getCategory());

    ValidationException invalidPage = assertThrows(
        ValidationException.class,
        () -> Paginator.builder("results").blocks(section("one")).page(2).build());
    assertEquals(ErrorCategory.OUT_OF_RANGE, invalidPage.getCategory());
  }

  @Test
  void blocksFitDirectlyIntoOfficialSlackSdkRequests() {
    SectionBlock section = section("Hello from slackblocks!");
    ChatPostMessageRequest request = ChatPostMessageRequest.builder()
        .token("xoxb-test")
        .channel("C0123456")
        .text("Hello from slackblocks!")
        .blocks(List.of(section))
        .build();

    List<LayoutBlock> nativeBlocks = request.getBlocks();
    assertEquals("C0123456", request.getChannel());
    assertEquals("section", nativeBlocks.get(0).getType());
    assertEquals("[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"Hello from slackblocks!\"}}]",
        GsonFactory.createSnakeCase().toJson(request.getBlocks()));
  }

  private static SectionBlock section(String text) {
    return SectionBlock.builder().markdownText(text).build();
  }
}
