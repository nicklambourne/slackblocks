package io.github.nicklambourne.slackblocks.payload;

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
 * Creates an App Home tab view payload.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class HomeTabView implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private HomeTabView(Map<String, Object> values) {
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
    return this == other || (other instanceof HomeTabView that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link HomeTabView}. */
  public static final class Builder implements Buildable<HomeTabView> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("HomeTab", "home");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Adds values to Slack's {@code blocks} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder blocks(SlackObject... values) {
      state.append("blocks", (Object[]) Objects.requireNonNull(values, "blocks"));
      return this;
    }

    /**
     * Sets private metadata.
     *
     * @param value value for Slack's {@code private_metadata} field
     * @return this builder
     */
    public Builder privateMetadata(String value) {
      state.set("private_metadata", Objects.requireNonNull(value, "privateMetadata"));
      return this;
    }

    /**
     * Sets callback id.
     *
     * @param value value for Slack's {@code callback_id} field
     * @return this builder
     */
    public Builder callbackId(String value) {
      state.set("callback_id", Objects.requireNonNull(value, "callbackId"));
      return this;
    }

    /**
     * Sets external id.
     *
     * @param value value for Slack's {@code external_id} field
     * @return this builder
     */
    public Builder externalId(String value) {
      state.set("external_id", Objects.requireNonNull(value, "externalId"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public HomeTabView build() {
      return state.build(HomeTabView::new);
    }
  }
}
