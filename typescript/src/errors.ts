export type ErrorCategory =
  | "length-exceeded"
  | "out-of-range"
  | "mutually-exclusive"
  | "type-mismatch"
  | "missing-required"
  | "invalid-usage";

export class InvalidUsageError extends Error {
  readonly category: ErrorCategory = "invalid-usage";
  readonly path: string;

  constructor(path: string, message: string) {
    super(path ? `${path}: ${message}` : message);
    this.name = new.target.name;
    this.path = path;
  }
}

export class LengthError extends InvalidUsageError {
  override readonly category = "length-exceeded" as const;
}

export class RangeError extends InvalidUsageError {
  override readonly category = "out-of-range" as const;
}

export class MutualExclusivityError extends InvalidUsageError {
  override readonly category = "mutually-exclusive" as const;
}

export class TypeMismatchError extends InvalidUsageError {
  override readonly category = "type-mismatch" as const;
}

export class MissingRequiredError extends InvalidUsageError {
  override readonly category = "missing-required" as const;
}
