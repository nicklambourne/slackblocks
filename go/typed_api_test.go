package slackblocks_test

import (
	"encoding/json"
	"reflect"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

func interfaceType[T any]() reflect.Type { return reflect.TypeOf((*T)(nil)).Elem() }

// TestMarkerInterfacesAcceptOnlyTheirRoles pins which builders satisfy each role, so a value of
// the wrong kind cannot be passed to a typed builder method.
func TestMarkerInterfacesAcceptOnlyTheirRoles(t *testing.T) {
	cases := []struct {
		role    reflect.Type
		value   any
		allowed bool
	}{
		{interfaceType[slackblocks.Element](), slackblocks.NewButton(), true},
		{interfaceType[slackblocks.Element](), slackblocks.NewDividerBlock(), false},
		{interfaceType[slackblocks.InputElement](), slackblocks.NewPlainTextInput(), true},
		{interfaceType[slackblocks.InputElement](), slackblocks.NewButton(), false},
		{interfaceType[slackblocks.ContextElement](), slackblocks.NewMarkdown(), true},
		{interfaceType[slackblocks.ContextElement](), slackblocks.NewImageElement(), true},
		{interfaceType[slackblocks.ContextElement](), slackblocks.NewButton(), false},
		{interfaceType[slackblocks.ContextActionsElement](), slackblocks.NewIconButton(), true},
		{interfaceType[slackblocks.ContextActionsElement](), slackblocks.NewButton(), false},
		{interfaceType[slackblocks.TableCell](), slackblocks.NewRawText(), true},
		{interfaceType[slackblocks.TableCell](), slackblocks.NewRawNumber(), false},
		{interfaceType[slackblocks.DataTableCell](), slackblocks.NewRawNumber(), true},
		{interfaceType[slackblocks.Chart](), slackblocks.NewPieChart(), true},
		{interfaceType[slackblocks.Chart](), slackblocks.NewDataSeries(), false},
		{interfaceType[slackblocks.RichTextBlockElement](), slackblocks.NewRichTextSection(), true},
		{interfaceType[slackblocks.RichTextBlockElement](), slackblocks.NewRichText(), false},
		{interfaceType[slackblocks.RichTextSectionElement](), slackblocks.NewRichTextLink(), true},
		{interfaceType[slackblocks.TextObject](), slackblocks.NewPlainText(), true},
		{interfaceType[slackblocks.TextObject](), slackblocks.NewRawText(), false},
		{interfaceType[slackblocks.Block](), slackblocks.NewSectionBlock(), true},
		{interfaceType[slackblocks.Block](), slackblocks.NewAccordion(), true},
		{interfaceType[slackblocks.Block](), slackblocks.NewPaginator(), true},
		{interfaceType[slackblocks.Block](), slackblocks.NewButton(), false},
	}
	for _, test := range cases {
		got := reflect.TypeOf(test.value).Implements(test.role)
		if got != test.allowed {
			t.Errorf("%T implements %s = %v, want %v", test.value, test.role.Name(), got, test.allowed)
		}
	}
}

func TestTypedBuilderMethodsProduceSlackJSON(t *testing.T) {
	block := slackblocks.NewSectionBlock().
		TextObject(slackblocks.NewPlainText().Text("Deploy").Emoji(true)).
		Fields("*Branch*", "*Tests*").
		Accessory(slackblocks.NewButton().Text("Open").ActionID("open").Style(slackblocks.ButtonStylePrimary))
	assertJSON(t, block, `{"type":"section","text":{"type":"plain_text","text":"Deploy","emoji":true},"fields":[{"type":"mrkdwn","text":"*Branch*"},{"type":"mrkdwn","text":"*Tests*"}],"accessory":{"type":"button","text":{"type":"plain_text","text":"Open"},"action_id":"open","style":"primary"}}`)

	// Untyped string constants still convert to named enum types.
	assertJSON(t, slackblocks.NewAlertBlock().Text("Heads up").Level("warning"), `{"type":"alert","text":{"type":"mrkdwn","text":"Heads up"},"level":"warning"}`)

	styled := slackblocks.NewRichText().Text("Ship it").Style(slackblocks.RichTextStyle{"bold": true, "italic": false})
	assertJSON(t, styled, `{"type":"text","text":"Ship it","style":{"bold":true,"italic":false}}`)

	table := slackblocks.NewDataTableBlock().Caption("Scores").Rows(
		[]slackblocks.DataTableCell{slackblocks.NewRawText().Text("Name"), slackblocks.NewRawText().Text("Score")},
		[]slackblocks.DataTableCell{slackblocks.NewRawText().Text("Ada"), slackblocks.NewRawNumber().Value(42).Text("42")},
	)
	assertJSON(t, table, `{"type":"data_table","caption":"Scores","page_size":5,"row_header_column_index":0,"rows":[[{"type":"raw_text","text":"Name"},{"type":"raw_text","text":"Score"}],[{"type":"raw_text","text":"Ada"},{"type":"raw_number","value":42,"text":"42"}]]}`)
}

func TestComponentsAreBlocks(t *testing.T) {
	accordion := slackblocks.NewAccordion().Sections(
		slackblocks.NewAccordionSection().Title("Details").Blocks(slackblocks.NewDividerBlock()),
	)
	message, err := slackblocks.NewMessage().Channel("C123").Blocks(accordion).Build()
	if err != nil {
		t.Fatal(err)
	}
	blocks := message["blocks"].([]any)
	if len(blocks) != 1 || blocks[0].(slackblocks.Object)["type"] != "container" {
		t.Fatalf("accordion did not expand into a container block: %#v", blocks)
	}
}

func assertJSON(t *testing.T, value slackblocks.Buildable, want string) {
	t.Helper()
	built, err := value.Build()
	if err != nil {
		t.Fatal(err)
	}
	var got, expected any
	data, _ := json.Marshal(built)
	_ = json.Unmarshal(data, &got)
	if err := json.Unmarshal([]byte(want), &expected); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(got, expected) {
		t.Fatalf("JSON mismatch\nwant: %s\n got: %s", want, data)
	}
}
