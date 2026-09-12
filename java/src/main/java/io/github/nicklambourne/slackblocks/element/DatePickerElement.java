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
 * Creates a calendar date picker.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class DatePickerElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private DatePickerElement(Map<String, Object> values) {
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
    return this == other || (other instanceof DatePickerElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link DatePickerElement}. */
  public static final class Builder implements Buildable<DatePickerElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("DatePicker", "datepicker");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

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

    /**
     * Sets initial date.
     *
     * @param value value for Slack's {@code initial_date} field
     * @return this builder
     */
    public Builder initialDate(String value) {
      state.set("initial_date", Objects.requireNonNull(value, "initialDate"));
      return this;
    }

    /**
     * Sets confirm from a string.
     *
     * @param value value for Slack's {@code confirm} field
     * @return this builder
     */
    public Builder confirm(String value) {
      state.set("confirm", Objects.requireNonNull(value, "confirm"));
      return this;
    }

    /**
     * Sets Slack's {@code confirm} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder confirm(SlackObject value) {
      state.set("confirm", Objects.requireNonNull(value, "confirm"));
      return this;
    }

    /**
     * Sets whether focus on load is enabled.
     *
     * @param value value for Slack's {@code focus_on_load} field
     * @return this builder
     */
    public Builder focusOnLoad(boolean value) {
      state.set("focus_on_load", value);
      return this;
    }

    /**
     * Sets placeholder from a string.
     *
     * @param value value for Slack's {@code placeholder} field
     * @return this builder
     */
    public Builder placeholder(String value) {
      state.set("placeholder", WireObjects.text("plain_text", Objects.requireNonNull(value, "placeholder")));
      return this;
    }

    /**
     * Sets Slack's {@code placeholder} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder placeholder(SlackObject value) {
      state.set("placeholder", Objects.requireNonNull(value, "placeholder"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public DatePickerElement build() {
      return state.build(DatePickerElement::new);
    }
  }
}
