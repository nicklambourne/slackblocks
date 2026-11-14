package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import io.github.nicklambourne.slackblocks.block.TableBlock;
import io.github.nicklambourne.slackblocks.object.ColumnAlign;
import io.github.nicklambourne.slackblocks.object.ColumnSettings;
import io.github.nicklambourne.slackblocks.object.RawText;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

final class SparseTableTest {
  @Test
  void gettersRetainSkippedPositionsAndDefensiveCopies() {
    var right = ColumnSettings.builder().align(ColumnAlign.RIGHT).build();
    ColumnSettings[] settings = {null, right, null};
    var table =
        TableBlock.builder()
            .rows(List.of(RawText.builder().text("A").build()))
            .columnSettings(settings)
            .build();
    settings[0] = right;
    assertEquals(Arrays.asList(null, right, null), table.getColumnSettings());
    assertThrows(UnsupportedOperationException.class, () -> table.getColumnSettings().add(right));
  }

  @Test
  void rejectsInvalidNonNullSettings() {
    for (Object invalid :
        List.of(false, 1, "right", Map.of("align", "invalid"), Map.of("is_wrapped", 1))) {
      var error =
          assertThrows(
              ValidationException.class,
              () ->
                  TableBlock.builder()
                      .rows(List.of(RawText.builder().text("A").build()))
                      .wireField("column_settings", List.of(invalid))
                      .build());
      assertEquals(ErrorCategory.TYPE_MISMATCH, error.getCategory());
    }
  }
}
