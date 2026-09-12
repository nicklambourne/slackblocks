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
 * Creates a numeric input.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class NumberInputElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private NumberInputElement(Map<String, Object> values) {
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
    return this == other || (other instanceof NumberInputElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link NumberInputElement}. */
  public static final class Builder implements Buildable<NumberInputElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("NumberInput", "number_input");

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
     * Sets whether is decimal allowed is enabled.
     *
     * @param value value for Slack's {@code is_decimal_allowed} field
     * @return this builder
     */
    public Builder isDecimalAllowed(boolean value) {
      state.set("is_decimal_allowed", value);
      return this;
    }

    /**
     * Sets initial value from a string.
     *
     * @param value value for Slack's {@code initial_value} field
     * @return this builder
     */
    public Builder initialValue(String value) {
      state.set("initial_value", Objects.requireNonNull(value, "initialValue"));
      return this;
    }

    /**
     * Sets Slack's {@code initial_value} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder initialValue(SlackObject value) {
      state.set("initial_value", Objects.requireNonNull(value, "initialValue"));
      return this;
    }

    /**
     * Sets min value.
     *
     * @param value value for Slack's {@code min_value} field
     * @return this builder
     */
    public Builder minValue(double value) {
      state.set("min_value", value);
      return this;
    }

    /**
     * Sets max value.
     *
     * @param value value for Slack's {@code max_value} field
     * @return this builder
     */
    public Builder maxValue(double value) {
      state.set("max_value", value);
      return this;
    }

    /**
     * Sets dispatch action config from a string.
     *
     * @param value value for Slack's {@code dispatch_action_config} field
     * @return this builder
     */
    public Builder dispatchActionConfig(String value) {
      state.set("dispatch_action_config", Objects.requireNonNull(value, "dispatchActionConfig"));
      return this;
    }

    /**
     * Sets Slack's {@code dispatch_action_config} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder dispatchActionConfig(SlackObject value) {
      state.set("dispatch_action_config", Objects.requireNonNull(value, "dispatchActionConfig"));
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
    public NumberInputElement build() {
      return state.build(NumberInputElement::new);
    }
  }
}
