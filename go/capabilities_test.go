package slackblocks_test

import (
	"encoding/json"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"
)

var capabilityByConstructor = map[string]string{
	"NewActionsBlock": "blocks.actions", "NewAlertBlock": "blocks.alert", "NewAreaChart": "blocks.data_visualization",
	"NewAttachment": "messages.attachment", "NewAxisConfig": "objects.axis_config", "NewBarChart": "blocks.data_visualization",
	"NewButton": "elements.button", "NewCardBlock": "blocks.card", "NewCarouselBlock": "blocks.carousel",
	"NewChannelMultiSelect": "elements.multi_select_channels", "NewChannelSelect": "elements.select_channels",
	"NewChartSegment": "objects.chart_segment", "NewCheckboxes": "elements.checkboxes",
	"NewColumnSettings": "objects.column_settings", "NewConfirmation": "objects.confirmation",
	"NewContainerBlock": "blocks.container", "NewContextActionsBlock": "blocks.context_actions",
	"NewContextBlock": "blocks.context", "NewConversationFilter": "objects.conversation_filter",
	"NewConversationMultiSelect": "elements.multi_select_conversations", "NewConversationSelect": "elements.select_conversations",
	"NewDataPoint": "objects.data_point", "NewDataSeries": "objects.data_series", "NewDataTableBlock": "blocks.data_table",
	"NewDataVisualizationBlock": "blocks.data_visualization", "NewDatePicker": "elements.date_picker",
	"NewDateTimePicker": "elements.datetime_picker", "NewDispatchActionConfiguration": "objects.dispatch_action_configuration",
	"NewDividerBlock": "blocks.divider", "NewEmailInput": "elements.email_input",
	"NewExternalMultiSelect": "elements.multi_select_external", "NewExternalSelect": "elements.select_external",
	"NewFeedbackButton": "objects.feedback_button", "NewFeedbackButtons": "elements.feedback_buttons",
	"NewFileBlock": "blocks.file", "NewFileInput": "elements.file_input", "NewHeaderBlock": "blocks.header",
	"NewHomeTab": "views.home_tab", "NewIconButton": "elements.icon_button", "NewImageBlock": "blocks.image",
	"NewImageElement": "elements.image", "NewInputBlock": "blocks.input", "NewInputParameter": "objects.input_parameter",
	"NewLineChart": "blocks.data_visualization", "NewMarkdown": "objects.markdown_text", "NewMarkdownBlock": "blocks.markdown",
	"NewMessage": "messages.message", "NewMessageResponse": "messages.message_response", "NewModal": "views.modal",
	"NewNumberInput": "elements.number_input", "NewOption": "objects.option", "NewOptionGroup": "objects.option_group",
	"NewOverflow": "elements.overflow", "NewPieChart": "blocks.data_visualization", "NewPlainText": "objects.plain_text",
	"NewPlainTextInput": "elements.plain_text_input", "NewPlanBlock": "blocks.plan", "NewRadioButtons": "elements.radio_buttons",
	"NewRawNumber": "objects.raw_number", "NewRawText": "objects.raw_text", "NewRichText": "rich_text.text",
	"NewRichTextBlock": "blocks.rich_text", "NewRichTextChannel": "rich_text.channel",
	"NewRichTextCodeBlock": "rich_text.code_block", "NewRichTextEmoji": "rich_text.emoji",
	"NewRichTextInput": "elements.rich_text_input", "NewRichTextLink": "rich_text.link",
	"NewRichTextList": "rich_text.list", "NewRichTextQuote": "rich_text.quote",
	"NewRichTextSection": "rich_text.section", "NewRichTextUser": "rich_text.user",
	"NewRichTextUserGroup": "rich_text.user_group", "NewSectionBlock": "blocks.section",
	"NewSlackFile": "objects.slack_file", "NewSlackIcon": "objects.slack_icon",
	"NewStaticMultiSelect": "elements.multi_select_static", "NewStaticSelect": "elements.select_static",
	"NewTableBlock": "blocks.table", "NewTaskCardBlock": "blocks.task_card", "NewTimePicker": "elements.time_picker",
	"NewTrigger": "objects.trigger", "NewURLInput": "elements.url_input", "NewURLSource": "elements.url_source",
	"NewUserMultiSelect": "elements.multi_select_users", "NewUserSelect": "elements.select_users",
	"NewVideoBlock": "blocks.video", "NewWebhookMessage": "messages.webhook_message",
	"NewWorkflow": "objects.workflow", "NewWorkflowButton": "elements.workflow_button",
}

func TestEveryConstructorMapsToSharedCapability(t *testing.T) {
	constructors := exportedConstructors(t)
	mapped := make([]string, 0, len(capabilityByConstructor))
	for constructor := range capabilityByConstructor {
		mapped = append(mapped, constructor)
	}
	sort.Strings(mapped)
	if !reflect.DeepEqual(constructors, mapped) {
		t.Fatalf("constructor registry mismatch\nexported: %v\nmapped:   %v", constructors, mapped)
	}
}

func TestEverySharedCapabilityHasGoImplementation(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "spec", "coverage.json"))
	if err != nil {
		t.Fatal(err)
	}
	var coverage struct {
		Capabilities map[string]json.RawMessage `json:"capabilities"`
	}
	if err := json.Unmarshal(data, &coverage); err != nil {
		t.Fatal(err)
	}
	implemented := map[string]bool{}
	for _, capability := range capabilityByConstructor {
		implemented[capability] = true
	}
	if len(implemented) != len(coverage.Capabilities) {
		t.Fatalf("implemented %d capabilities; shared registry contains %d", len(implemented), len(coverage.Capabilities))
	}
	for capability := range coverage.Capabilities {
		if !implemented[capability] {
			t.Errorf("missing Go capability %s", capability)
		}
	}
}

func exportedConstructors(t *testing.T) []string {
	t.Helper()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatal(err)
	}
	set := token.NewFileSet()
	names := []string{}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".go") || strings.HasSuffix(entry.Name(), "_test.go") {
			continue
		}
		file, err := parser.ParseFile(set, entry.Name(), nil, 0)
		if err != nil {
			t.Fatal(err)
		}
		for _, declaration := range file.Decls {
			function, ok := declaration.(*ast.FuncDecl)
			if ok && function.Recv == nil && strings.HasPrefix(function.Name.Name, "New") {
				names = append(names, function.Name.Name)
			}
		}
	}
	sort.Strings(names)
	return names
}
