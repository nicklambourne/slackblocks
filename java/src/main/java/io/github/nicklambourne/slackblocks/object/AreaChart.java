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
 * Creates an area-chart object.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class AreaChart implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private AreaChart(Map<String, Object> values) {
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
    return this == other || (other instanceof AreaChart that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link AreaChart}. */
  public static final class Builder implements Buildable<AreaChart> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("AreaChart", "area");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code series} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder series(SlackObject... values) {
      state.append("series", (Object[]) Objects.requireNonNull(values, "series"));
      return this;
    }

    /**
     * Sets axis config from a string.
     *
     * @param value value for Slack's {@code axis_config} field
     * @return this builder
     */
    public Builder axisConfig(String value) {
      state.set("axis_config", Objects.requireNonNull(value, "axisConfig"));
      return this;
    }

    /**
     * Sets Slack's {@code axis_config} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder axisConfig(SlackObject value) {
      state.set("axis_config", Objects.requireNonNull(value, "axisConfig"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public AreaChart build() {
      return state.build(AreaChart::new);
    }
  }
}
