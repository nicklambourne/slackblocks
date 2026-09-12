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
 * Creates a payload for chat.postMessage and related Web API methods.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class MessagePayload implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private MessagePayload(Map<String, Object> values) {
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
    return this == other || (other instanceof MessagePayload that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link MessagePayload}. */
  public static final class Builder implements Buildable<MessagePayload> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Message", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {
      state.set("mrkdwn", true);
      state.set("text", "");
    }

    /**
     * Sets channel.
     *
     * @param value value for Slack's {@code channel} field
     * @return this builder
     */
    public Builder channel(String value) {
      state.set("channel", Objects.requireNonNull(value, "channel"));
      return this;
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
     * Adds values to Slack's {@code attachments} field in order.
     *
     * @param values Slack values to append
     * @return this builder
     */
    public Builder attachments(SlackObject... values) {
      state.append("attachments", (Object[]) Objects.requireNonNull(values, "attachments"));
      return this;
    }

    /**
     * Sets text from a string.
     *
     * @param value value for Slack's {@code text} field
     * @return this builder
     */
    public Builder text(String value) {
      state.set("text", Objects.requireNonNull(value, "text"));
      return this;
    }

    /**
     * Sets Slack's {@code text} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder text(SlackObject value) {
      state.set("text", Objects.requireNonNull(value, "text"));
      return this;
    }

    /**
     * Sets whether mrkdwn is enabled.
     *
     * @param value value for Slack's {@code mrkdwn} field
     * @return this builder
     */
    public Builder mrkdwn(boolean value) {
      state.set("mrkdwn", value);
      return this;
    }

    /**
     * Sets whether unfurl links is enabled.
     *
     * @param value value for Slack's {@code unfurl_links} field
     * @return this builder
     */
    public Builder unfurlLinks(boolean value) {
      state.set("unfurl_links", value);
      return this;
    }

    /**
     * Sets whether unfurl media is enabled.
     *
     * @param value value for Slack's {@code unfurl_media} field
     * @return this builder
     */
    public Builder unfurlMedia(boolean value) {
      state.set("unfurl_media", value);
      return this;
    }

    /**
     * Sets metadata from a string.
     *
     * @param value value for Slack's {@code metadata} field
     * @return this builder
     */
    public Builder metadata(String value) {
      state.set("metadata", Objects.requireNonNull(value, "metadata"));
      return this;
    }

    /**
     * Sets Slack's {@code metadata} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder metadata(SlackObject value) {
      state.set("metadata", Objects.requireNonNull(value, "metadata"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public MessagePayload build() {
      return state.build(MessagePayload::new);
    }
  }
}
