package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** Internal conversion and defensive-copy helpers. */
public final class WireObjects {
  private WireObjects() {}

  /** Creates one immutable Slack text-object map. */
  public static Map<String, Object> text(String type, String text) {
    Map<String, Object> result = new LinkedHashMap<>();
    result.put("type", type);
    result.put("text", text);
    return Collections.unmodifiableMap(result);
  }

  /**
   * Converts nested builders and Slack values into a deeply unmodifiable tree of JSON-compatible
   * maps, lists, and scalars.
   */
  public static Map<String, Object> materialize(Map<String, ?> values) {
    Map<String, Object> result = new LinkedHashMap<>();
    values.forEach((key, value) -> result.put(key, materializeValue(value)));
    return Collections.unmodifiableMap(result);
  }

  private static Object materializeValue(Object value) {
    if (value instanceof Buildable<?> buildable) {
      return buildable.build().toMap();
    }
    if (value instanceof SlackObject slackObject) {
      return slackObject.toMap();
    }
    if (value instanceof List<?> list) {
      List<Object> result = new ArrayList<>(list.size());
      list.forEach(item -> result.add(materializeValue(item)));
      return Collections.unmodifiableList(result);
    }
    if (value instanceof Map<?, ?> map) {
      Map<String, Object> result = new LinkedHashMap<>();
      map.forEach((key, nested) -> result.put(String.valueOf(key), materializeValue(nested)));
      return Collections.unmodifiableMap(result);
    }
    return value;
  }
}
