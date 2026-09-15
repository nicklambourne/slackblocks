package io.github.nicklambourne.slackblocks;

/**
 * Thrown when a builder cannot produce a valid Slack Block Kit value.
 *
 * <p>This is an {@link IllegalArgumentException}, so code that already treats invalid arguments
 * uniformly continues to work. Catch {@code ValidationException} when you need the stable {@link
 * #getCategory() category} or the {@link #getPath() path} of the invalid field. Paths start with
 * the public Java type being built, for example {@code SectionBlock.fields[2].text}.
 */
public final class ValidationException extends IllegalArgumentException {
  private static final long serialVersionUID = 1L;

  /** Stable failure category. */
  private final ErrorCategory category;

  /** Dotted path of the invalid value. */
  private final String path;

  /**
   * Creates a structured validation failure.
   *
   * @param category stable cross-language failure category
   * @param path dotted path of the invalid value
   * @param message human-readable explanation
   */
  public ValidationException(ErrorCategory category, String path, String message) {
    super(path + ": " + message);
    this.category = category;
    this.path = path;
  }

  /**
   * Returns the stable cross-language error category.
   *
   * @return the error category
   */
  public ErrorCategory getCategory() {
    return category;
  }

  /**
   * Returns the dotted path of the invalid field, starting with the Java type being built.
   *
   * @return the field path
   */
  public String getPath() {
    return path;
  }
}
