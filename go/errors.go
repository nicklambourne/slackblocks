package slackblocks

import "fmt"

// ErrorCategory identifies a language-neutral conformance error category.
type ErrorCategory string

const (
	// LengthExceeded reports a value outside an allowed length bound.
	LengthExceeded ErrorCategory = "length-exceeded"
	// OutOfRange reports a numeric value outside its allowed range.
	OutOfRange ErrorCategory = "out-of-range"
	// MutuallyExclusive reports fields that cannot be used together.
	MutuallyExclusive ErrorCategory = "mutually-exclusive"
	// TypeMismatch reports a value with an unexpected wire type.
	TypeMismatch ErrorCategory = "type-mismatch"
	// MissingRequired reports an absent value required by Slack's schema.
	MissingRequired ErrorCategory = "missing-required"
	// InvalidUsage reports a structurally invalid combination of values.
	InvalidUsage ErrorCategory = "invalid-usage"
)

// ValidationError reports why a completed Slack object is invalid.
type ValidationError struct {
	Category ErrorCategory
	Path     string
	Detail   string
}

func (e *ValidationError) Error() string {
	if e.Path == "" {
		return fmt.Sprintf("%s: %s", e.Category, e.Detail)
	}
	return fmt.Sprintf("%s at %s: %s", e.Category, e.Path, e.Detail)
}

func validationError(category ErrorCategory, path, format string, args ...any) error {
	return &ValidationError{Category: category, Path: path, Detail: fmt.Sprintf(format, args...)}
}
