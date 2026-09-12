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
 * Creates an embedded video block.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class VideoBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private VideoBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof VideoBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link VideoBlock}. */
  public static final class Builder implements Buildable<VideoBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("VideoBlock", "video");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

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
     * Sets thumbnail url.
     *
     * @param value value for Slack's {@code thumbnail_url} field
     * @return this builder
     */
    public Builder thumbnailUrl(String value) {
      state.set("thumbnail_url", Objects.requireNonNull(value, "thumbnailUrl"));
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
     * Sets video url.
     *
     * @param value value for Slack's {@code video_url} field
     * @return this builder
     */
    public Builder videoUrl(String value) {
      state.set("video_url", Objects.requireNonNull(value, "videoUrl"));
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

    /**
     * Sets author name.
     *
     * @param value value for Slack's {@code author_name} field
     * @return this builder
     */
    public Builder authorName(String value) {
      state.set("author_name", Objects.requireNonNull(value, "authorName"));
      return this;
    }

    /**
     * Sets description from a string.
     *
     * @param value value for Slack's {@code description} field
     * @return this builder
     */
    public Builder description(String value) {
      state.set("description", WireObjects.text("plain_text", Objects.requireNonNull(value, "description")));
      return this;
    }

    /**
     * Sets Slack's {@code description} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder description(SlackObject value) {
      state.set("description", Objects.requireNonNull(value, "description"));
      return this;
    }

    /**
     * Sets provider icon url.
     *
     * @param value value for Slack's {@code provider_icon_url} field
     * @return this builder
     */
    public Builder providerIconUrl(String value) {
      state.set("provider_icon_url", Objects.requireNonNull(value, "providerIconUrl"));
      return this;
    }

    /**
     * Sets provider name.
     *
     * @param value value for Slack's {@code provider_name} field
     * @return this builder
     */
    public Builder providerName(String value) {
      state.set("provider_name", Objects.requireNonNull(value, "providerName"));
      return this;
    }

    /**
     * Sets title url.
     *
     * @param value value for Slack's {@code title_url} field
     * @return this builder
     */
    public Builder titleUrl(String value) {
      state.set("title_url", Objects.requireNonNull(value, "titleUrl"));
      return this;
    }
    /** {@inheritDoc} */
    @Override
    public VideoBlock build() {
      return state.build(VideoBlock::new);
    }
  }
}
