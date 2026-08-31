package slackblocks

// NewAlertBlock creates a severity-labelled notice for a modal.
func NewAlertBlock() *AlertBlockBuilder {
	return newAlertBlockBuilder(newBuilder("AlertBlock", "alert").coerce("text", markdownLike))
}

// NewCardBlock creates a compact card with text, imagery, and actions.
func NewCardBlock() *CardBlockBuilder {
	return newCardBlockBuilder(newBuilder("CardBlock", "card").
		coerce("title", markdownLike).
		coerce("subtitle", markdownLike).
		coerce("body", markdownLike).
		coerce("subtext", markdownLike))
}

// NewCarouselBlock creates a horizontally scrolling collection of cards.
func NewCarouselBlock() *CarouselBlockBuilder {
	return newCarouselBlockBuilder(newBuilder("CarouselBlock", "carousel"))
}

// NewContainerBlock creates a titled group of child blocks.
func NewContainerBlock() *ContainerBlockBuilder {
	return newContainerBlockBuilder(newBuilder("ContainerBlock", "container").
		coerce("title", plainTextLike).
		coerce("subtitle", markdownLike))
}

// NewContextActionsBlock creates contextual feedback or icon controls.
func NewContextActionsBlock() *ContextActionsBlockBuilder {
	return newContextActionsBlockBuilder(newBuilder("ContextActionsBlock", "context_actions"))
}

// NewDataTableBlock creates a sortable, paginated data table.
func NewDataTableBlock() *DataTableBlockBuilder {
	builder := newBuilder("DataTableBlock", "data_table")
	builder.values["page_size"] = 5
	builder.values["row_header_column_index"] = 0
	return newDataTableBlockBuilder(builder)
}

// NewDataVisualizationBlock creates a Slack-rendered chart block.
func NewDataVisualizationBlock() *DataVisualizationBlockBuilder {
	return newDataVisualizationBlockBuilder(newBuilder("DataVisualizationBlock", "data_visualization"))
}

// NewTaskCardBlock creates one task in a plan.
func NewTaskCardBlock() *TaskCardBlockBuilder {
	return newTaskCardBlockBuilder(newBuilder("TaskCardBlock", "task_card"))
}

// NewPlanBlock creates a titled sequence of tasks.
func NewPlanBlock() *PlanBlockBuilder {
	return newPlanBlockBuilder(newBuilder("PlanBlock", "plan").withTransform(func(object Object) (Object, error) {
		tasks, ok := object["tasks"].([]any)
		if !ok {
			return object, nil
		}
		for index, value := range tasks {
			task, ok := value.(Object)
			if !ok {
				continue
			}
			taskCopy := Object{}
			for key, nested := range task {
				if key != "type" && key != "block_id" {
					taskCopy[key] = nested
				}
			}
			tasks[index] = taskCopy
		}
		object["tasks"] = tasks
		return object, nil
	}))
}

// NewSectionBlock creates a section block. String text and fields are coerced
// to Slack mrkdwn objects.
func NewSectionBlock() *SectionBlockBuilder {
	return newSectionBlockBuilder(newBuilder("SectionBlock", "section").
		coerce("text", markdownLike).
		coerce("fields", markdownLike))
}

// NewDividerBlock creates a visual divider block.
func NewDividerBlock() *DividerBlockBuilder {
	return newDividerBlockBuilder(newBuilder("DividerBlock", "divider"))
}

// NewActionsBlock creates a row of interactive elements.
func NewActionsBlock() *ActionsBlockBuilder {
	return newActionsBlockBuilder(newBuilder("ActionsBlock", "actions"))
}

// NewContextBlock creates compact contextual text and images.
func NewContextBlock() *ContextBlockBuilder {
	return newContextBlockBuilder(newBuilder("ContextBlock", "context"))
}

// NewFileBlock creates a block for a Slack remote file.
func NewFileBlock() *FileBlockBuilder {
	builder := newBuilder("FileBlock", "file")
	builder.values["source"] = "remote"
	return newFileBlockBuilder(builder)
}

// NewHeaderBlock creates a prominent plain-text heading.
func NewHeaderBlock() *HeaderBlockBuilder {
	return newHeaderBlockBuilder(newBuilder("HeaderBlock", "header").coerce("text", plainTextLike))
}

// NewImageBlock creates an image block with optional title text.
func NewImageBlock() *ImageBlockBuilder {
	return newImageBlockBuilder(newBuilder("ImageBlock", "image").coerce("title", plainTextLike))
}

// NewInputBlock creates a labelled form control.
func NewInputBlock() *InputBlockBuilder {
	return newInputBlockBuilder(newBuilder("InputBlock", "input").
		coerce("label", plainTextLike).
		coerce("hint", plainTextLike))
}

// NewMarkdownBlock creates a block rendered from GitHub-flavoured Markdown.
func NewMarkdownBlock() *MarkdownBlockBuilder {
	return newMarkdownBlockBuilder(newBuilder("MarkdownBlock", "markdown"))
}

// NewRichTextBlock creates a rich-text layout block.
func NewRichTextBlock() *RichTextBlockBuilder {
	return newRichTextBlockBuilder(newBuilder("RichTextBlock", "rich_text"))
}

// NewTableBlock creates a table from raw-text or rich-text cells.
func NewTableBlock() *TableBlockBuilder {
	return newTableBlockBuilder(newBuilder("TableBlock", "table"))
}

// NewVideoBlock creates an embedded video block.
func NewVideoBlock() *VideoBlockBuilder {
	return newVideoBlockBuilder(newBuilder("VideoBlock", "video").
		coerce("title", plainTextLike).
		coerce("description", plainTextLike))
}
