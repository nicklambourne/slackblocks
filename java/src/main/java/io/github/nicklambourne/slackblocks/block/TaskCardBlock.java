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
 * Creates one task in a plan.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class TaskCardBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private TaskCardBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof TaskCardBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link TaskCardBlock}. */
  public static final class Builder implements Buildable<TaskCardBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("TaskCardBlock", "task_card");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets task id.
     *
     * @param value value for Slack's {@code task_id} field
     * @return this builder
     */
    public Builder taskId(String value) {
      state.set("task_id", Objects.requireNonNull(value, "taskId"));
      return this;
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
     * Sets details from a string.
     *
     * @param value value for Slack's {@code details} field
     * @return this builder
     */
    public Builder details(String value) {
      state.set("details", Objects.requireNonNull(value, "details"));
      return this;
    }

    /**
     * Sets Slack's {@code details} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder details(SlackObject value) {
      state.set("details", Objects.requireNonNull(value, "details"));
      return this;
    }

    /**
     * Sets output from a string.
     *
     * @param value value for Slack's {@code output} field
     * @return this builder
     */
    public Builder output(String value) {
      state.set("output", Objects.requireNonNull(value, "output"));
      return this;
    }

    /**
     * Sets Slack's {@code output} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder output(SlackObject value) {
      state.set("output", Objects.requireNonNull(value, "output"));
      return this;
    }

    /**
     * Adds values to Slack's {@code sources} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder sources(SlackObject... values) {
      state.append("sources", (Object[]) Objects.requireNonNull(values, "sources"));
      return this;
    }

    /**
     * Sets status.
     *
     * @param value value for Slack's {@code status} field
     * @return this builder
     */
    public Builder status(String value) {
      state.set("status", Objects.requireNonNull(value, "status"));
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
    public TaskCardBlock build() {
      return state.build(TaskCardBlock::new);
    }
  }
}
