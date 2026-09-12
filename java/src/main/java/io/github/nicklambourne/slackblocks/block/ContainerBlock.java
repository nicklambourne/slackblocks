package io.github.nicklambourne.slackblocks.block;

import com.google.gson.annotations.JsonAdapter;
import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import io.github.nicklambourne.slackblocks.internal.WireObjects;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Creates a titled group of child blocks.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ContainerBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ContainerBlock(Map<String, Object> values) {
    this.values = values;
  }

  /**
   * Starts a new concrete fluent builder.
   *
   * @return a new builder
   */
  public static Builder builder() {
    return new Builder();
  }


  /** {@inheritDoc} */
  @Override
  public Map<String, Object> toMap() {
    return WireObjects.materialize(values);
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof ContainerBlock that && toMap().equals(that.toMap()));
  }

  /** {@inheritDoc} */
  @Override
  public int hashCode() {
    return toMap().hashCode();
  }

  /** {@inheritDoc} */
  @Override
  public String toString() {
    return toJson();
  }

  /** Concrete fluent builder for {@link ContainerBlock}. */
  public static final class Builder implements Buildable<ContainerBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ContainerBlock", "container");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code child_blocks} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder childBlocks(SlackObject... values) {
      state.append("child_blocks", (Object[]) Objects.requireNonNull(values, "childBlocks"));
      return this;
    }

    /**
     * Sets title from a string.
     *
     * @param value value for Slack's {@code title} field
     * @return this builder
     */
    public Builder title(String value) {
      state.set("title", WireObjects.text("plain_text", Objects.requireNonNull(value, "title")));
      return this;
    }

    /**
     * Sets Slack's {@code title} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder title(SlackObject value) {
      state.set("title", Objects.requireNonNull(value, "title"));
      return this;
    }

    /**
     * Sets rich text title from a string.
     *
     * @param value value for Slack's {@code rich_text_title} field
     * @return this builder
     */
    public Builder richTextTitle(String value) {
      state.set("rich_text_title", Objects.requireNonNull(value, "richTextTitle"));
      return this;
    }

    /**
     * Sets Slack's {@code rich_text_title} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder richTextTitle(SlackObject value) {
      state.set("rich_text_title", Objects.requireNonNull(value, "richTextTitle"));
      return this;
    }

    /**
     * Sets subtitle from a string.
     *
     * @param value value for Slack's {@code subtitle} field
     * @return this builder
     */
    public Builder subtitle(String value) {
      state.set("subtitle", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "subtitle")));
      return this;
    }

    /**
     * Sets Slack's {@code subtitle} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder subtitle(SlackObject value) {
      state.set("subtitle", Objects.requireNonNull(value, "subtitle"));
      return this;
    }

    /**
     * Sets width.
     *
     * @param value value for Slack's {@code width} field
     * @return this builder
     */
    public Builder width(String value) {
      state.set("width", Objects.requireNonNull(value, "width"));
      return this;
    }

    /**
     * Sets icon from a string.
     *
     * @param value value for Slack's {@code icon} field
     * @return this builder
     */
    public Builder icon(String value) {
      state.set("icon", Objects.requireNonNull(value, "icon"));
      return this;
    }

    /**
     * Sets Slack's {@code icon} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder icon(SlackObject value) {
      state.set("icon", Objects.requireNonNull(value, "icon"));
      return this;
    }

    /**
     * Sets whether is collapsible is enabled.
     *
     * @param value value for Slack's {@code is_collapsible} field
     * @return this builder
     */
    public Builder isCollapsible(boolean value) {
      state.set("is_collapsible", value);
      return this;
    }

    /**
     * Sets whether default collapsed is enabled.
     *
     * @param value value for Slack's {@code default_collapsed} field
     * @return this builder
     */
    public Builder defaultCollapsed(boolean value) {
      state.set("default_collapsed", value);
      return this;
    }

    /**
     * Sets whether has header divider is enabled.
     *
     * @param value value for Slack's {@code has_header_divider} field
     * @return this builder
     */
    public Builder hasHeaderDivider(boolean value) {
      state.set("has_header_divider", value);
      return this;
    }

    /**
     * Sets block id.
     *
     * @param value value for Slack's {@code block_id} field
     * @return this builder
     */
    public Builder blockId(String value) {
      state.set("block_id", Objects.requireNonNull(value, "blockId"));
      return this;
    }
    /**
     * Sets a forward-compatible wire field that does not yet have a named fluent method.
     * Prefer the named methods for normal Block Kit use.
     *
     * @param field Slack JSON field name
     * @param value wire-compatible value
     * @return this builder
     */
    public Builder wireField(String field, Object value) {
      if (field.isEmpty()) {
        throw new IllegalArgumentException("field must not be empty");
      }
      state.set(field, Objects.requireNonNull(value, "value"));
      return this;
    }

    /** {@inheritDoc} */
    @Override
    public ContainerBlock build() {
      return state.build(ContainerBlock::new);
    }
  }
}
