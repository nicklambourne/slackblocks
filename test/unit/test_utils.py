import pytest

from slackblocks.errors import InvalidUsageError
from slackblocks.utils import (
    coerce_to_list,
    is_hex,
    validate_action_id,
    validate_int,
    validate_string,
    validate_string_nonnull,
)


def test_coerce_to_list_single_item() -> None:
    assert coerce_to_list("a", class_=str) == [
        "a",
    ]


def test_coerce_to_list_list() -> None:
    assert coerce_to_list(["a", "b", "c"], class_=str) == ["a", "b", "c"]


def test_coerce_to_list_wrong_class() -> None:
    with pytest.raises(InvalidUsageError):
        assert coerce_to_list(["a", 1], class_=str)


def test_coerce_to_list_allow_none() -> None:
    assert coerce_to_list(None, class_=str, allow_none=True) is None


def test_coerce_to_list_disallow_none() -> None:
    with pytest.raises(InvalidUsageError):
        assert coerce_to_list(None, class_=str)


def test_coerce_to_list_lower_bound() -> None:
    with pytest.raises(InvalidUsageError):
        assert coerce_to_list(["a"], class_=str, min_size=2)


def test_coerce_to_list_upper_bound() -> None:
    with pytest.raises(InvalidUsageError):
        assert coerce_to_list(["a", 1], class_=str, max_size=1)


def test_is_hex_valid() -> None:
    assert is_hex("1234abcdef")


def test_is_hex_invalid() -> None:
    assert not is_hex("1234g")


def test_validate_validate_action_id_basic() -> None:
    assert validate_action_id("action_id") == "action_id"


def test_validate_validate_action_id_allow_none() -> None:
    assert validate_action_id(None, allow_none=True) is None


def test_validate_validate_action_id_disallow_none() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_action_id(None, allow_none=False)


def test_validate_validate_action_id_lower_bound() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_action_id("")


def test_validate_validate_action_id_upper_bound() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_action_id("a" * 256) == "a" * 256


def test_validate_string_basic() -> None:
    assert validate_string("abc123", field_name="field") == "abc123"


def test_validate_string_allow_none() -> None:
    assert validate_string(None, field_name="field", allow_none=True) is None


def test_validate_string_disallow_none() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_string(None, field_name="field", allow_none=False)


def test_validate_string_exceed_max_length() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_string("a" * 5, field_name="field", max_length=4)


def test_validate_int_min_value_error_message() -> None:
    with pytest.raises(InvalidUsageError, match="is less than the minimum"):
        validate_int(1, min_value=5)


def test_validate_int_max_value_error_message() -> None:
    """Regression test for #120: max-value error message must reflect the
    maximum violation, not the minimum."""
    with pytest.raises(InvalidUsageError, match="exceeds the maximum"):
        validate_int(10, max_value=5)


def test_validate_string_zero_max_length_enforced() -> None:
    """Regression test for #122: max_length=0 must be honored (only empty
    string is valid), not silently ignored as a falsy value."""
    with pytest.raises(InvalidUsageError):
        validate_string("a", field_name="field", max_length=0)
    # Empty string should pass (length 0 is not > 0).
    assert validate_string("", field_name="field", max_length=0) == ""


def test_validate_string_zero_min_length_passes() -> None:
    """Regression test for #122: min_length=0 must be honored explicitly."""
    assert validate_string("", field_name="field", min_length=0) == ""
    assert validate_string("abc", field_name="field", min_length=0) == "abc"


def test_validate_string_nonnull_positional_arg_order() -> None:
    """Regression test for #124: validate_string_nonnull must accept
    (string, field_name, max_length, min_length) in that positional order,
    matching validate_string."""
    assert validate_string_nonnull("hi", "field", 10, 1) == "hi"
    with pytest.raises(InvalidUsageError, match="field"):
        validate_string_nonnull("toolong", "field", 3)


def test_validate_string_nonnull_keyword_args() -> None:
    """validate_string_nonnull continues to accept keyword arguments."""
    assert (
        validate_string_nonnull("ok", field_name="x", max_length=10, min_length=1)
        == "ok"
    )
