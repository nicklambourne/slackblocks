package slackblocks

import (
	"fmt"
	"unicode/utf8"
)

var requiredFields = map[string][]string{
	"button":                 {"text", "action_id"},
	"channel":                {"channel_id"},
	"emoji":                  {"name"},
	"link":                   {"url"},
	"rich_text_list":         {"style", "elements"},
	"rich_text_preformatted": {"elements"},
	"rich_text_quote":        {"elements"},
	"rich_text_section":      {"elements"},
	"text":                   {"text"},
	"user":                   {"user_id"},
	"usergroup":              {"usergroup_id"},
}

// Validate recursively checks a Slack wire object.
func Validate(object Object) error { return validateObject(object, "") }

func validateObject(object Object, path string) error {
	typeName, _ := object["type"].(string)
	for _, field := range requiredFields[typeName] {
		if _, ok := object[field]; !ok {
			return validationError(MissingRequired, path, "expected %s", field)
		}
	}

	if blockID, ok := object["block_id"].(string); ok && utf8.RuneCountInString(blockID) > 255 {
		return validationError(LengthExceeded, child(path, "block_id"), "exceeds maximum 255")
	}
	if actionID, ok := object["action_id"].(string); ok && utf8.RuneCountInString(actionID) > 255 {
		return validationError(LengthExceeded, child(path, "action_id"), "exceeds maximum 255")
	}

	switch typeName {
	case "plain_text", "mrkdwn":
		text, ok := object["text"].(string)
		if !ok {
			return validationError(TypeMismatch, child(path, "text"), "expected a string")
		}
		length := utf8.RuneCountInString(text)
		if length < 1 || length > 3000 {
			return validationError(LengthExceeded, child(path, "text"), "length %d is outside 1..3000", length)
		}
	case "section":
		_, hasText := object["text"]
		_, hasFields := object["fields"]
		if !hasText && !hasFields {
			return validationError(MissingRequired, path, "expected text, fields, or both")
		}
	case "pie":
		segments, ok := object["segments"].([]any)
		if !ok || len(segments) == 0 {
			return validationError(LengthExceeded, child(path, "segments"), "expected at least one segment")
		}
	}

	for field, value := range object {
		if field == "type" {
			continue
		}
		if err := validateNested(value, child(path, field)); err != nil {
			return err
		}
	}
	return nil
}

func validateNested(value any, path string) error {
	switch typed := value.(type) {
	case Object:
		return validateObject(typed, path)
	case []any:
		for index, nested := range typed {
			if err := validateNested(nested, fmt.Sprintf("%s[%d]", path, index)); err != nil {
				return err
			}
		}
	}
	return nil
}
