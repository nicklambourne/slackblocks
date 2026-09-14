package io.github.nicklambourne.slackblocks.internal;

import io.github.nicklambourne.slackblocks.ErrorCategory;
import io.github.nicklambourne.slackblocks.ValidationException;
import java.util.Map;

/** Central validation entry point populated by the conformance implementation. */
public final class Validator {
  private Validator() {}

  /** Validates one fully materialized Slack object. */
  public static void validate(String name, Map<String, Object> value) {
    if (name.isEmpty()) {
      throw new IllegalArgumentException("builder name must not be empty");
    }
    if (value.isEmpty()) {
      throw new ValidationException(
          ErrorCategory.MISSING_REQUIRED, name, "expected at least one field");
    }
  }
}
