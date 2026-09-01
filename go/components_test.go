package slackblocks_test

import (
	"encoding/json"
	"reflect"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
	slackapi "github.com/slack-go/slack"
)

func TestAccordionExpandsInsideParentBlocks(t *testing.T) {
	message, err := slackblocks.NewMessage().Channel("C123").Blocks(
		slackblocks.NewAccordion().Sections(
			slackblocks.NewAccordionSection().Title("First").Expanded(true).Blocks(slackblocks.NewSectionBlock().Text("One")),
			slackblocks.NewAccordionSection().Title("Second").Blocks(slackblocks.NewSectionBlock().Text("Two")),
		),
	).Build()
	if err != nil {
		t.Fatal(err)
	}
	blocks := message["blocks"].([]any)
	if len(blocks) != 2 {
		t.Fatalf("expected two expanded blocks, got %d", len(blocks))
	}
	first := blocks[0].(slackblocks.Object)
	second := blocks[1].(slackblocks.Object)
	if first["default_collapsed"] != false || second["default_collapsed"] != true {
		t.Fatalf("unexpected accordion state: %#v %#v", first, second)
	}
}

func TestPaginatorRendersPageAndControls(t *testing.T) {
	blocks, err := slackblocks.NewPaginator().
		Blocks(
			slackblocks.NewSectionBlock().Text("One"),
			slackblocks.NewSectionBlock().Text("Two"),
			slackblocks.NewSectionBlock().Text("Three"),
		).
		ActionIDPrefix("results").
		PageSize(1).
		Page(2).
		BuildMany()
	if err != nil {
		t.Fatal(err)
	}
	if got := []string{blocks[0]["type"].(string), blocks[1]["type"].(string), blocks[2]["type"].(string)}; !reflect.DeepEqual(got, []string{"section", "context", "actions"}) {
		t.Fatalf("unexpected block sequence: %v", got)
	}
	controls := blocks[2]["elements"].([]any)
	if len(controls) != 2 {
		t.Fatalf("expected previous and next controls, got %d", len(controls))
	}
}

func TestPaginatorValidatesInputs(t *testing.T) {
	_, err := slackblocks.NewPaginator().Blocks(slackblocks.NewDividerBlock()).ActionIDPrefix("pages").Page(2).BuildMany()
	if err == nil {
		t.Fatal("expected page range error")
	}
}

func TestPaginatorRendersCustomNavigationOptions(t *testing.T) {
	blocks, err := slackblocks.NewPaginator().
		Blocks(
			slackblocks.NewDividerBlock(),
			slackblocks.NewDividerBlock(),
			slackblocks.NewDividerBlock(),
		).
		ActionIDPrefix("results").
		PageSize(1).
		Page(2).
		PreviousText("Back").
		NextText("More").
		ShowPageIndicator(false).
		BlockID("results-navigation").
		BuildMany()
	if err != nil {
		t.Fatal(err)
	}
	if len(blocks) != 2 || blocks[1]["type"] != "actions" {
		t.Fatalf("unexpected block sequence: %#v", blocks)
	}
	if blocks[1]["block_id"] != "results-navigation" {
		t.Fatalf("unexpected navigation block ID: %#v", blocks[1]["block_id"])
	}
	controls := blocks[1]["elements"].([]any)
	labels := make([]string, len(controls))
	for index, raw := range controls {
		button := raw.(slackblocks.Object)
		labels[index] = button["text"].(slackblocks.Object)["text"].(string)
	}
	if !reflect.DeepEqual(labels, []string{"Back", "More"}) {
		t.Fatalf("unexpected navigation labels: %v", labels)
	}
}

func TestPaginatorSlackBlocksPreservesValidation(t *testing.T) {
	_, err := slackblocks.NewPaginator().SlackBlocks()
	if err == nil {
		t.Fatal("expected missing blocks validation error")
	}
}

func TestAccordionInModalPreservesSurfaceValidation(t *testing.T) {
	_, err := slackblocks.NewModal().
		Title("Details").
		Blocks(slackblocks.NewAccordion().Sections(
			slackblocks.NewAccordionSection().Title("More").Blocks(
				slackblocks.NewSectionBlock().Text("Not supported in modal containers"),
			),
		)).
		Build()
	if err == nil {
		t.Fatal("expected modal surface validation error")
	}
}

func TestPaginatorExpandsToNativeSlackBlocks(t *testing.T) {
	blocks, err := slackblocks.NewPaginator().
		Blocks(
			slackblocks.NewSectionBlock().Text("One").BlockID("page-item"),
			slackblocks.NewSectionBlock().Text("Two"),
		).
		ActionIDPrefix("results").
		PageSize(1).
		SlackBlocks()
	if err != nil {
		t.Fatal(err)
	}

	_ = slackapi.MsgOptionBlocks(blocks...)
	if got := []string{string(blocks[0].BlockType()), string(blocks[1].BlockType()), string(blocks[2].BlockType())}; !reflect.DeepEqual(got, []string{"section", "context", "actions"}) {
		t.Fatalf("unexpected block sequence: %v", got)
	}
	if blocks[0].ID() != "page-item" {
		t.Fatalf("unexpected block ID: %q", blocks[0].ID())
	}
	encoded, err := json.Marshal(blocks)
	if err != nil {
		t.Fatal(err)
	}
	if string(encoded) != `[{"block_id":"page-item","text":{"text":"One","type":"mrkdwn"},"type":"section"},{"elements":[{"text":"Page 1 of 2","type":"mrkdwn"}],"type":"context"},{"elements":[{"action_id":"results.next","text":{"text":"Next","type":"plain_text"},"type":"button","value":"2"}],"type":"actions"}]` {
		t.Fatalf("unexpected native block JSON: %s", encoded)
	}
}

func TestAccordionSlackBlocksPreservesValidation(t *testing.T) {
	_, err := slackblocks.NewAccordion().SlackBlocks()
	if err == nil {
		t.Fatal("expected missing sections validation error")
	}
}
