package io.github.nicklambourne.slackblocks;

/** Stable validation categories shared by every slackblocks implementation. */
public enum ErrorCategory {
  /** A string or collection falls outside Slack's documented length limits. */
  LENGTH_EXCEEDED("length-exceeded"),
  /** A numeric value falls outside Slack's documented range. */
  OUT_OF_RANGE("out-of-range"),
  /** Two fields that cannot coexist were supplied together. */
  MUTUALLY_EXCLUSIVE("mutually-exclusive"),
  /** A value is not an accepted Slack object or scalar type. */
  TYPE_MISMATCH("type-mismatch"),
  /** A required field or collection is absent. */
  MISSING_REQUIRED("missing-required"),
  /** A combination of otherwise valid values cannot be used together. */
  INVALID_USAGE("invalid-usage");

  private final String wireValue;

  ErrorCategory(String wireValue) {
    this.wireValue = wireValue;
  }

  /**
   * Returns the language-neutral category used by the conformance manifest.
   *
   * @return the kebab-case category value
   */
  public String wireValue() {
    return wireValue;
  }
}
