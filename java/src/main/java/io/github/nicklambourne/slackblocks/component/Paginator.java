package io.github.nicklambourne.slackblocks.component;

import io.github.nicklambourne.slackblocks.ErrorCategory;
import io.github.nicklambourne.slackblocks.ValidationException;
import io.github.nicklambourne.slackblocks.block.ActionsBlock;
import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.ContextBlock;
import io.github.nicklambourne.slackblocks.element.ButtonElement;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import org.jspecify.annotations.Nullable;

/**
 * Renders one page of blocks plus previous and next buttons.
 *
 * <p>The buttons' action IDs are {@code <prefix>.previous} and {@code <prefix>.next}, and their
 * values hold the one-based page to show next, so an action handler can rebuild the message for
 * that page. Pass the result to the Slack SDK with {@code List.copyOf}, which widens it to {@code
 * List<LayoutBlock>}:
 *
 * <pre>{@code
 * List<Block> page = Paginator.builder("results").blocks(results).page(2).build();
 * client.chatPostMessage(ChatPostMessageRequest.builder()
 *     .channel("C0123456")
 *     .text("Search results")
 *     .blocks(List.copyOf(page))
 *     .build());
 * }</pre>
 */
public final class Paginator {
  private Paginator() {}

  /**
   * Starts a paginator with the prefix used for generated action identifiers.
   *
   * @param actionIdPrefix prefix for previous and next action IDs
   * @return a new builder
   */
  public static Builder builder(String actionIdPrefix) {
    return new Builder(actionIdPrefix);
  }

  /** Concrete fluent paginator builder. */
  public static final class Builder {
    private final String actionIdPrefix;
    private final List<Block> blocks = new ArrayList<>();
    private int page = 1;
    private int pageSize = 5;
    private String previousText = "Previous";
    private String nextText = "Next";
    private boolean showPageIndicator = true;
    private @Nullable String blockId;

    private Builder(String actionIdPrefix) {
      this.actionIdPrefix = Objects.requireNonNull(actionIdPrefix, "actionIdPrefix");
    }

    /**
     * Appends source blocks in display order.
     *
     * @param values source blocks
     * @return this builder
     */
    public Builder blocks(Block... values) {
      blocks.addAll(List.of(values));
      return this;
    }

    /**
     * Selects the one-based page to render.
     *
     * @param value one-based page number
     * @return this builder
     */
    public Builder page(int value) {
      page = value;
      return this;
    }

    /**
     * Sets the number of source blocks displayed per page.
     *
     * @param value positive page size
     * @return this builder
     */
    public Builder pageSize(int value) {
      pageSize = value;
      return this;
    }

    /**
     * Sets the previous-page button label.
     *
     * @param value button label
     * @return this builder
     */
    public Builder previousText(String value) {
      previousText = Objects.requireNonNull(value, "previousText");
      return this;
    }

    /**
     * Sets the next-page button label.
     *
     * @param value button label
     * @return this builder
     */
    public Builder nextText(String value) {
      nextText = Objects.requireNonNull(value, "nextText");
      return this;
    }

    /**
     * Controls whether a “Page n of m” context block is rendered.
     *
     * @param value whether to show the indicator
     * @return this builder
     */
    public Builder showPageIndicator(boolean value) {
      showPageIndicator = value;
      return this;
    }

    /**
     * Sets the identifier of the generated actions block. It is not used when every block fits on
     * one page, because no controls are rendered.
     *
     * @param value block identifier
     * @return this builder
     */
    public Builder blockId(String value) {
      blockId = Objects.requireNonNull(value, "blockId");
      return this;
    }

    /**
     * Builds the selected page followed, when there is more than one page, by a "Page n of m"
     * context block and the navigation buttons.
     *
     * @return immutable list of ordinary blocks
     * @throws ValidationException if there are no blocks, the prefix is empty, or the page or page
     *     size is out of range
     */
    public List<Block> build() {
      if (blocks.isEmpty()) {
        throw new ValidationException(
            ErrorCategory.MISSING_REQUIRED, "Paginator.blocks", "expected at least one block");
      }
      if (actionIdPrefix.isEmpty()) {
        throw new ValidationException(
            ErrorCategory.MISSING_REQUIRED,
            "Paginator.actionIdPrefix",
            "expected a non-empty prefix");
      }
      positive("Paginator.page", page);
      positive("Paginator.pageSize", pageSize);
      int pageCount = (blocks.size() + pageSize - 1) / pageSize;
      if (page > pageCount) {
        throw new ValidationException(
            ErrorCategory.OUT_OF_RANGE,
            "Paginator.page",
            "expected a value between 1 and " + pageCount);
      }
      int start = (page - 1) * pageSize;
      int end = Math.min(start + pageSize, blocks.size());
      List<Block> result = new ArrayList<>(blocks.subList(start, end));
      if (pageCount == 1) {
        return List.copyOf(result);
      }

      List<ButtonElement> controls = new ArrayList<>();
      if (page > 1) {
        controls.add(
            ButtonElement.builder(previousText, actionIdPrefix + ".previous")
                .value(Integer.toString(page - 1))
                .build());
      }
      if (page < pageCount) {
        controls.add(
            ButtonElement.builder(nextText, actionIdPrefix + ".next")
                .value(Integer.toString(page + 1))
                .build());
      }
      if (showPageIndicator) {
        result.add(
            ContextBlock.builder()
                .elements(MarkdownText.of("Page " + page + " of " + pageCount))
                .build());
      }
      ActionsBlock.Builder actions =
          ActionsBlock.builder().elements(controls.toArray(ButtonElement[]::new));
      String identifier = blockId;
      if (identifier != null) {
        actions.blockId(identifier);
      }
      result.add(actions.build());
      return List.copyOf(result);
    }

    private static void positive(String path, int value) {
      if (value < 1) {
        throw new ValidationException(
            ErrorCategory.OUT_OF_RANGE, path, "expected a positive integer");
      }
    }
  }
}
