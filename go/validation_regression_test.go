package slackblocks_test

import (
	"errors"
	"strings"
	"testing"

	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
)

func TestSectionWithEmptyFieldsIsMissingContent(t *testing.T) {
	_, err := slackblocks.NewSectionBlock().Fields().Build()
	assertValidationError(t, err, slackblocks.MissingRequired, "")
}

func TestChartRejectsDuplicatePointsForOneCategory(t *testing.T) {
	axis := slackblocks.NewAxisConfig().Categories("A", "B")
	series := slackblocks.NewDataSeries().Name("Series").Data(
		slackblocks.NewDataPoint().Label("A").Value(1),
		slackblocks.NewDataPoint().Label("A").Value(2),
	)
	_, err := slackblocks.NewLineChart().AxisConfig(axis).Series(series).Build()
	assertValidationError(t, err, slackblocks.InvalidUsage, "series[0].data")
}

func TestMessageWithoutChannelIsMissingRequired(t *testing.T) {
	_, err := slackblocks.NewMessage().Text("Hello").Build()
	assertValidationError(t, err, slackblocks.MissingRequired, "Message.channel")
}

func TestNestedValidationUsesDeterministicFieldOrder(t *testing.T) {
	payload := slackblocks.Object{
		"z": slackblocks.Object{"type": "plain_text", "text": ""},
		"a": slackblocks.Object{"type": "plain_text", "text": ""},
	}
	for iteration := 0; iteration < 100; iteration++ {
		err := slackblocks.Validate(payload)
		assertValidationError(t, err, slackblocks.LengthExceeded, "a.text")
	}
}

func TestDispatchActionConfigurationWithoutTriggersIsValid(t *testing.T) {
	if _, err := slackblocks.NewPlainTextInput().ActionID("a").DispatchActionConfig(slackblocks.NewDispatchActionConfiguration()).Build(); err != nil {
		t.Fatal(err)
	}
	input := slackblocks.Object{"type": "plain_text_input", "action_id": "a", "dispatch_action_config": slackblocks.Object{}}
	if err := slackblocks.Validate(input); err != nil {
		t.Fatal(err)
	}
}

func TestDispatchActionConfigurationRejectsEmptyTriggers(t *testing.T) {
	input := slackblocks.Object{"type": "plain_text_input", "action_id": "a", "dispatch_action_config": slackblocks.Object{"trigger_actions_on": []any{}}}
	err := slackblocks.Validate(input)
	assertValidationError(t, err, slackblocks.LengthExceeded, "DispatchActionConfiguration.trigger_actions_on")
}

func assertValidationError(
	t *testing.T,
	err error,
	category slackblocks.ErrorCategory,
	path string,
) {
	t.Helper()
	if err == nil {
		t.Fatal("expected validation error")
	}
	var validation *slackblocks.ValidationError
	if !errors.As(err, &validation) {
		t.Fatalf("expected ValidationError, got %T: %v", err, err)
	}
	if validation.Category != category || validation.Path != path {
		t.Fatalf("validation error = (%s, %q), want (%s, %q): %v", validation.Category, validation.Path, category, path, err)
	}
}

func TestDataTableRejectsColumnSettings(t *testing.T) {
	rows := []any{
		[]any{slackblocks.Object{"type": "raw_text", "text": "Name"}},
		[]any{slackblocks.Object{"type": "raw_text", "text": "Ada"}},
	}
	dataTable := slackblocks.Object{"type": "data_table", "caption": "People", "rows": rows}
	if err := slackblocks.Validate(dataTable); err != nil {
		t.Fatal(err)
	}
	dataTable["column_settings"] = []any{slackblocks.Object{"align": "left"}}
	assertValidationError(t, slackblocks.Validate(dataTable), slackblocks.InvalidUsage, "column_settings")

	table := slackblocks.Object{"type": "table", "rows": rows, "column_settings": []any{slackblocks.Object{"align": "left"}}}
	if err := slackblocks.Validate(table); err != nil {
		t.Fatal(err)
	}
}

func TestVideoAltTextAcceptsEmptyAndMaximumLength(t *testing.T) {
	for _, altText := range []string{"", strings.Repeat("x", slackblocks.LimitVideoAltTextMaxLength)} {
		if _, err := validVideo().AltText(altText).Build(); err != nil {
			t.Fatalf("alt text of %d characters: %v", len(altText), err)
		}
	}
}
