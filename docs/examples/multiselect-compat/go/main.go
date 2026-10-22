package main

import (
	"encoding/json"
	s "github.com/nicklambourne/slackblocks/go/v3"
	"os"
)

type choice interface {
	s.Element
	s.InputElement
}

func emit(family string, element choice) {
	blocks := map[string]s.Block{
		"actions": s.NewActionsBlock().Elements(element).BlockID("choices"),
		"section": s.NewSectionBlock().Text("Choose values").Accessory(element).BlockID("choices"),
		"input":   s.NewInputBlock().Label("Choose values").Element(element).BlockID("choices"),
	}
	for _, parent := range []string{"actions", "section", "input"} {
		var payload any
		var err error
		surface := "message"
		if parent == "input" {
			surface = "modal"
			payload, err = s.NewModal().Title("Multi-select probe").Blocks(blocks[parent]).Submit("Save").Build()
		} else {
			payload, err = s.NewMessage().Channel("C0123456").Text("Choose values").Blocks(blocks[parent]).Build()
		}
		if err != nil {
			panic(err)
		}
		if err := json.NewEncoder(os.Stdout).Encode(map[string]any{"family": family, "parent": parent, "surface": surface, "payload": payload}); err != nil {
			panic(err)
		}
	}
}
func main() {
	emit("static", s.NewStaticMultiSelect().ActionID("choose").Options(s.NewOption().Text("One").Value("one")))
	emit("external", s.NewExternalMultiSelect().ActionID("choose").MinQueryLength(0))
	emit("users", s.NewUserMultiSelect().ActionID("choose"))
	emit("conversations", s.NewConversationMultiSelect().ActionID("choose"))
	emit("channels", s.NewChannelMultiSelect().ActionID("choose"))
}
