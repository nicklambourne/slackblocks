package io.github.nicklambourne.slackblocks;

import java.util.Map;

/** An immutable value that can be serialized to Slack's JSON wire format. */
public interface SlackObject {
  /**
   * Returns an immutable, JSON-compatible representation of this value.
   *
   * @return the Slack wire representation
   */
  Map<String, Object> toMap();

  /**
   * Serializes this value to compact Slack-compatible JSON.
   *
   * @return compact JSON containing this value
   */
  default String toJson() {
    return SlackblocksJson.write(this);
  }
}
