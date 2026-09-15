package io.github.nicklambourne.slackblocks;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.google.gson.JsonParser;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.PrintStream;
import java.lang.reflect.Method;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import javax.tools.DiagnosticCollector;
import javax.tools.JavaCompiler;
import javax.tools.JavaFileObject;
import javax.tools.StandardJavaFileManager;
import javax.tools.ToolProvider;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

/**
 * Compiles every Java snippet in the documentation and READMEs against this build, runs the shared
 * executable example, and checks each "Using Blocks" snippet produces its documented JSON.
 */
final class DocsExamplesTest {
  private static final Path REPOSITORY = Path.of("..").toAbsolutePath().normalize();
  private static final Path OUTPUT = Path.of("target", "docs-snippets");
  private static final Pattern JAVA_SECTION = Pattern.compile("<Java>(.*?)</Java>", Pattern.DOTALL);
  private static final Pattern JAVA_FENCE = Pattern.compile("```java\\n(.*?)```", Pattern.DOTALL);
  private static final Pattern JSON_FENCE = Pattern.compile("```json\\n(.*?)```", Pattern.DOTALL);
  private static final Pattern HEADING = Pattern.compile("^## (.+)$", Pattern.MULTILINE);
  private static final List<String> DEFAULT_IMPORTS =
      List.of(
          "io.github.nicklambourne.slackblocks.*",
          "io.github.nicklambourne.slackblocks.block.*",
          "io.github.nicklambourne.slackblocks.component.*",
          "io.github.nicklambourne.slackblocks.element.*",
          "io.github.nicklambourne.slackblocks.object.*",
          "io.github.nicklambourne.slackblocks.payload.*",
          "com.slack.api.Slack",
          "com.slack.api.methods.MethodsClient",
          "com.slack.api.methods.request.chat.ChatPostMessageRequest",
          "com.slack.api.model.block.LayoutBlock",
          "java.util.List",
          "java.util.Map");
  private static final String CONTEXT =
      """
      static MethodsClient client = Slack.getInstance().methods("xoxb-test");
      static SectionBlock block = SectionBlock.builder().markdownText("Hello").build();
      static org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger("docs");
      """;

  private static final Map<String, String> SNIPPETS = new LinkedHashMap<>();
  private static final Map<String, String> USING_BLOCKS_JSON = new LinkedHashMap<>();
  private static URLClassLoader loader;

  @BeforeAll
  static void compileSnippets() throws Exception {
    List<Path> documents = new ArrayList<>();
    try (Stream<Path> files = Files.walk(REPOSITORY.resolve("docs/docs"))) {
      files.filter(file -> file.toString().endsWith(".mdx")).sorted().forEach(documents::add);
    }
    documents.add(REPOSITORY.resolve("README.md"));
    documents.add(REPOSITORY.resolve("java/README.md"));
    for (Path document : documents) {
      String text = Files.readString(document);
      List<String> regions = new ArrayList<>();
      if (document.toString().endsWith(".mdx")) {
        Matcher sections = JAVA_SECTION.matcher(text);
        while (sections.find()) {
          regions.add(sections.group(1));
        }
      } else {
        regions.add(text);
      }
      int index = 0;
      for (String region : regions) {
        Matcher fences = JAVA_FENCE.matcher(region);
        while (fences.find()) {
          String name = className(REPOSITORY.relativize(document), ++index);
          SNIPPETS.put(name, wrap(name, fences.group(1)));
        }
      }
    }
    collectUsingBlocks();

    Files.createDirectories(OUTPUT);
    JavaCompiler compiler = ToolProvider.getSystemJavaCompiler();
    DiagnosticCollector<JavaFileObject> diagnostics = new DiagnosticCollector<>();
    List<File> sources = new ArrayList<>();
    for (Map.Entry<String, String> snippet : SNIPPETS.entrySet()) {
      Path source = OUTPUT.resolve(snippet.getKey() + ".java");
      Files.writeString(source, snippet.getValue());
      sources.add(source.toFile());
    }
    Path example = REPOSITORY.resolve("docs/examples/java/SectionHello.java");
    sources.add(example.toFile());
    try (StandardJavaFileManager files =
        compiler.getStandardFileManager(diagnostics, Locale.ROOT, StandardCharsets.UTF_8)) {
      boolean compiled =
          compiler
              .getTask(
                  null,
                  files,
                  diagnostics,
                  List.of("-d", OUTPUT.toString(), "-classpath", classPath(), "-proc:none"),
                  null,
                  files.getJavaFileObjectsFromFiles(sources))
              .call();
      String errors =
          diagnostics.getDiagnostics().stream()
              .filter(diagnostic -> diagnostic.getKind() == javax.tools.Diagnostic.Kind.ERROR)
              .map(
                  diagnostic ->
                      diagnostic.getSource().getName()
                          + ":"
                          + diagnostic.getLineNumber()
                          + ": "
                          + diagnostic.getMessage(Locale.ROOT))
              .collect(Collectors.joining("\n"));
      assertTrue(compiled, "Documentation snippets do not compile:\n" + errors);
    }
    loader =
        new URLClassLoader(
            new URL[] {OUTPUT.toUri().toURL()}, DocsExamplesTest.class.getClassLoader());
  }

  @Test
  void everyDocumentationSnippetCompiles() {
    assertTrue(SNIPPETS.size() >= 30, "expected the Java guides and READMEs to contain snippets");
  }

  @Test
  void sectionHelloExampleMatchesTheSharedJson() throws Exception {
    String output = runMain(loader.loadClass("SectionHello"));
    String expected = Files.readString(REPOSITORY.resolve("docs/examples/section_hello.json"));
    assertEquals(
        FluentDriver.canonical(JsonParser.parseString(expected)),
        FluentDriver.canonical(JsonParser.parseString(output)));
  }

  @TestFactory
  Stream<DynamicTest> everyUsingBlocksSnippetProducesItsDocumentedJson() {
    return USING_BLOCKS_JSON.entrySet().stream()
        .map(
            entry ->
                DynamicTest.dynamicTest(
                    entry.getKey(),
                    () -> {
                      Class<?> snippet = loader.loadClass(usingBlocksClass(entry.getKey()));
                      Object value = snippet.getMethod("value").invoke(null);
                      assertEquals(
                          FluentDriver.canonical(JsonParser.parseString(entry.getValue())),
                          FluentDriver.canonical(
                              JsonParser.parseString(((SlackObject) value).toJson())));
                    }));
  }

  private static void collectUsingBlocks() throws Exception {
    String text = Files.readString(REPOSITORY.resolve("docs/docs/usage/using_blocks.mdx"));
    Matcher headings = HEADING.matcher(text);
    List<int[]> bounds = new ArrayList<>();
    List<String> titles = new ArrayList<>();
    while (headings.find()) {
      titles.add(headings.group(1));
      bounds.add(new int[] {headings.end(), 0});
    }
    for (int index = 0; index < titles.size(); index++) {
      int end = index + 1 < bounds.size() ? bounds.get(index + 1)[0] : text.length();
      String body = text.substring(bounds.get(index)[0], end);
      Matcher java = JAVA_SECTION.matcher(body);
      Matcher json = JSON_FENCE.matcher(body);
      if (!java.find() || !json.find()) {
        continue;
      }
      Matcher fence = JAVA_FENCE.matcher(java.group(1));
      if (!fence.find()
          || !Pattern.compile("^\\w+ block = ", Pattern.MULTILINE).matcher(fence.group(1)).find()) {
        continue;
      }
      String name = usingBlocksClass(titles.get(index));
      USING_BLOCKS_JSON.put(titles.get(index), json.group(1));
      SNIPPETS.put(name, valueClass(name, fence.group(1)));
    }
    assertTrue(USING_BLOCKS_JSON.size() >= 21, "every block section must have a Java snippet");
  }

  private static String usingBlocksClass(String title) {
    return "UsingBlocks" + title.replaceAll("[^A-Za-z]", "");
  }

  private static String className(Path document, int index) {
    return "Doc" + document.toString().replaceAll("[^A-Za-z]", "_") + "_" + index;
  }

  private static String imports(List<String> declared) {
    List<String> lines = new ArrayList<>(declared);
    DEFAULT_IMPORTS.forEach(item -> lines.add("import " + item + ";"));
    return String.join("\n", lines) + "\n";
  }

  private static String wrap(String name, String snippet) {
    List<String> imports = new ArrayList<>();
    List<String> body = new ArrayList<>();
    for (String line : snippet.lines().toList()) {
      (line.startsWith("import ") ? imports : body).add(line);
    }
    String code = String.join("\n", body);
    if (code.contains(" class ")) {
      return imports(imports) + code.replaceFirst("public final class \\w+", "final class " + name);
    }
    if (Pattern.compile("^static ", Pattern.MULTILINE).matcher(code).find()) {
      return imports(imports) + "final class " + name + " {\n" + CONTEXT + code + "\n}\n";
    }
    return imports(imports)
        + "final class "
        + name
        + " {\n"
        + CONTEXT
        + "void run() throws Exception {\n"
        + code
        + "\n}\n}\n";
  }

  private static String valueClass(String name, String snippet) {
    return imports(List.of())
        + "public final class "
        + name
        + " {\n"
        + "public static SlackObject value() {\n"
        + snippet
        + "\nreturn block;\n}\n}\n";
  }

  private static String classPath() {
    return System.getProperty("surefire.test.class.path", System.getProperty("java.class.path"))
        + File.pathSeparator
        + Path.of("target", "classes").toAbsolutePath()
        + File.pathSeparator
        + Path.of("target", "test-classes").toAbsolutePath();
  }

  private static String runMain(Class<?> type) throws Exception {
    Method main = type.getMethod("main", String[].class);
    PrintStream original = System.out;
    ByteArrayOutputStream captured = new ByteArrayOutputStream();
    System.setOut(new PrintStream(captured, true, StandardCharsets.UTF_8));
    try {
      main.invoke(null, (Object) new String[0]);
    } finally {
      System.setOut(original);
    }
    return captured.toString(StandardCharsets.UTF_8);
  }
}
