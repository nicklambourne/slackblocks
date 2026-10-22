package slackblocks_test

import (
	sb "github.com/nicklambourne/slackblocks/go/v3"
	"testing"
)

func TestTextLengthBounds(t *testing.T) {
	for _, b := range []sb.Buildable{sb.NewPlainTextInput(), sb.NewPlainTextInput().MinLength(20), sb.NewPlainTextInput().MaxLength(10), sb.NewPlainTextInput().MinLength(0).MaxLength(1), sb.NewPlainTextInput().MinLength(10).MaxLength(10), sb.NewPlainTextInput().MinLength(3000).MaxLength(3000)} {
		if _, err := b.Build(); err != nil {
			t.Fatal(err)
		}
	}
	_, err := sb.NewPlainTextInput().MinLength(20).MaxLength(10).Build()
	assertValidationError(t, err, sb.OutOfRange, "min_length")
}

func TestValidatedBoundaryControls(t *testing.T) {
	for _, b := range []sb.Buildable{
		sb.NewExternalSelect().MinQueryLength(0), sb.NewExternalMultiSelect().MinQueryLength(0),
		sb.NewDatePicker().InitialDate("2028-02-29"), sb.NewDatePicker().InitialDate("2028-02-30"),
		sb.NewTimePicker().InitialTime("00:00"), sb.NewTimePicker().InitialTime("23:59"),
		sb.NewStaticSelect().Options(sb.NewOption().Text("A").Value("a")),
		sb.NewStaticMultiSelect().OptionGroups(sb.NewOptionGroup().Label("A").Options(sb.NewOption().Text("A").Value("a"))),
		sb.NewImageElement().ImageURL("https://example.com/image.png").AltText(""),
	} {
		if _, err := b.Build(); err != nil {
			t.Fatal(err)
		}
	}
	for _, c := range []struct {
		b        sb.Buildable
		category sb.ErrorCategory
		path     string
	}{
		{sb.NewExternalSelect().MinQueryLength(-1), sb.OutOfRange, "min_query_length"},
		{sb.NewDatePicker().InitialDate("not a date"), sb.TypeMismatch, "initial_date"},
		{sb.NewTimePicker().InitialTime("25:99"), sb.TypeMismatch, "initial_time"},
		{sb.NewStaticSelect().Options(), sb.LengthExceeded, "options"},
		{sb.NewStaticSelect().OptionGroups(), sb.LengthExceeded, "option_groups"},
	} {
		_, err := c.b.Build()
		assertValidationError(t, err, c.category, c.path)
	}
}
