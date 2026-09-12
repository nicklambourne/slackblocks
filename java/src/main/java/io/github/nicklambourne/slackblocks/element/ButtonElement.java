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
 * Creates an interactive button.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ButtonElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ButtonElement(Map<String, Object> values) {
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
   * Starts a button builder with its required label and action identifier.
   *
   * @param text button label
   * @param actionId identifier returned in interaction payloads
   * @return a new builder
   */
  public static Builder builder(String text, String actionId) {
    return builder().text(text).actionId(actionId);
  }
  /** {@inheritDoc} */
  @Override
  public Map<String, Object> toMap() {
    return WireObjects.materialize(values);
  }

  /** {@inheritDoc} */
  @Override
  public boolean equals(Object other) {
    return this == other || (other instanceof ButtonElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ButtonElement}. */
  public static final class Builder implements Buildable<ButtonElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("Button", "button");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

    }

    /**
     * Sets text from a string.
     *
     * @param value value for Slack's {@code text} field
     * @return this builder
     */
    public Builder text(String value) {
      state.set("text", WireObjects.text("plain_text", Objects.requireNonNull(value, "text")));
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
     * Sets the text using plain text.
     *
     * @param value text content
     * @return this builder
     */
    public Builder plainText(String value) {
      return text(value);
    }

    /**
     * Sets action id.
     *
     * @param value value for Slack's {@code action_id} field
     * @return this builder
     */
    public Builder actionId(String value) {
      state.set("action_id", Objects.requireNonNull(value, "actionId"));
      return this;
    }

    /**
     * Sets url.
     *
     * @param value value for Slack's {@code url} field
     * @return this builder
     */
    public Builder url(String value) {
      state.set("url", Objects.requireNonNull(value, "url"));
      return this;
    }

    /**
     * Sets the application-defined value.
     *
     * @param value value for Slack's {@code value} field
     * @return this builder
     */
    public Builder value(String value) {
      state.set("value", Objects.requireNonNull(value, "value"));
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

    /**
     * Sets confirm from a string.
     *
     * @param value value for Slack's {@code confirm} field
     * @return this builder
     */
    public Builder confirm(String value) {
      state.set("confirm", Objects.requireNonNull(value, "confirm"));
      return this;
    }

    /**
     * Sets Slack's {@code confirm} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder confirm(SlackObject value) {
      state.set("confirm", Objects.requireNonNull(value, "confirm"));
      return this;
    }

    /**
     * Sets accessibility label.
     *
     * @param value value for Slack's {@code accessibility_label} field
     * @return this builder
     */
    public Builder accessibilityLabel(String value) {
      state.set("accessibility_label", Objects.requireNonNull(value, "accessibilityLabel"));
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
    public ButtonElement build() {
      return state.build(ButtonElement::new);
    }
  }
}
