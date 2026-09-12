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
 * Creates a sortable, paginated data table.
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
public final class DataTableBlock implements Block {
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private DataTableBlock(Map<String, Object> values) {
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
    return this == other || (other instanceof DataTableBlock that && toMap().equals(that.toMap()));
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

  /** Concrete fluent builder for {@link DataTableBlock}. */
  public static final class Builder implements Buildable<DataTableBlock> {
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("DataTableBlock", "data_table");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {
      state.set("page_size", 5);
      state.set("row_header_column_index", 0);
    }

    /**
     * Adds one or more complete table rows in display order.
     *
     * @param rows rows whose entries are Slack cell values
     * @return this builder
     */
    @SafeVarargs
    @SuppressWarnings({"varargs", "unchecked"})
    public final Builder rows(List<? extends SlackObject>... rows) {
      state.append("rows", (Object[]) Objects.requireNonNull(rows, "rows"));
      return this;
    }

    /**
     * Sets caption.
     *
     * @param value value for Slack's {@code caption} field
     * @return this builder
     */
    public Builder caption(String value) {
      state.set("caption", Objects.requireNonNull(value, "caption"));
      return this;
    }

    /**
     * Sets page size.
     *
     * @param value value for Slack's {@code page_size} field
     * @return this builder
     */
    public Builder pageSize(int value) {
      state.set("page_size", value);
      return this;
    }

    /**
     * Sets row header column index.
     *
     * @param value value for Slack's {@code row_header_column_index} field
     * @return this builder
     */
    public Builder rowHeaderColumnIndex(int value) {
      state.set("row_header_column_index", value);
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
    public DataTableBlock build() {
      return state.build(DataTableBlock::new);
    }
  }
}
