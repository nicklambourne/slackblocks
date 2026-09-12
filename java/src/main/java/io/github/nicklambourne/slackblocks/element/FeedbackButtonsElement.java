package io.github.nicklambourne.slackblocks.element;

import com.google.gson.annotations.JsonAdapter;
import com.slack.api.model.block.element.BlockElement;
import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import io.github.nicklambourne.slackblocks.internal.WireObjects;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Creates a paired positive/negative feedback control.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class FeedbackButtonsElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private FeedbackButtonsElement(Map<String, Object> values) {
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
    return this == other || (other instanceof FeedbackButtonsElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link FeedbackButtonsElement}. */
  public static final class Builder implements Buildable<FeedbackButtonsElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("FeedbackButtons", "feedback_buttons");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets positive button from a string.
     *
     * @param value value for Slack's {@code positive_button} field
     * @return this builder
     */
    public Builder positiveButton(String value) {
      state.set("positive_button", Objects.requireNonNull(value, "positiveButton"));
      return this;
    }

    /**
     * Sets Slack's {@code positive_button} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder positiveButton(SlackObject value) {
      state.set("positive_button", Objects.requireNonNull(value, "positiveButton"));
      return this;
    }

    /**
     * Sets negative button from a string.
     *
     * @param value value for Slack's {@code negative_button} field
     * @return this builder
     */
    public Builder negativeButton(String value) {
      state.set("negative_button", Objects.requireNonNull(value, "negativeButton"));
      return this;
    }

    /**
     * Sets Slack's {@code negative_button} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder negativeButton(SlackObject value) {
      state.set("negative_button", Objects.requireNonNull(value, "negativeButton"));
      return this;
    }

    /**
     * Sets action id.
     *
     * @param value value for Slack's {@code action_id} field
     * @return this builder
     */
    public Builder actionId(String value) {
      state.set("action_id", Objects.requireNonNull(value, "actionId"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public FeedbackButtonsElement build() {
      return state.build(FeedbackButtonsElement::new);
    }
  }
}
