package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import io.github.nicklambourne.slackblocks.block.DividerBlock;
import io.github.nicklambourne.slackblocks.block.InputBlock;
import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.element.ImageElement;
import io.github.nicklambourne.slackblocks.element.PlainTextInputElement;
import io.github.nicklambourne.slackblocks.object.Option;
import io.github.nicklambourne.slackblocks.object.SlackFile;
import io.github.nicklambourne.slackblocks.payload.ModalView;
import io.github.nicklambourne.slackblocks.payload.WebhookMessage;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

final class ValidationContractTest {
  @Test
  void pathsStartWithTheJavaTypeBeingBuilt() {
    ValidationException mutuallyExclusive =
        assertThrows(
            ValidationException.class,
            () ->
                ImageElement.builder()
                    .altText("Logo")
                    .imageUrl("https://example.com/logo.png")
                    .slackFile(SlackFile.builder().id("F123").build())
                    .build());
    assertEquals(ErrorCategory.MUTUALLY_EXCLUSIVE, mutuallyExclusive.getCategory());
    assertEquals("ImageElement", mutuallyExclusive.getPath());

    ValidationException tooLong =
        assertThrows(
            ValidationException.class,
            () -> SectionBlock.builder().markdownFields("ok", "x".repeat(2001)).build());
    assertEquals("SectionBlock.fields[1].text", tooLong.getPath());

    ValidationException option =
        assertThrows(ValidationException.class, () -> Option.builder("x".repeat(76), "v").build());
    assertEquals("Option.text.text", option.getPath());
  }

  @Test
  void emptyPayloadsFailWithAStructuredError() {
    ValidationException error =
        assertThrows(ValidationException.class, () -> WebhookMessage.builder().build());

    assertEquals(ErrorCategory.MISSING_REQUIRED, error.getCategory());
    assertEquals("WebhookMessage", error.getPath());
  }

  @Test
  void modalSubmitErrorsNameTheInputBlock() {
    ValidationException error =
        assertThrows(
            ValidationException.class,
            () ->
                ModalView.builder()
                    .title("Request")
                    .blocks(
                        DividerBlock.create(),
                        InputBlock.builder()
                            .label("Name")
                            .element(PlainTextInputElement.builder().actionId("name").build())
                            .build())
                    .build());

    assertEquals("ModalView.submit", error.getPath());
    assertEquals(
        "ModalView.submit: required when the modal contains an input block (ModalView.blocks[1])",
        error.getMessage());
  }

  @Test
  void wireFieldAcceptsOnlyJsonCompatibleValues() {
    DividerBlock.Builder builder = DividerBlock.builder();

    IllegalArgumentException unsupported =
        assertThrows(
            IllegalArgumentException.class, () -> builder.wireField("future", new Object()));
    assertEquals(
        "future: java.lang.Object cannot be written as Slack JSON; use a string, number, boolean,"
            + " list, map, or slackblocks value",
        unsupported.getMessage());
    assertThrows(
        NullPointerException.class,
        () -> builder.wireField("future", java.util.Arrays.asList("a", null)));
    assertThrows(IllegalArgumentException.class, () -> builder.wireField("", "value"));

    DividerBlock value =
        builder.wireField("future", Map.of("enabled", true, "labels", List.of("a"))).build();
    assertEquals(Map.of("enabled", true, "labels", List.of("a")), value.toMap().get("future"));
  }

  @Test
  void wireFieldValuesAreStillValidated() {
    ValidationException error =
        assertThrows(
            ValidationException.class,
            () -> SectionBlock.builder().wireField("fields", List.of()).build());

    assertEquals(ErrorCategory.MISSING_REQUIRED, error.getCategory());
  }
}
