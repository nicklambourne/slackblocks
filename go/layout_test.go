package slackblocks_test

import (
	"encoding/json"
	s "github.com/nicklambourne/slackblocks/go/v3"
	slack "github.com/slack-go/slack"
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
