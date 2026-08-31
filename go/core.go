package slackblocks

import (
	"encoding/json"
	"fmt"
)

// Object is a JSON-compatible Slack wire object.
type Object map[string]any

// Buildable is implemented by every fluent Slack object builder.
type Buildable interface {
	Build() (Object, error)
}

type coercion func(any) (any, error)

// Builder incrementally configures one Slack wire object. Named constructors
// provide the discriminator and field-specific coercions; Build validates the
// completed result.
type Builder struct {
	name      string
	values    Object
	coercions map[string]coercion
	err       error
}

func newBuilder(name, objectType string) *Builder {
	values := Object{}
	if objectType != "" {
		values["type"] = objectType
	}
	return &Builder{name: name, values: values, coercions: map[string]coercion{}}
}

func (b *Builder) coerce(field string, fn coercion) *Builder {
	b.coercions[field] = fn
	return b
}

func (b *Builder) set(field string, value any) *Builder {
	if b.err != nil {
		return b
	}
	if fn := b.coercions[field]; fn != nil {
		value, b.err = fn(value)
		if b.err != nil {
			return b
		}
	}
	b.values[field] = value
	return b
}

func (b *Builder) append(field string, values ...any) *Builder {
	if b.err != nil {
		return b
	}
	current, _ := b.values[field].([]any)
	for _, value := range values {
		if slice, ok := value.([]any); ok {
			for _, item := range slice {
				if fn := b.coercions[field]; fn != nil {
					item, b.err = fn(item)
					if b.err != nil {
						return b
					}
				}
				current = append(current, item)
			}
			continue
		}
		if fn := b.coercions[field]; fn != nil {
			value, b.err = fn(value)
			if b.err != nil {
				return b
			}
		}
		current = append(current, value)
	}
	b.values[field] = current
	return b
}

// Set configures an advanced wire-format field that does not yet have a
// dedicated fluent method. Prefer named methods for ordinary Block Kit use.
func (b *Builder) Set(field string, value any) *Builder {
	if field == "" {
		b.err = validationError(MissingRequired, "Builder.Set", "field must not be empty")
		return b
	}
	return b.set(field, value)
}

// Build materialises nested builders and validates the completed Slack object.
func (b *Builder) Build() (Object, error) {
	if b == nil {
		return nil, validationError(InvalidUsage, "Builder", "cannot build a nil builder")
	}
	if b.err != nil {
		return nil, b.err
	}
	materialised, err := materialise(b.values)
	if err != nil {
		return nil, err
	}
	object, ok := materialised.(Object)
	if !ok {
		return nil, validationError(TypeMismatch, b.name, "expected an object")
	}
	if err := Validate(object); err != nil {
		return nil, err
	}
	return object, nil
}

// MustBuild is Build for package-level declarations and tests. It panics when
// the configured object is invalid.
func (b *Builder) MustBuild() Object {
	object, err := b.Build()
	if err != nil {
		panic(err)
	}
	return object
}

// MarshalJSON lets a completed builder be passed directly to encoding/json.
func (b *Builder) MarshalJSON() ([]byte, error) {
	object, err := b.Build()
	if err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

func materialise(value any) (any, error) {
	switch typed := value.(type) {
	case Buildable:
		return typed.Build()
	case Object:
		result := Object{}
		for key, nested := range typed {
			built, err := materialise(nested)
			if err != nil {
				return nil, err
			}
			result[key] = built
		}
		return result, nil
	case map[string]any:
		return materialise(Object(typed))
	case []Object:
		result := make([]any, 0, len(typed))
		for _, nested := range typed {
			built, err := materialise(nested)
			if err != nil {
				return nil, err
			}
			result = append(result, built)
		}
		return result, nil
	case []Buildable:
		result := make([]any, 0, len(typed))
		for _, nested := range typed {
			built, err := materialise(nested)
			if err != nil {
				return nil, err
			}
			result = append(result, built)
		}
		return result, nil
	case []any:
		result := make([]any, 0, len(typed))
		for _, nested := range typed {
			built, err := materialise(nested)
			if err != nil {
				return nil, err
			}
			result = append(result, built)
		}
		return result, nil
	default:
		return value, nil
	}
}

func markdownLike(value any) (any, error) {
	if text, ok := value.(string); ok {
		return Object{"type": "mrkdwn", "text": text}, nil
	}
	return value, nil
}

func plainTextLike(value any) (any, error) {
	if text, ok := value.(string); ok {
		return Object{"type": "plain_text", "text": text}, nil
	}
	return value, nil
}

func child(path, field string) string {
	if path == "" {
		return field
	}
	return fmt.Sprintf("%s.%s", path, field)
}
