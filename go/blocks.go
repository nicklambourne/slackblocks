package slackblocks

// NewSectionBlock creates a section block. String text and fields are coerced
// to Slack mrkdwn objects.
func NewSectionBlock() *Builder {
	return newBuilder("SectionBlock", "section").
		coerce("text", markdownLike).
		coerce("fields", markdownLike)
}

// NewDividerBlock creates a visual divider block.
func NewDividerBlock() *Builder { return newBuilder("DividerBlock", "divider") }
