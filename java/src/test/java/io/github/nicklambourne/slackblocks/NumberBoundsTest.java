package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import io.github.nicklambourne.slackblocks.element.NumberInputElement;
import org.junit.jupiter.api.Test;

class NumberBoundsTest {
  @Test
  void stringWireAndNumericConvenience() {
    var value =
        NumberInputElement.builder()
            .isDecimalAllowed(true)
            .minValue(-1.5)
            .maxValue("10.25")
            .build();
    assertEquals("-1.5", value.getMinValue().orElseThrow());
    assertEquals("10.25", value.getMaxValue().orElseThrow());
    assertTrue(value.toJson().contains("\"min_value\":\"-1.5\""));
    assertEquals(
        "0",
        NumberInputElement.builder()
            .isDecimalAllowed(false)
            .minValue(0L)
            .build()
            .getMinValue()
            .orElseThrow());
    assertFalse(
        NumberInputElement.builder().isDecimalAllowed(true).build().getMinValue().isPresent());
  }

  @Test
  void invalidAndInvertedBounds() {
    for (String v : new String[] {"NaN", "Infinity", "invalid", ""})
      assertThrows(
          RuntimeException.class,
          () -> NumberInputElement.builder().isDecimalAllowed(true).minValue(v).build());
    assertThrows(
        RuntimeException.class,
        () -> NumberInputElement.builder().isDecimalAllowed(true).minValue(Double.NaN).build());
    assertThrows(
        RuntimeException.class,
        () ->
            NumberInputElement.builder()
                .isDecimalAllowed(true)
                .minValue("10")
                .maxValue("2")
                .build());
  }

  @org.junit.jupiter.api.Test
  void exactDecimalContract() {
    for (String[] pair :
        new String[][] {
          {"9007199254740993", "9007199254740992"},
          {"0.10000000000000001", "0.1"},
          {"-9007199254740992", "-9007199254740993"},
          {"-0.1", "-0.10000000000000001"},
          {"1e-1000", "9e-1001"}
        }) {
      org.junit.jupiter.api.Assertions.assertThrows(
          ValidationException.class,
          () ->
              NumberInputElement.builder()
                  .isDecimalAllowed(true)
                  .minValue(pair[0])
                  .maxValue(pair[1])
                  .build());
      NumberInputElement.builder()
          .isDecimalAllowed(true)
          .minValue(pair[1])
          .maxValue(pair[0])
          .build();
    }
    for (String value : new String[] {"0x10", "1_000", " 1", "1 ", "NaN", "Infinity"})
      org.junit.jupiter.api.Assertions.assertThrows(
          ValidationException.class,
          () -> NumberInputElement.builder().isDecimalAllowed(true).minValue(value).build());
    NumberInputElement.builder().isDecimalAllowed(true).minValue("1e309").maxValue("2e309").build();
    NumberInputElement.builder().isDecimalAllowed(true).minValue("0.1000").maxValue("1e-1").build();
  }
}
