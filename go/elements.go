package slackblocks

func elementBuilder(name, objectType string) *builder {
	return newBuilder(name, objectType).coerce("placeholder", plainTextLike)
}

// NewButton creates an interactive button.
func NewButton() *ButtonBuilder {
	return newButtonBuilder(elementBuilder("Button", "button").coerce("text", plainTextLike))
}

// NewFeedbackButton creates one positive or negative feedback choice.
func NewFeedbackButton() *FeedbackButtonBuilder {
	return newFeedbackButtonBuilder(newBuilder("FeedbackButton", "").coerce("text", plainTextLike))
}

// NewFeedbackButtons creates a paired positive/negative feedback control.
func NewFeedbackButtons() *FeedbackButtonsBuilder {
	return newFeedbackButtonsBuilder(elementBuilder("FeedbackButtons", "feedback_buttons"))
}

// NewIconButton creates a compact context action.
func NewIconButton() *IconButtonBuilder {
	builder := elementBuilder("IconButton", "icon_button").coerce("text", plainTextLike)
	builder.values["icon"] = "trash"
	return newIconButtonBuilder(builder)
}

// NewURLSource creates a source link for a task card.
func NewURLSource() *URLSourceBuilder { return newURLSourceBuilder(elementBuilder("URLSource", "url")) }

// NewCheckboxes creates a checkbox group.
func NewCheckboxes() *CheckboxesBuilder {
	return newCheckboxesBuilder(elementBuilder("Checkboxes", "checkboxes"))
}

// NewDatePicker creates a calendar date picker.
func NewDatePicker() *DatePickerBuilder {
	return newDatePickerBuilder(elementBuilder("DatePicker", "datepicker"))
}

// NewDateTimePicker creates a combined date-and-time picker.
func NewDateTimePicker() *DateTimePickerBuilder {
	return newDateTimePickerBuilder(elementBuilder("DateTimePicker", "datetimepicker"))
}

// NewEmailInput creates an email-address input.
func NewEmailInput() *EmailInputBuilder {
	return newEmailInputBuilder(elementBuilder("EmailInput", "email_text_input"))
}

// NewFileInput creates a file-upload input.
func NewFileInput() *FileInputBuilder {
	return newFileInputBuilder(elementBuilder("FileInput", "file_input"))
}

// NewImageElement creates an image element from a URL or Slack-hosted file.
func NewImageElement() *ImageElementBuilder {
	return newImageElementBuilder(elementBuilder("ImageElement", "image"))
}

// NewChannelMultiSelect creates a multi-select of public channels.
func NewChannelMultiSelect() *ChannelMultiSelectBuilder {
	return newChannelMultiSelectBuilder(elementBuilder("ChannelMultiSelect", "multi_channels_select"))
}

// NewConversationMultiSelect creates a multi-select of conversations.
func NewConversationMultiSelect() *ConversationMultiSelectBuilder {
	return newConversationMultiSelectBuilder(elementBuilder("ConversationMultiSelect", "multi_conversations_select"))
}

// NewExternalMultiSelect creates an externally populated multi-select.
func NewExternalMultiSelect() *ExternalMultiSelectBuilder {
	return newExternalMultiSelectBuilder(elementBuilder("ExternalMultiSelect", "multi_external_select"))
}

// NewStaticMultiSelect creates a multi-select with embedded options.
func NewStaticMultiSelect() *StaticMultiSelectBuilder {
	return newStaticMultiSelectBuilder(elementBuilder("StaticMultiSelect", "multi_static_select"))
}

// NewUserMultiSelect creates a workspace-user multi-select.
func NewUserMultiSelect() *UserMultiSelectBuilder {
	return newUserMultiSelectBuilder(elementBuilder("UserMultiSelect", "multi_users_select"))
}

// NewNumberInput creates a numeric input.
func NewNumberInput() *NumberInputBuilder {
	return newNumberInputBuilder(elementBuilder("NumberInput", "number_input"))
}

// NewOverflow creates a compact overflow menu.
func NewOverflow() *OverflowBuilder {
	return newOverflowBuilder(elementBuilder("Overflow", "overflow"))
}

// NewPlainTextInput creates a free-form text input.
func NewPlainTextInput() *PlainTextInputBuilder {
	return newPlainTextInputBuilder(elementBuilder("PlainTextInput", "plain_text_input"))
}

// NewRadioButtons creates a single-choice radio group.
func NewRadioButtons() *RadioButtonsBuilder {
	return newRadioButtonsBuilder(elementBuilder("RadioButtons", "radio_buttons"))
}

// NewChannelSelect creates a public-channel single-select.
func NewChannelSelect() *ChannelSelectBuilder {
	return newChannelSelectBuilder(elementBuilder("ChannelSelect", "channels_select"))
}

// NewConversationSelect creates a conversation single-select.
func NewConversationSelect() *ConversationSelectBuilder {
	return newConversationSelectBuilder(elementBuilder("ConversationSelect", "conversations_select"))
}

// NewExternalSelect creates an externally populated single-select.
func NewExternalSelect() *ExternalSelectBuilder {
	return newExternalSelectBuilder(elementBuilder("ExternalSelect", "external_select"))
}

// NewStaticSelect creates a single-select with embedded options.
func NewStaticSelect() *StaticSelectBuilder {
	return newStaticSelectBuilder(elementBuilder("StaticSelect", "static_select"))
}

// NewUserSelect creates a workspace-user single-select.
func NewUserSelect() *UserSelectBuilder {
	return newUserSelectBuilder(elementBuilder("UserSelect", "users_select"))
}

// NewTimePicker creates a time-of-day picker.
func NewTimePicker() *TimePickerBuilder {
	return newTimePickerBuilder(elementBuilder("TimePicker", "timepicker"))
}

// NewURLInput creates a URL input.
func NewURLInput() *URLInputBuilder {
	return newURLInputBuilder(elementBuilder("URLInput", "url_text_input"))
}

// NewWorkflowButton creates a button that launches a Slack workflow.
func NewWorkflowButton() *WorkflowButtonBuilder {
	return newWorkflowButtonBuilder(elementBuilder("WorkflowButton", "workflow_button").coerce("text", plainTextLike))
}

// NewRichTextInput creates a WYSIWYG rich-text editor.
func NewRichTextInput() *RichTextInputBuilder {
	return newRichTextInputBuilder(elementBuilder("RichTextInput", "rich_text_input"))
}
