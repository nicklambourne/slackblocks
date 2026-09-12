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
 * Creates a multi-select of conversations.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class ConversationMultiSelectElement extends BlockElement implements Element {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private ConversationMultiSelectElement(Map<String, Object> values) {
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
    return this == other || (other instanceof ConversationMultiSelectElement that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link ConversationMultiSelectElement}. */
  public static final class Builder implements Buildable<ConversationMultiSelectElement> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("ConversationMultiSelect", "multi_conversations_select");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {

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
     * Adds values to Slack's {@code initial_conversations} field in order.
     *
     * @param values values to append
     * @return this builder
     */
    public Builder initialConversations(String... values) {
      state.append("initial_conversations", (Object[]) Objects.requireNonNull(values, "initialConversations"));
      return this;
    }

    /**
     * Sets whether default to current conversation is enabled.
     *
     * @param value value for Slack's {@code default_to_current_conversation} field
     * @return this builder
     */
    public Builder defaultToCurrentConversation(boolean value) {
      state.set("default_to_current_conversation", value);
      return this;
    }

    /**
     * Sets filter from a string.
     *
     * @param value value for Slack's {@code filter} field
     * @return this builder
     */
    public Builder filter(String value) {
      state.set("filter", Objects.requireNonNull(value, "filter"));
      return this;
    }

    /**
     * Sets Slack's {@code filter} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder filter(SlackObject value) {
      state.set("filter", Objects.requireNonNull(value, "filter"));
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
     * Sets max selected items.
     *
     * @param value value for Slack's {@code max_selected_items} field
     * @return this builder
     */
    public Builder maxSelectedItems(int value) {
      state.set("max_selected_items", value);
      return this;
    }

    /**
     * Sets whether focus on load is enabled.
     *
     * @param value value for Slack's {@code focus_on_load} field
     * @return this builder
     */
    public Builder focusOnLoad(boolean value) {
      state.set("focus_on_load", value);
      return this;
    }

    /**
     * Sets placeholder from a string.
     *
     * @param value value for Slack's {@code placeholder} field
     * @return this builder
     */
    public Builder placeholder(String value) {
      state.set("placeholder", WireObjects.text("plain_text", Objects.requireNonNull(value, "placeholder")));
      return this;
    }

    /**
     * Sets Slack's {@code placeholder} field from a typed Slack value.
     *
     * @param value nested Slack value
     * @return this builder
     */
    public Builder placeholder(SlackObject value) {
      state.set("placeholder", Objects.requireNonNull(value, "placeholder"));
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
    public ConversationMultiSelectElement build() {
      return state.build(ConversationMultiSelectElement::new);
    }
  }
}
