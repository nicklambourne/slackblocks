package slackblocks

import (
	"encoding/json"
	"fmt"

	slackapi "github.com/slack-go/slack"
)

type builtSlackBlock struct {
	object Object
}

func (b builtSlackBlock) BlockType() slackapi.MessageBlockType {
	return slackapi.MessageBlockType(objectType(b.object))
}

func (b builtSlackBlock) ID() string {
	value, _ := b.object["block_id"].(string)
	return value
}

func (b builtSlackBlock) MarshalJSON() ([]byte, error) {
	return json.Marshal(b.object)
}

func nativeSlackBlocks(objects []Object) []slackapi.Block {
	blocks := make([]slackapi.Block, len(objects))
	for index, object := range objects {
		blocks[index] = builtSlackBlock{object: object}
	}
	return blocks
}

// NewAccordionSection creates one Slack-native collapsible container for an
// accordion. Add its content with Blocks and choose its initial state with
// Expanded.
func NewAccordionSection() *AccordionSectionBuilder {
	builder := newBuilder("AccordionSection", "container").
		coerce("title", plainTextLike).
		coerce("subtitle", markdownLike)
	builder.values["is_collapsible"] = true
	builder.values["default_collapsed"] = true
	return newAccordionSectionBuilder(builder.withTransform(func(object Object) (Object, error) {
		if blocks, ok := object["blocks"]; ok {
			object["child_blocks"] = blocks
			delete(object, "blocks")
		}
		if expanded, ok := object["_expanded"].(bool); ok {
			object["default_collapsed"] = !expanded
			delete(object, "_expanded")
		}
		return object, nil
	}))
}

// AccordionBuilder assembles independently collapsible sections.
type AccordionBuilder struct {
	sections []any
}

// NewAccordion creates a Slack-native accordion component.
func NewAccordion() *AccordionBuilder { return &AccordionBuilder{} }

// Sections appends accordion sections in display order.
func (b *AccordionBuilder) Sections(values ...any) *AccordionBuilder {
	b.sections = append(b.sections, values...)
	return b
}

// BuildMany materialises the accordion as ordinary container blocks.
func (b *AccordionBuilder) BuildMany() ([]Object, error) {
	if b == nil || len(b.sections) == 0 {
		return nil, validationError(MissingRequired, "Accordion.sections", "expected at least one section")
	}
	result := make([]Object, 0, len(b.sections))
	for index, raw := range b.sections {
		built, err := materialise(raw)
		if err != nil {
			return nil, err
		}
		section, ok := built.(Object)
		if !ok || objectType(section) != "container" || section["is_collapsible"] != true {
			return nil, validationError(TypeMismatch, fmt.Sprintf("Accordion.sections[%d]", index), "expected an AccordionSection")
		}
		result = append(result, section)
	}
	return result, nil
}

// SlackBlocks expands the accordion for direct use with slack-go.
func (b *AccordionBuilder) SlackBlocks() ([]slackapi.Block, error) {
	objects, err := b.BuildMany()
	if err != nil {
		return nil, err
	}
	return nativeSlackBlocks(objects), nil
}

// PaginatorBuilder renders one page of blocks plus navigation controls.
type PaginatorBuilder struct {
	blocks            []any
	actionIDPrefix    string
	page              int
	pageSize          int
	previousText      string
	nextText          string
	showPageIndicator bool
	blockID           string
}

// NewPaginator creates a pure Block Kit paginator. Interaction handling remains
// in the application; generated button values contain the next one-based page.
func NewPaginator() *PaginatorBuilder {
	return &PaginatorBuilder{
		page:              1,
		pageSize:          5,
		previousText:      "Previous",
		nextText:          "Next",
		showPageIndicator: true,
	}
}

// Blocks appends source blocks in display order.
func (b *PaginatorBuilder) Blocks(values ...any) *PaginatorBuilder {
	b.blocks = append(b.blocks, values...)
	return b
}

// ActionIDPrefix sets the prefix used for generated action IDs.
func (b *PaginatorBuilder) ActionIDPrefix(value string) *PaginatorBuilder {
	b.actionIDPrefix = value
	return b
}

// Page selects the one-based page to render.
func (b *PaginatorBuilder) Page(value int) *PaginatorBuilder { b.page = value; return b }

// PageSize sets the number of source blocks displayed per page.
func (b *PaginatorBuilder) PageSize(value int) *PaginatorBuilder { b.pageSize = value; return b }

// PreviousText sets the previous-page button label.
func (b *PaginatorBuilder) PreviousText(value string) *PaginatorBuilder {
	b.previousText = value
	return b
}

// NextText sets the next-page button label.
func (b *PaginatorBuilder) NextText(value string) *PaginatorBuilder {
	b.nextText = value
	return b
}

// ShowPageIndicator controls the Page n of m context block.
func (b *PaginatorBuilder) ShowPageIndicator(value bool) *PaginatorBuilder {
	b.showPageIndicator = value
	return b
}

// BlockID sets the generated actions block identifier.
func (b *PaginatorBuilder) BlockID(value string) *PaginatorBuilder { b.blockID = value; return b }

// BuildMany materialises the selected page and its controls.
func (b *PaginatorBuilder) BuildMany() ([]Object, error) {
	if b == nil || len(b.blocks) == 0 {
		return nil, validationError(MissingRequired, "Paginator.blocks", "expected at least one block")
	}
	if b.actionIDPrefix == "" {
		return nil, validationError(MissingRequired, "Paginator.actionIDPrefix", "expected a non-empty prefix")
	}
	if b.page < 1 {
		return nil, validationError(OutOfRange, "Paginator.page", "expected a positive integer")
	}
	if b.pageSize < 1 {
		return nil, validationError(OutOfRange, "Paginator.pageSize", "expected a positive integer")
	}

	blocks := make([]Object, 0, len(b.blocks))
	for index, raw := range b.blocks {
		built, err := materialise(raw)
		if err != nil {
			return nil, err
		}
		block, ok := built.(Object)
		if !ok || objectType(block) == "" {
			return nil, validationError(TypeMismatch, fmt.Sprintf("Paginator.blocks[%d]", index), "expected a block")
		}
		blocks = append(blocks, block)
	}

	pageCount := (len(blocks) + b.pageSize - 1) / b.pageSize
	if b.page > pageCount {
		return nil, validationError(OutOfRange, "Paginator.page", "expected a value between 1 and %d", pageCount)
	}
	start := (b.page - 1) * b.pageSize
	end := start + b.pageSize
	if end > len(blocks) {
		end = len(blocks)
	}
	result := append([]Object(nil), blocks[start:end]...)
	if pageCount == 1 {
		return result, nil
	}

	controls := []any{}
	if b.page > 1 {
		controls = append(controls, NewButton().Text(b.previousText).ActionID(b.actionIDPrefix+".previous").Value(fmt.Sprint(b.page-1)))
	}
	if b.page < pageCount {
		controls = append(controls, NewButton().Text(b.nextText).ActionID(b.actionIDPrefix+".next").Value(fmt.Sprint(b.page+1)))
	}
	if b.showPageIndicator {
		indicator, err := NewContextBlock().Elements(NewMarkdown().Text(fmt.Sprintf("Page %d of %d", b.page, pageCount))).Build()
		if err != nil {
			return nil, err
		}
		result = append(result, indicator)
	}
	actions := NewActionsBlock().Elements(controls...)
	if b.blockID != "" {
		actions.BlockID(b.blockID)
	}
	builtActions, err := actions.Build()
	if err != nil {
		return nil, err
	}
	return append(result, builtActions), nil
}

// SlackBlocks expands the selected page for direct use with slack-go.
func (b *PaginatorBuilder) SlackBlocks() ([]slackapi.Block, error) {
	objects, err := b.BuildMany()
	if err != nil {
		return nil, err
	}
	return nativeSlackBlocks(objects), nil
}
