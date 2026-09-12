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
 * Creates a workflow link trigger.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class Trigger implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private Trigger(Map<String, Object> values) {
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
    return this == other || (other instanceof Trigger that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link Trigger}. */
  public static final class Builder implements Buildable<Trigger> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Trigger", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets url.
     *
     * @param value value for Slack's {@code url} field
     * @return this builder
     */
    public Builder url(String value) {
      state.set("url", Objects.requireNonNull(value, "url"));
      return this;
    }

    /**
     * Adds values to Slack's {@code customizable_input_parameters} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder customizableInputParameters(SlackObject... values) {
      state.append("customizable_input_parameters", (Object[]) Objects.requireNonNull(values, "customizableInputParameters"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public Trigger build() {
      return state.build(Trigger::new);
    }
  }
}
