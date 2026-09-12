package io.github.nicklambourne.slackblocks.block;

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
 * Creates a Slack-rendered chart block.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class DataVisualizationBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private DataVisualizationBlock(Map<String, Object> values) {
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
  public String getType() {
    return (String) values.get("type");
  }

  /** {@inheritDoc} */
  @Override
  public String getBlockId() {
    return (String) values.get("block_id");
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof DataVisualizationBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link DataVisualizationBlock}. */
  public static final class Builder implements Buildable<DataVisualizationBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("DataVisualizationBlock", "data_visualization");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets title from a string.
     *
     * @param value value for Slack's {@code title} field
     * @return this builder
     */
    public Builder title(String value) {
      state.set("title", Objects.requireNonNull(value, "title"));
      return this;
    }

    /**
     * Sets Slack's {@code title} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder title(SlackObject value) {
      state.set("title", Objects.requireNonNull(value, "title"));
      return this;
    }

    /**
     * Sets chart from a string.
     *
     * @param value value for Slack's {@code chart} field
     * @return this builder
     */
    public Builder chart(String value) {
      state.set("chart", Objects.requireNonNull(value, "chart"));
      return this;
    }

    /**
     * Sets Slack's {@code chart} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder chart(SlackObject value) {
      state.set("chart", Objects.requireNonNull(value, "chart"));
      return this;
    }

    /**
     * Sets block id.
     *
     * @param value value for Slack's {@code block_id} field
     * @return this builder
     */
    public Builder blockId(String value) {
      state.set("block_id", Objects.requireNonNull(value, "blockId"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public DataVisualizationBlock build() {
      return state.build(DataVisualizationBlock::new);
    }
  }
}
