package io.github.nicklambourne.slackblocks.element;

import com.google.gson.annotations.JsonAdapter;
import com.slack.api.model.block.element.BlockElement;
import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.internal.BuilderState;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import io.github.nicklambourne.slackblocks.internal.WireObjects;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Creates an image element from a URL or Slack-hosted file.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ImageElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ImageElement(Map<String, Object> values) {
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
    return this == other || (other instanceof ImageElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ImageElement}. */
  public static final class Builder implements Buildable<ImageElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ImageElement", "image");

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
     * Sets slack file from a string.
     *
     * @param value value for Slack's {@code slack_file} field
     * @return this builder
     */
    public Builder slackFile(String value) {
      state.set("slack_file", Objects.requireNonNull(value, "slackFile"));
      return this;
    }

    /**
     * Sets Slack's {@code slack_file} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder slackFile(SlackObject value) {
      state.set("slack_file", Objects.requireNonNull(value, "slackFile"));
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
    public ImageElement build() {
      return state.build(ImageElement::new);
    }
  }
}
