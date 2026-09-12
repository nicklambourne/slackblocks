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
 * Creates an immediate interaction-response payload.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class MessageResponse implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private MessageResponse(Map<String, Object> values) {
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
    return this == other || (other instanceof MessageResponse that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link MessageResponse}. */
  public static final class Builder implements Buildable<MessageResponse> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("MessageResponse", "");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {
      state.set("mrkdwn", true);
      state.set("text", "");
      state.set("replace_original", false);
      state.set("response_type", "in_channel");
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
     * Sets whether replace original is enabled.
     *
     * @param value value for Slack's {@code replace_original} field
     * @return this builder
     */
    public Builder replaceOriginal(boolean value) {
      state.set("replace_original", value);
      return this;
    }

    /**
     * Sets response type.
     *
     * @param value value for Slack's {@code response_type} field
     * @return this builder
     */
    public Builder responseType(String value) {
      state.set("response_type", Objects.requireNonNull(value, "responseType"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public MessageResponse build() {
      return state.build(MessageResponse::new);
    }
  }
}
