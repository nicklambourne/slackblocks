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
 * Creates a legacy secondary message attachment.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class Attachment implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private Attachment(Map<String, Object> values) {
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
    return this == other || (other instanceof Attachment that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link Attachment}. */
  public static final class Builder implements Buildable<Attachment> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Attachment", "");

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
     * Sets color.
     *
     * @param value value for Slack's {@code color} field
     * @return this builder
     */
    public Builder color(String value) {
      state.set("color", Objects.requireNonNull(value, "color"));
      return this;
    }

    /**
     * Sets fallback.
     *
     * @param value value for Slack's {@code fallback} field
     * @return this builder
     */
    public Builder fallback(String value) {
      state.set("fallback", Objects.requireNonNull(value, "fallback"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public Attachment build() {

      Object color = state.get("color");
      if (color instanceof String text
          && text.length() == 6
          && !text.equals("danger")
          && !text.equals("good")
          && !text.equals("warning")) {
        state.set("color", "#" + text);
      }
      return state.build(Attachment::new);
    }
  }
}
