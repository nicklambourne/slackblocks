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
 * Creates a labelled group of selectable options.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class OptionGroup implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private OptionGroup(Map<String, Object> values) {
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
    return this == other || (other instanceof OptionGroup that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link OptionGroup}. */
  public static final class Builder implements Buildable<OptionGroup> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("OptionGroup", "");

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
     * Adds values to Slack's {@code options} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder options(SlackObject... values) {
      state.append("options", (Object[]) Objects.requireNonNull(values, "options"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public OptionGroup build() {
      return state.build(OptionGroup::new);
    }
  }
}
