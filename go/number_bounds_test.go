package slackblocks_test

import (
	"fmt"
	slackblocks "github.com/nicklambourne/slackblocks/go/v2"
	"math"
	"testing"
)

func TestNumberBoundWireStrings(t *testing.T) {
	for _, v := range []any{0, -2, 1.25, "-1.5", "10.25"} {
		wire, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue(v).MaxValue(v).Build()
		if err != nil {
			t.Fatal(err)
		}
		if wire["min_value"] != fmt.Sprint(v) || wire["max_value"] != fmt.Sprint(v) {
			t.Fatalf("wrong wire bounds: %#v", wire)
		}
	}
	wire, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).Build()
	if err != nil {
		t.Fatal(err)
	}
	if _, present := wire["min_value"]; present {
		t.Fatal("omitted bound emitted")
	}
	for _, v := range []any{math.Inf(1), math.NaN(), "invalid", ""} {
		if _, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue(v).Build(); err == nil {
			t.Fatalf("accepted invalid bound %#v", v)
		}
	}
	if _, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue("10").MaxValue("2").Build(); err == nil {
		t.Fatal("accepted inverted string range")
	}
}

func TestExactDecimalBoundOrdering(t *testing.T) {
	for _, pair := range [][2]string{{"9007199254740993", "9007199254740992"}, {"0.10000000000000001", "0.1"}, {"-9007199254740992", "-9007199254740993"}, {"-0.1", "-0.10000000000000001"}, {"1e-1000", "9e-1001"}} {
		if _, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue(pair[0]).MaxValue(pair[1]).Build(); err == nil {
			t.Fatalf("accepted inverted range %v", pair)
		}
		if _, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue(pair[1]).MaxValue(pair[0]).Build(); err != nil {
			t.Fatalf("rejected ordered range %v: %v", pair, err)
		}
	}
	if _, err := slackblocks.NewNumberInput().IsDecimalAllowed(true).MinValue("0.1000").MaxValue("1e-1").Build(); err != nil {
		t.Fatal(err)
	}
}
