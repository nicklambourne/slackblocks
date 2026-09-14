package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.Buildable;
import io.github.nicklambourne.slackblocks.SlackObject;
import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

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

  /**
   * Returns copies of nested objects without their {@code type} discriminator, for fields whose
   * items Slack expects untyped.
   */
  public static List<Object> withoutType(List<?> items) {
    List<Object> result = new ArrayList<>(items.size());
    for (Object item : items) {
      Object materialized = materializeValue(Objects.requireNonNull(item, "item"));
      if (materialized instanceof Map<?, ?> map) {
        Map<String, Object> copy = new LinkedHashMap<>();
        map.forEach(
            (key, nested) -> {
              if (!"type".equals(key)) {
                copy.put(String.valueOf(key), nested);
              }
            });
        result.add(Collections.unmodifiableMap(copy));
      } else {
        result.add(materialized);
      }
    }
    return result;
  }

  /**
   * Checks that a value can be written as Slack JSON: a string, number, boolean, slackblocks value
   * or builder, or a list or map containing only such values.
   *
   * @param name the builder method or field being set, used in the error message
   * @param value the value to check
   * @return the value, for fluent use
   * @throws IllegalArgumentException if the value, or anything nested in it, cannot be serialized
   */
  public static <T> T requireWireValue(String name, T value) {
    checkWireValue(name, Objects.requireNonNull(value, name));
    return value;
  }

  private static void checkWireValue(String path, Object value) {
    if (value instanceof String
        || value instanceof Boolean
        || value instanceof Number
        || value instanceof SlackObject
        || value instanceof Buildable<?>) {
      return;
    }
    if (value instanceof List<?> list) {
      for (int index = 0; index < list.size(); index++) {
        Object item = list.get(index);
        checkWireValue(
            path + "[" + index + "]", Objects.requireNonNull(item, path + "[" + index + "]"));
      }
      return;
    }
    if (value instanceof Map<?, ?> map) {
      for (Map.Entry<?, ?> entry : map.entrySet()) {
        if (!(entry.getKey() instanceof String key)) {
          throw new IllegalArgumentException(path + ": map keys must be strings");
        }
        checkWireValue(
            path + "." + key, Objects.requireNonNull(entry.getValue(), path + "." + key));
      }
      return;
    }
    throw new IllegalArgumentException(
        path
            + ": "
            + value.getClass().getName()
            + " cannot be written as Slack JSON; use a string, number, boolean, list, map, or"
            + " slackblocks value");
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
