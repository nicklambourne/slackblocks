package slackblocks

// Text sets a text-bearing field. Constructors coerce strings to the Slack
// text-object form required by that field.
func (b *builder) Text(value any) *builder { return b.set("text", value) }

// Emoji controls Slack emoji shortcode conversion for plain text.
func (b *builder) Emoji(value bool) *builder { return b.set("emoji", value) }

// Verbatim controls automatic Slack mrkdwn parsing.
func (b *builder) Verbatim(value bool) *builder { return b.set("verbatim", value) }

// BlockID assigns an application-defined block identifier.
func (b *builder) BlockID(value string) *builder { return b.set("block_id", value) }

// Fields appends section fields in display order.
func (b *builder) Fields(values ...any) *builder { return b.append("fields", values...) }

// Accessory sets the optional section accessory element.
func (b *builder) Accessory(value any) *builder { return b.set("accessory", value) }

// Title sets a title field.
func (b *builder) Title(value any) *builder { return b.set("title", value) }

// Confirm sets a confirmation object or its confirm-button label, according to
// the builder being configured.
func (b *builder) Confirm(value any) *builder { return b.set("confirm", value) }

// Deny sets the denial-button label on a confirmation object.
func (b *builder) Deny(value any) *builder { return b.set("deny", value) }

// Style sets a Slack-supported visual or layout style.
func (b *builder) Style(value any) *builder { return b.set("style", value) }

// Value sets an application-defined value.
func (b *builder) Value(value any) *builder { return b.set("value", value) }

// URL sets a URL field.
func (b *builder) URL(value string) *builder { return b.set("url", value) }

// Description sets supporting descriptive text.
func (b *builder) Description(value any) *builder { return b.set("description", value) }

// Options appends selectable option objects.
func (b *builder) Options(values ...any) *builder { return b.append("options", values...) }

// OptionGroups appends selectable option groups.
func (b *builder) OptionGroups(values ...any) *builder {
	return b.append("option_groups", values...)
}

// Label sets a label field.
func (b *builder) Label(value any) *builder { return b.set("label", value) }

// Include appends conversation kinds to a conversation filter.
func (b *builder) Include(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("include", items...)
}

// ExcludeExternalSharedChannels configures conversation filtering.
func (b *builder) ExcludeExternalSharedChannels(value bool) *builder {
	return b.set("exclude_external_shared_channels", value)
}

// ExcludeBotUsers configures conversation filtering.
func (b *builder) ExcludeBotUsers(value bool) *builder {
	return b.set("exclude_bot_users", value)
}

// TriggerActionsOn appends dispatch-action trigger names.
func (b *builder) TriggerActionsOn(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("trigger_actions_on", items...)
}

// Name sets a named Slack value.
func (b *builder) Name(value string) *builder { return b.set("name", value) }

// Trigger sets a workflow trigger.
func (b *builder) Trigger(value any) *builder { return b.set("trigger", value) }

// CustomizableInputParameters appends workflow parameters.
func (b *builder) CustomizableInputParameters(values ...any) *builder {
	return b.append("customizable_input_parameters", values...)
}

// ID sets an identifier field.
func (b *builder) ID(value string) *builder { return b.set("id", value) }

// SlackFileID sets a Slack-hosted file identifier.
func (b *builder) SlackFileID(value string) *builder { return b.set("id", value) }

// SlackFileURL sets a Slack-hosted file URL.
func (b *builder) SlackFileURL(value string) *builder { return b.set("url", value) }

// Align sets table-column alignment.
func (b *builder) Align(value string) *builder { return b.set("align", value) }

// Wrap controls table-column text wrapping.
func (b *builder) Wrap(value bool) *builder { return b.set("wrap", value) }

// IsWrapped controls table-column text wrapping.
func (b *builder) IsWrapped(value bool) *builder { return b.set("is_wrapped", value) }

// Data appends data points.
func (b *builder) Data(values ...any) *builder { return b.append("data", values...) }

// Series appends chart series.
func (b *builder) Series(values ...any) *builder { return b.append("series", values...) }

// Segments appends pie-chart segments.
func (b *builder) Segments(values ...any) *builder { return b.append("segments", values...) }

// Categories appends chart axis categories.
func (b *builder) Categories(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("categories", items...)
}

// XLabel sets a chart x-axis label.
func (b *builder) XLabel(value string) *builder { return b.set("x_label", value) }

// YLabel sets a chart y-axis label.
func (b *builder) YLabel(value string) *builder { return b.set("y_label", value) }

// AxisConfig sets chart-axis configuration.
func (b *builder) AxisConfig(value any) *builder { return b.set("axis_config", value) }

// Elements appends Slack elements.
func (b *builder) Elements(values ...any) *builder { return b.append("elements", values...) }

// Border sets a rich-text border depth.
func (b *builder) Border(value int) *builder { return b.set("border", value) }

// Offset sets a rich-text list nesting offset.
func (b *builder) Offset(value int) *builder { return b.set("offset", value) }

// Indent sets a rich-text list indent level.
func (b *builder) Indent(value int) *builder { return b.set("indent", value) }

// SkinTone sets an emoji skin-tone index.
func (b *builder) SkinTone(value int) *builder { return b.set("skin_tone", value) }

// ChannelID sets a Slack channel ID.
func (b *builder) ChannelID(value string) *builder { return b.set("channel_id", value) }

// UserID sets a Slack user ID.
func (b *builder) UserID(value string) *builder { return b.set("user_id", value) }

// UserGroupID sets a Slack user-group ID.
func (b *builder) UserGroupID(value string) *builder { return b.set("usergroup_id", value) }

// ActionID sets an interaction identifier.
func (b *builder) ActionID(value string) *builder { return b.set("action_id", value) }

// AccessibilityLabel sets assistive text for an interactive control.
func (b *builder) AccessibilityLabel(value string) *builder {
	return b.set("accessibility_label", value)
}

// Actions appends actions to a card.
func (b *builder) Actions(values ...any) *builder { return b.append("actions", values...) }

// AltText sets an accessible image or video description.
func (b *builder) AltText(value string) *builder { return b.set("alt_text", value) }

// Attachments appends legacy secondary attachments.
func (b *builder) Attachments(values ...any) *builder {
	return b.append("attachments", values...)
}

// AuthorName sets video author attribution.
func (b *builder) AuthorName(value string) *builder { return b.set("author_name", value) }

// Blocks appends Block Kit blocks.
func (b *builder) Blocks(values ...any) *builder { return b.append("blocks", values...) }

// Body sets main card copy.
func (b *builder) Body(value any) *builder { return b.set("body", value) }

// Caption sets an accessible data-table caption.
func (b *builder) Caption(value string) *builder { return b.set("caption", value) }

// Channel sets a message destination.
func (b *builder) Channel(value string) *builder { return b.set("channel", value) }

// Chart sets a chart object on a data-visualization block.
func (b *builder) Chart(value any) *builder { return b.set("chart", value) }

// ChildBlocks appends blocks to a container.
func (b *builder) ChildBlocks(values ...any) *builder {
	return b.append("child_blocks", values...)
}

// ClearOnClose controls modal stack clearing.
func (b *builder) ClearOnClose(value bool) *builder { return b.set("clear_on_close", value) }

// CallbackID sets an application-defined view callback identifier.
func (b *builder) CallbackID(value string) *builder { return b.set("callback_id", value) }

// Close sets a modal close-button label.
func (b *builder) Close(value any) *builder { return b.set("close", value) }

// Code enables rich-text code styling.
func (b *builder) Code(value bool) *builder { return b.set("code", value) }

// Color sets an attachment side-border color.
func (b *builder) Color(value string) *builder { return b.set("color", value) }

// ColumnSettings appends table column display settings.
func (b *builder) ColumnSettings(values ...any) *builder {
	return b.append("column_settings", values...)
}

// DefaultCollapsed controls the initial state of a collapsible container.
func (b *builder) DefaultCollapsed(value bool) *builder {
	return b.set("default_collapsed", value)
}

// Expanded controls whether an accordion section starts open. It stores a
// private sentinel that the accordion-section transform converts to Slack's
// inverse default_collapsed field during Build.
func (b *builder) Expanded(value bool) *builder { return b.set("_expanded", value) }

// DefaultToCurrentConversation selects the current conversation by default.
func (b *builder) DefaultToCurrentConversation(value bool) *builder {
	return b.set("default_to_current_conversation", value)
}

// DeleteOriginal controls response-URL message deletion.
func (b *builder) DeleteOriginal(value bool) *builder {
	return b.set("delete_original", value)
}

// Details sets rich task details.
func (b *builder) Details(value any) *builder { return b.set("details", value) }

// DispatchAction controls immediate input dispatch.
func (b *builder) DispatchAction(value bool) *builder {
	return b.set("dispatch_action", value)
}

// DispatchActionConfig sets input dispatch behavior.
func (b *builder) DispatchActionConfig(value any) *builder {
	return b.set("dispatch_action_config", value)
}

// Element sets the interactive element in an input block.
func (b *builder) Element(value any) *builder { return b.set("element", value) }

// ExternalID sets an application-defined external identifier.
func (b *builder) ExternalID(value string) *builder { return b.set("external_id", value) }

// Fallback sets attachment fallback text.
func (b *builder) Fallback(value string) *builder { return b.set("fallback", value) }

// Filetypes appends allowed file extensions.
func (b *builder) Filetypes(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("filetypes", items...)
}

// Filter sets a conversation filter.
func (b *builder) Filter(value any) *builder { return b.set("filter", value) }

// FocusOnLoad controls initial focus in a view.
func (b *builder) FocusOnLoad(value bool) *builder { return b.set("focus_on_load", value) }

// HasHeaderDivider controls a container header divider.
func (b *builder) HasHeaderDivider(value bool) *builder {
	return b.set("has_header_divider", value)
}

// HeroImage sets the large image on a card.
func (b *builder) HeroImage(value any) *builder { return b.set("hero_image", value) }

// Highlight enables rich-text highlight styling.
func (b *builder) Highlight(value bool) *builder { return b.set("highlight", value) }

// Hint sets supporting input help text.
func (b *builder) Hint(value any) *builder { return b.set("hint", value) }

// Icon sets an icon or icon element.
func (b *builder) Icon(value any) *builder { return b.set("icon", value) }

// ImageURL sets a public image URL.
func (b *builder) ImageURL(value string) *builder { return b.set("image_url", value) }

// InitialChannel sets an initially selected public channel.
func (b *builder) InitialChannel(value string) *builder { return b.set("initial_channel", value) }

// InitialChannels appends initially selected public channels.
func (b *builder) InitialChannels(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("initial_channels", items...)
}

// InitialConversation sets an initially selected conversation.
func (b *builder) InitialConversation(value string) *builder {
	return b.set("initial_conversation", value)
}

// InitialConversations appends initially selected conversations.
func (b *builder) InitialConversations(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("initial_conversations", items...)
}

// InitialDate sets an ISO calendar date.
func (b *builder) InitialDate(value string) *builder { return b.set("initial_date", value) }

// InitialDateTime sets an initial Unix timestamp in seconds.
func (b *builder) InitialDateTime(value int64) *builder {
	return b.set("initial_date_time", value)
}

// InitialOption sets a single initially selected option.
func (b *builder) InitialOption(value any) *builder { return b.set("initial_option", value) }

// InitialOptions appends initially selected options.
func (b *builder) InitialOptions(values ...any) *builder {
	return b.append("initial_options", values...)
}

// InitialTime sets an initial 24-hour time.
func (b *builder) InitialTime(value string) *builder { return b.set("initial_time", value) }

// InitialUser sets an initially selected user.
func (b *builder) InitialUser(value string) *builder { return b.set("initial_user", value) }

// InitialUsers appends initially selected users.
func (b *builder) InitialUsers(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("initial_users", items...)
}

// InitialValue sets initial input content.
func (b *builder) InitialValue(value any) *builder { return b.set("initial_value", value) }

// IsCollapsible controls container expansion.
func (b *builder) IsCollapsible(value bool) *builder { return b.set("is_collapsible", value) }

// IsDecimalAllowed controls decimal input.
func (b *builder) IsDecimalAllowed(value bool) *builder {
	return b.set("is_decimal_allowed", value)
}

// Italic enables rich-text italic styling.
func (b *builder) Italic(value bool) *builder { return b.set("italic", value) }

// Level sets alert severity.
func (b *builder) Level(value string) *builder { return b.set("level", value) }

// MaxFiles sets a file-upload count limit.
func (b *builder) MaxFiles(value int) *builder { return b.set("max_files", value) }

// MaxLength sets an input character limit.
func (b *builder) MaxLength(value int) *builder { return b.set("max_length", value) }

// MaxLines sets a rich-text editor line limit.
func (b *builder) MaxLines(value int) *builder { return b.set("max_lines", value) }

// MaxSelectedItems sets a select-menu selection limit.
func (b *builder) MaxSelectedItems(value int) *builder {
	return b.set("max_selected_items", value)
}

// MaxValue sets a numeric input maximum.
func (b *builder) MaxValue(value float64) *builder { return b.set("max_value", value) }

// Metadata sets message metadata.
func (b *builder) Metadata(value any) *builder { return b.set("metadata", value) }

// MinLength sets a minimum input character count.
func (b *builder) MinLength(value int) *builder { return b.set("min_length", value) }

// MinLines sets a rich-text editor minimum line count.
func (b *builder) MinLines(value int) *builder { return b.set("min_lines", value) }

// MinQueryLength sets the external select query threshold.
func (b *builder) MinQueryLength(value int) *builder { return b.set("min_query_length", value) }

// MinValue sets a numeric input minimum.
func (b *builder) MinValue(value float64) *builder { return b.set("min_value", value) }

// Mrkdwn controls fallback text parsing.
func (b *builder) Mrkdwn(value bool) *builder { return b.set("mrkdwn", value) }

// Multiline controls text input layout.
func (b *builder) Multiline(value bool) *builder { return b.set("multiline", value) }

// NegativeButton sets the negative feedback choice.
func (b *builder) NegativeButton(value any) *builder {
	return b.set("negative_button", value)
}

// NotifyOnClose controls modal close events.
func (b *builder) NotifyOnClose(value bool) *builder { return b.set("notify_on_close", value) }

// Optional makes an input block optional.
func (b *builder) Optional(value bool) *builder { return b.set("optional", value) }

// Output sets rich task output.
func (b *builder) Output(value any) *builder { return b.set("output", value) }

// PageSize sets data-table rows per page.
func (b *builder) PageSize(value int) *builder { return b.set("page_size", value) }

// Placeholder sets empty-state prompt text.
func (b *builder) Placeholder(value any) *builder { return b.set("placeholder", value) }

// PositiveButton sets the positive feedback choice.
func (b *builder) PositiveButton(value any) *builder {
	return b.set("positive_button", value)
}

// PrivateMetadata sets opaque view metadata.
func (b *builder) PrivateMetadata(value string) *builder {
	return b.set("private_metadata", value)
}

// ProviderIconURL sets a video provider icon.
func (b *builder) ProviderIconURL(value string) *builder {
	return b.set("provider_icon_url", value)
}

// ProviderName sets video provider attribution.
func (b *builder) ProviderName(value string) *builder { return b.set("provider_name", value) }

// ReplaceOriginal controls interaction message replacement.
func (b *builder) ReplaceOriginal(value bool) *builder {
	return b.set("replace_original", value)
}

// ResponseType sets response visibility.
func (b *builder) ResponseType(value string) *builder { return b.set("response_type", value) }

// ResponseURLEnabled requests a modal submission response URL.
func (b *builder) ResponseURLEnabled(value bool) *builder {
	return b.set("response_url_enabled", value)
}

// RichTextTitle sets a container rich-text title.
func (b *builder) RichTextTitle(value any) *builder { return b.set("rich_text_title", value) }

// RowHeaderColumnIndex sets the zero-based data-table row-header column.
func (b *builder) RowHeaderColumnIndex(value int) *builder {
	return b.set("row_header_column_index", value)
}

// Rows appends table rows. Each argument should be a slice of cells.
func (b *builder) Rows(values ...any) *builder { return b.appendNested("rows", values...) }

// Sender sets card sender information.
func (b *builder) Sender(value any) *builder { return b.set("sender", value) }

// SlackFile sets a Slack-hosted image reference.
func (b *builder) SlackFile(value any) *builder { return b.set("slack_file", value) }

// SlackIcon sets a named Slack icon.
func (b *builder) SlackIcon(value any) *builder { return b.set("slack_icon", value) }

// Source sets a file source.
func (b *builder) Source(value string) *builder { return b.set("source", value) }

// Sources appends task source links.
func (b *builder) Sources(values ...any) *builder { return b.append("sources", values...) }

// Status sets task lifecycle state.
func (b *builder) Status(value string) *builder { return b.set("status", value) }

// Strike enables rich-text strike styling.
func (b *builder) Strike(value bool) *builder { return b.set("strike", value) }

// Submit sets a modal submit-button label.
func (b *builder) Submit(value any) *builder { return b.set("submit", value) }

// SubmitDisabled controls the initial modal submit state.
func (b *builder) SubmitDisabled(value bool) *builder {
	return b.set("submit_disabled", value)
}

// Subtext sets supporting card copy.
func (b *builder) Subtext(value any) *builder { return b.set("subtext", value) }

// Subtitle sets secondary heading copy.
func (b *builder) Subtitle(value any) *builder { return b.set("subtitle", value) }

// TaskID sets a stable task identifier.
func (b *builder) TaskID(value string) *builder { return b.set("task_id", value) }

// Tasks appends task-card blocks.
func (b *builder) Tasks(values ...any) *builder { return b.append("tasks", values...) }

// ThumbnailURL sets a video preview image.
func (b *builder) ThumbnailURL(value string) *builder {
	return b.set("thumbnail_url", value)
}

// Timezone sets an IANA timezone.
func (b *builder) Timezone(value string) *builder { return b.set("timezone", value) }

// TitleURL sets a video or card title URL.
func (b *builder) TitleURL(value string) *builder { return b.set("title_url", value) }

// UnfurlLinks controls message link unfurling.
func (b *builder) UnfurlLinks(value bool) *builder { return b.set("unfurl_links", value) }

// UnfurlMedia controls message media unfurling.
func (b *builder) UnfurlMedia(value bool) *builder { return b.set("unfurl_media", value) }

// Unlink enables rich-text unlink styling.
func (b *builder) Unlink(value bool) *builder { return b.set("unlink", value) }

// Unsafe marks raw text as unsafe where Slack supports it.
func (b *builder) Unsafe(value bool) *builder { return b.set("unsafe", value) }

// VideoURL sets the hosted video URL.
func (b *builder) VideoURL(value string) *builder { return b.set("video_url", value) }

// VisibleToUserIDs appends user IDs allowed to see an icon action.
func (b *builder) VisibleToUserIDs(values ...string) *builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("visible_to_user_ids", items...)
}

// Width sets container width.
func (b *builder) Width(value string) *builder { return b.set("width", value) }

// Workflow sets a workflow object on a workflow button.
func (b *builder) Workflow(value any) *builder { return b.set("workflow", value) }

// Bold enables rich-text bold styling.
func (b *builder) Bold(value bool) *builder { return b.set("bold", value) }
