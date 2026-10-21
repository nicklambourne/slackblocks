import pytest

from slackblocks import (
    DatePicker,
    ExternalMultiSelectMenu,
    ExternalSelectMenu,
    PlainTextInput,
    TimePicker,
)
from slackblocks.errors import RangeError, TypeMismatchError


@pytest.mark.parametrize(
    "minimum,maximum", [(None, None), (20, None), (None, 10), (0, 1), (10, 10), (3000, 3000)]
)
def test_satisfiable_text_bounds(minimum, maximum):
    value = PlainTextInput(min_length=minimum, max_length=maximum)
    assert value.min_length == minimum
    assert value.max_length == maximum


def test_inverted_text_bounds():
    with pytest.raises(RangeError, match="PlainTextInput.min_length"):
        PlainTextInput(min_length=20, max_length=10)


def test_boundary_controls_and_existing_calendar_parser():
    assert ExternalSelectMenu(min_query_length=0).min_query_length == 0
    assert ExternalMultiSelectMenu(min_query_length=0).min_query_length == 0
    assert DatePicker(initial_date="2028-02-29").initial_date == "2028-02-29"
    with pytest.raises(ValueError):
        DatePicker(initial_date="2028-02-30")
    for time in ["00:00", "23:59"]:
        assert TimePicker(initial_time=time).initial_time == time


@pytest.mark.parametrize("value", [1, False, b"12:00", [], {}])
def test_picker_wrong_types_have_field_diagnostics(value):
    with pytest.raises(TypeMismatchError, match="initial_date"):
        DatePicker(initial_date=value)
    with pytest.raises(TypeMismatchError, match="initial_time"):
        TimePicker(initial_time=value)
