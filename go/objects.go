package slackblocks

// NewPlainText creates a plain-text composition object.
func NewPlainText() *Builder { return newBuilder("PlainText", "plain_text") }

// NewMarkdown creates a Slack mrkdwn composition object.
func NewMarkdown() *Builder { return newBuilder("Markdown", "mrkdwn") }

// NewConfirmation creates a confirmation-dialog composition object.
func NewConfirmation() *Builder {
	return newBuilder("Confirmation", "").
		coerce("title", plainTextLike).
		coerce("text", markdownLike).
		coerce("confirm", plainTextLike).
		coerce("deny", plainTextLike)
}

// NewOption creates a selectable option composition object.
func NewOption() *Builder {
	return newBuilder("Option", "").
		coerce("text", plainTextLike).
		coerce("description", plainTextLike)
}

// NewOptionGroup creates a labelled group of selectable options.
func NewOptionGroup() *Builder {
	return newBuilder("OptionGroup", "").coerce("label", plainTextLike)
}

// NewConversationFilter creates a conversation select-menu filter.
func NewConversationFilter() *Builder { return newBuilder("ConversationFilter", "") }

// NewDispatchActionConfiguration creates an input dispatch configuration.
func NewDispatchActionConfiguration() *Builder {
	return newBuilder("DispatchActionConfiguration", "")
}

// NewInputParameter creates a customizable workflow input parameter.
func NewInputParameter() *Builder { return newBuilder("InputParameter", "") }

// NewTrigger creates a workflow link trigger.
func NewTrigger() *Builder { return newBuilder("Trigger", "") }

// NewWorkflow creates a workflow composition object.
func NewWorkflow() *Builder { return newBuilder("Workflow", "") }

// NewSlackFile creates a Slack-hosted file reference.
func NewSlackFile() *Builder { return newBuilder("SlackFile", "") }

// NewColumnSettings creates table-column display settings.
func NewColumnSettings() *Builder { return newBuilder("ColumnSettings", "") }

// NewRawText creates an unformatted table cell.
func NewRawText() *Builder { return newBuilder("RawText", "raw_text") }

// NewRawNumber creates a numeric data-table cell.
func NewRawNumber() *Builder { return newBuilder("RawNumber", "raw_number") }

// NewSlackIcon creates a named Slack-rendered icon.
func NewSlackIcon() *Builder { return newBuilder("SlackIcon", "icon") }

// NewChartSegment creates a labelled pie-chart segment.
func NewChartSegment() *Builder { return newBuilder("ChartSegment", "") }

// NewDataPoint creates a labelled chart data point.
func NewDataPoint() *Builder { return newBuilder("DataPoint", "") }

// NewDataSeries creates a named chart data series.
func NewDataSeries() *Builder { return newBuilder("DataSeries", "") }

// NewAxisConfig creates category and label settings for a chart axis.
func NewAxisConfig() *Builder { return newBuilder("AxisConfig", "") }

// NewPieChart creates a pie-chart object.
func NewPieChart() *Builder { return newBuilder("PieChart", "pie") }

// NewBarChart creates a bar-chart object.
func NewBarChart() *Builder { return newBuilder("BarChart", "bar") }

// NewAreaChart creates an area-chart object.
func NewAreaChart() *Builder { return newBuilder("AreaChart", "area") }

// NewLineChart creates a line-chart object.
func NewLineChart() *Builder { return newBuilder("LineChart", "line") }

// NewRichText creates a rich-text text run.
func NewRichText() *Builder { return newBuilder("RichText", "text") }

// NewRichTextChannel creates a rich-text channel mention.
func NewRichTextChannel() *Builder { return newBuilder("RichTextChannel", "channel") }

// NewRichTextEmoji creates a rich-text emoji run.
func NewRichTextEmoji() *Builder { return newBuilder("RichTextEmoji", "emoji") }

// NewRichTextLink creates a rich-text link run.
func NewRichTextLink() *Builder { return newBuilder("RichTextLink", "link") }

// NewRichTextUser creates a rich-text user mention.
func NewRichTextUser() *Builder { return newBuilder("RichTextUser", "user") }

// NewRichTextUserGroup creates a rich-text user-group mention.
func NewRichTextUserGroup() *Builder { return newBuilder("RichTextUserGroup", "usergroup") }

// NewRichTextSection creates a rich-text inline section.
func NewRichTextSection() *Builder { return newBuilder("RichTextSection", "rich_text_section") }

// NewRichTextList creates an ordered or bullet rich-text list.
func NewRichTextList() *Builder { return newBuilder("RichTextList", "rich_text_list") }

// NewRichTextCodeBlock creates a preformatted rich-text block.
func NewRichTextCodeBlock() *Builder {
	return newBuilder("RichTextCodeBlock", "rich_text_preformatted")
}

// NewRichTextQuote creates a quoted rich-text block.
func NewRichTextQuote() *Builder { return newBuilder("RichTextQuote", "rich_text_quote") }
