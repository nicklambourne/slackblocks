package slackblocks

import (
	"math/big"
	"regexp"
	"strings"
)

var decimalBoundPattern = regexp.MustCompile(`^[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?$`)

func compareDecimal(left, right string) int {
	type decimal struct {
		sign   int
		digits string
		rank   *big.Int
	}
	parts := func(value string) decimal {
		split := strings.SplitN(strings.ToLower(value), "e", 2)
		exponent := new(big.Int)
		if len(split) == 2 {
			exponent.SetString(split[1], 10)
		}
		negative := strings.HasPrefix(split[0], "-")
		mantissa := strings.TrimLeft(split[0], "+-")
		fraction := 0
		if dot := strings.IndexByte(mantissa, '.'); dot >= 0 {
			fraction = len(mantissa) - dot - 1
		}
		digits := strings.TrimLeft(strings.ReplaceAll(mantissa, ".", ""), "0")
		sign := 1
		if negative {
			sign = -1
		}
		if digits == "" {
			sign = 0
		}
		exponent.Add(exponent, big.NewInt(int64(len(digits)-fraction)))
		return decimal{sign, digits, exponent}
	}
	a, b := parts(left), parts(right)
	if a.sign != b.sign {
		return a.sign - b.sign
	}
	if a.sign == 0 {
		return 0
	}
	if order := a.rank.Cmp(b.rank); order != 0 {
		return order * a.sign
	}
	width := max(len(a.digits), len(b.digits))
	return strings.Compare(a.digits+strings.Repeat("0", width-len(a.digits)), b.digits+strings.Repeat("0", width-len(b.digits))) * a.sign
}
