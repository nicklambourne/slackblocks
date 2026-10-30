package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.*;

import com.slack.api.util.json.GsonFactory;
import io.github.nicklambourne.slackblocks.block.*;
import io.github.nicklambourne.slackblocks.object.LayoutItem;
import io.github.nicklambourne.slackblocks.object.LayoutRow;
import io.github.nicklambourne.slackblocks.payload.HomeTabView;
import io.github.nicklambourne.slackblocks.payload.MessagePayload;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import org.junit.jupiter.api.Test;

final class LayoutTest {
  private LayoutBlock grid() {
    return LayoutBlock.builder()
        .label("Grid")
        .rows(
            LayoutRow.builder()
                .items(LayoutItem.builder().block(DividerBlock.builder().build()).build())
                .build())
        .build();
  }

  @Test
  void acceptsBoundariesAndReplacementFallback() {
    var g = grid();
    assertEquals(
        25,
        MessagePayload.builder()
            .channel("C123")
            .blocks(Collections.nCopies(25, g).toArray(Block[]::new))
            .build()
            .getBlocks()
            .size());
    assertEquals(
        50,
        HomeTabView.builder()
            .blocks(Collections.nCopies(50, g).toArray(Block[]::new))
            .build()
            .getBlocks()
            .size());
    var fallback =
        LayoutBlock.builder()
            .label("Grid")
            .rows(g.getRows().toArray(LayoutRow[]::new))
            .fallback(
                Collections.nCopies(50, DividerBlock.builder().build())
                    .toArray(LayoutFallback[]::new))
            .build();
    MessagePayload.builder().channel("C123").blocks(fallback).build();
    var existing =
        ContainerBlock.builder()
            .title("Existing")
            .childBlocks(
                Collections.nCopies(10, DividerBlock.builder().build()).toArray(Block[]::new))
            .build();
    MessagePayload.builder()
        .channel("C123")
        .blocks(Collections.nCopies(50, existing).toArray(Block[]::new))
        .build();
  }

  @Test
  void exposesNativeGettersImmutableCollectionsAndSdkWire() {
    var item =
        LayoutItem.builder()
            .block(DividerBlock.builder().build())
            .width(LayoutWidth.WIDE)
            .horizontalAlignment(LayoutAlignment.END)
            .build();
    var items = new ArrayList<>(List.of(item));
    var row = LayoutRow.builder().items(items.toArray(LayoutItem[]::new)).build();
    items.clear();
    assertEquals(1, row.getItems().size());
    assertEquals(LayoutWidth.WIDE, row.getItems().get(0).getWidth().orElseThrow());
    assertEquals(LayoutAlignment.END, item.getHorizontalAlignment().orElseThrow());
    var value =
        LayoutBlock.builder()
            .label("Grid")
            .rows(row)
            .isLabelVisible(false)
            .gridLines(LayoutGridLines.ROWS)
            .fallback()
            .build();
    assertThrows(UnsupportedOperationException.class, () -> value.getRows().clear());
    com.slack.api.model.block.LayoutBlock sdk = value;
    assertEquals(SlackblocksJson.write(value), GsonFactory.createSnakeCase().toJson(sdk));
    assertEquals("layout", sdk.getType());
    assertEquals(false, value.getLabelVisible().orElseThrow());
    assertFalse(LayoutChild.class.isAssignableFrom(LayoutBlock.class));
    assertFalse(LayoutChild.class.isAssignableFrom(MarkdownBlock.class));
    assertFalse(LayoutFallback.class.isAssignableFrom(ActionsBlock.class));
  }
}
