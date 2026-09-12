package io.github.nicklambourne.slackblocks.block;

import com.slack.api.model.block.LayoutBlock;
import io.github.nicklambourne.slackblocks.SlackObject;
import org.jspecify.annotations.Nullable;

/** A validated Slack layout block accepted directly by Slack's official Java SDK. */
public interface Block extends LayoutBlock, SlackObject {
  /** {@inheritDoc} */
  @Override
  default String getType() {
    return (String) toMap().get("type");
  }

  /** {@inheritDoc} */
  @Override
  @Nullable
  default String getBlockId() {
    return (String) toMap().get("block_id");
  }
}
