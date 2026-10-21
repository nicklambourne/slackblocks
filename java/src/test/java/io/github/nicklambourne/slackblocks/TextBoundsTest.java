package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import io.github.nicklambourne.slackblocks.element.PlainTextInputElement;
import org.junit.jupiter.api.Test;

final class TextBoundsTest {
  @Test
  void validatesOnlyWhenBothBoundsAreSupplied() {
    for (Integer[] bounds :
        new Integer[][] {{null, null}, {20, null}, {null, 10}, {0, 1}, {10, 10}, {3000, 3000}}) {
      var builder = PlainTextInputElement.builder();
      if (bounds[0] != null) builder.minLength(bounds[0]);
      if (bounds[1] != null) builder.maxLength(bounds[1]);
      assertNotNull(builder.build());
    }
    var error =
        assertThrows(
            ValidationException.class,
            () -> PlainTextInputElement.builder().minLength(20).maxLength(10).build());
    assertEquals(ErrorCategory.OUT_OF_RANGE, error.getCategory());
    assertEquals("PlainTextInputElement.min_length", error.getPath());
  }

  @Test
  void structuralControlsRemainAccepted() {
    assertNotNull(
        io.github.nicklambourne.slackblocks.element.ExternalSelectElement.builder()
            .minQueryLength(0)
            .build());
    assertNotNull(
        io.github.nicklambourne.slackblocks.element.ExternalMultiSelectElement.builder()
            .minQueryLength(0)
            .build());
    for (String date : new String[] {"2028-02-29", "2028-02-30"})
      assertNotNull(
          io.github.nicklambourne.slackblocks.element.DatePickerElement.builder()
              .initialDate(date)
              .build());
    for (String time : new String[] {"00:00", "23:59"})
      assertNotNull(
          io.github.nicklambourne.slackblocks.element.TimePickerElement.builder()
              .initialTime(time)
              .build());
    assertNotNull(
        io.github.nicklambourne.slackblocks.element.ImageElement.builder()
            .imageUrl("https://example.com/image.png")
            .altText("")
            .build());
  }
}
