package io.github.nicklambourne.slackblocks.block;

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
 * Creates an image block with optional title text.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ImageBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ImageBlock(Map<String, Object> values) {
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

  /**
   * Starts an image builder with its required URL and alternative text.
   *
   * @param imageUrl public image URL
   * @param altText accessible image description
   * @return a new builder
   */
  public static Builder builder(String imageUrl, String altText) {
    return builder().imageUrl(imageUrl).altText(altText);
  }
  /** {@inheritDoc} */
  @Override
  public Map<String, Object> toMap() {
    return WireObjects.materialize(values);
  }

  /** {@inheritDoc} */
  @Override
  public String getType() {
    return (String) values.get("type");
  }

  /** {@inheritDoc} */
  @Override
  public String getBlockId() {
    return (String) values.get("block_id");
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof ImageBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ImageBlock}. */
  public static final class Builder implements Buildable<ImageBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ImageBlock", "image");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets image url.
     *
     * @param value value for Slack's {@code image_url} field
     * @return this builder
     */
    public Builder imageUrl(String value) {
      state.set("image_url", Objects.requireNonNull(value, "imageUrl"));
      return this;
    }

    /**
     * Sets alt text.
     *
     * @param value value for Slack's {@code alt_text} field
     * @return this builder
     */
    public Builder altText(String value) {
      state.set("alt_text", Objects.requireNonNull(value, "altText"));
      return this;
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
     * Sets block id.
     *
     * @param value value for Slack's {@code block_id} field
     * @return this builder
     */
    public Builder blockId(String value) {
      state.set("block_id", Objects.requireNonNull(value, "blockId"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public ImageBlock build() {
      return state.build(ImageBlock::new);
    }
  }
}
