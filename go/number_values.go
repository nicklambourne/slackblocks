package slackblocks

import (
	"encoding/json"
	"regexp"
)

var jsonNumberPattern = regexp.MustCompile(`^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?$`)

// ValueNumber sets a raw-number cell from an exact JSON number, without a
// float64 conversion. Build rejects invalid JSON number syntax. Use Value for
// ordinary float64 values and Text to set the cell's separate display label.
func (b *RawNumberBuilder) ValueNumber(value json.Number) *RawNumberBuilder {
	b.core.set("value", value)
	return b
}
