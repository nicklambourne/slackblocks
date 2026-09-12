package io.github.nicklambourne.slackblocks.object;

import com.google.gson.annotations.JsonAdapter;
import com.slack.api.model.block.composition.TextObject;
import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import io.github.nicklambourne.slackblocks.internal.WireObjects;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Creates a Slack mrkdwn composition object.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class MarkdownText extends TextObject implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private MarkdownText(Map<String, Object> values) {
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

  /**
   * Creates and validates this text value in one call.
   *
   * @param text text content
   * @return an immutable MarkdownText
   */
  public static MarkdownText of(String text) {
    return builder().text(text).build();
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
  public String getText() {
    return (String) values.get("text");
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof MarkdownText that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link MarkdownText}. */
  public static final class Builder implements Buildable<MarkdownText> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Markdown", "mrkdwn");

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
      state.set("text", Objects.requireNonNull(value, "text"));
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
     * Sets whether verbatim is enabled.
     *
     * @param value value for Slack's {@code verbatim} field
     * @return this builder
     */
    public Builder verbatim(boolean value) {
      state.set("verbatim", value);
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
    public MarkdownText build() {
      return state.build(MarkdownText::new);
    }
  }
}
