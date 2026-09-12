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
 * Creates a named chart data series.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class DataSeries implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private DataSeries(Map<String, Object> values) {
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
    return this == other || (other instanceof DataSeries that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link DataSeries}. */
  public static final class Builder implements Buildable<DataSeries> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("DataSeries", "");

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
     * Adds values to Slack's {@code data} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder data(SlackObject... values) {
      state.append("data", (Object[]) Objects.requireNonNull(values, "data"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public DataSeries build() {
      return state.build(DataSeries::new);
    }
  }
}
