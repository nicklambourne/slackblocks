package io.github.nicklambourne.slackblocks;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import java.util.Collection;

/** JSON serialization helpers for clients that do not use Slack's Java SDK. */
public final class SlackblocksJson {
  private static final Gson GSON = new GsonBuilder().disableHtmlEscaping().create();

  private SlackblocksJson() {}

  /**
   * Serializes one slackblocks value or payload to compact JSON.
   *
   * @param value a built slackblocks value
   * @return compact Slack-compatible JSON
   */
  public static String write(SlackObject value) {
    return GSON.toJson(value.toMap());
  }

  /**
   * Serializes an ordered collection of slackblocks values, such as a list of blocks, to a compact
   * JSON array.
   *
   * @param values built slackblocks values
   * @return compact Slack-compatible JSON array
   */
  public static String write(Collection<? extends SlackObject> values) {
    return GSON.toJson(values.stream().map(SlackObject::toMap).toList());
  }
}
