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
 * Creates a compact card with text, imagery, and actions.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class CardBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private CardBlock(Map<String, Object> values) {
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
  public String getType() {
    return (String) values.get("type");
  }

  /** {@inheritDoc} */
  @Override
  public String getBlockId() {
    return (String) values.get("block_id");
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof CardBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link CardBlock}. */
  public static final class Builder implements Buildable<CardBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("CardBlock", "card");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets hero image from a string.
     *
     * @param value value for Slack's {@code hero_image} field
     * @return this builder
     */
    public Builder heroImage(String value) {
      state.set("hero_image", Objects.requireNonNull(value, "heroImage"));
      return this;
    }

    /**
     * Sets Slack's {@code hero_image} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder heroImage(SlackObject value) {
      state.set("hero_image", Objects.requireNonNull(value, "heroImage"));
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
     * Sets title from a string.
     *
     * @param value value for Slack's {@code title} field
     * @return this builder
     */
    public Builder title(String value) {
      state.set("title", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "title")));
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
     * Sets body from a string.
     *
     * @param value value for Slack's {@code body} field
     * @return this builder
     */
    public Builder body(String value) {
      state.set("body", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "body")));
      return this;
    }

    /**
     * Sets Slack's {@code body} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder body(SlackObject value) {
      state.set("body", Objects.requireNonNull(value, "body"));
      return this;
    }

    /**
     * Adds values to Slack's {@code actions} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder actions(SlackObject... values) {
      state.append("actions", (Object[]) Objects.requireNonNull(values, "actions"));
      return this;
    }

    /**
     * Sets slack icon from a string.
     *
     * @param value value for Slack's {@code slack_icon} field
     * @return this builder
     */
    public Builder slackIcon(String value) {
      state.set("slack_icon", Objects.requireNonNull(value, "slackIcon"));
      return this;
    }

    /**
     * Sets Slack's {@code slack_icon} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder slackIcon(SlackObject value) {
      state.set("slack_icon", Objects.requireNonNull(value, "slackIcon"));
      return this;
    }

    /**
     * Sets subtext from a string.
     *
     * @param value value for Slack's {@code subtext} field
     * @return this builder
     */
    public Builder subtext(String value) {
      state.set("subtext", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "subtext")));
      return this;
    }

    /**
     * Sets Slack's {@code subtext} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder subtext(SlackObject value) {
      state.set("subtext", Objects.requireNonNull(value, "subtext"));
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
    public CardBlock build() {
      return state.build(CardBlock::new);
    }
  }
}
