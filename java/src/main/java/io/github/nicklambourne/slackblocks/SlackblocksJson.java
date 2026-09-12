package io.github.nicklambourne.slackblocks;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

/** JSON serialization helpers for clients that do not use Slack's Java SDK. */
public final class SlackblocksJson {
  private static final Gson GSON = new GsonBuilder().disableHtmlEscaping().create();

  private SlackblocksJson() {}

  /**
   * Serializes a Slackblocks value, collection, or payload to compact JSON.
   *
   * @param value a Slackblocks value or a collection containing Slackblocks values
   * @return compact Slack-compatible JSON
   */
  public static String write(Object value) {
    return GSON.toJson(value);
  }
}
