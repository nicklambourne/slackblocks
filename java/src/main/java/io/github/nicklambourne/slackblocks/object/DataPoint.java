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
 * Creates a labelled chart data point.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class DataPoint implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private DataPoint(Map<String, Object> values) {
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
    return this == other || (other instanceof DataPoint that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link DataPoint}. */
  public static final class Builder implements Buildable<DataPoint> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("DataPoint", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets label from a string.
     *
     * @param value value for Slack's {@code label} field
     * @return this builder
     */
    public Builder label(String value) {
      state.set("label", Objects.requireNonNull(value, "label"));
      return this;
    }

    /**
     * Sets Slack's {@code label} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder label(SlackObject value) {
      state.set("label", Objects.requireNonNull(value, "label"));
      return this;
    }

    /**
     * Sets the numeric value.
     *
     * @param value value for Slack's {@code value} field
     * @return this builder
     */
    public Builder value(double value) {
      state.set("value", value);
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public DataPoint build() {
      return state.build(DataPoint::new);
    }
  }
}
