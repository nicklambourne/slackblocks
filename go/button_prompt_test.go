package slackblocks_test

import (
	slackblocks "github.com/nicklambourne/slackblocks/go/v3"
	"testing"
)

func TestButtonPromptNativeFields(t *testing.T) {
	builder := slackblocks.NewButton().Text("Ask").AgentPrompt("Ask Slackbot").AgentPromptDisplay("Summary")
	wire, err := builder.Build()
	if err != nil {
		t.Fatal(err)
	}
	if wire["agent_prompt"] != "Ask Slackbot" || wire["agent_prompt_display"] != "Summary" {
		t.Fatalf("lost native fields: %#v", wire)
	}
	builder.AgentPrompt("Changed")
	if wire["agent_prompt"] != "Ask Slackbot" {
		t.Fatal("builder changed an earlier result")
	}
	for _, field := range []string{"agent_prompt", "agent_prompt_display"} {
		if _, err := slackblocks.NewButton().Text("Ask").Set(field, 1).Build(); err == nil {
			t.Fatalf("accepted non-string %s", field)
		}
	}
}
