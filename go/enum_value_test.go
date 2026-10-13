package slackblocks_test

import (
	sb "github.com/nicklambourne/slackblocks/go/v2"
	"testing"
)

func TestSuppliedEnumsRejectEmptyAndUnknownValues(t *testing.T) {
	cases := []struct {
		name, field, path string
		build             func(string) sb.Buildable
		valid             []string
	}{
		{"button", "style", "style", func(v string) sb.Buildable { return sb.NewButton().Text("Go").Style(sb.ButtonStyle(v)) }, []string{"primary", "danger"}},
		{"workflow button", "style", "style", func(v string) sb.Buildable {
			return sb.NewWorkflowButton().Text("Go").ActionID("go").Workflow(sb.NewWorkflow().Trigger(sb.NewTrigger().URL("https://example.com/trigger"))).Style(sb.ButtonStyle(v))
		}, []string{"primary", "danger"}},
		{"column", "align", "ColumnSettings.align", func(v string) sb.Buildable { return sb.NewColumnSettings().Align(sb.ColumnAlign(v)) }, []string{"left", "center", "right"}},
		{"confirmation", "style", "Confirmation.style", func(v string) sb.Buildable {
			return sb.NewConfirmation().Title("Confirm").Text("Proceed?").Confirm("Yes").Deny("No").Style(sb.ButtonStyle(v))
		}, []string{"primary", "danger"}},
		{"list", "style", "style", func(v string) sb.Buildable {
			return sb.NewRichTextList().Style(sb.RichTextListStyle(v)).Elements(sb.NewRichTextSection().Elements(sb.NewRichText().Text("Item")))
		}, []string{"bullet", "ordered"}},
		{"response", "response_type", "MessageResponse.response_type", func(v string) sb.Buildable {
			return sb.NewMessageResponse().Text("Reply").ResponseType(sb.ResponseType(v))
		}, []string{"in_channel", "ephemeral"}},
		{"webhook", "response_type", "WebhookMessage.response_type", func(v string) sb.Buildable {
			return sb.NewWebhookMessage().Text("Reply").ResponseType(sb.ResponseType(v))
		}, []string{"in_channel", "ephemeral"}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			for _, value := range []string{"", "unknown"} {
				_, err := c.build(value).Build()
				assertValidationError(t, err, sb.TypeMismatch, c.path)
			}
			for _, value := range c.valid {
				object, err := c.build(value).Build()
				if err != nil {
					t.Fatalf("valid %q: %v", value, err)
				}
				if object[c.field] != value {
					t.Fatalf("got %v, want %q", object[c.field], value)
				}
			}
		})
	}
}

func TestEnumOmissionKeepsExistingDefaults(t *testing.T) {
	for _, object := range []sb.Object{
		sb.NewButton().Text("Go").MustBuild(),
		sb.NewWorkflowButton().Text("Go").ActionID("go").Workflow(sb.NewWorkflow().Trigger(sb.NewTrigger().URL("https://example.com/trigger"))).MustBuild(),
		sb.NewConfirmation().Title("Confirm").Text("Proceed?").Confirm("Yes").Deny("No").MustBuild(),
	} {
		if _, present := object["style"]; present {
			t.Fatal("omitted style serialized")
		}
	}
	if _, present := sb.NewColumnSettings().MustBuild()["align"]; present {
		t.Fatal("omitted alignment serialized")
	}
	if _, present := sb.NewWebhookMessage().Text("Reply").MustBuild()["response_type"]; present {
		t.Fatal("omitted webhook visibility serialized")
	}
	if sb.NewMessageResponse().MustBuild()["response_type"] != "in_channel" {
		t.Fatal("response default changed")
	}
	_, err := sb.NewRichTextList().Elements(sb.NewRichTextSection().Elements(sb.NewRichText().Text("Item"))).Build()
	assertValidationError(t, err, sb.MissingRequired, "")
}

func TestNestedRawEnumValuesAreValidatedInTheirContext(t *testing.T) {
	button := sb.NewButton().Text("Go").MustBuild()
	button["style"] = 17
	assertValidationError(t, sb.Validate(button), sb.TypeMismatch, "style")
	confirmation := sb.NewConfirmation().Title("Confirm").Text("Proceed?").Confirm("Yes").Deny("No").MustBuild()
	confirmation["style"] = ""
	delete(button, "style")
	button["confirm"] = confirmation
	assertValidationError(t, sb.Validate(button), sb.TypeMismatch, "confirm.style")
	table := sb.NewTableBlock().Rows([]sb.TableCell{sb.NewRawText().Text("A")}).MustBuild()
	table["column_settings"] = []any{sb.Object{"align": ""}}
	assertValidationError(t, sb.Validate(table), sb.TypeMismatch, "column_settings[0].align")
}
