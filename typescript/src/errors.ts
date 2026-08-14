/**
 * Typed validation errors returned by eager construction and explicit validation.
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

/** A string, array, or collection exceeded an allowed minimum or maximum length. */
export class LengthError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "length-exceeded" as const;
}

/** A numeric value fell outside its allowed range. */
export class OutOfRangeError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "out-of-range" as const;
}

/** Fields that cannot be used together were both supplied. */
export class MutualExclusivityError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "mutually-exclusive" as const;
}

/** A payload field or nested object has the wrong runtime type. */
export class TypeMismatchError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "type-mismatch" as const;
}

/** A required field or one-of requirement was not satisfied. */
export class MissingRequiredError extends InvalidUsageError {
  /** Machine-readable failure category. */
  override readonly category = "missing-required" as const;
}
