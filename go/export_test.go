package slackblocks

// LimitVideoAltTextMaxLength exposes the generated spec limit to the external
// test package.
const LimitVideoAltTextMaxLength = limitVideoAltTextMaxLength

// Further generated spec limits exposed to the external test package, so the
// invalid corpus is built from limits.json rather than magic numbers.
const (
	LimitPlainTextInputMinLengthMin       = limitPlainTextInputMinLengthMin
	LimitPlainTextInputMinLengthMax       = limitPlainTextInputMinLengthMax
	LimitPlainTextInputMaxLengthMin       = limitPlainTextInputMaxLengthMin
	LimitRichTextInputMinLinesMin         = limitRichTextInputMinLinesMin
	LimitRichTextInputMinLinesMax         = limitRichTextInputMinLinesMax
	LimitRichTextInputMaxLinesMin         = limitRichTextInputMaxLinesMin
	LimitRichTextInputMaxLinesMax         = limitRichTextInputMaxLinesMax
	LimitMultiSelectMaxSelectedItemsMin   = limitMultiSelectMaxSelectedItemsMin
	LimitImageElementImageURLMaxLength    = limitImageElementImageURLMaxLength
	LimitImageElementAltTextMaxLength     = limitImageElementAltTextMaxLength
	LimitImageTitleMaxLength              = limitImageTitleMaxLength
	LimitTableColumnSettingsMaxItems      = limitTableColumnSettingsMaxItems
	LimitDataTableRowHeaderColumnIndexMin = limitDataTableRowHeaderColumnIndexMin
	LimitDataTableTotalContentMaxLength   = limitDataTableTotalContentMaxLength
	LimitMarkdownTotalTextMaxLength       = limitMarkdownTotalTextMaxLength
	LimitPlanTasksMaxItems                = limitPlanTasksMaxItems
	LimitRichTextListIndentMin            = limitRichTextListIndentMin
	LimitRichTextListIndentMax            = limitRichTextListIndentMax
	LimitRichTextListOffsetMin            = limitRichTextListOffsetMin
	LimitRichTextListBorderMin            = limitRichTextListBorderMin
	LimitRichTextListBorderMax            = limitRichTextListBorderMax
	LimitRichTextQuoteBorderMin           = limitRichTextQuoteBorderMin
	LimitRichTextQuoteBorderMax           = limitRichTextQuoteBorderMax
	LimitRichTextPreformattedBorderMin    = limitRichTextPreformattedBorderMin
	LimitRichTextPreformattedBorderMax    = limitRichTextPreformattedBorderMax
	LimitVideoThumbnailURLMaxLength       = limitVideoThumbnailURLMaxLength
	LimitVideoVideoURLMaxLength           = limitVideoVideoURLMaxLength
	LimitVideoTitleURLMaxLength           = limitVideoTitleURLMaxLength
	LimitVideoProviderIconURLMaxLength    = limitVideoProviderIconURLMaxLength
	LimitViewExternalIDMaxLength          = limitViewExternalIDMaxLength
)
