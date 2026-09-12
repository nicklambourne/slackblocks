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
 * Creates a section block. String text and fields are coerced to Slack mrkdwn objects.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class SectionBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private SectionBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof SectionBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link SectionBlock}. */
  public static final class Builder implements Buildable<SectionBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("SectionBlock", "section");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets text from a string.
     *
     * @param value value for Slack's {@code text} field
     * @return this builder
     */
    public Builder text(String value) {
      state.set("text", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "text")));
      return this;
    }

    /**
     * Sets Slack's {@code text} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder text(SlackObject value) {
      state.set("text", Objects.requireNonNull(value, "text"));
      return this;
    }

    /**
     * Sets the text using Slack Markdown.
     *
     * @param value text content
     * @return this builder
     */
    public Builder markdownText(String value) {
      return text(value);
    }

    /**
     * Adds values to Slack's {@code fields} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder fields(SlackObject... values) {
      state.append("fields", (Object[]) Objects.requireNonNull(values, "fields"));
      return this;
    }

    /**
     * Adds Markdown section fields in display order.
     *
     * @param values Markdown strings to append
     * @return this builder
     */
    public Builder markdownFields(String... values) {
      Objects.requireNonNull(values, "values");
      for (String value : values) {
        state.append("fields", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "field")));
      }
      return this;
    }

    /**
     * Sets accessory from a string.
     *
     * @param value value for Slack's {@code accessory} field
     * @return this builder
     */
    public Builder accessory(String value) {
      state.set("accessory", Objects.requireNonNull(value, "accessory"));
      return this;
    }

    /**
     * Sets Slack's {@code accessory} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder accessory(SlackObject value) {
      state.set("accessory", Objects.requireNonNull(value, "accessory"));
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
    public SectionBlock build() {
      return state.build(SectionBlock::new);
    }
  }
}
