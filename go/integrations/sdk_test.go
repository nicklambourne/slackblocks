package integrations_test

import (
	"encoding/json"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v3"
	slack "github.com/slack-go/slack"
)

// Exercise a consumer's selected SDK with both native and slackblocks values.
func TestMixedNativeBlocks(t *testing.T) {
	blocks := slack.Blocks{BlockSet: []slack.Block{
		slack.NewDividerBlock(),
		slackblocks.NewSectionBlock().Text("Hello from the newer SDK").BlockID("greeting"),
	}}
	encoded, err := json.Marshal(blocks)
	if err != nil {
		t.Fatal(err)
	}
	var decoded []struct {
		Type    string `json:"type"`
		BlockID string `json:"block_id"`
		Text    struct {
			Text string `json:"text"`
		} `json:"text"`
	}
	if err := json.Unmarshal(encoded, &decoded); err != nil {
		t.Fatal(err)
	}
	if len(decoded) != 2 || decoded[0].Type != "divider" || decoded[1].Type != "section" || decoded[1].BlockID != "greeting" || decoded[1].Text.Text != "Hello from the newer SDK" {
		t.Fatalf("native SDK lost block content: %s", encoded)
	}
}
