package slackblocks_test

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

type fixtureManifest struct {
	SpecVersion string `json:"spec_version"`
	Fixtures    []struct {
		ID string `json:"id"`
	} `json:"fixtures"`
}

func TestSharedValidFixtures(t *testing.T) {
	root := filepath.Join("..", "spec")
	manifestData, err := os.ReadFile(filepath.Join(root, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest fixtureManifest
	if err := json.Unmarshal(manifestData, &manifest); err != nil {
		t.Fatal(err)
	}
	if manifest.SpecVersion != slackblocks.SpecVersion {
		t.Fatalf("spec version = %q, want %q", slackblocks.SpecVersion, manifest.SpecVersion)
	}
	if len(manifest.Fixtures) != 100 {
		t.Fatalf("expected the complete 100-fixture corpus, got %d", len(manifest.Fixtures))
	}
	assertEmptySkipList(t)

	for _, entry := range manifest.Fixtures {
		entry := entry
		t.Run(entry.ID, func(t *testing.T) {
			data, err := os.ReadFile(filepath.Join(root, "fixtures", "valid", entry.ID+".json"))
			if err != nil {
				t.Fatal(err)
			}
			var expected any
			if err := json.Unmarshal(data, &expected); err != nil {
				t.Fatal(err)
			}
			constructed, err := constructFixture(expected, entry.ID)
			if err != nil {
				t.Fatal(err)
			}
			actual, err := materialiseFixture(constructed)
			if err != nil {
				t.Fatal(err)
			}
			if !reflect.DeepEqual(actual, expected) {
				want, _ := json.MarshalIndent(expected, "", "  ")
				got, _ := json.MarshalIndent(actual, "", "  ")
				t.Fatalf("fixture mismatch\nwant:\n%s\ngot:\n%s", want, got)
			}
		})
	}
}

func assertEmptySkipList(t *testing.T) {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("conformance", "skiplist.txt"))
	if err != nil {
		t.Fatal(err)
	}
	for _, line := range strings.Split(string(data), "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			t.Fatalf("Go conformance skip list must be empty, found %q", line)
		}
	}
}

func constructFixture(value any, fixtureID string) (any, error) {
	switch typed := value.(type) {
	case []any:
		items := make([]any, 0, len(typed))
		for _, item := range typed {
			constructed, err := constructFixture(item, fixtureID)
			if err != nil {
				return nil, err
			}
			items = append(items, constructed)
		}
		return items, nil
	case map[string]any:
		builder := fixtureBuilder(typed, fixtureID)
		if builder == nil {
			object := slackblocks.Object{}
			for key, nested := range typed {
				constructed, err := constructFixture(nested, fixtureID)
				if err != nil {
					return nil, err
				}
				object[key] = constructed
			}
			return object, nil
		}
		for key, nested := range typed {
			if key == "type" {
				continue
			}
			constructed, err := constructFixture(nested, fixtureID)
			if err != nil {
				return nil, err
			}
			builder.Set(key, constructed)
		}
		return builder, nil
	default:
		return value, nil
	}
}

func materialiseFixture(value any) (any, error) {
	encoded, err := json.Marshal(value)
	if err != nil {
		return nil, err
	}
	var decoded any
	if err := json.Unmarshal(encoded, &decoded); err != nil {
		return nil, err
	}
	return decoded, nil
}

type fixtureBuilderValue interface {
	slackblocks.Buildable
	json.Marshaler
	Set(string, any) slackblocks.Buildable
}

func fixtureBuilder(object map[string]any, fixtureID string) fixtureBuilderValue {
	typeName, _ := object["type"].(string)
	switch typeName {
	case "actions":
		return slackblocks.NewActionsBlock()
	case "alert":
		return slackblocks.NewAlertBlock()
	case "area":
		return slackblocks.NewAreaChart()
	case "bar":
		return slackblocks.NewBarChart()
	case "button":
		return slackblocks.NewButton()
	case "card":
		return slackblocks.NewCardBlock()
	case "carousel":
		return slackblocks.NewCarouselBlock()
	case "channel":
		return slackblocks.NewRichTextChannel()
	case "channels_select":
		return slackblocks.NewChannelSelect()
	case "checkboxes":
		return slackblocks.NewCheckboxes()
	case "container":
		return slackblocks.NewContainerBlock()
	case "context":
		return slackblocks.NewContextBlock()
	case "context_actions":
		return slackblocks.NewContextActionsBlock()
	case "conversations_select":
		return slackblocks.NewConversationSelect()
	case "data_table":
		return slackblocks.NewDataTableBlock()
	case "data_visualization":
		return slackblocks.NewDataVisualizationBlock()
	case "datepicker":
		return slackblocks.NewDatePicker()
	case "datetimepicker":
		return slackblocks.NewDateTimePicker()
	case "divider":
		return slackblocks.NewDividerBlock()
	case "email_text_input":
		return slackblocks.NewEmailInput()
	case "emoji":
		return slackblocks.NewRichTextEmoji()
	case "external_select":
		return slackblocks.NewExternalSelect()
	case "feedback_buttons":
		return slackblocks.NewFeedbackButtons()
	case "file":
		return slackblocks.NewFileBlock()
	case "file_input":
		return slackblocks.NewFileInput()
	case "header":
		return slackblocks.NewHeaderBlock()
	case "home":
		return slackblocks.NewHomeTab()
	case "icon":
		return slackblocks.NewSlackIcon()
	case "icon_button":
		return slackblocks.NewIconButton()
	case "image":
		if filepath.Dir(fixtureID) == "blocks" {
			return slackblocks.NewImageBlock()
		}
		return slackblocks.NewImageElement()
	case "input":
		return slackblocks.NewInputBlock()
	case "line":
		return slackblocks.NewLineChart()
	case "link":
		return slackblocks.NewRichTextLink()
	case "markdown":
		return slackblocks.NewMarkdownBlock()
	case "modal":
		return slackblocks.NewModal()
	case "mrkdwn":
		return slackblocks.NewMarkdown()
	case "multi_channels_select":
		return slackblocks.NewChannelMultiSelect()
	case "multi_conversations_select":
		return slackblocks.NewConversationMultiSelect()
	case "multi_external_select":
		return slackblocks.NewExternalMultiSelect()
	case "multi_static_select":
		return slackblocks.NewStaticMultiSelect()
	case "multi_users_select":
		return slackblocks.NewUserMultiSelect()
	case "number_input":
		return slackblocks.NewNumberInput()
	case "overflow":
		return slackblocks.NewOverflow()
	case "pie":
		return slackblocks.NewPieChart()
	case "plain_text":
		return slackblocks.NewPlainText()
	case "plain_text_input":
		return slackblocks.NewPlainTextInput()
	case "plan":
		return slackblocks.NewPlanBlock()
	case "radio_buttons":
		return slackblocks.NewRadioButtons()
	case "raw_number":
		return slackblocks.NewRawNumber()
	case "raw_text":
		return slackblocks.NewRawText()
	case "rich_text":
		return slackblocks.NewRichTextBlock()
	case "rich_text_input":
		return slackblocks.NewRichTextInput()
	case "rich_text_list":
		return slackblocks.NewRichTextList()
	case "rich_text_preformatted":
		return slackblocks.NewRichTextCodeBlock()
	case "rich_text_quote":
		return slackblocks.NewRichTextQuote()
	case "rich_text_section":
		return slackblocks.NewRichTextSection()
	case "section":
		return slackblocks.NewSectionBlock()
	case "static_select":
		return slackblocks.NewStaticSelect()
	case "table":
		return slackblocks.NewTableBlock()
	case "task_card":
		return slackblocks.NewTaskCardBlock()
	case "text":
		return slackblocks.NewRichText()
	case "timepicker":
		return slackblocks.NewTimePicker()
	case "url":
		return slackblocks.NewURLSource()
	case "url_text_input":
		return slackblocks.NewURLInput()
	case "user":
		return slackblocks.NewRichTextUser()
	case "usergroup":
		return slackblocks.NewRichTextUserGroup()
	case "users_select":
		return slackblocks.NewUserSelect()
	case "video":
		return slackblocks.NewVideoBlock()
	case "workflow_button":
		return slackblocks.NewWorkflowButton()
	case "":
		return nil
	default:
		panic(fmt.Sprintf("no Go builder registered for Slack type %q", typeName))
	}
}
