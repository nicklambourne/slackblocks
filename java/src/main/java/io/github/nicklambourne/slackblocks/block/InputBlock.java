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
 * Creates a labelled form control.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class InputBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private InputBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof InputBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link InputBlock}. */
  public static final class Builder implements Buildable<InputBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("InputBlock", "input");

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
      state.set("label", WireObjects.text("plain_text", Objects.requireNonNull(value, "label")));
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
     * Sets element from a string.
     *
     * @param value value for Slack's {@code element} field
     * @return this builder
     */
    public Builder element(String value) {
      state.set("element", Objects.requireNonNull(value, "element"));
      return this;
    }

    /**
     * Sets Slack's {@code element} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder element(SlackObject value) {
      state.set("element", Objects.requireNonNull(value, "element"));
      return this;
    }

    /**
     * Sets whether dispatch action is enabled.
     *
     * @param value value for Slack's {@code dispatch_action} field
     * @return this builder
     */
    public Builder dispatchAction(boolean value) {
      state.set("dispatch_action", value);
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
     * Sets hint from a string.
     *
     * @param value value for Slack's {@code hint} field
     * @return this builder
     */
    public Builder hint(String value) {
      state.set("hint", WireObjects.text("plain_text", Objects.requireNonNull(value, "hint")));
      return this;
    }

    /**
     * Sets Slack's {@code hint} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder hint(SlackObject value) {
      state.set("hint", Objects.requireNonNull(value, "hint"));
      return this;
    }

    /**
     * Sets whether optional is enabled.
     *
     * @param value value for Slack's {@code optional} field
     * @return this builder
     */
    public Builder optional(boolean value) {
      state.set("optional", value);
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public InputBlock build() {
      return state.build(InputBlock::new);
    }
  }
}
