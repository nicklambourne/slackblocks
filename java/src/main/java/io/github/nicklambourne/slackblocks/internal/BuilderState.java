package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.SlackObject;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.function.Function;
import org.jspecify.annotations.Nullable;

/** Mutable state composed by generated concrete builders. */
public final class BuilderState {
  private final String name;
  private final Map<String, Object> values = new LinkedHashMap<>();

  /**
   * Creates state for one named Slack object and optional wire discriminator.
   *
   * @param name public Java type name used as the root of validation paths
   * @param type Slack wire {@code type}, or an empty string for untyped objects
   */
  public BuilderState(String name, String type) {
    this.name = name;
    if (!type.isEmpty()) {
      values.put("type", type);
    }
  }

  /** Sets one wire field. */
  public void set(String field, Object value) {
    values.put(field, Objects.requireNonNull(value, field));
  }

  /**
   * Appends values to one ordered wire collection. Appending nothing leaves the field unset, and
   * {@code null} items are rejected.
   */
  public void append(String field, Object... additions) {
    Objects.requireNonNull(additions, field);
    if (additions.length == 0) {
      return;
    }
    List<Object> existing =
        values.get(field) instanceof List<?> list ? new ArrayList<>(list) : new ArrayList<>();
    for (Object addition : additions) {
      int index = existing.size();
      existing.add(Objects.requireNonNull(addition, () -> field + "[" + index + "] is null"));
    }
    values.put(field, existing);
  }

  /** Returns a configured field for builder-specific transforms. */
  public @Nullable Object get(String field) {
    return values.get(field);
  }

  /** Removes a field before materialization. */
  public @Nullable Object remove(String field) {
    return values.remove(field);
  }

  /**
   * Materializes, validates, and wraps one immutable value. Nested builders are built exactly once
   * here, so later changes to them cannot affect the returned value.
   */
  public <T extends SlackObject> T build(Function<Map<String, Object>, T> factory) {
    Map<String, Object> materialized = WireObjects.materialize(values);
    Validator.validate(name, materialized);
    return factory.apply(materialized);
  }
}
