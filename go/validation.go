package slackblocks

import (
	"fmt"
	"math"
	"regexp"
	"sort"
	"unicode/utf8"
)

var requiredFields = map[string][]string{
	"actions":                {"elements"},
	"alert":                  {"text"},
	"area":                   {"series", "axis_config"},
	"bar":                    {"series", "axis_config"},
	"button":                 {"text"},
	"carousel":               {"elements"},
	"channel":                {"channel_id"},
	"checkboxes":             {"options"},
	"container":              {"child_blocks"},
	"context":                {"elements"},
	"context_actions":        {"elements"},
	"data_table":             {"rows", "caption"},
	"data_visualization":     {"title", "chart"},
	"emoji":                  {"name"},
	"feedback_buttons":       {"positive_button", "negative_button"},
	"file":                   {"external_id", "source"},
	"header":                 {"text"},
	"home":                   {"blocks"},
	"icon":                   {"name"},
	"icon_button":            {"text", "icon"},
	"image":                  {"alt_text"},
	"input":                  {"label", "element"},
	"line":                   {"series", "axis_config"},
	"link":                   {"url"},
	"markdown":               {"text"},
	"modal":                  {"title", "blocks"},
	"number_input":           {"is_decimal_allowed"},
	"overflow":               {"options"},
	"pie":                    {"segments"},
	"plan":                   {"title", "tasks"},
	"radio_buttons":          {"options"},
	"raw_number":             {"value", "text"},
	"raw_text":               {"text"},
	"rich_text":              {"elements"},
	"rich_text_input":        {"action_id"},
	"rich_text_list":         {"style", "elements"},
	"rich_text_preformatted": {"elements"},
	"rich_text_quote":        {"elements"},
	"rich_text_section":      {"elements"},
	"table":                  {"rows"},
	"task_card":              {"task_id", "title", "status"},
	"text":                   {"text"},
	"url":                    {"url", "text"},
	"user":                   {"user_id"},
	"usergroup":              {"usergroup_id"},
	"video":                  {"alt_text", "thumbnail_url", "title", "video_url"},
	"workflow_button":        {"text", "workflow", "action_id"},
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

var inputPlaceholderMaxLengths = map[string]int{
	"plain_text_input": limitPlainTextInputPlaceholderMaxLength,
	"email_text_input": limitEmailInputPlaceholderMaxLength,
	"url_text_input":   limitURLInputPlaceholderMaxLength,
	"number_input":     limitNumberInputPlaceholderMaxLength,
	"datepicker":       limitDatePickerPlaceholderMaxLength,
	"timepicker":       limitTimePickerPlaceholderMaxLength,
	"rich_text_input":  limitRichTextInputPlaceholderMaxLength,
}

var attachmentColorPattern = regexp.MustCompile(`^#[0-9a-fA-F]{6}$`)

var slackFileIDPattern = regexp.MustCompile(`^F[A-Z0-9]{8,}$`)

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
	conversationTypes   = stringSet("im", "mpim", "private", "public")
	multiSelectTypes    = stringSet("multi_channels_select", "multi_conversations_select", "multi_external_select", "multi_static_select", "multi_users_select")
	tableCellTypes      = stringSet("raw_text", "rich_text")
	dataTableCellTypes  = stringSet("raw_text", "rich_text", "raw_number")
)

var surfaceBlocks = map[string]map[string]bool{
	"message": stringSet("actions", "card", "carousel", "container", "context", "context_actions", "data_table", "data_visualization", "divider", "file", "header", "image", "input", "markdown", "plan", "rich_text", "section", "table", "task_card", "video"),
	"modal":   stringSet("actions", "alert", "card", "context", "divider", "header", "image", "input", "rich_text", "section", "video"),
	"home":    stringSet("actions", "card", "carousel", "container", "context", "data_table", "data_visualization", "divider", "header", "image", "input", "rich_text", "section", "table", "video"),
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
		if err := textLength(label, child(name, "label.text"), 0, limitOptionGroupLabelMaxLength); err != nil {
			return err
		}
		options, err := sliceAt(object["options"], child(name, "options"))
		if err != nil {
			return err
		}
		if err := sliceLength(options, child(name, "options"), limitOptionGroupOptionsMinItems, limitOptionGroupOptionsMaxItems); err != nil {
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
		if raw, ok := object["include"]; ok {
			include, err := sliceAt(raw, child(name, "include"))
			if err != nil {
				return err
			}
			if err := sliceLength(include, child(name, "include"), limitConversationFilterIncludeMinItems, 0); err != nil {
				return err
			}
			for index, value := range include {
				if kind, ok := value.(string); !ok || !conversationTypes[kind] {
					return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(name, "include"), index), "expected im, mpim, private, or public")
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
	case "DispatchActionConfiguration":
		raw, ok := object["trigger_actions_on"]
		if !ok {
			return validationError(MissingRequired, child(name, "trigger_actions_on"), "expected trigger_actions_on")
		}
		triggers, err := sliceAt(raw, child(name, "trigger_actions_on"))
		if err != nil {
			return err
		}
		return sliceLength(triggers, child(name, "trigger_actions_on"), limitDispatchActionConfigurationTriggerActionsOnMinItems, limitDispatchActionConfigurationTriggerActionsOnMaxItems)
	case "SlackFile":
		return validateSlackFile(object, name)
	case "ChartSegment":
		return validateLabelValue(object, name, limitDataVisualizationSegmentLabelMaxLength, true)
	case "DataPoint":
		return validateLabelValue(object, name, limitDataVisualizationPointLabelMaxLength, false)
	case "DataSeries":
		nameValue, ok := object["name"].(string)
		if !ok {
			return validationError(MissingRequired, name, "expected name")
		}
		if err := stringLength(nameValue, child(name, "name"), 0, limitDataVisualizationSeriesNameMaxLength); err != nil {
			return err
		}
		data, err := sliceAt(object["data"], child(name, "data"))
		if err != nil {
			return err
		}
		return sliceLength(data, child(name, "data"), limitDataVisualizationDataMinItems, limitDataVisualizationDataMaxItems)
	case "AxisConfig":
		categories, err := sliceAt(object["categories"], child(name, "categories"))
		if err != nil {
			return err
		}
		if err := sliceLength(categories, child(name, "categories"), limitDataVisualizationCategoriesMinItems, limitDataVisualizationCategoriesMaxItems); err != nil {
			return err
		}
		seen := map[string]bool{}
		for index, raw := range categories {
			category, ok := raw.(string)
			if !ok {
				return validationError(TypeMismatch, fmt.Sprintf("%s[%d]", child(name, "categories"), index), "expected a string")
			}
			if err := stringLength(category, fmt.Sprintf("%s[%d]", child(name, "categories"), index), 0, limitDataVisualizationCategoryLabelMaxLength); err != nil {
				return err
			}
			if seen[category] {
				return validationError(InvalidUsage, child(name, "categories"), "expected unique labels")
			}
			seen[category] = true
		}
		for _, field := range []string{"x_label", "y_label"} {
			if value, ok := object[field].(string); ok {
				if err := stringLength(value, child(name, field), 0, limitDataVisualizationAxisLabelMaxLength); err != nil {
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
		if err := textLength(object["text"], child(name, "text.text"), 0, limitFeedbackButtonTextMaxLength); err != nil {
			return err
		}
		if value, ok := object["value"].(string); ok {
			if err := stringLength(value, child(name, "value"), 0, limitFeedbackButtonValueMaxLength); err != nil {
				return err
			}
		}
		if label, ok := object["accessibility_label"].(string); ok {
			if err := stringLength(label, child(name, "accessibility_label"), 0, limitFeedbackButtonAccessibilityLabelMaxLength); err != nil {
				return err
			}
		}
	case "Attachment":
		if _, ok := object["blocks"]; !ok {
			return validationError(MissingRequired, child(name, "blocks"), "expected blocks")
		}
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
		if err := stringLength(channel, child(name, "channel"), limitMessageChannelMinLength, 0); err != nil {
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
		if err := stringLength(blockID, child(path, "block_id"), 0, limitBlockIDMaxLength); err != nil {
			return err
		}
	}
	if actionID, ok := object["action_id"].(string); ok {
		if err := stringLength(actionID, child(path, "action_id"), 0, limitActionIDMaxLength); err != nil {
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
	if maximum, ok := inputPlaceholderMaxLengths[typeName]; ok {
		if placeholder, ok := object["placeholder"]; ok {
			if err := textLength(placeholder, child(path, "placeholder.text"), 0, maximum); err != nil {
				return err
			}
		}
	}
	if multiSelectTypes[typeName] {
		if err := numberBetween(object, "max_selected_items", path, limitMultiSelectMaxSelectedItemsMin, math.MaxInt); err != nil {
			return err
		}
	}
	if typeName == "conversations_select" || typeName == "multi_conversations_select" {
		if raw, ok := object["filter"]; ok {
			filter, err := objectAt(raw, child(path, "filter"))
			if err != nil {
				return err
			}
			if err := validateBuilder("ConversationFilter", filter); err != nil {
				return err
			}
		}
	}
	if config, ok := object["dispatch_action_config"]; ok {
		value, err := objectAt(config, child(path, "dispatch_action_config"))
		if err != nil {
			return err
		}
		if err := validateBuilder("DispatchActionConfiguration", value); err != nil {
			return err
		}
	}

	switch typeName {
	case "plain_text", "mrkdwn":
		return textLength(object, child(path, "text"), limitTextMinLength, limitTextMaxLength)
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
			if err := textLength(object["text"], child(path, "text.text"), 0, limitSectionTextMaxLength); err != nil {
				return err
			}
		}
		if hasFields {
			if err := sliceLength(fields, child(path, "fields"), 0, limitSectionFieldsMaxItems); err != nil {
				return err
			}
			for index, field := range fields {
				if err := textLength(field, fmt.Sprintf("%s[%d].text", child(path, "fields"), index), 0, limitSectionFieldsItemMaxLength); err != nil {
					return err
				}
			}
		}
	case "header":
		return textLength(object["text"], child(path, "text.text"), 0, limitHeaderTextMaxLength)
	case "button", "workflow_button":
		textMaximum, labelMaximum := limitButtonTextMaxLength, limitButtonAccessibilityLabelMaxLength
		if typeName == "workflow_button" {
			textMaximum, labelMaximum = limitWorkflowButtonTextMaxLength, limitWorkflowButtonAccessibilityLabelMaxLength
		}
		if err := textLength(object["text"], child(path, "text.text"), 0, textMaximum); err != nil {
			return err
		}
		for field, maximum := range map[string]int{"url": limitButtonURLMaxLength, "value": limitButtonValueMaxLength, "accessibility_label": labelMaximum} {
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
			if err := stringLength(value, child(path, "value"), 0, limitIconButtonValueMaxLength); err != nil {
				return err
			}
		}
		if label, ok := object["accessibility_label"].(string); ok {
			if err := stringLength(label, child(path, "accessibility_label"), 0, limitIconButtonAccessibilityLabelMaxLength); err != nil {
				return err
			}
		}
		if users, ok := object["visible_to_user_ids"].([]any); ok {
			if err := sliceLength(users, child(path, "visible_to_user_ids"), 0, limitIconButtonVisibleToUserIDsMaxItems); err != nil {
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
		if value, ok := number(object["max_files"]); ok && (value < limitFileInputMaxFilesMin || value > limitFileInputMaxFilesMax) {
			return validationError(OutOfRange, child(path, "max_files"), "expected a value between %d and %d", limitFileInputMaxFilesMin, limitFileInputMaxFilesMax)
		}
	case "plain_text_input":
		if value, ok := number(object["max_length"]); ok && value > limitPlainTextInputMaxLengthMax {
			return validationError(OutOfRange, child(path, "max_length"), "exceeds maximum %d", limitPlainTextInputMaxLengthMax)
		}
		if err := numberBetween(object, "max_length", path, limitPlainTextInputMaxLengthMin, limitPlainTextInputMaxLengthMax); err != nil {
			return err
		}
		if err := numberBetween(object, "min_length", path, limitPlainTextInputMinLengthMin, limitPlainTextInputMinLengthMax); err != nil {
			return err
		}
	case "rich_text_input":
		if err := numberBetween(object, "min_lines", path, limitRichTextInputMinLinesMin, limitRichTextInputMinLinesMax); err != nil {
			return err
		}
		if err := numberBetween(object, "max_lines", path, limitRichTextInputMaxLinesMin, limitRichTextInputMaxLinesMax); err != nil {
			return err
		}
	case "rich_text_list":
		if err := numberBetween(object, "indent", path, limitRichTextListIndentMin, limitRichTextListIndentMax); err != nil {
			return err
		}
		if err := numberBetween(object, "offset", path, limitRichTextListOffsetMin, math.MaxInt); err != nil {
			return err
		}
		if err := numberBetween(object, "border", path, limitRichTextListBorderMin, limitRichTextListBorderMax); err != nil {
			return err
		}
	case "rich_text_quote":
		if err := numberBetween(object, "border", path, limitRichTextQuoteBorderMin, limitRichTextQuoteBorderMax); err != nil {
			return err
		}
	case "rich_text_preformatted":
		if err := numberBetween(object, "border", path, limitRichTextPreformattedBorderMin, limitRichTextPreformattedBorderMax); err != nil {
			return err
		}
	case "overflow", "checkboxes", "radio_buttons":
		options, err := sliceAt(object["options"], child(path, "options"))
		if err != nil {
			return err
		}
		minimumOptions, maximumOptions := limitCheckboxesOptionsMinItems, limitCheckboxesOptionsMaxItems
		switch typeName {
		case "overflow":
			minimumOptions, maximumOptions = limitOverflowOptionsMinItems, limitOverflowOptionsMaxItems
		case "radio_buttons":
			minimumOptions, maximumOptions = limitRadioButtonsOptionsMinItems, limitRadioButtonsOptionsMaxItems
		}
		if err := sliceLength(options, child(path, "options"), minimumOptions, maximumOptions); err != nil {
			return err
		}
		if err := validateOptions(options, child(path, "options")); err != nil {
			return err
		}
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
			if err := sliceLength(options, child(path, "options"), 0, limitSelectOptionsMaxItems); err != nil {
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
			if err := sliceLength(groups, child(path, "option_groups"), 0, limitSelectOptionGroupsMaxItems); err != nil {
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
			if err := textLength(placeholder, child(path, "placeholder.text"), 0, limitSelectPlaceholderMaxLength); err != nil {
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
		if slackFile {
			file, err := objectAt(object["slack_file"], child(path, "slack_file"))
			if err != nil {
				return err
			}
			if err := validateSlackFile(file, child(path, "slack_file")); err != nil {
				return err
			}
		}
		// Image blocks and image elements share the "image" type. Only a block can carry a
		// title or block_id; anything else is checked against the image element's limits.
		urlMaximum, altMaximum := limitImageElementImageURLMaxLength, limitImageElementAltTextMaxLength
		_, title := object["title"]
		_, blockID := object["block_id"]
		if title || blockID {
			urlMaximum, altMaximum = limitImageImageURLMaxLength, limitImageAltTextMaxLength
		}
		if value, ok := object["image_url"].(string); ok {
			if err := stringLength(value, child(path, "image_url"), 0, urlMaximum); err != nil {
				return err
			}
		}
		if value, ok := object["alt_text"].(string); ok {
			if err := stringLength(value, child(path, "alt_text"), 0, altMaximum); err != nil {
				return err
			}
		}
		if title {
			if err := textLength(object["title"], child(path, "title.text"), 0, limitImageTitleMaxLength); err != nil {
				return err
			}
		}
	case "context":
		elements, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		if err := sliceLength(elements, child(path, "elements"), 0, limitContextElementsMaxItems); err != nil {
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
		return sliceLength(elements, child(path, "elements"), 0, limitActionsElementsMaxItems)
	case "alert":
		if err := textLength(object["text"], child(path, "text.text"), 0, limitAlertTextMaxLength); err != nil {
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
		for field, maximum := range map[string]int{"title": limitCardTitleMaxLength, "subtitle": limitCardSubtitleMaxLength, "body": limitCardBodyMaxLength, "subtext": limitCardSubtextMaxLength} {
			if value, ok := object[field]; ok {
				if err := textLength(value, child(path, field+".text"), 0, maximum); err != nil {
					return err
				}
			}
		}
		if values, ok := object["actions"].([]any); ok {
			if err := sliceLength(values, child(path, "actions"), 0, limitCardActionsMaxItems); err != nil {
				return err
			}
		}
	case "carousel":
		values, err := sliceAt(object["elements"], child(path, "elements"))
		if err != nil {
			return err
		}
		if err := sliceLength(values, child(path, "elements"), limitCarouselElementsMinItems, limitCarouselElementsMaxItems); err != nil {
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
			if err := textLength(object["title"], child(path, "title.text"), 0, limitContainerTitleMaxLength); err != nil {
				return err
			}
		}
		if value, ok := object["subtitle"]; ok {
			if err := textLength(value, child(path, "subtitle.text"), 0, limitContainerSubtitleMaxLength); err != nil {
				return err
			}
		}
		blocks, err := sliceAt(object["child_blocks"], child(path, "child_blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "child_blocks"), limitContainerChildBlocksMinItems, limitContainerChildBlocksMaxItems); err != nil {
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
		if err := sliceLength(values, child(path, "elements"), 1, limitContextActionsElementsMaxItems); err != nil {
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
			if err := stringLength(title, child(path, "title"), 0, limitDataVisualizationTitleMaxLength); err != nil {
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
		if err := sliceLength(segments, child(path, "segments"), limitDataVisualizationSegmentsMinItems, limitDataVisualizationSegmentsMaxItems); err != nil {
			return err
		}
		for index, raw := range segments {
			segment, err := objectAt(raw, fmt.Sprintf("%s[%d]", child(path, "segments"), index))
			if err != nil {
				return err
			}
			if err := validateLabelValue(segment, fmt.Sprintf("%s[%d]", child(path, "segments"), index), limitDataVisualizationSegmentLabelMaxLength, true); err != nil {
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
		if object["status"] == "pending" {
			return validationError(TypeMismatch, child(path, "status"), "pending is only valid for plan tasks")
		}
	case "plan":
		if err := validatePlanTasks(object["tasks"], child(path, "tasks")); err != nil {
			return err
		}
	case "input":
		if err := textLength(object["label"], child(path, "label.text"), 0, limitInputLabelMaxLength); err != nil {
			return err
		}
		if value, ok := object["hint"]; ok {
			if err := textLength(value, child(path, "hint.text"), 0, limitInputHintMaxLength); err != nil {
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
		return stringLength(value, child(path, "text"), 0, limitMarkdownTextMaxLength)
	case "video":
		if value, ok := object["alt_text"].(string); ok {
			if err := stringLength(value, child(path, "alt_text"), 0, limitVideoAltTextMaxLength); err != nil {
				return err
			}
		}
		if err := textLength(object["title"], child(path, "title.text"), 0, limitVideoTitleMaxLength); err != nil {
			return err
		}
		for field, maximum := range map[string]int{
			"author_name":       limitVideoAuthorNameMaxLength,
			"provider_name":     limitVideoProviderNameMaxLength,
			"thumbnail_url":     limitVideoThumbnailURLMaxLength,
			"video_url":         limitVideoVideoURLMaxLength,
			"title_url":         limitVideoTitleURLMaxLength,
			"provider_icon_url": limitVideoProviderIconURLMaxLength,
		} {
			if value, ok := object[field].(string); ok {
				if err := stringLength(value, child(path, field), 0, maximum); err != nil {
					return err
				}
			}
		}
		if value, ok := object["description"]; ok {
			if err := textLength(value, child(path, "description.text"), 0, limitVideoDescriptionMaxLength); err != nil {
				return err
			}
		}
	case "modal", "home":
		blocks, err := sliceAt(object["blocks"], child(path, "blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "blocks"), 0, limitViewBlocksMaxItems); err != nil {
			return err
		}
		if err := validateSurface(blocks, typeName, child(path, "blocks")); err != nil {
			return err
		}
		if value, ok := object["private_metadata"].(string); ok {
			if err := stringLength(value, child(path, "private_metadata"), 0, limitViewPrivateMetadataMaxLength); err != nil {
				return err
			}
		}
		if value, ok := object["callback_id"].(string); ok {
			if err := stringLength(value, child(path, "callback_id"), 0, limitViewCallbackIDMaxLength); err != nil {
				return err
			}
		}
		if value, ok := object["external_id"].(string); ok {
			if err := stringLength(value, child(path, "external_id"), 0, limitViewExternalIDMaxLength); err != nil {
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
			for field, maximum := range map[string]int{"title": limitViewTitleMaxLength, "close": limitViewCloseMaxLength, "submit": limitViewSubmitMaxLength} {
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
		if field == "type" || field == "event_payload" || (typeName == "plan" && field == "tasks") {
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
	for field, maximum := range map[string]int{"title": limitConfirmationTitleMaxLength, "text": limitConfirmationTextMaxLength, "confirm": limitConfirmationConfirmMaxLength, "deny": limitConfirmationDenyMaxLength} {
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
	if err := textLength(object["text"], child(path, "text.text"), 0, limitOptionTextMaxLength); err != nil {
		return err
	}
	if value, ok := object["value"].(string); ok {
		if err := stringLength(value, child(path, "value"), 0, limitOptionValueMaxLength); err != nil {
			return err
		}
	}
	if value, ok := object["description"]; ok {
		if err := textLength(value, child(path, "description.text"), 0, limitOptionDescriptionMaxLength); err != nil {
			return err
		}
	}
	if value, ok := object["url"].(string); ok {
		if err := stringLength(value, child(path, "url"), 0, limitOptionURLMaxLength); err != nil {
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
	minRows, maxRows, minColumns, maxColumns := 1, limitTableRowsMaxItems, 0, limitTableColumnsMaxItems
	if dataTable {
		minRows, maxRows, minColumns, maxColumns = limitDataTableRowsMinItems, limitDataTableRowsMaxItems, limitDataTableColumnsMinItems, limitDataTableColumnsMaxItems
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
		if err := sliceLength(row, fmt.Sprintf("%s[%d]", child(path, "rows"), rowIndex), minColumns, maxColumns); err != nil {
			return err
		}
		// Only data tables require every row to have the same number of cells.
		if dataTable {
			if columns < 0 {
				columns = len(row)
			} else if len(row) != columns {
				return validationError(InvalidUsage, fmt.Sprintf("%s[%d]", child(path, "rows"), rowIndex), "column count differs")
			}
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
					if err := stringLength(text, child(cellPath, "text"), limitDataTableCellTextMinLength, 0); err != nil {
						return err
					}
				}
			}
		}
	}
	if settings, ok := object["column_settings"]; ok {
		if dataTable {
			return validationError(InvalidUsage, child(path, "column_settings"), "data tables do not support column_settings")
		}
		values, err := sliceAt(settings, child(path, "column_settings"))
		if err != nil {
			return err
		}
		if err := sliceLength(values, child(path, "column_settings"), 0, limitTableColumnSettingsMaxItems); err != nil {
			return err
		}
	}
	if dataTable {
		if pageSize, ok := number(object["page_size"]); ok && (pageSize < limitDataTablePageSizeMin || pageSize > limitDataTablePageSizeMax) {
			return validationError(OutOfRange, child(path, "page_size"), "expected a value between %d and %d", limitDataTablePageSizeMin, limitDataTablePageSizeMax)
		}
		if err := numberBetween(object, "row_header_column_index", path, limitDataTableRowHeaderColumnIndexMin, math.MaxInt); err != nil {
			return err
		}
		caption, ok := object["caption"].(string)
		if !ok {
			return validationError(TypeMismatch, child(path, "caption"), "expected a string")
		}
		if err := stringLength(caption, child(path, "caption"), 1, 0); err != nil {
			return err
		}
		if contentLength > limitDataTableContentMaxLength {
			return validationError(LengthExceeded, child(path, "rows"), "content exceeds maximum %d", limitDataTableContentMaxLength)
		}
	}
	return nil
}

func validateSeriesChart(object Object, path string) error {
	series, err := sliceAt(object["series"], child(path, "series"))
	if err != nil {
		return err
	}
	if err := sliceLength(series, child(path, "series"), limitDataVisualizationSeriesMinItems, limitDataVisualizationSeriesMaxItems); err != nil {
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
			if err := validateLabelValue(point, fmt.Sprintf("%s.data[%d]", itemPath, pointIndex), limitDataVisualizationPointLabelMaxLength, false); err != nil {
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
	if positive && value <= limitDataVisualizationSegmentValueExclusiveMin {
		return validationError(OutOfRange, child(path, "value"), "expected a value greater than %d", limitDataVisualizationSegmentValueExclusiveMin)
	}
	return nil
}

func validateMessageCollections(object Object, path string) error {
	if raw, ok := object["blocks"]; ok {
		blocks, err := sliceAt(raw, child(path, "blocks"))
		if err != nil {
			return err
		}
		if err := sliceLength(blocks, child(path, "blocks"), 0, limitMessageBlocksMaxItems); err != nil {
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
		if err := sliceLength(attachments, child(path, "attachments"), 0, limitMessageAttachmentsMaxItems); err != nil {
			return err
		}
	}
	markdown, tableContent := messageTotals(object)
	if markdown > limitMarkdownTotalTextMaxLength {
		return validationError(LengthExceeded, path, "markdown block text totals %d characters, exceeding maximum %d", markdown, limitMarkdownTotalTextMaxLength)
	}
	if tableContent > limitDataTableTotalContentMaxLength {
		return validationError(LengthExceeded, path, "data table cell text totals %d characters, exceeding maximum %d", tableContent, limitDataTableTotalContentMaxLength)
	}
	return nil
}

// messageTotals counts the markdown block text and data table cell text anywhere in a message,
// including attachment blocks and container children.
func messageTotals(value any) (markdown, tableContent int) {
	switch typed := value.(type) {
	case Object:
		switch objectType(typed) {
		case "markdown":
			text, _ := typed["text"].(string)
			return utf8.RuneCountInString(text), 0
		case "data_table":
			return 0, textCharacterCount(typed["rows"])
		}
		for _, nested := range typed {
			nestedMarkdown, nestedContent := messageTotals(nested)
			markdown += nestedMarkdown
			tableContent += nestedContent
		}
	case []any:
		for _, nested := range typed {
			nestedMarkdown, nestedContent := messageTotals(nested)
			markdown += nestedMarkdown
			tableContent += nestedContent
		}
	}
	return markdown, tableContent
}

// validatePlanTasks checks a plan's task cards. Plan tasks are sent without a type and, unlike
// standalone task cards, may be pending.
func validatePlanTasks(value any, path string) error {
	tasks, err := sliceAt(value, path)
	if err != nil {
		return err
	}
	if err := sliceLength(tasks, path, 0, limitPlanTasksMaxItems); err != nil {
		return err
	}
	seen := map[string]bool{}
	for index, raw := range tasks {
		taskPath := fmt.Sprintf("%s[%d]", path, index)
		task, err := objectAt(raw, taskPath)
		if err != nil {
			return err
		}
		for _, field := range requiredFields["task_card"] {
			if _, ok := task[field]; !ok {
				return validationError(MissingRequired, taskPath, "expected %s", field)
			}
		}
		if status, ok := task["status"].(string); !ok || !taskStatuses[status] {
			return validationError(TypeMismatch, child(taskPath, "status"), "unknown task status")
		}
		if id, ok := task["task_id"].(string); ok {
			if seen[id] {
				return validationError(InvalidUsage, child(taskPath, "task_id"), "task IDs must be unique within a plan")
			}
			seen[id] = true
		}
		untyped := Object{}
		for key, nested := range task {
			if key != "type" {
				untyped[key] = nested
			}
		}
		if err := validateObject(untyped, taskPath); err != nil {
			return err
		}
	}
	return nil
}

func validateSlackFile(object Object, path string) error {
	_, hasID := object["id"]
	_, hasURL := object["url"]
	if hasID == hasURL {
		return validationError(MutuallyExclusive, path, "expected exactly one of id or url")
	}
	if id, ok := object["id"].(string); ok && !slackFileIDPattern.MatchString(id) {
		return validationError(TypeMismatch, child(path, "id"), "expected an ID matching %s", slackFileIDPattern)
	}
	return nil
}

// numberBetween checks an optional numeric field against an inclusive range. Pass math.MaxInt
// as the maximum for a field with only a lower bound.
func numberBetween(object Object, field, path string, minimum, maximum int) error {
	value, ok := number(object[field])
	if !ok || (value >= float64(minimum) && value <= float64(maximum)) {
		return nil
	}
	if maximum == math.MaxInt {
		return validationError(OutOfRange, child(path, field), "expected a value of at least %d", minimum)
	}
	return validationError(OutOfRange, child(path, field), "expected a value between %d and %d", minimum, maximum)
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
