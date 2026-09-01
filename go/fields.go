package slackblocks

// Text sets a text-bearing field. Constructors coerce strings to the Slack
// text-object form required by that field.
func (b *Builder) Text(value any) *Builder { return b.set("text", value) }

// Emoji controls Slack emoji shortcode conversion for plain text.
func (b *Builder) Emoji(value bool) *Builder { return b.set("emoji", value) }

// Verbatim controls automatic Slack mrkdwn parsing.
func (b *Builder) Verbatim(value bool) *Builder { return b.set("verbatim", value) }

// BlockID assigns an application-defined block identifier.
func (b *Builder) BlockID(value string) *Builder { return b.set("block_id", value) }

// Fields appends section fields in display order.
func (b *Builder) Fields(values ...any) *Builder { return b.append("fields", values...) }

// Accessory sets the optional section accessory element.
func (b *Builder) Accessory(value any) *Builder { return b.set("accessory", value) }

// Title sets a title field.
func (b *Builder) Title(value any) *Builder { return b.set("title", value) }

// Confirm sets a confirmation object or its confirm-button label, according to
// the builder being configured.
func (b *Builder) Confirm(value any) *Builder { return b.set("confirm", value) }

// Deny sets the denial-button label on a confirmation object.
func (b *Builder) Deny(value any) *Builder { return b.set("deny", value) }

// Style sets a Slack-supported visual or layout style.
func (b *Builder) Style(value string) *Builder { return b.set("style", value) }

// Value sets an application-defined value.
func (b *Builder) Value(value any) *Builder { return b.set("value", value) }

// URL sets a URL field.
func (b *Builder) URL(value string) *Builder { return b.set("url", value) }

// Description sets supporting descriptive text.
func (b *Builder) Description(value any) *Builder { return b.set("description", value) }

// Options appends selectable option objects.
func (b *Builder) Options(values ...any) *Builder { return b.append("options", values...) }

// OptionGroups appends selectable option groups.
func (b *Builder) OptionGroups(values ...any) *Builder {
	return b.append("option_groups", values...)
}

// Label sets a label field.
func (b *Builder) Label(value any) *Builder { return b.set("label", value) }

// Include appends conversation kinds to a conversation filter.
func (b *Builder) Include(values ...string) *Builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("include", items...)
}

// ExcludeExternalSharedChannels configures conversation filtering.
func (b *Builder) ExcludeExternalSharedChannels(value bool) *Builder {
	return b.set("exclude_external_shared_channels", value)
}

// ExcludeBotUsers configures conversation filtering.
func (b *Builder) ExcludeBotUsers(value bool) *Builder {
	return b.set("exclude_bot_users", value)
}

// TriggerActionsOn appends dispatch-action trigger names.
func (b *Builder) TriggerActionsOn(values ...string) *Builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("trigger_actions_on", items...)
}

// Name sets a named Slack value.
func (b *Builder) Name(value string) *Builder { return b.set("name", value) }

// Trigger sets a workflow trigger.
func (b *Builder) Trigger(value any) *Builder { return b.set("trigger", value) }

// CustomizableInputParameters appends workflow parameters.
func (b *Builder) CustomizableInputParameters(values ...any) *Builder {
	return b.append("customizable_input_parameters", values...)
}

// ID sets an identifier field.
func (b *Builder) ID(value string) *Builder { return b.set("id", value) }

// SlackFileID sets a Slack-hosted file identifier.
func (b *Builder) SlackFileID(value string) *Builder { return b.set("id", value) }

// SlackFileURL sets a Slack-hosted file URL.
func (b *Builder) SlackFileURL(value string) *Builder { return b.set("url", value) }

// Align sets table-column alignment.
func (b *Builder) Align(value string) *Builder { return b.set("align", value) }

// Wrap controls table-column text wrapping.
func (b *Builder) Wrap(value bool) *Builder { return b.set("wrap", value) }

// Data appends data points.
func (b *Builder) Data(values ...any) *Builder { return b.append("data", values...) }

// Series appends chart series.
func (b *Builder) Series(values ...any) *Builder { return b.append("series", values...) }

// Segments appends pie-chart segments.
func (b *Builder) Segments(values ...any) *Builder { return b.append("segments", values...) }

// Categories appends chart axis categories.
func (b *Builder) Categories(values ...string) *Builder {
	items := make([]any, len(values))
	for index, value := range values {
		items[index] = value
	}
	return b.append("categories", items...)
}

// XLabel sets a chart x-axis label.
func (b *Builder) XLabel(value string) *Builder { return b.set("x_label", value) }

// YLabel sets a chart y-axis label.
func (b *Builder) YLabel(value string) *Builder { return b.set("y_label", value) }

// AxisConfig sets chart-axis configuration.
func (b *Builder) AxisConfig(value any) *Builder { return b.set("axis_config", value) }

// Elements appends Slack elements.
func (b *Builder) Elements(values ...any) *Builder { return b.append("elements", values...) }

// Border sets a rich-text border depth.
func (b *Builder) Border(value int) *Builder { return b.set("border", value) }

// Offset sets a rich-text list nesting offset.
func (b *Builder) Offset(value int) *Builder { return b.set("offset", value) }

// Indent sets a rich-text list indent level.
func (b *Builder) Indent(value int) *Builder { return b.set("indent", value) }

// SkinTone sets an emoji skin-tone index.
func (b *Builder) SkinTone(value int) *Builder { return b.set("skin_tone", value) }

// ChannelID sets a Slack channel ID.
func (b *Builder) ChannelID(value string) *Builder { return b.set("channel_id", value) }

// UserID sets a Slack user ID.
func (b *Builder) UserID(value string) *Builder { return b.set("user_id", value) }

// UserGroupID sets a Slack user-group ID.
func (b *Builder) UserGroupID(value string) *Builder { return b.set("usergroup_id", value) }
