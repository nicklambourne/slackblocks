package slackblocks_test

import (
	"encoding/json"
	"fmt"
	"strings"
	"testing"

	sb "github.com/nicklambourne/slackblocks/go/v3"
)

func TestExactRawNumber(t *testing.T) {
	for _, value := range []string{"9007199254740993", "-9007199254740993", "0", "-0", "0.12345678901234567890123456789", "1e+100"} {
		t.Run(value, func(t *testing.T) {
			cell := sb.NewRawNumber().ValueNumber(json.Number(value)).Text(value)
			payload, err := json.Marshal(sb.NewDataTableBlock().Caption("Exact values").Rows([]sb.DataTableCell{sb.NewRawText().Text("Value")}, []sb.DataTableCell{cell}))
			if err != nil {
				t.Fatal(err)
			}
			if !strings.Contains(string(payload), `"value":`+value) {
				t.Fatalf("number changed or became a string: %s", payload)
			}
			decoder := json.NewDecoder(strings.NewReader(string(payload)))
			decoder.UseNumber()
			var object map[string]any
			if err := decoder.Decode(&object); err != nil {
				t.Fatal(err)
			}
			got := object["rows"].([]any)[1].([]any)[0].(map[string]any)["value"]
			if got != json.Number(value) {
				t.Fatalf("got %v, want exact %s", got, value)
			}
		})
	}
}

func TestExactRawNumberRejectsInvalidSyntaxAndCanBeCorrected(t *testing.T) {
	for _, value := range []string{"", "NaN", "Infinity", "01", "+1", ".5", "1.", "1e", " 1", "1\n", `"1"`} {
		cell := sb.NewRawNumber().ValueNumber(json.Number(value)).Text("Number")
		_, err := cell.Build()
		assertValidationError(t, err, sb.TypeMismatch, "value")
		if _, err := json.Marshal(cell); err == nil {
			t.Fatalf("marshal accepted %q", value)
		}
		if _, err := cell.ValueNumber("1").Build(); err != nil {
			t.Fatalf("correction failed: %v", err)
		}
	}
	object, err := sb.NewRawNumber().Value(1.5).Text("1.5").Build()
	if err != nil || object["value"] != 1.5 {
		t.Fatalf("existing Value changed: %v %v", object, err)
	}
}

func TestGoNestedBuilderAndBuiltSnapshotOwnership(t *testing.T) {
	child := sb.NewSectionBlock().Text("First")
	parent := sb.NewMessage().Channel("C123").Blocks(child)
	first := parent.MustBuild()
	child.Text("Second")
	second := parent.MustBuild()
	getText := func(object sb.Object) sb.Object { return object["blocks"].([]any)[0].(sb.Object)["text"].(sb.Object) }
	if getText(first)["text"] != "First" || getText(second)["text"] != "Second" {
		t.Fatal("nested reference or snapshot semantics changed")
	}
	getText(first)["text"] = "Export edit"
	if getText(second)["text"] != "Second" || getText(parent.MustBuild())["text"] != "Second" {
		t.Fatal("export mutation reached another snapshot or builder")
	}
	plain := sb.NewPlainText().Text("Hi").MustBuild()
	explicit := sb.NewPlainText().Text("Hi").Emoji(false).MustBuild()
	if _, exists := plain["emoji"]; exists {
		t.Fatal("omitted field was added")
	}
	if value, exists := explicit["emoji"]; !exists || value != false {
		t.Fatal("explicit false was dropped")
	}
}

func ExampleRawNumberBuilder_ValueNumber() {
	cell := sb.NewRawNumber().ValueNumber(json.Number("9007199254740993")).Text("9,007,199,254,740,993")
	data, err := json.Marshal(cell)
	if err != nil {
		panic(err)
	}
	fmt.Println(string(data))
	// Output:
	// {"text":"9,007,199,254,740,993","type":"raw_number","value":9007199254740993}
}
