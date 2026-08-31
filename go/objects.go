package slackblocks

// NewPlainText creates a plain-text composition object.
func NewPlainText() *PlainTextBuilder {
	return newPlainTextBuilder(newBuilder("PlainText", "plain_text"))
}

// NewMarkdown creates a Slack mrkdwn composition object.
func NewMarkdown() *MarkdownBuilder { return newMarkdownBuilder(newBuilder("Markdown", "mrkdwn")) }

// NewConfirmation creates a confirmation-dialog composition object.
func NewConfirmation() *ConfirmationBuilder {
	return newConfirmationBuilder(newBuilder("Confirmation", "").
		coerce("title", plainTextLike).
		coerce("text", markdownLike).
		coerce("confirm", plainTextLike).
		coerce("deny", plainTextLike))
}

// NewOption creates a selectable option composition object.
func NewOption() *OptionBuilder {
	return newOptionBuilder(newBuilder("Option", "").
		coerce("text", plainTextLike).
		coerce("description", plainTextLike))
}

// NewOptionGroup creates a labelled group of selectable options.
func NewOptionGroup() *OptionGroupBuilder {
	return newOptionGroupBuilder(newBuilder("OptionGroup", "").coerce("label", plainTextLike))
}

// NewConversationFilter creates a conversation select-menu filter.
func NewConversationFilter() *ConversationFilterBuilder {
	return newConversationFilterBuilder(newBuilder("ConversationFilter", ""))
}

// NewDispatchActionConfiguration creates an input dispatch configuration.
func NewDispatchActionConfiguration() *DispatchActionConfigurationBuilder {
	return newDispatchActionConfigurationBuilder(newBuilder("DispatchActionConfiguration", ""))
}

// NewInputParameter creates a customizable workflow input parameter.
func NewInputParameter() *InputParameterBuilder {
	return newInputParameterBuilder(newBuilder("InputParameter", ""))
}

// NewTrigger creates a workflow link trigger.
func NewTrigger() *TriggerBuilder { return newTriggerBuilder(newBuilder("Trigger", "")) }

// NewWorkflow creates a workflow composition object.
func NewWorkflow() *WorkflowBuilder { return newWorkflowBuilder(newBuilder("Workflow", "")) }

// NewSlackFile creates a Slack-hosted file reference.
func NewSlackFile() *SlackFileBuilder { return newSlackFileBuilder(newBuilder("SlackFile", "")) }

// NewColumnSettings creates table-column display settings.
func NewColumnSettings() *ColumnSettingsBuilder {
	return newColumnSettingsBuilder(newBuilder("ColumnSettings", ""))
}

// NewRawText creates an unformatted table cell.
func NewRawText() *RawTextBuilder { return newRawTextBuilder(newBuilder("RawText", "raw_text")) }

// NewRawNumber creates a numeric data-table cell.
func NewRawNumber() *RawNumberBuilder {
	return newRawNumberBuilder(newBuilder("RawNumber", "raw_number"))
}

// NewSlackIcon creates a named Slack-rendered icon.
func NewSlackIcon() *SlackIconBuilder { return newSlackIconBuilder(newBuilder("SlackIcon", "icon")) }

// NewChartSegment creates a labelled pie-chart segment.
func NewChartSegment() *ChartSegmentBuilder {
	return newChartSegmentBuilder(newBuilder("ChartSegment", ""))
}

// NewDataPoint creates a labelled chart data point.
func NewDataPoint() *DataPointBuilder { return newDataPointBuilder(newBuilder("DataPoint", "")) }

// NewDataSeries creates a named chart data series.
func NewDataSeries() *DataSeriesBuilder { return newDataSeriesBuilder(newBuilder("DataSeries", "")) }

// NewAxisConfig creates category and label settings for a chart axis.
func NewAxisConfig() *AxisConfigBuilder { return newAxisConfigBuilder(newBuilder("AxisConfig", "")) }

// NewPieChart creates a pie-chart object.
func NewPieChart() *PieChartBuilder { return newPieChartBuilder(newBuilder("PieChart", "pie")) }

// NewBarChart creates a bar-chart object.
func NewBarChart() *BarChartBuilder { return newBarChartBuilder(newBuilder("BarChart", "bar")) }

// NewAreaChart creates an area-chart object.
func NewAreaChart() *AreaChartBuilder { return newAreaChartBuilder(newBuilder("AreaChart", "area")) }

// NewLineChart creates a line-chart object.
func NewLineChart() *LineChartBuilder { return newLineChartBuilder(newBuilder("LineChart", "line")) }

// NewRichText creates a rich-text text run.
func NewRichText() *RichTextBuilder { return newRichTextBuilder(newBuilder("RichText", "text")) }

// NewRichTextChannel creates a rich-text channel mention.
func NewRichTextChannel() *RichTextChannelBuilder {
	return newRichTextChannelBuilder(newBuilder("RichTextChannel", "channel"))
}

// NewRichTextEmoji creates a rich-text emoji run.
func NewRichTextEmoji() *RichTextEmojiBuilder {
	return newRichTextEmojiBuilder(newBuilder("RichTextEmoji", "emoji"))
}

// NewRichTextLink creates a rich-text link run.
func NewRichTextLink() *RichTextLinkBuilder {
	return newRichTextLinkBuilder(newBuilder("RichTextLink", "link"))
}

// NewRichTextUser creates a rich-text user mention.
func NewRichTextUser() *RichTextUserBuilder {
	return newRichTextUserBuilder(newBuilder("RichTextUser", "user"))
}

// NewRichTextUserGroup creates a rich-text user-group mention.
func NewRichTextUserGroup() *RichTextUserGroupBuilder {
	return newRichTextUserGroupBuilder(newBuilder("RichTextUserGroup", "usergroup"))
}

// NewRichTextSection creates a rich-text inline section.
func NewRichTextSection() *RichTextSectionBuilder {
	return newRichTextSectionBuilder(newBuilder("RichTextSection", "rich_text_section"))
}

// NewRichTextList creates an ordered or bullet rich-text list.
func NewRichTextList() *RichTextListBuilder {
	return newRichTextListBuilder(newBuilder("RichTextList", "rich_text_list"))
}

// NewRichTextCodeBlock creates a preformatted rich-text block.
func NewRichTextCodeBlock() *RichTextCodeBlockBuilder {
	return newRichTextCodeBlockBuilder(newBuilder("RichTextCodeBlock", "rich_text_preformatted"))
}

// NewRichTextQuote creates a quoted rich-text block.
func NewRichTextQuote() *RichTextQuoteBuilder {
	return newRichTextQuoteBuilder(newBuilder("RichTextQuote", "rich_text_quote"))
}
