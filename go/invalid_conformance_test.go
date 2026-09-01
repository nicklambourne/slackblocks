package slackblocks_test

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

type invalidManifest struct {
	SpecVersion string `json:"spec_version"`
	Cases       []struct {
		ID       string                    `json:"id"`
		Category slackblocks.ErrorCategory `json:"category"`
	} `json:"cases"`
}

func TestSharedInvalidFixtures(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "spec", "fixtures", "invalid", "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest invalidManifest
	if err := json.Unmarshal(data, &manifest); err != nil {
		t.Fatal(err)
	}
	if manifest.SpecVersion != slackblocks.SpecVersion {
		t.Fatalf("spec version = %q, want %q", slackblocks.SpecVersion, manifest.SpecVersion)
	}
	if len(manifest.Cases) != 115 {
		t.Fatalf("expected 115 invalid cases, got %d", len(manifest.Cases))
	}

	for _, testCase := range manifest.Cases {
		testCase := testCase
		t.Run(testCase.ID, func(t *testing.T) {
			err := invalidConstruction(testCase.ID)
			if err == nil {
				t.Fatal("construction did not fail")
			}
			var validation *slackblocks.ValidationError
			if !errors.As(err, &validation) {
				t.Fatalf("expected ValidationError, got %T: %v", err, err)
			}
			if validation.Category != testCase.Category {
				t.Fatalf("expected %s, got %s: %v", testCase.Category, validation.Category, err)
			}
		})
	}
}

func buildError(builder slackblocks.Buildable) error { _, err := builder.Build(); return err }
func repeated(value string, count int) string        { return strings.Repeat(value, count) }
func choice() *slackblocks.OptionBuilder             { return slackblocks.NewOption().Text("A").Value("a") }
func feedbackChoice() *slackblocks.FeedbackButtonBuilder {
	return slackblocks.NewFeedbackButton().Text("Good").Value("good")
}
func validCard() *slackblocks.CardBlockBuilder         { return slackblocks.NewCardBlock().Title("Card") }
func rawText(value string) *slackblocks.RawTextBuilder { return slackblocks.NewRawText().Text(value) }
func validRows() []any                                 { return []any{[]any{rawText("Name")}, []any{rawText("Alice")}} }
func validAxis() *slackblocks.AxisConfigBuilder        { return slackblocks.NewAxisConfig().Categories("A") }
func validSeries(name string) *slackblocks.DataSeriesBuilder {
	return slackblocks.NewDataSeries().Name(name).Data(slackblocks.NewDataPoint().Label("A").Value(1))
}
func validVideo() *slackblocks.VideoBlockBuilder {
	return slackblocks.NewVideoBlock().AltText("Video").ThumbnailURL("https://example.com/thumbnail.png").Title("Title").VideoURL("https://example.com/video.mp4")
}
func copies(count int, factory func(int) any) []any {
	values := make([]any, count)
	for index := range values {
		values[index] = factory(index)
	}
	return values
}

func invalidConstruction(id string) error {
	switch id {
	case "text-empty":
		return buildError(slackblocks.NewPlainText().Text(""))
	case "text-too-long":
		return buildError(slackblocks.NewPlainText().Text(repeated("x", 3001)))
	case "button-action-id-too-long":
		return buildError(slackblocks.NewButton().Text("A").ActionID(repeated("x", 256)))
	case "button-text-too-long":
		return buildError(slackblocks.NewButton().Text(repeated("x", 76)).ActionID("a"))
	case "button-url-too-long":
		return buildError(slackblocks.NewButton().Text("A").ActionID("a").URL(repeated("x", 3001)))
	case "button-value-too-long":
		return buildError(slackblocks.NewButton().Text("A").ActionID("a").Value(repeated("x", 2001)))
	case "confirmation-title-too-long":
		return buildError(slackblocks.NewConfirmation().Title(repeated("x", 101)).Text("Text").Confirm("Yes").Deny("No"))
	case "confirmation-text-too-long":
		return buildError(slackblocks.NewConfirmation().Title("Title").Text(repeated("x", 301)).Confirm("Yes").Deny("No"))
	case "confirmation-confirm-too-long":
		return buildError(slackblocks.NewConfirmation().Title("Title").Text("Text").Confirm(repeated("x", 31)).Deny("No"))
	case "confirmation-deny-too-long":
		return buildError(slackblocks.NewConfirmation().Title("Title").Text("Text").Confirm("Yes").Deny(repeated("x", 31)))
	case "option-text-too-long":
		return buildError(slackblocks.NewOption().Text(repeated("x", 76)).Value("a"))
	case "option-value-too-long":
		return buildError(slackblocks.NewOption().Text("A").Value(repeated("x", 151)))
	case "option-url-too-long":
		return buildError(slackblocks.NewOption().Text("A").Value("a").URL(repeated("🙂", 3001)))
	case "option-description-too-long":
		return buildError(slackblocks.NewOption().Text("A").Value("a").Description(repeated("x", 76)))
	case "option-group-label-too-long":
		return buildError(slackblocks.NewOptionGroup().Label(repeated("x", 76)).Options(choice()))
	case "option-group-empty":
		return buildError(slackblocks.NewOptionGroup().Label("Group").Set("options", []any{}))
	case "option-group-too-many-options":
		return buildError(slackblocks.NewOptionGroup().Label("Group").Options(copies(101, func(int) any { return choice() })...))
	case "select-placeholder-too-long":
		return buildError(slackblocks.NewStaticSelect().ActionID("a").Options(choice()).Placeholder(repeated("x", 151)))
	case "select-too-many-options":
		return buildError(slackblocks.NewStaticSelect().ActionID("a").Options(copies(101, func(int) any { return choice() })...))
	case "select-too-many-option-groups":
		return buildError(slackblocks.NewStaticSelect().ActionID("a").OptionGroups(copies(101, func(int) any { return slackblocks.NewOptionGroup().Label("Group").Options(choice()) })...))
	case "overflow-empty":
		return buildError(slackblocks.NewOverflow().ActionID("a").Set("options", []any{}))
	case "overflow-too-many-options":
		return buildError(slackblocks.NewOverflow().ActionID("a").Options(copies(6, func(int) any { return choice() })...))
	case "checkboxes-empty":
		return buildError(slackblocks.NewCheckboxes().ActionID("a").Set("options", []any{}))
	case "checkboxes-too-many-options":
		return buildError(slackblocks.NewCheckboxes().ActionID("a").Options(copies(11, func(int) any { return choice() })...))
	case "radio-buttons-empty":
		return buildError(slackblocks.NewRadioButtons().ActionID("a").Set("options", []any{}))
	case "radio-buttons-too-many-options":
		return buildError(slackblocks.NewRadioButtons().ActionID("a").Options(copies(11, func(int) any { return choice() })...))
	case "url-source-url-empty":
		return buildError(slackblocks.NewURLSource().URL("").Text("text"))
	case "url-source-url-too-long":
		return buildError(slackblocks.NewURLSource().URL(repeated("x", 3001)).Text("text"))
	case "table-too-many-rows":
		return buildError(slackblocks.NewTableBlock().Rows(copies(101, func(int) any { return []any{rawText("A")} })...))
	case "table-ragged-rows":
		return buildError(slackblocks.NewTableBlock().Rows([]any{rawText("A"), rawText("B")}, []any{rawText("C")}))
	case "table-column-settings-mismatch":
		return buildError(slackblocks.NewTableBlock().Rows([]any{rawText("A"), rawText("B")}).ColumnSettings(slackblocks.NewColumnSettings().IsWrapped(true)))
	case "file-input-max-files-too-small":
		return buildError(slackblocks.NewFileInput().ActionID("a").MaxFiles(0))
	case "file-input-max-files-too-large":
		return buildError(slackblocks.NewFileInput().ActionID("a").MaxFiles(11))
	case "plain-text-input-max-length-too-large":
		return buildError(slackblocks.NewPlainTextInput().ActionID("a").MaxLength(3001))
	case "actions-too-many-elements":
		return buildError(slackblocks.NewActionsBlock().Elements(copies(26, func(int) any { return slackblocks.NewButton().Text("A").ActionID("a") })...))
	case "context-too-many-elements":
		return buildError(slackblocks.NewContextBlock().Elements(copies(11, func(int) any { return slackblocks.NewMarkdown().Text("A") })...))
	case "header-text-too-long":
		return buildError(slackblocks.NewHeaderBlock().Text(repeated("x", 151)))
	case "image-url-too-long":
		return buildError(slackblocks.NewImageBlock().ImageURL(repeated("x", 3001)).AltText("Alt"))
	case "image-alt-text-too-long":
		return buildError(slackblocks.NewImageBlock().ImageURL("https://example.com/image.png").AltText(repeated("x", 2001)))
	case "input-label-too-long":
		return buildError(slackblocks.NewInputBlock().Label(repeated("x", 2001)).Element(slackblocks.NewPlainTextInput().ActionID("a")))
	case "input-hint-too-long":
		return buildError(slackblocks.NewInputBlock().Label("Label").Hint(repeated("x", 2001)).Element(slackblocks.NewPlainTextInput().ActionID("a")))
	case "markdown-empty":
		return buildError(slackblocks.NewMarkdownBlock().Text(""))
	case "markdown-too-long":
		return buildError(slackblocks.NewMarkdownBlock().Text(repeated("x", 12001)))
	case "section-text-too-long":
		return buildError(slackblocks.NewSectionBlock().Text(repeated("x", 3001)))
	case "section-too-many-fields":
		return buildError(slackblocks.NewSectionBlock().Fields(copies(11, func(int) any { return "x" })...))
	case "section-field-too-long":
		return buildError(slackblocks.NewSectionBlock().Fields(repeated("x", 2001)))
	case "video-alt-text-empty":
		return buildError(validVideo().AltText(""))
	case "video-alt-text-too-long":
		return buildError(validVideo().AltText(repeated("x", 201)))
	case "video-title-too-long":
		return buildError(validVideo().Title(repeated("x", 201)))
	case "video-author-name-too-long":
		return buildError(validVideo().AuthorName(repeated("x", 51)))
	case "video-description-too-long":
		return buildError(validVideo().Description(repeated("x", 201)))
	case "video-provider-name-too-long":
		return buildError(validVideo().ProviderName(repeated("x", 51)))
	case "message-channel-empty":
		return buildError(slackblocks.NewMessage().Channel(""))
	case "message-too-many-blocks":
		return buildError(slackblocks.NewMessage().Channel("C123").Blocks(copies(51, func(int) any { return slackblocks.NewDividerBlock() })...))
	case "message-too-many-attachments":
		return buildError(slackblocks.NewMessage().Channel("C123").Attachments(copies(101, func(int) any { return slackblocks.NewAttachment().Blocks(slackblocks.NewDividerBlock()) })...))
	case "message-invalid-block-surface":
		return buildError(slackblocks.NewMessage().Channel("C123").Blocks(slackblocks.NewAlertBlock().Text("Modal only")))
	case "modal-invalid-block-surface":
		return buildError(slackblocks.NewModal().Title("Invalid").Blocks(slackblocks.NewMarkdownBlock().Text("Message only")))
	case "home-invalid-block-surface":
		return buildError(slackblocks.NewHomeTab().Blocks(slackblocks.NewAlertBlock().Text("Modal only")))
	case "modal-input-requires-submit":
		return buildError(slackblocks.NewModal().Title("Missing submit").Blocks(slackblocks.NewInputBlock().Label("Name").Element(slackblocks.NewPlainTextInput().ActionID("name"))))
	case "view-missing-blocks":
		return buildError(slackblocks.NewHomeTab().Set("blocks", []any{}))
	case "view-too-many-blocks":
		return buildError(slackblocks.NewHomeTab().Blocks(copies(101, func(int) any { return slackblocks.NewDividerBlock() })...))
	case "view-private-metadata-too-long":
		return buildError(slackblocks.NewHomeTab().Blocks(slackblocks.NewDividerBlock()).PrivateMetadata(repeated("x", 3001)))
	case "view-callback-id-too-long":
		return buildError(slackblocks.NewHomeTab().Blocks(slackblocks.NewDividerBlock()).Set("callback_id", repeated("x", 256)))
	case "view-title-too-long":
		return buildError(slackblocks.NewModal().Title(repeated("x", 25)).Blocks(slackblocks.NewDividerBlock()))
	case "view-close-too-long":
		return buildError(slackblocks.NewModal().Title("Title").Close(repeated("x", 25)).Blocks(slackblocks.NewDividerBlock()))
	case "view-submit-too-long":
		return buildError(slackblocks.NewModal().Title("Title").Submit(repeated("x", 25)).Blocks(slackblocks.NewDividerBlock()))
	case "section-missing-content":
		return buildError(slackblocks.NewSectionBlock())
	case "section-empty-fields":
		return buildError(slackblocks.NewSectionBlock().Fields())
	case "static-select-options-and-groups":
		return buildError(slackblocks.NewStaticSelect().ActionID("a").Options(choice()).Set("option_groups", []any{slackblocks.Object{"label": slackblocks.NewPlainText().Text("A")}}))
	case "image-url-and-slack-file":
		return buildError(slackblocks.NewImageElement().AltText("image").ImageURL("https://example.com/image.png").SlackFile(slackblocks.Object{"id": "F123"}))
	case "number-input-inverted-range":
		return buildError(slackblocks.NewNumberInput().ActionID("a").IsDecimalAllowed(true).MinValue(2).MaxValue(1))
	case "context-invalid-element":
		return buildError(slackblocks.NewContextBlock().Elements(slackblocks.NewDividerBlock()))
	case "input-invalid-element":
		return buildError(slackblocks.NewInputBlock().Label("Label").Element(slackblocks.NewButton().Text("A").ActionID("a")))
	case "block-id-too-long":
		return buildError(slackblocks.NewDividerBlock().BlockID(repeated("x", 256)))
	case "button-accessibility-label-too-long":
		return buildError(slackblocks.NewButton().Text("A").ActionID("a").AccessibilityLabel(repeated("x", 76)))
	case "alert-text-too-long":
		return buildError(slackblocks.NewAlertBlock().Text(repeated("x", 201)))
	case "card-title-too-long":
		return buildError(slackblocks.NewCardBlock().Title(repeated("x", 151)))
	case "card-subtitle-too-long":
		return buildError(slackblocks.NewCardBlock().Title("Card").Subtitle(repeated("x", 151)))
	case "card-body-too-long":
		return buildError(slackblocks.NewCardBlock().Body(repeated("x", 201)))
	case "card-too-many-actions":
		return buildError(slackblocks.NewCardBlock().Actions(copies(4, func(index int) any { return slackblocks.NewButton().Text("A").ActionID("a") })...))
	case "card-subtext-too-long":
		return buildError(slackblocks.NewCardBlock().Title("Card").Subtext(repeated("x", 201)))
	case "carousel-empty":
		return buildError(slackblocks.NewCarouselBlock().Set("elements", []any{}))
	case "carousel-too-many-cards":
		return buildError(slackblocks.NewCarouselBlock().Elements(copies(11, func(int) any { return validCard() })...))
	case "container-title-too-long":
		return buildError(slackblocks.NewContainerBlock().Title(repeated("x", 151)).ChildBlocks(slackblocks.NewDividerBlock()))
	case "container-subtitle-too-long":
		return buildError(slackblocks.NewContainerBlock().Title("Container").Subtitle(repeated("x", 151)).ChildBlocks(slackblocks.NewDividerBlock()))
	case "container-too-many-child-blocks":
		return buildError(slackblocks.NewContainerBlock().Title("Container").ChildBlocks(copies(11, func(int) any { return slackblocks.NewDividerBlock() })...))
	case "context-actions-too-many-elements":
		return buildError(slackblocks.NewContextActionsBlock().Elements(copies(6, func(int) any { return slackblocks.NewIconButton().Text("Delete") })...))
	case "feedback-button-text-too-long":
		return buildError(slackblocks.NewFeedbackButtons().PositiveButton(slackblocks.NewFeedbackButton().Text(repeated("x", 76)).Value("good")).NegativeButton(feedbackChoice()))
	case "feedback-button-value-too-long":
		return buildError(slackblocks.NewFeedbackButtons().PositiveButton(slackblocks.NewFeedbackButton().Text("Good").Value(repeated("x", 2001))).NegativeButton(feedbackChoice()))
	case "feedback-button-accessibility-label-too-long":
		return buildError(slackblocks.NewFeedbackButtons().PositiveButton(slackblocks.NewFeedbackButton().Text("Good").Value("good").AccessibilityLabel(repeated("x", 76))).NegativeButton(feedbackChoice()))
	case "icon-button-too-many-visible-users":
		return buildError(slackblocks.NewIconButton().Text("Delete").VisibleToUserIDs(copiesStrings(11)...))
	case "data-table-too-few-rows":
		return buildError(slackblocks.NewDataTableBlock().Rows([]any{rawText("Name")}).Caption("Names"))
	case "data-table-too-many-rows":
		return buildError(slackblocks.NewDataTableBlock().Rows(copies(202, func(int) any { return []any{rawText("A")} })...).Caption("Names"))
	case "data-table-too-few-columns":
		return buildError(slackblocks.NewDataTableBlock().Rows([]any{}, []any{}).Caption("Empty"))
	case "data-table-too-many-columns":
		return buildError(slackblocks.NewDataTableBlock().Rows(copies(2, func(int) any { return copies(21, func(int) any { return rawText("A") }) })...).Caption("Wide"))
	case "data-table-page-size-too-small":
		return buildError(slackblocks.NewDataTableBlock().Rows(validRows()...).Caption("Names").PageSize(0))
	case "data-table-page-size-too-large":
		return buildError(slackblocks.NewDataTableBlock().Rows(validRows()...).Caption("Names").PageSize(101))
	case "data-table-cell-text-empty":
		return buildError(slackblocks.NewDataTableBlock().Rows([]any{rawText("Name")}, []any{rawText("")}).Caption("Names"))
	case "data-table-content-too-long":
		return buildError(slackblocks.NewDataTableBlock().Rows([]any{rawText("Name")}, []any{rawText(repeated("x", 20000))}).Caption("Names"))
	case "data-visualization-title-too-long":
		return buildError(slackblocks.NewDataVisualizationBlock().Title(repeated("x", 51)).Chart(slackblocks.NewPieChart().Segments(slackblocks.NewChartSegment().Label("A").Value(1))))
	case "pie-chart-empty":
		return buildError(slackblocks.NewPieChart().Set("segments", []any{}))
	case "pie-chart-too-many-segments":
		return buildError(slackblocks.NewPieChart().Segments(copies(13, func(index int) any { return slackblocks.NewChartSegment().Label("S").Value(1) })...))
	case "chart-segment-label-too-long":
		return buildError(slackblocks.NewChartSegment().Label(repeated("x", 21)).Value(1))
	case "chart-segment-value-not-positive":
		return buildError(slackblocks.NewChartSegment().Label("A").Value(0))
	case "chart-series-empty":
		return buildError(slackblocks.NewLineChart().AxisConfig(validAxis()).Set("series", []any{}))
	case "chart-duplicate-point-labels":
		return buildError(slackblocks.NewLineChart().
			AxisConfig(slackblocks.NewAxisConfig().Categories("A", "B")).
			Series(slackblocks.NewDataSeries().Name("Series").Data(
				slackblocks.NewDataPoint().Label("A").Value(1),
				slackblocks.NewDataPoint().Label("A").Value(2),
			)))
	case "chart-too-many-series":
		return buildError(slackblocks.NewLineChart().Series(copies(13, func(index int) any { return validSeries("S") })...).AxisConfig(validAxis()))
	case "data-series-name-too-long":
		return buildError(slackblocks.NewDataSeries().Name(repeated("x", 21)).Data(slackblocks.NewDataPoint().Label("A").Value(1)))
	case "data-series-empty":
		return buildError(slackblocks.NewDataSeries().Name("Series").Set("data", []any{}))
	case "data-series-too-many-points":
		return buildError(slackblocks.NewDataSeries().Name("Series").Data(copies(21, func(index int) any { return slackblocks.NewDataPoint().Label("P").Value(index) })...))
	case "data-point-label-too-long":
		return buildError(slackblocks.NewDataPoint().Label(repeated("x", 21)).Value(1))
	case "axis-categories-empty":
		return buildError(slackblocks.NewAxisConfig().Set("categories", []any{}))
	case "axis-too-many-categories":
		return buildError(slackblocks.NewAxisConfig().Categories(copiesStrings(21)...))
	case "axis-category-label-too-long":
		return buildError(slackblocks.NewAxisConfig().Categories(repeated("x", 21)))
	case "axis-label-too-long":
		return buildError(slackblocks.NewAxisConfig().Categories("A").XLabel(repeated("x", 51)))
	default:
		return nil
	}
}

func copiesStrings(count int) []string {
	values := make([]string, count)
	for index := range values {
		values[index] = "X"
	}
	return values
}
