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
 * Creates table-column display settings.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ColumnSettings implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ColumnSettings(Map<String, Object> values) {
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
    return this == other || (other instanceof ColumnSettings that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ColumnSettings}. */
  public static final class Builder implements Buildable<ColumnSettings> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ColumnSettings", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets align.
     *
     * @param value value for Slack's {@code align} field
     * @return this builder
     */
    public Builder align(String value) {
      state.set("align", Objects.requireNonNull(value, "align"));
      return this;
    }

    /**
     * Sets whether is wrapped is enabled.
     *
     * @param value value for Slack's {@code is_wrapped} field
     * @return this builder
     */
    public Builder isWrapped(boolean value) {
      state.set("is_wrapped", value);
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public ColumnSettings build() {
      return state.build(ColumnSettings::new);
    }
  }
}
