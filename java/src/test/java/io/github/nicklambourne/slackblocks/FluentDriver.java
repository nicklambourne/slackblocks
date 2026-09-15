package io.github.nicklambourne.slackblocks;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import com.google.gson.JsonPrimitive;
import java.lang.reflect.Array;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

/**
 * Builds Slack JSON through the public, typed fluent API described by java/generator/model.json.
 *
 * <p>Every JSON field is set with its named builder method. A field is only set with {@code
 * wireField} when no named method can accept the value; those fallbacks are recorded so the valid
 * corpus can require that there are none.
 */
final class FluentDriver {
  private static final String BASE = "io.github.nicklambourne.slackblocks.";
  private static final Path MODEL = Path.of("generator", "model.json");

  private final Map<String, JsonObject> types = new LinkedHashMap<>();
  private final Map<String, String> packages = new LinkedHashMap<>();
  private final List<String> fallbacks = new ArrayList<>();

  FluentDriver() throws Exception {
    JsonObject model = JsonParser.parseString(Files.readString(MODEL)).getAsJsonObject();
    for (String group : List.of("interfaces", "enums", "types")) {
      for (JsonElement item : model.getAsJsonArray(group)) {
        JsonObject object = item.getAsJsonObject();
        packages.put(object.get("name").getAsString(), object.get("package").getAsString());
        if (group.equals("types")) {
          types.put(object.get("name").getAsString(), object);
        }
      }
    }
    packages.put("RichTextStyle", "object");
  }

  /** Returns the fields that had to be set with {@code wireField}. */
  List<String> fallbacks() {
    return fallbacks;
  }

  Class<?> type(String simpleName) throws ClassNotFoundException {
    String packageName = packages.get(simpleName);
    if (packageName == null) {
      throw new AssertionError("Unknown slackblocks type " + simpleName);
    }
    return Class.forName(BASE + packageName + "." + simpleName);
  }

  /**
   * Builds a JSON value as the given Java type. For typed JSON objects whose wire type maps to
   * several Java types (such as {@code image}), the expected type chooses between them.
   */
  Object build(JsonElement json, Class<?> expected, String path) throws Exception {
    if (json.isJsonArray()) {
      List<Object> result = new ArrayList<>();
      for (JsonElement item : json.getAsJsonArray()) {
        result.add(build(item, Object.class, path + "[]"));
      }
      return result;
    }
    if (!json.isJsonObject()) {
      return scalar(json);
    }
    JsonObject object = json.getAsJsonObject();
    String typeName = resolve(object, expected, path);
    if (typeName == null) {
      return plainMap(object, path);
    }
    JsonObject spec = types.get(typeName);
    Class<?> valueClass = type(typeName);
    Object builder = valueClass.getMethod("builder").invoke(null);
    Class<?> builderClass = builder.getClass();
    String wireType = spec.get("wireType").getAsString();
    for (Map.Entry<String, JsonElement> entry : object.entrySet()) {
      String key = entry.getKey();
      if (key.equals("type")
          && entry.getValue().isJsonPrimitive()
          && entry.getValue().getAsString().equals(wireType)) {
        continue;
      }
      JsonObject field = field(spec, key);
      String fieldPath = typeName + "." + key;
      if (field == null || !setNamed(builder, builderClass, field, entry.getValue(), fieldPath)) {
        if (!(entry.getValue().isJsonArray()
            && entry.getValue().getAsJsonArray().isEmpty()
            && field != null)) {
          fallbacks.add(fieldPath);
        }
        Method wireField = builderClass.getMethod("wireField", String.class, Object.class);
        invoke(wireField, builder, key, build(entry.getValue(), Object.class, fieldPath));
      }
    }
    return invoke(builderClass.getMethod("build"), builder);
  }

  private String resolve(JsonObject object, Class<?> expected, String path) throws Exception {
    if (types.containsKey(expected.getSimpleName()) && expected.getName().startsWith(BASE)) {
      return expected.getSimpleName();
    }
    if (!object.has("type") || !object.get("type").isJsonPrimitive()) {
      return null;
    }
    String wire = object.get("type").getAsString();
    List<String> candidates = new ArrayList<>();
    for (JsonObject spec : types.values()) {
      if (spec.get("wireType").getAsString().equals(wire)
          && expected.isAssignableFrom(type(spec.get("name").getAsString()))) {
        candidates.add(spec.get("name").getAsString());
      }
    }
    if (candidates.size() > 1
        && candidates.contains("ImageBlock")
        && candidates.contains("ImageElement")) {
      candidates.remove(
          path.contains("elements/") || path.startsWith("ImageElement")
              ? "ImageBlock"
              : "ImageElement");
    }
    if (candidates.size() > 1) {
      throw new AssertionError(
          "Ambiguous Java type for " + wire + " at " + path + ": " + candidates);
    }
    return candidates.isEmpty() ? null : candidates.get(0);
  }

  private static JsonObject field(JsonObject spec, String wire) {
    for (JsonElement element : spec.getAsJsonArray("fields")) {
      if (element.getAsJsonObject().get("wire").getAsString().equals(wire)) {
        return element.getAsJsonObject();
      }
    }
    return null;
  }

  /** Sets one field with its named method, returning false when no named method accepts it. */
  private boolean setNamed(
      Object builder, Class<?> builderClass, JsonObject field, JsonElement value, String path)
      throws Exception {
    String method = field.get("method").getAsString();
    String kind = field.get("kind").getAsString();
    Class<?> target = field.has("type") ? type(field.get("type").getAsString()) : Object.class;
    if (value.isJsonArray() && value.getAsJsonArray().isEmpty()) {
      return false;
    }
    switch (kind) {
      case "string" -> {
        return isString(value)
            && call(builder, builderClass, method, String.class, value.getAsString());
      }
      case "boolean" -> {
        return isBoolean(value)
            && call(builder, builderClass, method, boolean.class, value.getAsBoolean());
      }
      case "int" -> {
        return isNumber(value)
            && isWhole(value)
            && call(builder, builderClass, method, int.class, value.getAsInt());
      }
      case "long" -> {
        return isNumber(value)
            && isWhole(value)
            && call(builder, builderClass, method, long.class, value.getAsLong());
      }
      case "double" -> {
        return isNumber(value)
            && call(builder, builderClass, method, double.class, value.getAsDouble());
      }
      case "number" -> {
        if (!isNumber(value)) {
          return false;
        }
        return isWhole(value)
            ? call(builder, builderClass, method, long.class, value.getAsLong())
            : call(builder, builderClass, method, double.class, value.getAsDouble());
      }
      case "stringList" -> {
        if (!value.isJsonArray()) {
          return false;
        }
        List<String> strings = new ArrayList<>();
        for (JsonElement item : value.getAsJsonArray()) {
          if (!isString(item)) {
            return false;
          }
          strings.add(item.getAsString());
        }
        return call(
            builder, builderClass, method, String[].class, (Object) strings.toArray(String[]::new));
      }
      case "enum" -> {
        if (!isString(value)) {
          return false;
        }
        for (Object constant : target.getEnumConstants()) {
          if (target.getMethod("wireValue").invoke(constant).equals(value.getAsString())) {
            return call(builder, builderClass, method, target, constant);
          }
        }
        return false;
      }
      case "map" -> {
        return value.isJsonObject()
            && call(
                builder, builderClass, method, Map.class, plainMap(value.getAsJsonObject(), path));
      }
      case "object" -> {
        Object built = value.isJsonObject() ? build(value, target, path) : null;
        return target.isInstance(built) && call(builder, builderClass, method, target, built);
      }
      case "list" -> {
        if (!value.isJsonArray()) {
          return false;
        }
        Object array = typedArray(value.getAsJsonArray(), target, path);
        return array != null && call(builder, builderClass, method, array.getClass(), array);
      }
      case "rows" -> {
        if (!value.isJsonArray()) {
          return false;
        }
        List<List<Object>> rows = new ArrayList<>();
        for (JsonElement row : value.getAsJsonArray()) {
          if (!row.isJsonArray()) {
            return false;
          }
          List<Object> cells = new ArrayList<>();
          for (JsonElement cell : row.getAsJsonArray()) {
            Object built = build(cell, target, path + "[][]");
            if (!target.isInstance(built)) {
              return false;
            }
            cells.add(built);
          }
          rows.add(List.copyOf(cells));
        }
        return call(
            builder, builderClass, method, List[].class, (Object) rows.toArray(List[]::new));
      }
      case "text" -> {
        String coerce = field.get("coerce").getAsString();
        if (isCoercible(value, coerce)) {
          return call(
              builder,
              builderClass,
              method,
              String.class,
              value.getAsJsonObject().get("text").getAsString());
        }
        Object built = value.isJsonObject() ? build(value, target, path) : null;
        return target.isInstance(built) && call(builder, builderClass, method, target, built);
      }
      case "textList" -> {
        if (!value.isJsonArray()) {
          return false;
        }
        String coerce = field.get("coerce").getAsString();
        List<String> strings = new ArrayList<>();
        for (JsonElement item : value.getAsJsonArray()) {
          if (isCoercible(item, coerce)) {
            strings.add(item.getAsJsonObject().get("text").getAsString());
          }
        }
        if (strings.size() == value.getAsJsonArray().size()) {
          String markdown =
              "markdown" + Character.toUpperCase(method.charAt(0)) + method.substring(1);
          return call(
              builder,
              builderClass,
              markdown,
              String[].class,
              (Object) strings.toArray(String[]::new));
        }
        Object array = typedArray(value.getAsJsonArray(), target, path);
        return array != null && call(builder, builderClass, method, array.getClass(), array);
      }
      case "style" -> {
        if (!value.isJsonObject()) {
          return false;
        }
        List<String> flags = new ArrayList<>();
        field.getAsJsonArray("flags").forEach(flag -> flags.add(flag.getAsString()));
        for (Map.Entry<String, JsonElement> flag : value.getAsJsonObject().entrySet()) {
          if (!flags.contains(flag.getKey()) || !isBoolean(flag.getValue())) {
            return false;
          }
        }
        for (Map.Entry<String, JsonElement> flag : value.getAsJsonObject().entrySet()) {
          call(
              builder,
              builderClass,
              camel(flag.getKey()),
              boolean.class,
              flag.getValue().getAsBoolean());
        }
        return true;
      }
      default -> {
        throw new AssertionError("Unknown field kind " + kind);
      }
    }
  }

  private Object typedArray(JsonArray items, Class<?> target, String path) throws Exception {
    Object array = Array.newInstance(target, items.size());
    for (int index = 0; index < items.size(); index++) {
      Object built = build(items.get(index), target, path + "[" + index + "]");
      if (!target.isInstance(built)) {
        return null;
      }
      Array.set(array, index, built);
    }
    return array;
  }

  private static boolean isCoercible(JsonElement value, String coerce) {
    if (!value.isJsonObject()) {
      return false;
    }
    JsonObject object = value.getAsJsonObject();
    return object.size() == 2
        && object.has("type")
        && isString(object.get("type"))
        && object.get("type").getAsString().equals(coerce)
        && object.has("text")
        && isString(object.get("text"));
  }

  private static boolean call(
      Object builder, Class<?> builderClass, String name, Class<?> parameter, Object argument)
      throws Exception {
    invoke(builderClass.getMethod(name, parameter), builder, argument);
    return true;
  }

  private Map<String, Object> plainMap(JsonObject object, String path) throws Exception {
    Map<String, Object> result = new LinkedHashMap<>();
    for (Map.Entry<String, JsonElement> entry : object.entrySet()) {
      result.put(
          entry.getKey(), build(entry.getValue(), Object.class, path + "." + entry.getKey()));
    }
    return result;
  }

  private static Object scalar(JsonElement json) {
    JsonPrimitive primitive = json.getAsJsonPrimitive();
    if (primitive.isBoolean()) {
      return primitive.getAsBoolean();
    }
    if (primitive.isNumber()) {
      return isWhole(primitive) ? (Object) primitive.getAsLong() : (Object) primitive.getAsDouble();
    }
    return primitive.getAsString();
  }

  private static boolean isString(JsonElement value) {
    return value.isJsonPrimitive() && value.getAsJsonPrimitive().isString();
  }

  private static boolean isBoolean(JsonElement value) {
    return value.isJsonPrimitive() && value.getAsJsonPrimitive().isBoolean();
  }

  private static boolean isNumber(JsonElement value) {
    return value.isJsonPrimitive() && value.getAsJsonPrimitive().isNumber();
  }

  private static boolean isWhole(JsonElement value) {
    String lexeme = value.getAsNumber().toString();
    return !lexeme.contains(".") && !lexeme.contains("e") && !lexeme.contains("E");
  }

  private static String camel(String snake) {
    StringBuilder result = new StringBuilder();
    boolean upper = false;
    for (int index = 0; index < snake.length(); index++) {
      char character = snake.charAt(index);
      if (character == '_') {
        upper = true;
      } else {
        result.append(upper ? Character.toUpperCase(character) : character);
        upper = false;
      }
    }
    return result.toString();
  }

  static Object invoke(Method method, Object receiver, Object... arguments) throws Exception {
    try {
      return method.invoke(receiver, arguments);
    } catch (InvocationTargetException error) {
      Throwable cause = error.getCause();
      if (cause instanceof Exception exception) {
        throw exception;
      }
      if (cause instanceof Error fatal) {
        throw fatal;
      }
      throw error;
    }
  }

  /**
   * Renders JSON with sorted keys and numbers exactly as written, so {@code 42} and {@code 42.0}
   * compare as different values.
   */
  static String canonical(JsonElement json) {
    if (json.isJsonObject()) {
      Map<String, String> sorted = new TreeMap<>();
      json.getAsJsonObject()
          .entrySet()
          .forEach(entry -> sorted.put(entry.getKey(), canonical(entry.getValue())));
      StringBuilder result = new StringBuilder("{");
      sorted.forEach(
          (key, value) ->
              result.append(new JsonPrimitive(key)).append(':').append(value).append(','));
      if (result.length() > 1) {
        result.setLength(result.length() - 1);
      }
      return result.append('}').toString();
    }
    if (json.isJsonArray()) {
      List<String> items = new ArrayList<>();
      json.getAsJsonArray().forEach(item -> items.add(canonical(item)));
      return "[" + String.join(",", items) + "]";
    }
    if (json.isJsonPrimitive() && json.getAsJsonPrimitive().isNumber()) {
      return json.getAsNumber().toString();
    }
    return json.toString();
  }
}
