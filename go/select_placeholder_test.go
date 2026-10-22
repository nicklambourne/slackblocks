package slackblocks_test

import (
	sb "github.com/nicklambourne/slackblocks/go/v3"
	"strings"
	"testing"
)

func TestAllSelectPlaceholderBoundaries(t *testing.T) {
	factories := map[string]func(string) sb.Buildable{
		"ExternalSelect":          func(s string) sb.Buildable { return sb.NewExternalSelect().Placeholder(s) },
		"ExternalMultiSelect":     func(s string) sb.Buildable { return sb.NewExternalMultiSelect().Placeholder(s) },
		"UserSelect":              func(s string) sb.Buildable { return sb.NewUserSelect().Placeholder(s) },
		"UserMultiSelect":         func(s string) sb.Buildable { return sb.NewUserMultiSelect().Placeholder(s) },
		"ChannelSelect":           func(s string) sb.Buildable { return sb.NewChannelSelect().Placeholder(s) },
		"ChannelMultiSelect":      func(s string) sb.Buildable { return sb.NewChannelMultiSelect().Placeholder(s) },
		"ConversationSelect":      func(s string) sb.Buildable { return sb.NewConversationSelect().Placeholder(s) },
		"ConversationMultiSelect": func(s string) sb.Buildable { return sb.NewConversationMultiSelect().Placeholder(s) },
		"StaticSelect": func(s string) sb.Buildable {
			return sb.NewStaticSelect().Placeholder(s).Options(sb.NewOption().Text("A").Value("a"))
		},
		"StaticMultiSelect": func(s string) sb.Buildable {
			return sb.NewStaticMultiSelect().Placeholder(s).Options(sb.NewOption().Text("A").Value("a"))
		},
	}
	for name, makeValue := range factories {
		t.Run(name, func(t *testing.T) {
			for _, unit := range []string{"x", "🚀"} {
				for _, n := range []int{149, 150, 151} {
					object, err := makeValue(strings.Repeat(unit, n)).Build()
					if n == 151 {
						assertValidationError(t, err, sb.LengthExceeded, "placeholder.text")
					} else if err != nil {
						t.Fatal(err)
					} else if object["placeholder"].(sb.Object)["text"] != strings.Repeat(unit, n) {
						t.Fatal("placeholder changed")
					}
				}
			}
		})
	}
}
