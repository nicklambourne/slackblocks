package main

import (
	"encoding/json"
	"fmt"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

func main() {
	payload, err := slackblocks.NewMessage().
		Channel("C0123456").
		Blocks(
			slackblocks.NewSectionBlock().
				Text("Hello from slackblocks!").
				BlockID("hello"),
		).
		Build()
	if err != nil {
		panic(err)
	}
	encoded, err := json.Marshal(payload)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(encoded))
}
