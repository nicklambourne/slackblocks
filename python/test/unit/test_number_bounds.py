import json

import pytest

from slackblocks import NumberInput
from slackblocks.errors import InvalidUsageError


@pytest.mark.parametrize("value", [0, -2, 1.25, "-1.5", "10.25"])
def test_number_bounds_are_wire_strings(value):
    payload = json.loads(
        repr(NumberInput(is_decimal_allowed=True, min_value=value, max_value=value))
    )
    assert payload["min_value"] == str(value)
    assert payload["max_value"] == str(value)


def test_bounds_omission_and_invalid_values():
    assert "min_value" not in json.loads(repr(NumberInput(is_decimal_allowed=True)))
    for value in [float("inf"), float("nan"), "not a number", ""]:
        with pytest.raises(InvalidUsageError):
            NumberInput(is_decimal_allowed=True, min_value=value)
    with pytest.raises(InvalidUsageError):
        NumberInput(is_decimal_allowed=True, min_value="10", max_value="2")


@pytest.mark.parametrize(
    "minimum,maximum",
    [
        ("9007199254740993", "9007199254740992"),
        ("0.10000000000000001", "0.1"),
        ("-9007199254740992", "-9007199254740993"),
        ("-0.1", "-0.10000000000000001"),
        ("1e-1000", "9e-1001"),
    ],
)
def test_exact_ordering(minimum, maximum):
    with pytest.raises(InvalidUsageError):
        NumberInput(True, min_value=minimum, max_value=maximum)
    NumberInput(True, min_value=maximum, max_value=minimum)


@pytest.mark.parametrize("value", ["0x10", "1_000", " 1", "1 ", "NaN", "Infinity"])
def test_decimal_grammar(value):
    with pytest.raises(InvalidUsageError):
        NumberInput(True, min_value=value)


def test_large_exponent_and_equivalence():
    NumberInput(True, min_value="1e309", max_value="2e309")
    NumberInput(True, min_value="0.1000", max_value="1e-1")
