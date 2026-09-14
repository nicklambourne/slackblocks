package io.github.nicklambourne.slackblocks.component;

import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.ContainerBlock;
import io.github.nicklambourne.slackblocks.block.ContainerWidth;
import io.github.nicklambourne.slackblocks.element.ImageElement;
import io.github.nicklambourne.slackblocks.object.Text;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/** Builds one independently collapsible, Slack-native accordion section. */
public final class AccordionSection {
  private AccordionSection() {}

  /**
   * Starts a section with its required heading.
   *
   * @param title plain-text heading
   * @return a new builder
   */
  public static Builder builder(String title) {
    return new Builder(title);
  }

  /** Concrete fluent accordion-section builder. */
  public static final class Builder {
    private final String title;
    private final List<Block> blocks = new ArrayList<>();
    private Text subtitle;
    private ImageElement icon;
    private boolean expanded;
    private ContainerWidth width = ContainerWidth.STANDARD;
    private boolean hasHeaderDivider;
    private String blockId;

    private Builder(String title) {
      this.title = Objects.requireNonNull(title, "title");
    }

    /**
     * Appends blocks revealed when this section is expanded.
     *
     * @param values blocks in display order
     * @return this builder
     */
    public Builder blocks(Block... values) {
      blocks.addAll(List.of(values));
      return this;
    }

    /**
     * Sets supporting Slack Markdown copy.
     *
     * @param value Markdown text
     * @return this builder
     */
    public Builder subtitle(String value) {
      return subtitle(io.github.nicklambourne.slackblocks.object.MarkdownText.of(value));
    }

    /**
     * Sets a typed supporting text value.
     *
     * @param value Slack text object
     * @return this builder
     */
    public Builder subtitle(Text value) {
      subtitle = Objects.requireNonNull(value, "subtitle");
      return this;
    }

    /**
     * Sets the section header icon.
     *
     * @param value image element
     * @return this builder
     */
    public Builder icon(ImageElement value) {
      icon = Objects.requireNonNull(value, "icon");
      return this;
    }

    /**
     * Controls whether the section starts expanded.
     *
     * @param value initial expanded state
     * @return this builder
     */
    public Builder expanded(boolean value) {
      expanded = value;
      return this;
    }

    /**
     * Sets the Slack-native container width.
     *
     * @param value container width
     * @return this builder
     */
    public Builder width(ContainerWidth value) {
      width = Objects.requireNonNull(value, "width");
      return this;
    }

    /**
     * Controls the divider below the section header.
     *
     * @param value whether to show the divider
     * @return this builder
     */
    public Builder hasHeaderDivider(boolean value) {
      hasHeaderDivider = value;
      return this;
    }

    /**
     * Sets a deterministic block identifier.
     *
     * @param value block identifier
     * @return this builder
     */
    public Builder blockId(String value) {
      blockId = Objects.requireNonNull(value, "blockId");
      return this;
    }

    /**
     * Builds this section as an ordinary Slack container block.
     *
     * @return validated container block
     */
    public ContainerBlock build() {
      ContainerBlock.Builder result = ContainerBlock.builder()
          .title(title)
          .childBlocks(blocks.toArray(Block[]::new))
          .isCollapsible(true)
          .defaultCollapsed(!expanded)
          .width(width)
          .hasHeaderDivider(hasHeaderDivider);
      if (subtitle != null) {
        result.subtitle(subtitle);
      }
      if (icon != null) {
        result.icon(icon);
      }
      if (blockId != null) {
        result.blockId(blockId);
      }
      return result.build();
    }
  }
}
