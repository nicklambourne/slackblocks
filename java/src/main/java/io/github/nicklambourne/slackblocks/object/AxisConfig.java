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
 * Creates category and label settings for a chart axis.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class AxisConfig implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private AxisConfig(Map<String, Object> values) {
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
    return this == other || (other instanceof AxisConfig that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link AxisConfig}. */
  public static final class Builder implements Buildable<AxisConfig> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("AxisConfig", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code categories} field in order.
     *
     * @param values values to append
     * @return this builder
     */
    public Builder categories(String... values) {
      state.append("categories", (Object[]) Objects.requireNonNull(values, "categories"));
      return this;
    }

    /**
     * Sets x label.
     *
     * @param value value for Slack's {@code x_label} field
     * @return this builder
     */
    public Builder xLabel(String value) {
      state.set("x_label", Objects.requireNonNull(value, "xLabel"));
      return this;
    }

    /**
     * Sets y label.
     *
     * @param value value for Slack's {@code y_label} field
     * @return this builder
     */
    public Builder yLabel(String value) {
      state.set("y_label", Objects.requireNonNull(value, "yLabel"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public AxisConfig build() {
      return state.build(AxisConfig::new);
    }
  }
}
