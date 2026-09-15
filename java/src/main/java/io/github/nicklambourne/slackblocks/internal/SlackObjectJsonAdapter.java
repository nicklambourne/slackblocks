package io.github.nicklambourne.slackblocks.internal;

import com.google.gson.JsonElement;
import com.google.gson.JsonSerializationContext;
import com.google.gson.JsonSerializer;
import io.github.nicklambourne.slackblocks.SlackObject;
import java.lang.reflect.Type;

/** Serializes map-backed immutable values as their Slack wire representation. */
public final class SlackObjectJsonAdapter implements JsonSerializer<SlackObject> {
  @Override
  public JsonElement serialize(
      SlackObject source, Type sourceType, JsonSerializationContext context) {
    return context.serialize(source.toMap());
  }
}
