package io.github.nicklambourne.slackblocks;

/** Thrown when a builder cannot produce a valid Slack Block Kit value. */
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
   * Returns the dotted path of the invalid field.
   *
   * @return the field path
   */
  public String getPath() {
    return path;
  }
}
