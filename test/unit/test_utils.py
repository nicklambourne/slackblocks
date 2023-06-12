import pytest

from slackblocks.errors import InvalidUsageError
from slackblocks.utils import (
    coerce_to_list,
    is_hex,
    validate_action_id,
    validate_string,
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
    assert coerce_to_list(None, class_=str, allow_none=True) == None


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
    assert validate_action_id(None, allow_none=True) == None


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
    assert validate_string(None, field_name="field", allow_none=True) == None


def test_validate_string_disallow_none() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_string(None, field_name="field", allow_none=False)


def test_validate_string_exceed_max_length() -> None:
    with pytest.raises(InvalidUsageError):
        assert validate_string("a" * 5, field_name="field", max_length=4)
