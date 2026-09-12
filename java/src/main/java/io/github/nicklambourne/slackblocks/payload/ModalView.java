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
 * Creates a modal view payload.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ModalView implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ModalView(Map<String, Object> values) {
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
    return this == other || (other instanceof ModalView that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ModalView}. */
  public static final class Builder implements Buildable<ModalView> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Modal", "modal");

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
      state.set("title", WireObjects.text("plain_text", Objects.requireNonNull(value, "title")));
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
     * Sets close from a string.
     *
     * @param value value for Slack's {@code close} field
     * @return this builder
     */
    public Builder close(String value) {
      state.set("close", WireObjects.text("plain_text", Objects.requireNonNull(value, "close")));
      return this;
    }

    /**
     * Sets Slack's {@code close} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder close(SlackObject value) {
      state.set("close", Objects.requireNonNull(value, "close"));
      return this;
    }

    /**
     * Sets submit from a string.
     *
     * @param value value for Slack's {@code submit} field
     * @return this builder
     */
    public Builder submit(String value) {
      state.set("submit", WireObjects.text("plain_text", Objects.requireNonNull(value, "submit")));
      return this;
    }

    /**
     * Sets Slack's {@code submit} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder submit(SlackObject value) {
      state.set("submit", Objects.requireNonNull(value, "submit"));
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
     * Sets whether clear on close is enabled.
     *
     * @param value value for Slack's {@code clear_on_close} field
     * @return this builder
     */
    public Builder clearOnClose(boolean value) {
      state.set("clear_on_close", value);
      return this;
    }

    /**
     * Sets whether notify on close is enabled.
     *
     * @param value value for Slack's {@code notify_on_close} field
     * @return this builder
     */
    public Builder notifyOnClose(boolean value) {
      state.set("notify_on_close", value);
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

    /**
     * Sets whether submit disabled is enabled.
     *
     * @param value value for Slack's {@code submit_disabled} field
     * @return this builder
     */
    public Builder submitDisabled(boolean value) {
      state.set("submit_disabled", value);
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
    public ModalView build() {
      return state.build(ModalView::new);
    }
  }
}
