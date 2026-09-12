package io.github.nicklambourne.slackblocks.object;

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
 * Creates a preformatted rich-text block.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class RichTextCodeBlock implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private RichTextCodeBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof RichTextCodeBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link RichTextCodeBlock}. */
  public static final class Builder implements Buildable<RichTextCodeBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("RichTextCodeBlock", "rich_text_preformatted");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code elements} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder elements(SlackObject... values) {
      state.append("elements", (Object[]) Objects.requireNonNull(values, "elements"));
      return this;
    }

    /**
     * Sets border.
     *
     * @param value value for Slack's {@code border} field
     * @return this builder
     */
    public Builder border(int value) {
      state.set("border", value);
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
    public RichTextCodeBlock build() {
      return state.build(RichTextCodeBlock::new);
    }
  }
}
