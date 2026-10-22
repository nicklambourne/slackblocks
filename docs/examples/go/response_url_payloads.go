package main

import (
	"encoding/json"
	"os"

	sb "github.com/nicklambourne/slackblocks/go/v3"
)

func main() {
	payloads := []sb.Object{
		sb.NewMessageResponse().Text("Only you").ResponseType(sb.ResponseTypeEphemeral).MustBuild(),
		sb.NewMessageResponse().Text("Everyone").ResponseType(sb.ResponseTypeInChannel).MustBuild(),
		sb.NewWebhookMessage().Text("Updated").ReplaceOriginal(true).MustBuild(),
		sb.NewWebhookMessage().DeleteOriginal(true).MustBuild(),
	}
	// POST one selected payload to the genuine interaction's response_url.
	if err := json.NewEncoder(os.Stdout).Encode(payloads); err != nil {
		panic(err)
	}
}
