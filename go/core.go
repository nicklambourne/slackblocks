package slackblocks

//go:generate go run ./internal/generatebuilders

import (
	"encoding/json"
	"fmt"

	slackapi "github.com/slack-go/slack"
)

// Object is a JSON-compatible Slack wire object.
type Object map[string]any

// Buildable is implemented by every fluent Slack object builder.
type Buildable interface {
	Build() (Object, error)
}

// GroupBuildable is implemented by higher-level components that expand to
// multiple ordinary Slack objects inside a parent collection.
type GroupBuildable interface {
	BuildMany() ([]Object, error)
}

type coercion func(any) (any, error)

// builder incrementally configures one Slack wire object. Named constructors
// provide the discriminator and field-specific coercions; Build validates the
// completed result.
type builder struct {
	name      string
	values    Object
	coercions map[string]coercion
	transform func(Object) (Object, error)
	err       error
}

// concreteBuilder provides the common terminal operations embedded by each
// public object-specific builder.
type concreteBuilder struct {
	core *builder
}

// slackBlockBuilder adds slack-go's native Block contract only to concrete
// block builders. Composition objects, elements, and payload builders
// deliberately do not implement slack.Block.
type slackBlockBuilder struct {
	*concreteBuilder
}

func newSlackBlockBuilder(core *builder) *slackBlockBuilder {
	return &slackBlockBuilder{concreteBuilder: newConcreteBuilder(core)}
}

// BlockType returns the Slack wire discriminator expected by slack.Block.
func (b *slackBlockBuilder) BlockType() slackapi.MessageBlockType {
	return slackapi.MessageBlockType(b.wireString("type"))
}

// ID returns the optional block_id expected by slack.Block.
func (b *slackBlockBuilder) ID() string {
	return b.wireString("block_id")
}

func (b *slackBlockBuilder) wireString(field string) string {
	if b == nil || b.concreteBuilder == nil || b.concreteBuilder.core == nil {
		return ""
	}
	value, _ := b.concreteBuilder.core.values[field].(string)
	return value
}

func newConcreteBuilder(core *builder) *concreteBuilder {
	return &concreteBuilder{core: core}
}

// Set configures an advanced wire-format field that does not yet have a
// dedicated fluent method. Prefer the named methods on the concrete builder.
func (b *concreteBuilder) Set(field string, value any) Buildable {
	if b == nil || b.core == nil {
		return b
	}
	b.core.Set(field, value)
	return b
}

// Build materialises nested builders and validates the completed Slack object.
func (b *concreteBuilder) Build() (Object, error) {
	if b == nil || b.core == nil {
		return nil, validationError(InvalidUsage, "Builder", "cannot build a nil builder")
	}
	return b.core.Build()
}

// MustBuild builds the configured object and panics if it is invalid.
func (b *concreteBuilder) MustBuild() Object {
	object, err := b.Build()
	if err != nil {
		panic(err)
	}
	return object
}

// MarshalJSON lets a completed concrete builder be passed to encoding/json.
func (b *concreteBuilder) MarshalJSON() ([]byte, error) {
	object, err := b.Build()
	if err != nil {
		return nil, err
	}
	return json.Marshal(object)
}

func newBuilder(name, objectType string) *builder {
	values := Object{}
	if objectType != "" {
		values["type"] = objectType
	}
	return &builder{name: name, values: values, coercions: map[string]coercion{}}
}

func (b *builder) coerce(field string, fn coercion) *builder {
	b.coercions[field] = fn
	return b
}

func (b *builder) withTransform(fn func(Object) (Object, error)) *builder {
	b.transform = fn
	return b
}

func (b *builder) set(field string, value any) *builder {
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

func (b *builder) append(field string, values ...any) *builder {
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

func (b *builder) appendNested(field string, values ...any) *builder {
	if b.err != nil {
		return b
	}
	current, _ := b.values[field].([]any)
	b.values[field] = append(current, values...)
	return b
}

// Set configures an advanced wire-format field that does not yet have a
// dedicated fluent method. Prefer named methods for ordinary Block Kit use.
func (b *builder) Set(field string, value any) *builder {
	if field == "" {
		b.err = validationError(MissingRequired, "Builder.Set", "field must not be empty")
		return b
	}
	return b.set(field, value)
}

// Build materialises nested builders and validates the completed Slack object.
func (b *builder) Build() (Object, error) {
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
	if b.transform != nil {
		object, err = b.transform(object)
		if err != nil {
			return nil, err
		}
	}
	if err := validateBuilder(b.name, object); err != nil {
		return nil, err
	}
	if err := Validate(object); err != nil {
		return nil, err
	}
	return object, nil
}

// MustBuild is Build for package-level declarations and tests. It panics when
// the configured object is invalid.
func (b *builder) MustBuild() Object {
	object, err := b.Build()
	if err != nil {
		panic(err)
	}
	return object
}

// MarshalJSON lets a completed builder be passed directly to encoding/json.
func (b *builder) MarshalJSON() ([]byte, error) {
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
			if group, ok := nested.(GroupBuildable); ok {
				built, err := group.BuildMany()
				if err != nil {
					return nil, err
				}
				for _, item := range built {
					result = append(result, item)
				}
				continue
			}
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
