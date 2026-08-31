package slackblocks_test

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"net/url"
	"sync/atomic"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
	slackapi "github.com/slack-go/slack"
)

func TestEveryBlockBuilderImplementsSlackBlock(t *testing.T) {
	t.Parallel()

	blocks := []slackapi.Block{
		slackblocks.NewAlertBlock(),
		slackblocks.NewCardBlock(),
		slackblocks.NewCarouselBlock(),
		slackblocks.NewContainerBlock(),
		slackblocks.NewContextActionsBlock(),
		slackblocks.NewDataTableBlock(),
		slackblocks.NewDataVisualizationBlock(),
		slackblocks.NewTaskCardBlock(),
		slackblocks.NewPlanBlock(),
		slackblocks.NewSectionBlock(),
		slackblocks.NewDividerBlock(),
		slackblocks.NewActionsBlock(),
		slackblocks.NewContextBlock(),
		slackblocks.NewFileBlock(),
		slackblocks.NewHeaderBlock(),
		slackblocks.NewImageBlock(),
		slackblocks.NewInputBlock(),
		slackblocks.NewMarkdownBlock(),
		slackblocks.NewRichTextBlock(),
		slackblocks.NewTableBlock(),
		slackblocks.NewVideoBlock(),
	}
	for index, block := range blocks {
		if block.BlockType() == "" {
			t.Errorf("block %d has an empty BlockType", index)
		}
	}
}

func TestOnlyBlockBuildersImplementSlackBlock(t *testing.T) {
	t.Parallel()

	for name, value := range map[string]any{
		"button":     slackblocks.NewButton(),
		"message":    slackblocks.NewMessage(),
		"plain text": slackblocks.NewPlainText(),
	} {
		if _, ok := value.(slackapi.Block); ok {
			t.Errorf("%s builder unexpectedly implements slack.Block", name)
		}
	}
}

func TestBlockBuilderSendsDirectlyThroughSlackGo(t *testing.T) {
	t.Parallel()

	section := slackblocks.NewSectionBlock().
		Text("Hello, world!").
		BlockID("greeting")
	table := slackblocks.NewTableBlock().Rows(
		[]any{slackblocks.NewRawText().Text("Name")},
		[]any{slackblocks.NewRawText().Text("Ada")},
	)

	var received url.Values
	server := httptest.NewServer(http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		if request.URL.Path != "/chat.postMessage" {
			t.Errorf("path = %q, want /chat.postMessage", request.URL.Path)
		}
		if err := request.ParseForm(); err != nil {
			t.Errorf("parse form: %v", err)
		}
		received = request.Form
		writer.Header().Set("Content-Type", "application/json")
		_, _ = writer.Write([]byte(`{"ok":true,"channel":"C0123456","ts":"123.456"}`))
	}))
	defer server.Close()

	client := slackapi.New("xoxb-test", slackapi.OptionAPIURL(server.URL+"/"))
	responseChannel, timestamp, err := client.PostMessageContext(
		context.Background(),
		"C0123456",
		slackapi.MsgOptionText("Hello from slackblocks!", false),
		slackapi.MsgOptionBlocks(section, table),
	)
	if err != nil {
		t.Fatal(err)
	}
	if responseChannel != "C0123456" || timestamp != "123.456" {
		t.Fatalf("response = (%q, %q)", responseChannel, timestamp)
	}

	var blocks []slackblocks.Object
	if err := json.Unmarshal([]byte(received.Get("blocks")), &blocks); err != nil {
		t.Fatal(err)
	}
	if len(blocks) != 2 || blocks[0]["block_id"] != "greeting" {
		t.Fatalf("blocks were not preserved: %#v", blocks)
	}
	if blocks[1]["type"] != "table" || blocks[1]["rows"] == nil {
		t.Fatalf("newer table block was not preserved: %#v", blocks[1])
	}
	if section.BlockType() != slackapi.MessageBlockType("section") || section.ID() != "greeting" {
		t.Fatalf("section identity = (%q, %q)", section.BlockType(), section.ID())
	}
}

func TestSlackGoReturnsBlockValidationErrorsBeforeSending(t *testing.T) {
	t.Parallel()

	var requests atomic.Int32
	server := httptest.NewServer(http.HandlerFunc(func(http.ResponseWriter, *http.Request) {
		requests.Add(1)
	}))
	defer server.Close()

	client := slackapi.New("xoxb-test", slackapi.OptionAPIURL(server.URL+"/"))
	_, _, err := client.PostMessageContext(
		context.Background(),
		"C0123456",
		slackapi.MsgOptionBlocks(slackblocks.NewSectionBlock()),
	)
	if err == nil {
		t.Fatal("expected invalid section block to fail")
	}
	var validation *slackblocks.ValidationError
	if !errors.As(err, &validation) {
		t.Fatalf("error = %T %v, want *ValidationError", err, err)
	}
	if validation.Category != slackblocks.MissingRequired {
		t.Fatalf("category = %q, want %q", validation.Category, slackblocks.MissingRequired)
	}
	if requests.Load() != 0 {
		t.Fatalf("server received %d requests, want zero", requests.Load())
	}
}

func TestBlockBuildersFitNativeSlackGoContainers(t *testing.T) {
	t.Parallel()

	block := slackblocks.NewSectionBlock().Text("Hello, world!").BlockID("greeting")
	containers := map[string]any{
		"webhook": &slackapi.WebhookMessage{
			Text: "Hello from slackblocks!",
			Blocks: &slackapi.Blocks{BlockSet: []slackapi.Block{
				block,
			}},
		},
		"modal": slackapi.ModalViewRequest{
			Type:  slackapi.VTModal,
			Title: slackapi.NewTextBlockObject("plain_text", "Greeting", false, false),
			Blocks: slackapi.Blocks{BlockSet: []slackapi.Block{
				block,
			}},
		},
		"home": slackapi.HomeTabViewRequest{
			Type: slackapi.VTHomeTab,
			Blocks: slackapi.Blocks{BlockSet: []slackapi.Block{
				block,
			}},
		},
	}

	for name, container := range containers {
		name, container := name, container
		t.Run(name, func(t *testing.T) {
			t.Parallel()
			encoded, err := json.Marshal(container)
			if err != nil {
				t.Fatal(err)
			}
			var payload struct {
				Blocks []slackblocks.Object `json:"blocks"`
			}
			if err := json.Unmarshal(encoded, &payload); err != nil {
				t.Fatal(err)
			}
			if len(payload.Blocks) != 1 || payload.Blocks[0]["type"] != "section" || payload.Blocks[0]["block_id"] != "greeting" {
				t.Fatalf("native %s container lost its slackblocks block: %s", name, encoded)
			}
		})
	}
}
