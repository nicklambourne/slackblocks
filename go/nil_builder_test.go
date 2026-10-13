package slackblocks_test

import (
	"encoding/json"
	"testing"

	sb "github.com/nicklambourne/slackblocks/go/v2"
)

func TestTypedNilNestedBuildersReturnValidationErrors(t *testing.T) {
	tests := []struct {
		name    string
		builder sb.Buildable
		path    string
	}{
		{"optional confirmation", sb.NewButton().Text("Save").Confirm(nil), "Button.confirm"},
		{"required text", sb.NewButton().TextObject(nil), "Button.text"},
		{"input role", sb.NewInputBlock().Label("Name").Element((*sb.PlainTextInputBuilder)(nil)), "InputBlock.element"},
		{"option collection", sb.NewStaticSelect().Options(nil), "StaticSelect.options[0]"},
		{"block collection", sb.NewMessage().Channel("C123").Blocks((*sb.SectionBlockBuilder)(nil)), "Message.blocks[0]"},
		{"table cell", sb.NewTableBlock().Rows([]sb.TableCell{(*sb.RawTextBuilder)(nil)}), "TableBlock.rows[0][0]"},
		{"plan task", sb.NewPlanBlock().Title("Release").Tasks(nil), "PlanBlock.tasks[0]"},
		{"nested initial value", sb.NewRichTextInput().ActionID("notes").InitialValue(nil), "RichTextInput.initial_value"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := tt.builder.Build()
			assertValidationError(t, err, sb.TypeMismatch, tt.path)
			_, err = json.Marshal(tt.builder)
			assertValidationError(t, err, sb.TypeMismatch, tt.path)
		})
	}
}

func TestNilFailureDoesNotChangeExistingSnapshot(t *testing.T) {
	confirm := sb.NewConfirmation().Title("Confirm").Text("Continue?").Confirm("Yes").Deny("No")
	button := sb.NewButton().Text("Save").Confirm(confirm)
	before := button.MustBuild()
	encoded, err := json.Marshal(before)
	if err != nil {
		t.Fatal(err)
	}
	_, err = button.Confirm(nil).Build()
	assertValidationError(t, err, sb.TypeMismatch, "Button.confirm")
	actual, err := json.Marshal(before)
	if err != nil {
		t.Fatal(err)
	}
	if string(actual) != string(encoded) {
		t.Fatal("failed build changed the previous snapshot")
	}
	if _, err := button.Confirm(confirm).Build(); err != nil {
		t.Fatal(err)
	}
	omitted := sb.NewButton().Text("Save").MustBuild()
	if _, exists := omitted["confirm"]; exists {
		t.Fatal("omitted confirmation was serialized")
	}
}

// A nil custom group must be rejected before its method is dispatched.
type nilGroup struct{}

func (*nilGroup) BuildMany() ([]sb.Object, error) { panic("nil group dispatched") }

func TestTypedNilGroupReturnsValidationError(t *testing.T) {
	_, err := sb.NewMessage().Channel("C123").Set("blocks", []any{(*nilGroup)(nil)}).Build()
	assertValidationError(t, err, sb.TypeMismatch, "Message.blocks[0]")
}
