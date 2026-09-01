package slackblocks_test

import (
	"encoding/json"
	"fmt"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
	slackapi "github.com/slack-go/slack"
)

func Example() {
	block := slackblocks.NewSectionBlock().
		Text("*Deployment ready*").
		BlockID("summary")
	options := []slackapi.MsgOption{
		slackapi.MsgOptionText("Deployment ready", false),
		slackapi.MsgOptionBlocks(block),
	}

	encoded, err := json.Marshal(block)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%s %d %s\n", block.BlockType(), len(options), encoded)
	// Output: section 2 {"block_id":"summary","text":{"text":"*Deployment ready*","type":"mrkdwn"},"type":"section"}
}
