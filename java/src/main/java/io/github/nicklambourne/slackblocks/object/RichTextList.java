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
 * Creates an ordered or bullet rich-text list.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class RichTextList implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private RichTextList(Map<String, Object> values) {
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
    return this == other || (other instanceof RichTextList that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link RichTextList}. */
  public static final class Builder implements Buildable<RichTextList> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("RichTextList", "rich_text_list");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets style from a string.
     *
     * @param value value for Slack's {@code style} field
     * @return this builder
     */
    public Builder style(String value) {
      state.set("style", Objects.requireNonNull(value, "style"));
      return this;
    }

    /**
     * Sets Slack's {@code style} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder style(SlackObject value) {
      state.set("style", Objects.requireNonNull(value, "style"));
      return this;
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
     * Sets indent.
     *
     * @param value value for Slack's {@code indent} field
     * @return this builder
     */
    public Builder indent(int value) {
      state.set("indent", value);
      return this;
    }

    /**
     * Sets offset.
     *
     * @param value value for Slack's {@code offset} field
     * @return this builder
     */
    public Builder offset(int value) {
      state.set("offset", value);
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
    /** {@inheritDoc} */
    @Override
    public RichTextList build() {
      return state.build(RichTextList::new);
    }
  }
}
