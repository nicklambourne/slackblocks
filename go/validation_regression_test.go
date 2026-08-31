package slackblocks_test

import (
	"errors"
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
