/**
 * Typed validation errors raised while builders materialize or explicit payload
 * validation runs. Every subclass includes a machine-readable category and a
 * path identifying the invalid field.
 *
 * @module errors
 */

/** Stable machine-readable category attached to every validation error. */
export type ErrorCategory =
  | "length-exceeded"
  | "out-of-range"
  | "mutually-exclusive"
  | "type-mismatch"
  | "missing-required"
  | "invalid-usage";

/**
 * Base class for invalid Block Kit input.
 *
 * Catch this class to handle every slackblocks validation failure, or catch a
 * subclass when the reason matters to application behavior.
 */
export class InvalidUsageError extends Error {
  /** Machine-readable failure category. */
  readonly category: ErrorCategory = "invalid-usage";
  /** Dot-and-index path to the invalid payload field. */
  readonly path: string;

  /**
   * Creates a validation error.
   *
   * @param path - Dot-and-index path to the invalid field.
   * @param message - Human-readable explanation of the constraint.
   */
  constructor(path: string, message: string) {
    super(path ? `${path}: ${message}` : message);
    this.name = new.target.name;
    this.path = path;
  }
}

/**
 * A string, array, or collection falls outside a field's allowed length. Typical
 * causes include text exceeding Slack's limit, too many options, or an action
 * identifier longer than 255 characters.
 */
export class LengthError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "length-exceeded" as const;
}

/**
 * A numeric value falls outside a field's allowed range. Typical causes include
 * minimum values greater than maximum values or counts outside Slack's supported
 * bounds.
 */
export class OutOfRangeError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "out-of-range" as const;
}

/**
 * Mutually exclusive fields are supplied together. Examples include combining an
 * image URL with a Slack file, or providing both options and option groups to a
 * static select menu.
 */
export class MutualExclusivityError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "mutually-exclusive" as const;
}

/**
 * A payload field or nested object has the wrong runtime type or an unsupported
 * discrete value. The error path identifies the exact field that failed runtime
 * validation.
 */
export class TypeMismatchError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "type-mismatch" as const;
}

/**
 * A required field or one-of requirement is not satisfied. Typical causes include
 * building a section without text or fields, or creating a conversation filter
 * without any filter options.
 */
export class MissingRequiredError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "missing-required" as const;
}
