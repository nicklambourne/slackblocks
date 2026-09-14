package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.SlackObject;
import io.github.nicklambourne.slackblocks.object.MarkdownText;
import io.github.nicklambourne.slackblocks.object.PlainText;
import io.github.nicklambourne.slackblocks.object.RichTextStyle;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.OptionalDouble;
import java.util.OptionalInt;
import java.util.OptionalLong;
import java.util.Set;
import java.util.function.Function;
import org.jspecify.annotations.Nullable;

/** Reads typed values back from the field snapshot kept by generated value classes. */
public final class TypedFields {
  private static final Set<String> TEXT_KEYS = Set.of("type", "text", "emoji", "verbatim");

  private TypedFields() {}

  /** Returns a field that validation guarantees is present. */
  public static <T> T required(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    return optional(fields, owner, field, type).orElseThrow(() -> missing(owner, field));
  }

  /** Returns an optional field. */
  public static <T> Optional<T> optional(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    Object value = fields.get(field);
    return value == null ? Optional.empty() : Optional.of(cast(value, owner, field, type));
  }

  /** Returns an optional {@code int} field. */
  public static OptionalInt optionalInt(Map<String, Object> fields, String owner, String field) {
    return optional(fields, owner, field, Number.class)
        .map(number -> OptionalInt.of(number.intValue()))
        .orElse(OptionalInt.empty());
  }

  /** Returns an optional {@code long} field. */
  public static OptionalLong optionalLong(Map<String, Object> fields, String owner, String field) {
    return optional(fields, owner, field, Number.class)
        .map(number -> OptionalLong.of(number.longValue()))
        .orElse(OptionalLong.empty());
  }

  /** Returns an optional {@code double} field. */
  public static OptionalDouble optionalDouble(
      Map<String, Object> fields, String owner, String field) {
    return optional(fields, owner, field, Number.class)
        .map(number -> OptionalDouble.of(number.doubleValue()))
        .orElse(OptionalDouble.empty());
  }

  /** Returns a list field, or an empty list when it was never set. */
  public static <T> List<T> list(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    Object value = fields.get(field);
    if (value == null) {
      return List.of();
    }
    List<?> items = cast(value, owner, field, List.class);
    List<T> result = new ArrayList<>(items.size());
    for (Object item : items) {
      result.add(cast(item, owner, field, type));
    }
    return Collections.unmodifiableList(result);
  }

  /** Returns table rows, or an empty list when none were set. */
  public static <T> List<List<T>> rows(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    Object value = fields.get(field);
    if (value == null) {
      return List.of();
    }
    List<List<T>> result = new ArrayList<>();
    for (Object row : cast(value, owner, field, List.class)) {
      List<T> cells = new ArrayList<>();
      for (Object cell : cast(row, owner, field, List.class)) {
        cells.add(cast(cell, owner, field, type));
      }
      result.add(Collections.unmodifiableList(cells));
    }
    return Collections.unmodifiableList(result);
  }

  /** Returns an enum field that validation guarantees is present. */
  public static <E> E requiredEnum(
      Map<String, Object> fields,
      String owner,
      String field,
      Function<String, Optional<E>> fromWireValue) {
    return optionalEnum(fields, owner, field, fromWireValue)
        .orElseThrow(() -> missing(owner, field));
  }

  /** Returns an optional enum field. */
  public static <E> Optional<E> optionalEnum(
      Map<String, Object> fields,
      String owner,
      String field,
      Function<String, Optional<E>> fromWireValue) {
    Optional<String> wire = optional(fields, owner, field, String.class);
    if (wire.isEmpty()) {
      return Optional.empty();
    }
    return Optional.of(
        fromWireValue
            .apply(wire.get())
            .orElseThrow(() -> incompatible(owner, field, "unknown value " + wire.get())));
  }

  /** Returns a text field that validation guarantees is present. */
  public static <T> T requiredText(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    return optionalText(fields, owner, field, type).orElseThrow(() -> missing(owner, field));
  }

  /** Returns an optional text field, turning strings coerced by the builder into text objects. */
  public static <T> Optional<T> optionalText(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    Object value = fields.get(field);
    return value == null ? Optional.empty() : Optional.of(text(value, owner, field, type));
  }

  /** Returns a list of text objects, or an empty list when none were set. */
  public static <T> List<T> textList(
      Map<String, Object> fields, String owner, String field, Class<T> type) {
    Object value = fields.get(field);
    if (value == null) {
      return List.of();
    }
    List<T> result = new ArrayList<>();
    for (Object item : cast(value, owner, field, List.class)) {
      result.add(text(item, owner, field, type));
    }
    return Collections.unmodifiableList(result);
  }

  /** Returns an optional rich text style, combining individually set style flags. */
  public static Optional<RichTextStyle> optionalStyle(
      Map<String, Object> fields, String owner, String field) {
    Object value = fields.get(field);
    if (value == null || value instanceof RichTextStyle) {
      return Optional.ofNullable((RichTextStyle) value);
    }
    RichTextStyle.Builder style = RichTextStyle.builder();
    Map<?, ?> flags = cast(value, owner, field, Map.class);
    for (Map.Entry<?, ?> flag : flags.entrySet()) {
      boolean enabled = cast(flag.getValue(), owner, field, Boolean.class);
      switch (String.valueOf(flag.getKey())) {
        case "bold" -> style.bold(enabled);
        case "italic" -> style.italic(enabled);
        case "strike" -> style.strike(enabled);
        case "code" -> style.code(enabled);
        case "highlight" -> style.highlight(enabled);
        case "client_highlight" -> style.clientHighlight(enabled);
        case "unlink" -> style.unlink(enabled);
        default -> throw incompatible(owner, field, "unknown style flag " + flag.getKey());
      }
    }
    return Optional.of(style.build());
  }

  /** Returns an optional JSON-compatible map from the value's wire representation. */
  @SuppressWarnings("unchecked") // materialized wire maps always have string keys
  public static Optional<Map<String, Object>> optionalMap(
      Map<String, Object> wire, String owner, String field) {
    return optional(wire, owner, field, Map.class).map(map -> (Map<String, Object>) map);
  }

  private static <T> T text(Object value, String owner, String field, Class<T> type) {
    if (type.isInstance(value)) {
      return type.cast(value);
    }
    if (value instanceof Map<?, ?> map
        && TEXT_KEYS.containsAll(map.keySet())
        && map.get("text") instanceof String text) {
      Object type_ = map.get("type");
      SlackObject built;
      if ("plain_text".equals(type_)) {
        PlainText.Builder builder = PlainText.builder().text(text);
        if (map.get("emoji") instanceof Boolean emoji) {
          builder.emoji(emoji);
        }
        built = builder.build();
      } else if ("mrkdwn".equals(type_)) {
        MarkdownText.Builder builder = MarkdownText.builder().text(text);
        if (map.get("verbatim") instanceof Boolean verbatim) {
          builder.verbatim(verbatim);
        }
        built = builder.build();
      } else {
        throw incompatible(owner, field, "unsupported text type " + type_);
      }
      return cast(built, owner, field, type);
    }
    return cast(value, owner, field, type);
  }

  private static <T> T cast(@Nullable Object value, String owner, String field, Class<T> type) {
    if (!type.isInstance(value)) {
      throw incompatible(
          owner, field, value == null ? "null" : "a " + value.getClass().getName() + " value");
    }
    return type.cast(value);
  }

  private static IllegalStateException missing(String owner, String field) {
    return new IllegalStateException(owner + "." + field + " is not set");
  }

  private static IllegalStateException incompatible(String owner, String field, String detail) {
    return new IllegalStateException(
        owner
            + "."
            + field
            + " holds "
            + detail
            + " that the typed getter cannot return; it was set through a raw wire field, so"
            + " read it with toMap()");
  }
}
