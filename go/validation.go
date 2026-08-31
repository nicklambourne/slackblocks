package slackblocks

import (
	"fmt"
	"math"
	"regexp"
	"sort"
	"unicode/utf8"
)

var requiredFields = map[string][]string{
	"actions":                    {"elements"},
	"alert":                      {"text"},
	"area":                       {"series", "axis_config"},
	"bar":                        {"series", "axis_config"},
	"button":                     {"text", "action_id"},
	"carousel":                   {"elements"},
	"channel":                    {"channel_id"},
	"channels_select":            {"action_id"},
	"checkboxes":                 {"action_id", "options"},
	"container":                  {"child_blocks"},
	"context":                    {"elements"},
	"context_actions":            {"elements"},
	"conversations_select":       {"action_id"},
	"data_table":                 {"rows", "caption"},
	"data_visualization":         {"title", "chart"},
	"datepicker":                 {"action_id"},
	"datetimepicker":             {"action_id"},
	"email_text_input":           {"action_id"},
	"emoji":                      {"name"},
	"external_select":            {"action_id"},
	"feedback_buttons":           {"positive_button", "negative_button"},
	"file":                       {"external_id"},
	"file_input":                 {"action_id"},
	"header":                     {"text"},
	"home":                       {"blocks"},
	"icon_button":                {"text"},
	"image":                      {"alt_text"},
	"input":                      {"label", "element"},
	"line":                       {"series", "axis_config"},
	"link":                       {"url"},
	"markdown":                   {"text"},
	"modal":                      {"title", "blocks"},
	"multi_channels_select":      {"action_id"},
	"multi_conversations_select": {"action_id"},
	"multi_external_select":      {"action_id"},
	"multi_static_select":        {"action_id"},
	"multi_users_select":         {"action_id"},
	"number_input":               {"action_id"},
	"overflow":                   {"action_id", "options"},
	"pie":                        {"segments"},
	"plain_text_input":           {"action_id"},
	"plan":                       {"title"},
	"radio_buttons":              {"action_id", "options"},
	"raw_number":                 {"value", "text"},
	"raw_text":                   {"text"},
	"rich_text":                  {"elements"},
	"rich_text_input":            {"action_id"},
	"rich_text_list":             {"style", "elements"},
	"rich_text_preformatted":     {"elements"},
	"rich_text_quote":            {"elements"},
	"rich_text_section":          {"elements"},
	"static_select":              {"action_id"},
	"table":                      {"rows"},
	"task_card":                  {"task_id", "title"},
	"text":                       {"text"},
	"timepicker":                 {"action_id"},
	"url":                        {"url", "text"},
	"url_text_input":             {"action_id"},
	"user":                       {"user_id"},
	"usergroup":                  {"usergroup_id"},
	"users_select":               {"action_id"},
	"video":                      {"alt_text", "thumbnail_url", "title", "video_url"},
	"workflow_button":            {"text", "workflow"},
}

var inputElementTypes = stringSet(
	"plain_text_input", "number_input", "checkboxes", "radio_buttons", "datepicker",
	"datetimepicker", "timepicker", "channels_select", "multi_channels_select",
	"conversations_select", "multi_conversations_select", "external_select",
	"multi_external_select", "static_select", "multi_static_select", "users_select",
	"multi_users_select", "rich_text_input", "email_text_input", "url_text_input", "file_input",
)

var confirmTypes = stringSet(
	"button", "channels_select", "checkboxes", "conversations_select", "datepicker",
	"datetimepicker", "external_select", "icon_button", "multi_channels_select",
	"multi_conversations_select", "multi_external_select", "multi_static_select",
	"multi_users_select", "overflow", "radio_buttons", "static_select", "timepicker",
	"users_select", "workflow_button",
)

var attachmentColorPattern = regexp.MustCompile(`^#[0-9a-fA-F]{6}$`)

var slackIconNames = stringSet(
	"archive", "book", "bookmark", "bot", "bug", "calendar", "call", "caret-left",
	"caret-right", "check", "clipboard", "code", "comment", "compass", "copy", "cube",
	"download", "edit", "email", "eye-closed", "eye-open", "file", "flag", "folder", "gear",
	"globe", "heart", "help", "image", "info", "key", "lightbulb", "link", "map", "mobile",
	"new-window", "pin", "plus", "refine", "refresh", "rocket", "save", "screen", "share",
	"sparkle", "star", "star-filled", "tag", "thumbs-down", "thumbs-up", "trash", "upload",
	"user", "warning",
)

var (
	contextElementTypes = stringSet("plain_text", "mrkdwn", "image")
	alertLevels         = stringSet("default", "info", "warning", "error", "success")
	containerWidths     = stringSet("narrow", "standard", "wide", "full")
	taskStatuses        = stringSet("pending", "in_progress", "complete", "error")
	tableCellTypes      = stringSet("raw_text", "rich_text")
	dataTableCellTypes  = stringSet("raw_text", "rich_text", "raw_number")
)

var surfaceBlocks = map[string]map[string]bool{
	"message": stringSet("actions", "card", "carousel", "container", "context", "context_actions", "data_table", "data_visualization", "divider", "file", "header", "image", "markdown", "plan", "rich_text", "section", "table", "task_card", "video"),
	"modal":   stringSet("actions", "alert", "card", "context", "divider", "header", "image", "input", "rich_text", "section", "video"),
	"home":    stringSet("actions", "card", "carousel", "container", "context", "data_table", "divider", "header", "image", "input", "rich_text", "section", "table", "video"),
}

func stringSet(values ...string) map[string]bool {
	set := make(map[string]bool, len(values))
	for _, value := range values {
		set[value] = true
	}
	return set
}

// Validate recursively checks a Slack wire object.
func Validate(object Object) error { return validateObject(object, "") }

func validateBuilder(name string, object Object) error {
	switch name {
	case "Confirmation":
		return validateConfirmation(object, name)
	case "Option":
		return validateOption(object, name)
	case "OptionGroup":
		label, ok := object["label"]
		if !ok {
			return validationError(MissingRequired, name, "expected label")
		}
		if err := textLength(label, child(name, "label.text"), 0, 75); err != nil {
			return err
		}
		options, err := sliceAt(object["options"], child(name, "options"))
		if err != nil {
			return err
		}
		if err := sliceLength(options, child(name, "options"), 1, 100); err != nil {
			return err
		}
		for index, value := range options {
			option, err := objectAt(value, fmt.Sprintf("%s[%d]", child(name, "options"), index))
			if err != nil {
				return err
			}
			if err := validateOption(option, fmt.Sprintf("%s[%d]", child(name, "options"), index)); err != nil {
				return err
			}
		}
	case "ConversationFilter":
		if _, include := object["include"]; !include {
			if _, external := object["exclude_external_shared_channels"]; !external {
				if _, bots := object["exclude_bot_users"]; !bots {
					return validationError(MissingRequired, name, "expected at least one filter field")
				}
			}
		}
	case "InputParameter":
		if _, ok := object["name"]; !ok {
			return validationError(MissingRequired, name, "expected name")
		}
		if _, ok := object["value"]; !ok {
			return validationError(MissingRequired, name, "expected value")
		}
	case "Trigger":
		if _, ok := object["url"]; !ok {
			return validationError(MissingRequired, name, "expected url")
		}
	case "Workflow":
		if _, ok := object["trigger"]; !ok {
			return validationError(MissingRequired, name, "expected trigger")
		}
	case "SlackFile":
		_, hasID := object["id"]
		_, hasURL := object["url"]
		if hasID == hasURL {
			return validationError(MutuallyExclusive, name, "expected exactly one of id or url")
		}
	case "ChartSegment":
		return validateLabelValue(object, name, 20, true)
	case "DataPoint":
		return validateLabelValue(object, name, 20, false)
	case "DataSeries":
		nameValue, ok := object["name"].(string)
		if !ok {
			return validationError(MissingRequired, name, "expected name")
		}
		if err := stringLength(nameValue, child(name, "name"), 0, 20); err != nil {
			return err
		}
		data, err := sliceAt(object["data"], child(name, "data"))
		if err != nil {
			return err
		}
		return sliceLength(data, child(name, "data"), 1, 20)
	case "AxisConfig":
		categories, err := sliceAt(object["categories"], child(name, "categories"))
		if err != nil {
			return err
		}
		if err := sliceLength(categories, child(name, "categories"), 1, 20); err != nil {
			return err
		}
		seen := map[string]bool{}
		for index, raw := range categories {
			category, ok := raw.(string)
			if !ok {
				return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(name, "categories"), index), "expected a string")
			}
			if err := stringLength(category, fmt.Sprintf("%s[%d]", child(name, "categories"), index), 0, 20); err != nil {
				return err
			}
			if seen[category] {
				return validationError(InvalidUsage, child(name, "categories"), "expected unique labels")
			}
			seen[category] = true
		}
		for _, field := range []string{"x_label", "y_label"} {
			if value, ok := object[field].(string); ok {
				if err := stringLength(value, child(name, field), 0, 50); err != nil {
					return err
				}
			}
		}
	case "FeedbackButton":
		if _, ok := object["text"]; !ok {
			return validationError(MissingRequired, name, "expected text")
		}
		if _, ok := object["value"]; !ok {
			return validationError(MissingRequired, name, "expected value")
		}
		if err := textLength(object["text"], child(name, "text.text"), 0, 75); err != nil {
			return err
		}
		if value, ok := object["value"].(string); ok {
			if err := stringLength(value, child(name, "value"), 0, 2000); err != nil {
				return err
			}
		}
		if label, ok := object["accessibility_label"].(string); ok {
			if err := stringLength(label, child(name, "accessibility_label"), 0, 75); err != nil {
				return err
			}
		}
	case "Attachment":
		if color, ok := object["color"].(string); ok && color != "good" && color != "warning" && color != "danger" && !attachmentColorPattern.MatchString(color) {
			return validationError(TypeMismatch, child(name, "color"), "expected a six-digit hex color or Slack alias")
		}
	case "Message":
		rawChannel, present := object["channel"]
		if !present {
			return validationError(MissingRequired, child(name, "channel"), "expected channel")
		}
		channel, ok := rawChannel.(string)
		if !ok {
			return validationError(TypeMismatch, child(name, "channel"), "expected a string")
		}
		if err := stringLength(channel, child(name, "channel"), 1, 0); err != nil {
			return err
		}
		return validateMessageCollections(object, name)
	case "MessageResponse", "WebhookMessage":
		return validateMessageCollections(object, name)
	}
	return nil
}

func validateObject(object Object, path string) error {
	typeName, _ := object["type"].(string)
	for _, field := range requiredFields[typeName] {
		if _, ok := object[field]; !ok {
			return validationError(MissingRequired, path, "expected %s", field)
		}
	}
	if blockID, ok := object["block_id"].(string); ok {
		if err := stringLength(blockID, child(path, "block_id"), 0, 255); err != nil {
			return err
		}
	}
	if actionID, ok := object["action_id"].(string); ok {
		if err := stringLength(actionID, child(path, "action_id"), 0, 255); err != nil {
			return err
		}
	}
	if confirmTypes[typeName] {
		if confirm, ok := object["confirm"]; ok {
			value, err := objectAt(confirm, child(path, "confirm"))
			if err != nil {
				return err
			}
			if err := validateConfirmation(value, child(path, "confirm")); err != nil {
				return err
			}
		}
	}

	switch typeName {
	case "plain_text", "mrkdwn":
		return textLength(object, child(path, "text"), 1, 3000)
	case "icon":
		name, ok := object["name"].(string)
		if !ok || !slackIconNames[name] {
			return validationError(TypeMismatch, child(path, "name"), "unknown Slack icon")
		}
	case "section":
		_, hasText := object["text"]
		fieldsValue, hasFields := object["fields"]
		var fields []any
		if hasFields {
			var err error
			fields, err = sliceAt(fieldsValue, child(path, "fields"))
			if err != nil {
				return err
			}
		}
		if !hasText && len(fields) == 0 {
			return validationError(MissingRequired, path, "expected text, fields, or both")
		}
		if hasText {
			if err := textLength(object["text"], child(path, "text.text"), 0, 3000); err != nil {
				return err
			}
		}
		if hasFields {
			if err := sliceLength(fields, child(path, "fields"), 0, 10); err != nil {
				return err
			}
			for index, field := range fields {
				if err := textLength(field, fmt.Sprintf("%s[%d].text", child(path, "fields"), index), 0, 2000); err != nil {
					return err
				}
			}
		}
	case "header":
		return textLength(object["text"], child(path, "text.text"), 0, 150)
	case "button", "workflow_button":
		if err := textLength(object["text"], child(path, "text.text"), 0, 75); err != nil {
			return err
		}
		for field, maximum := range map[string]int{"url": 3000, "value": 2000, "accessibility_label": 75} {
			if value, ok := object[field].(string); ok {
				if err := stringLength(value, child(path, field), 0, maximum); err != nil {
					return err
				}
			}
		}
	case "icon_button":
		if object["icon"] != "trash" {
			return validationError(TypeMismatch, child(path, "icon"), "expected trash")
		}
		if value, ok := object["value"].(string); ok {
			if err := stringLength(value, child(path, "value"), 0, 2000); err != nil {
				return err
			}
		}
		if label, ok := object["accessibility_label"].(string); ok {
			if err := stringLength(label, child(path, "accessibility_label"), 0, 75); err != nil {
				return err
			}
		}
		if users, ok := object["visible_to_user_ids"].([]any); ok {
			if err := sliceLength(users, child(path, "visible_to_user_ids"), 0, 10); err != nil {
				return err
			}
		}
	case "feedback_buttons":
		for _, field := range []string{"positive_button", "negative_button"} {
			button, err := objectAt(object[field], child(path, field))
			if err != nil {
				return err
			}
			if err := validateBuilder("FeedbackButton", button); err != nil {
				return err
			}
		}
	case "file_input":
		if value, ok := number(object["max_files"]); ok && (value < 1 || value > 10) {
			return validationError(OutOfRange, child(path, "max_files"), "expected a value between 1 and 10")
		}
	case "plain_text_input":
		if value, ok := number(object["max_length"]); ok && value > 3000 {
			return validationError(OutOfRange, child(path, "max_length"), "exceeds maximum 3000")
		}
		if placeholder, ok := object["placeholder"]; ok {
			if err := textLength(placeholder, child(path, "placeholder.text"), 0, 150); err != nil {
				return err
			}
		}
	case "overflow", "checkboxes", "radio_buttons":
		options, err := sliceAt(object["options"], child(path, "options"))
		if err != nil {
			return err
		}
		maximumOptions := 10
		if typeName == "overflow" {
			maximumOptions = 5
		}
		if err := sliceLength(options, child(path, "options"), 1, maximumOptions); err != nil {
			return err
		}
		if err := validateOptions(options, child(path, "options")); err != nil {
			return err
		}
	case "url":
		value, _ := object["url"].(string)
		return stringLength(value, child(path, "url"), 1, 3000)
	case "static_select", "multi_static_select":
		_, hasOptions := object["options"]
		_, hasGroups := object["option_groups"]
		if hasOptions && hasGroups {
			return validationError(MutuallyExclusive, path, "options and option_groups cannot be provided together")
		}
		if hasOptions {
			options, err := sliceAt(object["options"], child(path, "options"))
			if err != nil {
				return err
			}
			if err := sliceLength(options, child(path, "options"), 0, 100); err != nil {
				return err
			}
			if err := validateOptions(options, child(path, "options")); err != nil {
				return err
			}
		}
		if hasGroups {
			groups, err := sliceAt(object["option_groups"], child(path, "option_groups"))
			if err != nil {
				return err
			}
			if err := sliceLength(groups, child(path, "option_groups"), 0, 100); err != nil {
				return err
			}
			for index, raw := range groups {
				group, err := objectAt(raw, fmt.Sprintf("%s[%d]", child(path, "option_groups"), index))
				if err != nil {
					return err
				}
				if err := validateBuilder("OptionGroup", group); err != nil {
					return err
				}
			}
		}
		if placeholder, ok := object["placeholder"]; ok {
			if err := textLength(placeholder, child(path, "placeholder.text"), 0, 150); err != nil {
				return err
			}
		}
	case "number_input":
		minimum, hasMin := number(object["min_value"])
		maximum, hasMax := number(object["max_value"])
		if hasMin && hasMax && minimum > maximum {
			return validationError(OutOfRange, path, "min_value cannot exceed max_value")
		}
	case "image":
		_, imageURL := object["image_url"]
		_, slackFile := object["slack_file"]
		if !imageURL && !slackFile {
			return validationError(MissingRequired, path, "expected image_url or slack_file")
		}
		if imageURL && slackFile {
			return validationError(MutuallyExclusive, path, "image_url and slack_file cannot be provided together")
		}
		if value, ok := object["image_url"].(string); ok {
			if err := stringLength(value, child(path, "image_url"), 0, 3000); err != nil {
				return err
			}
		}
		if value, ok := object["alt_text"].(string); ok {
			if err := stringLength(value, child(path, "alt_text"), 0, 2000); err != nil {
				return err
			}
		}
	case "context":
		elements, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		if err := sliceLength(elements, child(path, "elements"), 0, 10); err != nil {
			return err
		}
		for index, raw := range elements {
			element, err := objectAt(raw, fmt.Sprintf("%s[%d]", child(path, "elements"), index))
			if err != nil {
				return err
			}
			if !contextElementTypes[objectType(element)] {
				return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(path, "elements"), index), "expected text or image element")
			}
		}
	case "actions":
		elements, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		return sliceLength(elements, child(path, "elements"), 0, 25)
	case "alert":
		if err := textLength(object["text"], child(path, "text.text"), 0, 200); err != nil {
			return err
		}
		if level, ok := object["level"].(string); ok && !alertLevels[level] {
			return validationError(TypeMismatch, child(path, "level"), "unknown alert level")
		}
	case "card":
		_, hero := object["hero_image"]
		_, title := object["title"]
		_, actions := object["actions"]
		_, body := object["body"]
		if !hero && !title && !actions && !body {
			return validationError(MissingRequired, path, "expected hero_image, title, actions, or body")
		}
		if _, icon := object["icon"]; icon {
			if _, slackIcon := object["slack_icon"]; slackIcon {
				return validationError(MutuallyExclusive, path, "icon and slack_icon cannot be provided together")
			}
		}
		for field, maximum := range map[string]int{"title": 150, "subtitle": 150, "body": 200, "subtext": 200} {
			if value, ok := object[field]; ok {
				if err := textLength(value, child(path, field+".text"), 0, maximum); err != nil {
					return err
				}
			}
		}
		if values, ok := object["actions"].([]any); ok {
			if err := sliceLength(values, child(path, "actions"), 0, 3); err != nil {
				return err
			}
		}
	case "carousel":
		values, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		if err := sliceLength(values, child(path, "elements"), 1, 10); err != nil {
			return err
		}
		for index, raw := range values {
			card, err := objectAt(raw, fmt.Sprintf("%s[%d]", child(path, "elements"), index))
			if err != nil {
				return err
			}
			if objectType(card) != "card" {
				return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(path, "elements"), index), "expected a card")
			}
		}
	case "container":
		_, title := object["title"]
		_, richTitle := object["rich_text_title"]
		if !title && !richTitle {
			return validationError(MissingRequired, path, "expected title or rich_text_title")
		}
		if title {
			if err := textLength(object["title"], child(path, "title.text"), 0, 150); err != nil {
				return err
			}
		}
		if value, ok := object["subtitle"]; ok {
			if err := textLength(value, child(path, "subtitle.text"), 0, 150); err != nil {
				return err
			}
		}
		blocks, err := sliceAt(object["child_blocks"], child(path, "child_blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "child_blocks"), 1, 10); err != nil {
			return err
		}
		if width, ok := object["width"].(string); ok && !containerWidths[width] {
			return validationError(TypeMismatch, child(path, "width"), "unknown container width")
		}
		if object["default_collapsed"] == true && object["is_collapsible"] != true {
			return validationError(InvalidUsage, child(path, "default_collapsed"), "requires is_collapsible")
		}
		if object["has_header_divider"] == true && object["is_collapsible"] == true {
			return validationError(InvalidUsage, child(path, "has_header_divider"), "requires a non-collapsible container")
		}
	case "context_actions":
		values, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		if err := sliceLength(values, child(path, "elements"), 1, 5); err != nil {
			return err
		}
	case "data_table":
		if err := validateTable(object, path, true); err != nil {
			return err
		}
	case "table":
		if err := validateTable(object, path, false); err != nil {
			return err
		}
	case "data_visualization":
		if title, ok := object["title"].(string); ok {
			if err := stringLength(title, child(path, "title"), 0, 50); err != nil {
				return err
			}
		}
		if _, err := objectAt(object["chart"], child(path, "chart")); err != nil {
			return err
		}
	case "pie":
		segments, err := sliceAt(object["segments"], child(path, "segments"))
		if err != nil {
			return err
		}
		if err := sliceLength(segments, child(path, "segments"), 1, 12); err != nil {
			return err
		}
		for index, raw := range segments {
			segment, err := objectAt(raw, fmt.Sprintf("%s[%d]", child(path, "segments"), index))
			if err != nil {
				return err
			}
			if err := validateLabelValue(segment, fmt.Sprintf("%s[%d]", child(path, "segments"), index), 20, true); err != nil {
				return err
			}
		}
	case "bar", "area", "line":
		if err := validateSeriesChart(object, path); err != nil {
			return err
		}
	case "task_card":
		if status, ok := object["status"].(string); ok && !taskStatuses[status] {
			return validationError(TypeMismatch, child(path, "status"), "unknown task status")
		}
	case "input":
		if err := textLength(object["label"], child(path, "label.text"), 0, 2000); err != nil {
			return err
		}
		if value, ok := object["hint"]; ok {
			if err := textLength(value, child(path, "hint.text"), 0, 2000); err != nil {
				return err
			}
		}
		element, err := objectAt(object["element"], child(path, "element"))
		if err != nil {
			return err
		}
		if !inputElementTypes[objectType(element)] {
			return validationError(TypeMismatch, child(path, "element"), "expected an input-compatible element")
		}
	case "markdown":
		value, _ := object["text"].(string)
		return stringLength(value, child(path, "text"), 1, 12000)
	case "video":
		if value, ok := object["alt_text"].(string); ok {
			if err := stringLength(value, child(path, "alt_text"), 1, 200); err != nil {
				return err
			}
		}
		if err := textLength(object["title"], child(path, "title.text"), 0, 200); err != nil {
			return err
		}
		for field, maximum := range map[string]int{"author_name": 50, "provider_name": 50} {
			if value, ok := object[field].(string); ok {
				if err := stringLength(value, child(path, field), 0, maximum); err != nil {
					return err
				}
			}
		}
		if value, ok := object["description"]; ok {
			if err := textLength(value, child(path, "description.text"), 0, 200); err != nil {
				return err
			}
		}
	case "modal", "home":
		blocks, err := sliceAt(object["blocks"], child(path, "blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "blocks"), 1, 100); err != nil {
			return err
		}
		if err := validateSurface(blocks, typeName, child(path, "blocks")); err != nil {
			return err
		}
		if value, ok := object["private_metadata"].(string); ok {
			if err := stringLength(value, child(path, "private_metadata"), 0, 3000); err != nil {
				return err
			}
		}
		if value, ok := object["callback_id"].(string); ok {
			if err := stringLength(value, child(path, "callback_id"), 0, 255); err != nil {
				return err
			}
		}
		if typeName == "modal" {
			if _, submit := object["submit"]; !submit {
				for _, raw := range blocks {
					if block, ok := raw.(Object); ok && objectType(block) == "input" {
						return validationError(MissingRequired, child(path, "submit"), "required when the modal contains an input block")
					}
				}
			}
			for field, maximum := range map[string]int{"title": 24, "close": 24, "submit": 24} {
				if value, ok := object[field]; ok {
					if err := textLength(value, child(path, field+".text"), 0, maximum); err != nil {
						return err
					}
				}
			}
		}
	}

	fields := make([]string, 0, len(object))
	for field := range object {
		fields = append(fields, field)
	}
	sort.Strings(fields)
	for _, field := range fields {
		if field == "type" || field == "event_payload" {
			continue
		}
		value := object[field]
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
	case float64:
		if math.IsNaN(typed) || math.IsInf(typed, 0) {
			return validationError(TypeMismatch, path, "expected a finite number")
		}
	}
	return nil
}

func validateConfirmation(object Object, path string) error {
	for field, maximum := range map[string]int{"title": 100, "text": 300, "confirm": 30, "deny": 30} {
		value, ok := object[field]
		if !ok {
			return validationError(MissingRequired, path, "expected %s", field)
		}
		if err := textLength(value, child(path, field+".text"), 0, maximum); err != nil {
			return err
		}
	}
	return nil
}

func validateOption(object Object, path string) error {
	for _, field := range []string{"text", "value"} {
		if _, ok := object[field]; !ok {
			return validationError(MissingRequired, path, "expected %s", field)
		}
	}
	if err := textLength(object["text"], child(path, "text.text"), 0, 75); err != nil {
		return err
	}
	if value, ok := object["value"].(string); ok {
		if err := stringLength(value, child(path, "value"), 0, 150); err != nil {
			return err
		}
	}
	if value, ok := object["description"]; ok {
		if err := textLength(value, child(path, "description.text"), 0, 75); err != nil {
			return err
		}
	}
	if value, ok := object["url"].(string); ok {
		if err := stringLength(value, child(path, "url"), 0, 3000); err != nil {
			return err
		}
	}
	return nil
}

func validateOptions(values []any, path string) error {
	for index, raw := range values {
		option, err := objectAt(raw, fmt.Sprintf("%s[%d]", path, index))
		if err != nil {
			return err
		}
		if err := validateOption(option, fmt.Sprintf("%s[%d]", path, index)); err != nil {
			return err
		}
	}
	return nil
}

func validateTable(object Object, path string, dataTable bool) error {
	rows, err := sliceAt(object["rows"], child(path, "rows"))
	if err != nil {
		return err
	}
	minRows, maxRows, minColumns := 1, 100, 0
	if dataTable {
		minRows, maxRows, minColumns = 2, 201, 1
	}
	if err := sliceLength(rows, child(path, "rows"), minRows, maxRows); err != nil {
		return err
	}
	columns := -1
	contentLength := 0
	for rowIndex, raw := range rows {
		row, ok := raw.([]any)
		if !ok {
			return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(path, "rows"), rowIndex), "expected an array")
		}
		if err := sliceLength(row, fmt.Sprintf("%s[%d]", child(path, "rows"), rowIndex), minColumns, 20); err != nil {
			return err
		}
		if columns < 0 {
			columns = len(row)
		} else if len(row) != columns {
			return validationError(InvalidUsage, fmt.Sprintf("%s[%d]", child(path, "rows"), rowIndex), "column count differs")
		}
		for cellIndex, rawCell := range row {
			cellPath := fmt.Sprintf("%s[%d][%d]", child(path, "rows"), rowIndex, cellIndex)
			cell, err := objectAt(rawCell, cellPath)
			if err != nil {
				return err
			}
			allowed := tableCellTypes
			if dataTable {
				allowed = dataTableCellTypes
			}
			if !allowed[objectType(cell)] {
				return validationError(TypeMismatch, cellPath, "unsupported table cell")
			}
			if dataTable && rowIndex == 0 && objectType(cell) == "rich_text" {
				return validationError(TypeMismatch, cellPath, "header cells cannot contain rich text")
			}
			contentLength += textCharacterCount(cell)
			if dataTable && (objectType(cell) == "raw_text" || objectType(cell) == "raw_number") {
				if text, ok := cell["text"].(string); ok {
					if err := stringLength(text, child(cellPath, "text"), 1, 0); err != nil {
						return err
					}
				}
			}
		}
	}
	if settings, ok := object["column_settings"]; ok {
		values, err := sliceAt(settings, child(path, "column_settings"))
		if err != nil {
			return err
		}
		if err := sliceLength(values, child(path, "column_settings"), 0, 20); err != nil {
			return err
		}
		if len(values) != columns {
			return validationError(InvalidUsage, child(path, "column_settings"), "expected one entry for every column")
		}
	}
	if dataTable {
		if pageSize, ok := number(object["page_size"]); ok && (pageSize < 1 || pageSize > 100) {
			return validationError(OutOfRange, child(path, "page_size"), "expected a value between 1 and 100")
		}
		caption, ok := object["caption"].(string)
		if !ok {
			return validationError(TypeMismatch, child(path, "caption"), "expected a string")
		}
		if err := stringLength(caption, child(path, "caption"), 1, 0); err != nil {
			return err
		}
		if contentLength > 20000 {
			return validationError(LengthExceeded, child(path, "rows"), "content exceeds maximum 20000")
		}
	}
	return nil
}

func validateSeriesChart(object Object, path string) error {
	series, err := sliceAt(object["series"], child(path, "series"))
	if err != nil {
		return err
	}
	if err := sliceLength(series, child(path, "series"), 1, 12); err != nil {
		return err
	}
	axis, err := objectAt(object["axis_config"], child(path, "axis_config"))
	if err != nil {
		return err
	}
	if err := validateBuilder("AxisConfig", axis); err != nil {
		return err
	}
	categories, _ := axis["categories"].([]any)
	categorySet := map[string]bool{}
	for _, raw := range categories {
		if value, ok := raw.(string); ok {
			categorySet[value] = true
		}
	}
	names := map[string]bool{}
	for index, raw := range series {
		itemPath := fmt.Sprintf("%s[%d]", child(path, "series"), index)
		item, err := objectAt(raw, itemPath)
		if err != nil {
			return err
		}
		if err := validateBuilder("DataSeries", item); err != nil {
			return err
		}
		name, _ := item["name"].(string)
		if names[name] {
			return validationError(InvalidUsage, child(path, "series"), "series names must be unique")
		}
		names[name] = true
		points, _ := item["data"].([]any)
		seen := map[string]bool{}
		for pointIndex, rawPoint := range points {
			point, err := objectAt(rawPoint, fmt.Sprintf("%s.data[%d]", itemPath, pointIndex))
			if err != nil {
				return err
			}
			if err := validateLabelValue(point, fmt.Sprintf("%s.data[%d]", itemPath, pointIndex), 20, false); err != nil {
				return err
			}
			label, _ := point["label"].(string)
			seen[label] = true
		}
		if len(points) != len(categorySet) || len(seen) != len(categorySet) {
			return validationError(InvalidUsage, child(itemPath, "data"), "expected exactly one point for every axis category")
		}
		for label := range seen {
			if !categorySet[label] {
				return validationError(InvalidUsage, child(itemPath, "data"), "expected exactly one point for every axis category")
			}
		}
	}
	return nil
}

func validateLabelValue(object Object, path string, labelMax int, positive bool) error {
	label, ok := object["label"].(string)
	if !ok {
		return validationError(MissingRequired, path, "expected label")
	}
	if err := stringLength(label, child(path, "label"), 0, labelMax); err != nil {
		return err
	}
	value, ok := number(object["value"])
	if !ok || math.IsNaN(value) || math.IsInf(value, 0) {
		return validationError(TypeMismatch, child(path, "value"), "expected a finite number")
	}
	if positive && value <= 0 {
		return validationError(OutOfRange, child(path, "value"), "expected a value greater than 0")
	}
	return nil
}

func validateMessageCollections(object Object, path string) error {
	if raw, ok := object["blocks"]; ok {
		blocks, err := sliceAt(raw, child(path, "blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "blocks"), 0, 50); err != nil {
			return err
		}
		if err := validateSurface(blocks, "message", child(path, "blocks")); err != nil {
			return err
		}
	}
	if raw, ok := object["attachments"]; ok {
		attachments, err := sliceAt(raw, child(path, "attachments"))
		if err != nil {
			return err
		}
		if err := sliceLength(attachments, child(path, "attachments"), 0, 100); err != nil {
			return err
		}
	}
	return nil
}

func validateSurface(blocks []any, surface, path string) error {
	allowed := surfaceBlocks[surface]
	for index, raw := range blocks {
		block, err := objectAt(raw, fmt.Sprintf("%s[%d]", path, index))
		if err != nil {
			return err
		}
		blockType := objectType(block)
		if !allowed[blockType] {
			return validationError(TypeMismatch, fmt.Sprintf("%s[%d].type", path, index), "block type %s is not supported on %s surfaces", blockType, surface)
		}
	}
	return nil
}

func objectType(object Object) string {
	value, _ := object["type"].(string)
	return value
}

func objectAt(value any, path string) (Object, error) {
	object, ok := value.(Object)
	if !ok {
		return nil, validationError(TypeMismatch, path, "expected an object")
	}
	return object, nil
}

func sliceAt(value any, path string) ([]any, error) {
	values, ok := value.([]any)
	if !ok {
		return nil, validationError(TypeMismatch, path, "expected an array")
	}
	return values, nil
}

func textValue(value any) (string, bool) {
	if text, ok := value.(string); ok {
		return text, true
	}
	if object, ok := value.(Object); ok {
		text, ok := object["text"].(string)
		return text, ok
	}
	return "", false
}

func textLength(value any, path string, minimum, maximum int) error {
	text, ok := textValue(value)
	if !ok {
		return validationError(TypeMismatch, path, "expected text")
	}
	return stringLength(text, path, minimum, maximum)
}

func stringLength(value, path string, minimum, maximum int) error {
	size := utf8.RuneCountInString(value)
	if minimum > 0 && size < minimum {
		return validationError(LengthExceeded, path, "%d is less than minimum %d", size, minimum)
	}
	if maximum > 0 && size > maximum {
		return validationError(LengthExceeded, path, "%d exceeds maximum %d", size, maximum)
	}
	return nil
}

func sliceLength(value []any, path string, minimum, maximum int) error {
	if minimum > 0 && len(value) < minimum {
		return validationError(LengthExceeded, path, "%d is less than minimum %d", len(value), minimum)
	}
	if maximum > 0 && len(value) > maximum {
		return validationError(LengthExceeded, path, "%d exceeds maximum %d", len(value), maximum)
	}
	return nil
}

func number(value any) (float64, bool) {
	switch typed := value.(type) {
	case int:
		return float64(typed), true
	case int32:
		return float64(typed), true
	case int64:
		return float64(typed), true
	case float32:
		return float64(typed), true
	case float64:
		return typed, true
	default:
		return 0, false
	}
}

func textCharacterCount(value any) int {
	switch typed := value.(type) {
	case Object:
		total := 0
		for key, nested := range typed {
			if key == "text" {
				if text, ok := nested.(string); ok {
					total += utf8.RuneCountInString(text)
					continue
				}
			}
			total += textCharacterCount(nested)
		}
		return total
	case []any:
		total := 0
		for _, nested := range typed {
			total += textCharacterCount(nested)
		}
		return total
	default:
		return 0
	}
}
