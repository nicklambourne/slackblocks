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
 * Creates a rich-text channel mention.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class RichTextChannel implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private RichTextChannel(Map<String, Object> values) {
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
    return this == other || (other instanceof RichTextChannel that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link RichTextChannel}. */
  public static final class Builder implements Buildable<RichTextChannel> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("RichTextChannel", "channel");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets channel id.
     *
     * @param value value for Slack's {@code channel_id} field
     * @return this builder
     */
    public Builder channelId(String value) {
      state.set("channel_id", Objects.requireNonNull(value, "channelId"));
      return this;
    }

    /**
     * Sets style from a string.
     *
     * @param value value for Slack's {@code style} field
     * @return this builder
     */
    public Builder style(String value) {
      state.set("style", Objects.requireNonNull(value, "style"));
      return this;
    }

    /**
     * Sets Slack's {@code style} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder style(SlackObject value) {
      state.set("style", Objects.requireNonNull(value, "style"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public RichTextChannel build() {
      return state.build(RichTextChannel::new);
    }
  }
}
