package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.SlackObject;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;

/** Mutable state composed by generated concrete builders. */
public final class BuilderState {
  private final String name;
  private final Map<String, Object> values = new LinkedHashMap<>();

  /** Creates state for one named Slack object and optional wire discriminator. */
  public BuilderState(String name, String type) {
    this.name = name;
    if (!type.isEmpty()) {
      values.put("type", type);
    }
  }

  /** Sets one wire field. */
  public void set(String field, Object value) {
    values.put(field, value);
  }

  /** Appends values to one ordered wire collection. */
  public void append(String field, Object... additions) {
    @SuppressWarnings("unchecked")
    List<Object> existing =
        values.containsKey(field)
            ? new ArrayList<>((List<Object>) values.get(field))
            : new ArrayList<>();
    for (Object addition : additions) {
      existing.add(addition);
    }
    values.put(field, existing);
  }

  /** Returns a configured field for builder-specific transforms. */
  public Object get(String field) {
    return values.get(field);
  }

  /** Removes a private builder sentinel before materialization. */
  public Object remove(String field) {
    return values.remove(field);
  }

  /** Builds a validated immutable value using the supplied concrete factory. */
  public <T extends SlackObject> T build(Function<Map<String, Object>, T> factory) {
    Map<String, Object> frozen = WireObjects.freeze(values);
    Validator.validate(name, WireObjects.materialize(frozen));
    return factory.apply(frozen);
  }
}
