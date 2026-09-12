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
 * Creates a customizable workflow input parameter.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class InputParameter implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private InputParameter(Map<String, Object> values) {
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
    return this == other || (other instanceof InputParameter that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link InputParameter}. */
  public static final class Builder implements Buildable<InputParameter> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("InputParameter", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets name.
     *
     * @param value value for Slack's {@code name} field
     * @return this builder
     */
    public Builder name(String value) {
      state.set("name", Objects.requireNonNull(value, "name"));
      return this;
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
    public InputParameter build() {
      return state.build(InputParameter::new);
    }
  }
}
