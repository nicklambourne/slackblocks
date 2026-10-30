package slackblocks_test

import (
	"encoding/json"
	"errors"
	s "github.com/nicklambourne/slackblocks/go/v3"
	slack "github.com/slack-go/slack"
	"strings"
	"testing"
)

func layoutGrid() *s.LayoutBlockBuilder {
	return s.NewLayoutBlock().Label("Grid").Rows(s.NewLayoutRow().Items(s.NewLayoutItem().Block(s.NewDividerBlock())))
}
func TestNativeLayoutBoundariesAndSdk(t *testing.T) {
	for _, value := range []s.Buildable{
		s.NewMessage().Channel("C123").Blocks(copies(50, func(int) s.Block { return layoutGrid() })...),
		s.NewHomeTab().Blocks(copies(100, func(int) s.Block { return layoutGrid() })...),
		s.NewMessage().Channel("C123").Blocks(layoutGrid().Fallback(copies(51, func(int) s.LayoutFallback { return s.NewDividerBlock() })...)),
		s.NewMessage().Channel("C123").Blocks(copies(50, func(int) s.Block {
			return s.NewContainerBlock().Title("Existing").ChildBlocks(copies(10, func(int) s.Block { return s.NewDividerBlock() })...)
		})...),
	} {
		if _, err := value.Build(); err != nil {
			t.Fatal(err)
		}
	}
	block := layoutGrid().IsLabelVisible(false).GridLines(s.LayoutGridLinesRows).Fallback()
	// This is the SDK's real Block interface and encoder, with the new layout wire type.
	var sdkBlock slack.Block = block
	data, err := json.Marshal(slack.Blocks{BlockSet: []slack.Block{sdkBlock}})
	if err != nil {
		t.Fatal(err)
	}
	var wire []map[string]any
	if err = json.Unmarshal(data, &wire); err != nil {
		t.Fatal(err)
	}
	if len(wire) != 1 || wire[0]["type"] != "layout" || wire[0]["is_label_visible"] != false {
		t.Fatalf("lost layout fields: %s", data)
	}
	assertJSON(t, s.NewLayoutItem().Block(s.NewDividerBlock()).Width(s.LayoutWidthWide).HorizontalAlignment(s.LayoutAlignmentEnd), `{"block":{"type":"divider"},"width":"wide","horizontal_alignment":"end"}`)
}

// Nested blocks must receive the same validation as standalone values, including
// when a caller passes an already materialized wire object to Validate or Set.
func TestLayoutValidatesNestedWireContent(t *testing.T) {
	for _, position := range []string{"child", "fallback"} {
		for _, bad := range []struct {
			name     string
			value    s.Object
			category s.ErrorCategory
		}{
			{"missing-header-text", s.Object{"type": "header"}, s.MissingRequired},
			{"missing-section-content", s.Object{"type": "section"}, s.MissingRequired},
			{"overlong-header", s.Object{"type": "header", "text": s.Object{"type": "plain_text", "text": strings.Repeat("x", 151)}}, s.LengthExceeded},
			{"invalid-image-source", s.Object{"type": "image", "alt_text": "Image", "image_url": "https://example.com/a.png", "slack_file": s.Object{"id": "F12345678"}}, s.MutuallyExclusive},
		} {
			t.Run(position+"/"+bad.name, func(t *testing.T) {
				value := layoutGrid().MustBuild()
				if position == "child" {
					value["rows"].([]any)[0].(s.Object)["items"].([]any)[0].(s.Object)["block"] = bad.value
				} else {
					value["fallback"] = []any{bad.value}
				}
				builder := s.NewLayoutBlock().Label("Grid")
				builder.Set("rows", value["rows"])
				if position == "fallback" {
					builder.Set("fallback", value["fallback"])
				}
				for _, entry := range []struct {
					name string
					err  error
				}{
					{"Validate", s.Validate(value)}, {"Build", buildError(builder)},
				} {
					var validation *s.ValidationError
					if !errors.As(entry.err, &validation) || validation.Category != bad.category {
						t.Fatalf("%s: got %v, want %s", entry.name, entry.err, bad.category)
					}
					field := "rows[0].items[0].block"
					if position == "fallback" {
						field = "fallback[0]"
					}
					if !strings.Contains(validation.Path, field) {
						t.Fatalf("lost nested path: %s", validation.Path)
					}
				}
			})
		}
	}
}
