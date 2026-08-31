package slackblocks_test

import (
	"encoding/json"
	"errors"
	"reflect"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

func TestSectionBlockFluentBuild(t *testing.T) {
	block, err := slackblocks.NewSectionBlock().
		Text("Hello, *world*!").
		Fields("*Status*\nReady").
		BlockID("status").
		Build()
	if err != nil {
		t.Fatal(err)
	}

	want := slackblocks.Object{
		"type": "section",
		"text": slackblocks.Object{"type": "mrkdwn", "text": "Hello, *world*!"},
		"fields": []any{
			slackblocks.Object{"type": "mrkdwn", "text": "*Status*\nReady"},
		},
		"block_id": "status",
	}
	if !reflect.DeepEqual(block, want) {
		t.Fatalf("unexpected block:\nwant: %#v\n got: %#v", want, block)
	}
}

func TestNestedBuildersMaterialise(t *testing.T) {
	option, err := slackblocks.NewOption().Text("Deploy").Value("deploy").Build()
	if err != nil {
		t.Fatal(err)
	}
	want := slackblocks.Object{
		"text":  slackblocks.Object{"type": "plain_text", "text": "Deploy"},
		"value": "deploy",
	}
	if !reflect.DeepEqual(option, want) {
		t.Fatalf("unexpected option: %#v", option)
	}
}

func TestValidationErrorCategory(t *testing.T) {
	_, err := slackblocks.NewPlainText().Text("").Build()
	if err == nil {
		t.Fatal("expected validation error")
	}
	var validation *slackblocks.ValidationError
	if !errors.As(err, &validation) {
		t.Fatalf("expected ValidationError, got %T", err)
	}
	if validation.Category != slackblocks.LengthExceeded {
		t.Fatalf("unexpected category: %s", validation.Category)
	}
}

func TestBuilderMarshalsAsSlackJSON(t *testing.T) {
	encoded, err := json.Marshal(slackblocks.NewMarkdown().Text("Hello"))
	if err != nil {
		t.Fatal(err)
	}
	if string(encoded) != `{"text":"Hello","type":"mrkdwn"}` {
		t.Fatalf("unexpected JSON: %s", encoded)
	}
}

func TestMustBuild(t *testing.T) {
	built := slackblocks.NewDividerBlock().MustBuild()
	if !reflect.DeepEqual(built, slackblocks.Object{"type": "divider"}) {
		t.Fatalf("unexpected block: %#v", built)
	}

	defer func() {
		if recovered := recover(); recovered == nil {
			t.Fatal("expected MustBuild to panic for an invalid object")
		}
	}()
	slackblocks.NewPlainText().Text("").MustBuild()
}

func TestValidationErrorFormatting(t *testing.T) {
	withPath := (&slackblocks.ValidationError{
		Category: slackblocks.MissingRequired,
		Path:     "SectionBlock.text",
		Detail:   "text or fields is required",
	}).Error()
	if withPath != "missing-required at SectionBlock.text: text or fields is required" {
		t.Fatalf("unexpected error: %q", withPath)
	}

	withoutPath := (&slackblocks.ValidationError{
		Category: slackblocks.InvalidUsage,
		Detail:   "invalid value",
	}).Error()
	if withoutPath != "invalid-usage: invalid value" {
		t.Fatalf("unexpected error: %q", withoutPath)
	}
}
