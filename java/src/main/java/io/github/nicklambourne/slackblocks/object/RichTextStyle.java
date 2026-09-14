package io.github.nicklambourne.slackblocks.object;

import com.google.gson.annotations.JsonAdapter;
import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;

/**
 * Formatting flags for inline rich text elements, reusable across elements.
 *
 * <p>Text and links support bold, italic, strike, and code. Channel, user, and user group mentions
 * support bold, italic, strike, highlight, client highlight, and unlink. Each rich text builder
 * also offers the individual flag methods it supports.
 *
 * <pre>{@code
 * RichTextStyle emphasis = RichTextStyle.builder().bold(true).italic(true).build();
 * RichTextText.builder().text("Deploy now").style(emphasis).build();
 * }</pre>
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit/blocks/rich-text-block">Slack
 *     reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class RichTextStyle implements SlackObject {
  private final Map<String, Object> values;

  private RichTextStyle(Map<String, Object> values) {
    this.values = values;
  }

  /**
   * Starts a new builder.
   *
   * @return a new builder
   */
  public static Builder builder() {
    return new Builder();
  }

  @Override
  public Map<String, Object> toMap() {
    return values;
  }

  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof RichTextStyle that && values.equals(that.values));
  }

  @Override
  public int hashCode() {
    return values.hashCode();
  }

  /**
   * Returns this style's compact Slack JSON.
   *
   * @return compact JSON
   */
  @Override
  public String toString() {
    return toJson();
  }

  /** Fluent builder for {@link RichTextStyle}. */
  public static final class Builder implements Buildable<RichTextStyle> {
    private final Map<String, Object> values = new LinkedHashMap<>();

    private Builder() {}

    /**
     * Sets whether text is bold.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder bold(boolean value) {
      values.put("bold", value);
      return this;
    }

    /**
     * Sets whether text is italic.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder italic(boolean value) {
      values.put("italic", value);
      return this;
    }

    /**
     * Sets whether text is struck through.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder strike(boolean value) {
      values.put("strike", value);
      return this;
    }

    /**
     * Sets whether text or a link is shown as inline code.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder code(boolean value) {
      values.put("code", value);
      return this;
    }

    /**
     * Sets whether a mention is highlighted.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder highlight(boolean value) {
      values.put("highlight", value);
      return this;
    }

    /**
     * Sets whether a mention uses the client's highlight color.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder clientHighlight(boolean value) {
      values.put("client_highlight", value);
      return this;
    }

    /**
     * Sets whether a mention is shown without a link.
     *
     * @param value whether to apply the style
     * @return this builder
     */
    public Builder unlink(boolean value) {
      values.put("unlink", value);
      return this;
    }

    /**
     * Creates an immutable style. A style with no flags serializes as an empty object.
     *
     * @return an immutable style
     */
    @Override
    public RichTextStyle build() {
      return new RichTextStyle(Collections.unmodifiableMap(new LinkedHashMap<>(values)));
    }
  }
}
