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
 * Creates a rich-text emoji run.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class RichTextEmoji implements SlackObject {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private RichTextEmoji(Map<String, Object> values) {
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
    return this == other || (other instanceof RichTextEmoji that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link RichTextEmoji}. */
  public static final class Builder implements Buildable<RichTextEmoji> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("RichTextEmoji", "emoji");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets name.
     *
     * @param value value for Slack's {@code name} field
     * @return this builder
     */
    public Builder name(String value) {
      state.set("name", Objects.requireNonNull(value, "name"));
      return this;
    }

    /**
     * Sets skin tone.
     *
     * @param value value for Slack's {@code skin_tone} field
     * @return this builder
     */
    public Builder skinTone(int value) {
      state.set("skin_tone", value);
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public RichTextEmoji build() {
      return state.build(RichTextEmoji::new);
    }
  }
}
