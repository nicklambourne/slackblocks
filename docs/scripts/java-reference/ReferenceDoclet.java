import com.sun.source.doctree.AttributeTree;
import com.sun.source.doctree.DocCommentTree;
import com.sun.source.doctree.DocTree;
import com.sun.source.doctree.EndElementTree;
import com.sun.source.doctree.EntityTree;
import com.sun.source.doctree.LinkTree;
import com.sun.source.doctree.LiteralTree;
import com.sun.source.doctree.ParamTree;
import com.sun.source.doctree.ReturnTree;
import com.sun.source.doctree.SeeTree;
import com.sun.source.doctree.StartElementTree;
import com.sun.source.doctree.TextTree;
import com.sun.source.doctree.ThrowsTree;
import com.sun.source.util.DocTreePath;
import com.sun.source.util.DocTrees;
import com.sun.source.util.TreePath;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.stream.Collectors;
import javax.lang.model.SourceVersion;
import javax.lang.model.element.Element;
import javax.lang.model.element.ElementKind;
import javax.lang.model.element.ExecutableElement;
import javax.lang.model.element.Modifier;
import javax.lang.model.element.PackageElement;
import javax.lang.model.element.TypeElement;
import javax.lang.model.element.VariableElement;
import javax.lang.model.util.Elements;
import jdk.javadoc.doclet.Doclet;
import jdk.javadoc.doclet.DocletEnvironment;
import jdk.javadoc.doclet.Reporter;

/**
 * A javadoc doclet that writes the public slackblocks API as JSON for the documentation site.
 *
 * <p>The javadoc tool parses the sources, resolves types and {@code @link} references, and hands
 * each documentation comment to this doclet as a tree. The doclet turns those trees into
 * MDX-safe Markdown, and {@code generate_java_reference.mjs} renders the pages.
 *
 * <p>Links are written as {@code [label](javadoc:TypeName)} and resolved to page anchors by the
 * renderer.
 */
public final class ReferenceDoclet implements Doclet {
  private static final Set<String> OBJECT_METHODS = Set.of("equals", "hashCode", "toString");
  private Path output = Path.of("reference.json");
  private Reporter reporter;

  @Override
  public void init(Locale locale, Reporter reporter) {
    this.reporter = reporter;
  }

  @Override
  public String getName() {
    return "ReferenceDoclet";
  }

  @Override
  public Set<? extends Option> getSupportedOptions() {
    return Set.of(
        new Option() {
          @Override
          public int getArgumentCount() {
            return 1;
          }

          @Override
          public String getDescription() {
            return "path of the JSON file to write";
          }

          @Override
          public Kind getKind() {
            return Kind.STANDARD;
          }

          @Override
          public List<String> getNames() {
            return List.of("-o");
          }

          @Override
          public String getParameters() {
            return "file";
          }

          @Override
          public boolean process(String option, List<String> arguments) {
            output = Path.of(arguments.get(0));
            return true;
          }
        });
  }

  @Override
  public SourceVersion getSupportedSourceVersion() {
    return SourceVersion.latest();
  }

  @Override
  public boolean run(DocletEnvironment environment) {
    DocTrees trees = environment.getDocTrees();
    Elements elements = environment.getElementUtils();
    List<TypeElement> types =
        environment.getIncludedElements().stream()
            .filter(element -> element instanceof TypeElement)
            .map(TypeElement.class::cast)
            .filter(type -> type.getNestingKind().isNested() == false)
            .filter(type -> type.getModifiers().contains(Modifier.PUBLIC))
            .sorted(Comparator.comparing(type -> type.getQualifiedName().toString()))
            .toList();
    StringBuilder json = new StringBuilder("{\"types\":[");
    for (int index = 0; index < types.size(); index++) {
      if (index > 0) {
        json.append(',');
      }
      json.append(type(types.get(index), trees, elements));
    }
    json.append("]}\n");
    try {
      Files.createDirectories(output.toAbsolutePath().getParent());
      Files.writeString(output, json, StandardCharsets.UTF_8);
    } catch (IOException error) {
      reporter.print(javax.tools.Diagnostic.Kind.ERROR, "Cannot write " + output + ": " + error);
      return false;
    }
    return true;
  }

  private String type(TypeElement type, DocTrees trees, Elements elements) {
    PackageElement packageElement = elements.getPackageOf(type);
    Doc doc = new Doc(trees, type, trees.getDocCommentTree(type));
    Map<String, Object> result = new TreeMap<>();
    result.put("name", type.getSimpleName().toString());
    result.put("package", packageElement.getQualifiedName().toString());
    result.put(
        "kind",
        switch (type.getKind()) {
          case INTERFACE -> "interface";
          case ENUM -> "enum";
          case RECORD -> "record";
          default -> "class";
        });
    result.put("doc", doc.body);
    result.put("see", doc.see);
    List<Map<String, Object>> constants = new ArrayList<>();
    List<Map<String, Object>> members = new ArrayList<>();
    for (Element member : type.getEnclosedElements()) {
      if (member.getKind() == ElementKind.ENUM_CONSTANT) {
        Map<String, Object> constant = new TreeMap<>();
        constant.put("name", member.getSimpleName().toString());
        constant.put("wire", enumArgument(trees, member));
        constant.put("doc", new Doc(trees, member, trees.getDocCommentTree(member)).body);
        constants.add(constant);
      }
    }
    members.addAll(methods(type, "", trees, elements));
    for (Element nested : type.getEnclosedElements()) {
      if (nested instanceof TypeElement builder
          && builder.getModifiers().contains(Modifier.PUBLIC)
          && builder.getSimpleName().contentEquals("Builder")) {
        members.addAll(methods(builder, "Builder", trees, elements));
      }
    }
    result.put("constants", constants);
    result.put("members", members);
    return Json.write(result);
  }

  private List<Map<String, Object>> methods(
      TypeElement owner, String ownerName, DocTrees trees, Elements elements) {
    List<Map<String, Object>> result = new ArrayList<>();
    for (Element element : owner.getEnclosedElements()) {
      if (!(element instanceof ExecutableElement method)
          || method.getKind() != ElementKind.METHOD
          || !method.getModifiers().contains(Modifier.PUBLIC)
          || elements.getOrigin(method) != Elements.Origin.EXPLICIT
          || OBJECT_METHODS.contains(method.getSimpleName().toString())) {
        continue;
      }
      DocCommentTree comment = trees.getDocCommentTree(method);
      if (comment == null) {
        continue;
      }
      Doc doc = new Doc(trees, method, comment);
      Map<String, Object> entry = new TreeMap<>();
      entry.put("owner", ownerName);
      entry.put("name", method.getSimpleName().toString());
      entry.put("static", method.getModifiers().contains(Modifier.STATIC));
      entry.put("signature", signature(method));
      entry.put(
          "parameterTypes",
          method.getParameters().stream()
              .map(parameter -> parameterType(method, parameter))
              .collect(Collectors.joining(", ")));
      entry.put("doc", doc.body);
      entry.put("params", doc.params);
      entry.put("returns", doc.returns);
      entry.put("throws", doc.throwsTags);
      result.add(entry);
    }
    return result;
  }

  private static String signature(ExecutableElement method) {
    StringBuilder result = new StringBuilder();
    for (Modifier modifier : method.getModifiers()) {
      if (modifier == Modifier.PUBLIC || modifier == Modifier.STATIC || modifier == Modifier.FINAL) {
        result.append(modifier).append(' ');
      }
    }
    if (method.getModifiers().contains(Modifier.DEFAULT)) {
      result.append("default ");
    }
    if (!method.getTypeParameters().isEmpty()) {
      result
          .append('<')
          .append(
              method.getTypeParameters().stream()
                  .map(parameter -> simple(parameter.toString()))
                  .collect(Collectors.joining(", ")))
          .append("> ");
    }
    result.append(simple(method.getReturnType().toString())).append(' ');
    result.append(method.getSimpleName()).append('(');
    result.append(
        method.getParameters().stream()
            .map(parameter -> parameterType(method, parameter) + " " + parameter.getSimpleName())
            .collect(Collectors.joining(", ")));
    return result.append(')').toString();
  }

  private static String parameterType(ExecutableElement method, VariableElement parameter) {
    String type = simple(parameter.asType().toString());
    List<? extends VariableElement> parameters = method.getParameters();
    if (method.isVarArgs() && parameter.equals(parameters.get(parameters.size() - 1))) {
      type = type.replaceFirst("\\[\\]$", "...");
    }
    return type;
  }

  /** Drops package qualifiers and type-use annotation packages from a type's source form. */
  static String simple(String type) {
    return type.replaceAll("\\b(?:[a-z_][a-z0-9_]*\\.)+(?=[A-Za-z_])", "");
  }

  /**
   * Returns the string passed to an enum constant's constructor. javadoc does not keep constant
   * arguments in its model, so this reads them from the constant's source text.
   */
  private static String enumArgument(DocTrees trees, Element constant) {
    TreePath path = trees.getPath(constant);
    if (path == null) {
      return "";
    }
    try {
      CharSequence source = path.getCompilationUnit().getSourceFile().getCharContent(true);
      java.util.regex.Matcher matcher =
          java.util.regex.Pattern.compile(
                  "\\b" + constant.getSimpleName() + "\\s*\\(\\s*\"((?:[^\"\\\\]|\\\\.)*)\"")
              .matcher(source);
      return matcher.find() ? matcher.group(1) : "";
    } catch (IOException error) {
      return "";
    }
  }

  /** One documentation comment converted to MDX-safe Markdown. */
  private static final class Doc {
    final String body;
    final List<Map<String, Object>> see = new ArrayList<>();
    final List<Map<String, Object>> params = new ArrayList<>();
    final List<Map<String, Object>> throwsTags = new ArrayList<>();
    String returns = "";
    private final DocTrees trees;
    private final Element owner;
    private final DocCommentTree comment;

    Doc(DocTrees trees, Element owner, DocCommentTree comment) {
      this.trees = trees;
      this.owner = owner;
      this.comment = comment;
      if (comment == null) {
        body = "";
        return;
      }
      body = markdown(comment.getFullBody());
      for (DocTree tag : comment.getBlockTags()) {
        if (tag instanceof ParamTree param) {
          params.add(
              Map.of(
                  "name", param.getName().getName().toString(),
                  "doc", markdown(param.getDescription())));
        } else if (tag instanceof ReturnTree returnTree) {
          returns = markdown(returnTree.getDescription());
        } else if (tag instanceof ThrowsTree throwsTree) {
          throwsTags.add(
              Map.of(
                  "type", simple(throwsTree.getExceptionName().getSignature()),
                  "doc", markdown(throwsTree.getDescription())));
        } else if (tag instanceof SeeTree seeTree) {
          List<? extends DocTree> reference = seeTree.getReference();
          if (!reference.isEmpty() && reference.get(0) instanceof StartElementTree anchor) {
            String url = attribute(anchor, "href");
            String label =
                reference.stream()
                    .filter(TextTree.class::isInstance)
                    .map(node -> ((TextTree) node).getBody())
                    .collect(Collectors.joining())
                    .replaceAll("\\s+", " ")
                    .trim();
            see.add(Map.of("url", url, "label", label.isEmpty() ? "Slack reference" : label));
          }
        }
      }
    }

    private String markdown(List<? extends DocTree> trees) {
      StringBuilder result = new StringBuilder();
      boolean preformatted = false;
      java.util.Deque<String> anchors = new java.util.ArrayDeque<>();
      StringBuilder code = new StringBuilder();
      for (DocTree tree : trees) {
        switch (tree.getKind()) {
          case TEXT -> {
            String text = ((TextTree) tree).getBody();
            if (preformatted) {
              code.append(text);
            } else {
              result.append(escape(text));
            }
          }
          case ENTITY -> {
            String entity = decode(((EntityTree) tree).getName().toString());
            if (preformatted) {
              code.append(entity);
            } else {
              result.append(escape(entity));
            }
          }
          case CODE, LITERAL -> {
            String literal = ((LiteralTree) tree).getBody().getBody();
            if (preformatted) {
              code.append(literal);
            } else {
              result.append('`').append(literal.strip()).append('`');
            }
          }
          case LINK, LINK_PLAIN -> result.append(link((LinkTree) tree));
          case START_ELEMENT -> {
            StartElementTree element = (StartElementTree) tree;
            switch (element.getName().toString().toLowerCase(Locale.ROOT)) {
              case "p" -> result.append("\n\n");
              case "ul", "ol" -> result.append("\n\n");
              case "li" -> result.append("\n- ");
              case "pre" -> {
                preformatted = true;
                code.setLength(0);
              }
              case "a" -> {
                anchors.push(attribute(element, "href"));
                result.append('[');
              }
              default -> {}
            }
          }
          case END_ELEMENT -> {
            EndElementTree element = (EndElementTree) tree;
            switch (element.getName().toString().toLowerCase(Locale.ROOT)) {
              case "ul", "ol" -> result.append("\n\n");
              case "pre" -> {
                preformatted = false;
                result.append("\n\n```java\n").append(dedent(code.toString())).append("\n```\n\n");
              }
              case "a" -> result.append("](").append(anchors.isEmpty() ? "" : anchors.pop()).append(')');
              default -> {}
            }
          }
          default -> {}
        }
      }
      return tidy(result.toString());
    }

    private String link(LinkTree link) {
      String label =
          link.getLabel().isEmpty()
              ? link.getReference().getSignature().replaceFirst("^#", "").replace('#', '.')
              : markdown(link.getLabel()).trim();
      DocTreePath path = DocTreePath.getPath(trees.getPath(owner), comment, link.getReference());
      Element target = path == null ? null : trees.getElement(path);
      while (target != null && !(target instanceof TypeElement)) {
        target = target.getEnclosingElement();
      }
      if (target instanceof TypeElement type) {
        while (type.getNestingKind().isNested() && type.getEnclosingElement() instanceof TypeElement outer) {
          type = outer;
        }
        return "[`" + label + "`](javadoc:" + type.getSimpleName() + ")";
      }
      return "`" + label + "`";
    }

    /** Removes blank edge lines and the indentation shared by every code line. */
    private static String dedent(String code) {
      List<String> lines = new ArrayList<>(List.of(code.split("\n", -1)));
      while (!lines.isEmpty() && lines.get(0).isBlank()) {
        lines.remove(0);
      }
      while (!lines.isEmpty() && lines.get(lines.size() - 1).isBlank()) {
        lines.remove(lines.size() - 1);
      }
      int indent =
          lines.stream()
              .filter(line -> !line.isBlank())
              .mapToInt(line -> line.length() - line.stripLeading().length())
              .min()
              .orElse(0);
      return lines.stream()
          .map(line -> line.isBlank() ? "" : line.substring(indent))
          .collect(Collectors.joining("\n"));
    }

    private static String attribute(StartElementTree element, String name) {
      for (DocTree attribute : element.getAttributes()) {
        if (attribute instanceof AttributeTree tree && tree.getName().contentEquals(name)) {
          return tree.getValue().stream().map(Object::toString).collect(Collectors.joining());
        }
      }
      return "";
    }

    private static String escape(String text) {
      return text.replace("{", "\\{").replace("}", "\\}").replace("<", "&lt;");
    }

    private static String decode(String entity) {
      return switch (entity) {
        case "lt" -> "<";
        case "gt" -> ">";
        case "amp" -> "&";
        case "quot" -> "\"";
        default -> "&" + entity + ";";
      };
    }

    /** Normalizes whitespace outside code blocks, where Javadoc line wrapping carries no meaning. */
    private static String tidy(String markdown) {
      String[] parts = markdown.split("(?=```java\\n)|(?<=\\n```\\n)");
      StringBuilder result = new StringBuilder();
      for (String part : parts) {
        if (part.startsWith("```java\n")) {
          result.append(part.replaceAll("```java\\n\\n+", "```java\n"));
        } else {
          String prose = part.replaceAll("[ \\t]*\\n[ \\t]*", "\n");
          prose = prose.replaceAll("(?<!\\n)\\n(?!\\n|- )", " ");
          result.append(prose.replaceAll("[ \\t]{2,}", " "));
        }
      }
      return result.toString().replaceAll("\\n{3,}", "\n\n").trim();
    }
  }

  /** A minimal JSON writer for strings, booleans, lists, and maps. */
  private static final class Json {
    static String write(Object value) {
      if (value instanceof String text) {
        StringBuilder result = new StringBuilder("\"");
        for (char character : text.toCharArray()) {
          switch (character) {
            case '"' -> result.append("\\\"");
            case '\\' -> result.append("\\\\");
            case '\n' -> result.append("\\n");
            case '\r' -> result.append("\\r");
            case '\t' -> result.append("\\t");
            default -> {
              if (character < 0x20) {
                result.append(String.format("\\u%04x", (int) character));
              } else {
                result.append(character);
              }
            }
          }
        }
        return result.append('"').toString();
      }
      if (value instanceof Boolean || value instanceof Number) {
        return value.toString();
      }
      if (value instanceof List<?> list) {
        return list.stream().map(Json::write).collect(Collectors.joining(",", "[", "]"));
      }
      if (value instanceof Map<?, ?> map) {
        return map.entrySet().stream()
            .map(entry -> write(String.valueOf(entry.getKey())) + ":" + write(entry.getValue()))
            .collect(Collectors.joining(",", "{", "}"));
      }
      return "null";
    }
  }
}
