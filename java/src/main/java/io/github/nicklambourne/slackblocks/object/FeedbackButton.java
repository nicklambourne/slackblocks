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
 * Creates one positive or negative feedback choice.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class FeedbackButton implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private FeedbackButton(Map<String, Object> values) {
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
    return this == other || (other instanceof FeedbackButton that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link FeedbackButton}. */
  public static final class Builder implements Buildable<FeedbackButton> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("FeedbackButton", "");

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
      state.set("text", WireObjects.text("plain_text", Objects.requireNonNull(value, "text")));
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
     * Sets the text using plain text.
     *
     * @param value text content
     * @return this builder
     */
    public Builder plainText(String value) {
      return text(value);
    }

    /**
     * Sets the application-defined value.
     *
     * @param value value for Slack's {@code value} field
     * @return this builder
     */
    public Builder value(String value) {
      state.set("value", Objects.requireNonNull(value, "value"));
      return this;
    }

    /**
     * Sets accessibility label.
     *
     * @param value value for Slack's {@code accessibility_label} field
     * @return this builder
     */
    public Builder accessibilityLabel(String value) {
      state.set("accessibility_label", Objects.requireNonNull(value, "accessibilityLabel"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public FeedbackButton build() {
      return state.build(FeedbackButton::new);
    }
  }
}
