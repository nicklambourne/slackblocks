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
 * Creates a conversation select-menu filter.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ConversationFilter implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ConversationFilter(Map<String, Object> values) {
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
    return this == other || (other instanceof ConversationFilter that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ConversationFilter}. */
  public static final class Builder implements Buildable<ConversationFilter> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ConversationFilter", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code include} field in order.
     *
     * @param values values to append
     * @return this builder
     */
    public Builder include(String... values) {
      state.append("include", (Object[]) Objects.requireNonNull(values, "include"));
      return this;
    }

    /**
     * Sets whether exclude external shared channels is enabled.
     *
     * @param value value for Slack's {@code exclude_external_shared_channels} field
     * @return this builder
     */
    public Builder excludeExternalSharedChannels(boolean value) {
      state.set("exclude_external_shared_channels", value);
      return this;
    }

    /**
     * Sets whether exclude bot users is enabled.
     *
     * @param value value for Slack's {@code exclude_bot_users} field
     * @return this builder
     */
    public Builder excludeBotUsers(boolean value) {
      state.set("exclude_bot_users", value);
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
    public ConversationFilter build() {
      return state.build(ConversationFilter::new);
    }
  }
}
