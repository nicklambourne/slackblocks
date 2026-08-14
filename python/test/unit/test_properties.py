"""Hypothesis property-based tests for ``slackblocks._core`` and the
validators in ``slackblocks.utils``.

These complement the example-based tests by generating inputs across
the validator boundary (empty / max-length strings, integer extremes,
mixed-type lists, etc.) so we catch boundary regressions without having
to enumerate every case by hand.
"""

from __future__ import annotations

import json
import math

import pytest
from hypothesis import HealthCheck, given, settings
from hypothesis import strategies as st

from slackblocks._core import omit_none, resolve
from slackblocks.errors import (
    InvalidUsageError,
    LengthError,
    MissingRequiredError,
    RangeError,
    TypeMismatchError,
)
from slackblocks.objects import (
    InputParameter,
    Option,
    SlackFile,
    Text,
    TextType,
)
from slackblocks.utils import (
    coerce_to_list,
    coerce_to_list_nonnull,
    is_hex,
    validate_action_id,
    validate_int,
    validate_string,
    validate_string_nonnull,
)

# Hypothesis defaults are fine for everything below; this profile makes the
# CI run a touch more reproducible.
HYPOTHESIS_PROFILE = settings(
    max_examples=100,
    deadline=None,
    suppress_health_check=[HealthCheck.too_slow],
)


# ---------------------------------------------------------------------------
# is_hex
# ---------------------------------------------------------------------------


@given(st.text(alphabet="0123456789abcdefABCDEF", min_size=0, max_size=64))
@HYPOTHESIS_PROFILE
def test_is_hex_accepts_any_hex_string(s: str) -> None:
    assert is_hex(s)


@given(st.text(min_size=1, max_size=64))
@HYPOTHESIS_PROFILE
def test_is_hex_rejects_non_hex(s: str) -> None:
    """Any string with at least one non-hex character is rejected."""
    if any(ch not in "0123456789abcdefABCDEF" for ch in s):
        assert not is_hex(s)


# ---------------------------------------------------------------------------
# validate_string / validate_string_nonnull
# ---------------------------------------------------------------------------


@given(st.text(min_size=1, max_size=200))
@HYPOTHESIS_PROFILE
def test_validate_string_nonnull_passes_within_no_bounds(s: str) -> None:
    assert validate_string_nonnull(s, field_name="x") == s


@given(
    st.text(min_size=0, max_size=200),
    st.integers(min_value=1, max_value=300),
)
@HYPOTHESIS_PROFILE
def test_validate_string_nonnull_max_length_boundary(s: str, max_length: int) -> None:
    """Equal-to-max passes; over-max raises LengthError."""
    if len(s) <= max_length:
        assert validate_string_nonnull(s, "x", max_length=max_length) == s
    else:
        with pytest.raises(LengthError):
            validate_string_nonnull(s, "x", max_length=max_length)


@given(
    st.text(min_size=0, max_size=200),
    st.integers(min_value=0, max_value=200),
)
@HYPOTHESIS_PROFILE
def test_validate_string_nonnull_min_length_boundary(s: str, min_length: int) -> None:
    """Equal-to-min passes; under-min raises LengthError."""
    if len(s) >= min_length:
        assert validate_string_nonnull(s, "x", min_length=min_length) == s
    else:
        with pytest.raises(LengthError):
            validate_string_nonnull(s, "x", min_length=min_length)


@given(st.text(min_size=1, max_size=100), st.booleans())
@HYPOTHESIS_PROFILE
def test_validate_string_passthrough_or_raises_on_none(s: str, allow_none: bool) -> None:
    """For non-None input the result is the input verbatim."""
    assert validate_string(s, "x", allow_none=allow_none) == s


@given(st.booleans())
@HYPOTHESIS_PROFILE
def test_validate_string_none_input_obeys_allow_none(allow_none: bool) -> None:
    if allow_none:
        assert validate_string(None, "x", allow_none=True) is None
    else:
        with pytest.raises(MissingRequiredError):
            validate_string(None, "x", allow_none=False)


# ---------------------------------------------------------------------------
# validate_int
# ---------------------------------------------------------------------------


@given(st.integers(min_value=-(10**9), max_value=10**9))
@HYPOTHESIS_PROFILE
def test_validate_int_passes_within_no_bounds(n: int) -> None:
    assert validate_int(n) == n


@given(
    st.integers(min_value=-1000, max_value=1000),
    st.integers(min_value=-1000, max_value=1000),
)
@HYPOTHESIS_PROFILE
def test_validate_int_min_value_boundary(n: int, min_value: int) -> None:
    if n >= min_value:
        assert validate_int(n, min_value=min_value) == n
    else:
        with pytest.raises(RangeError):
            validate_int(n, min_value=min_value)


@given(
    st.integers(min_value=-1000, max_value=1000),
    st.integers(min_value=-1000, max_value=1000),
)
@HYPOTHESIS_PROFILE
def test_validate_int_max_value_boundary(n: int, max_value: int) -> None:
    if n <= max_value:
        assert validate_int(n, max_value=max_value) == n
    else:
        with pytest.raises(RangeError):
            validate_int(n, max_value=max_value)


@given(st.booleans())
@HYPOTHESIS_PROFILE
def test_validate_int_none_input_obeys_allow_none(allow_none: bool) -> None:
    if allow_none:
        assert validate_int(None, allow_none=True) is None
    else:
        with pytest.raises(MissingRequiredError):
            validate_int(None, allow_none=False)


# ---------------------------------------------------------------------------
# validate_action_id
# ---------------------------------------------------------------------------


@given(st.text(min_size=1, max_size=255))
@HYPOTHESIS_PROFILE
def test_validate_action_id_accepts_any_1_to_255_char_string(s: str) -> None:
    assert validate_action_id(s) == s


@given(st.text(min_size=256, max_size=300))
@HYPOTHESIS_PROFILE
def test_validate_action_id_rejects_strings_over_255(s: str) -> None:
    with pytest.raises(LengthError):
        validate_action_id(s)


def test_validate_action_id_rejects_empty_string() -> None:
    """Hypothesis-style boundary: an empty action_id must raise LengthError
    even though length 0 is technically below 'over 255'."""
    with pytest.raises(LengthError):
        validate_action_id("")


# ---------------------------------------------------------------------------
# coerce_to_list / coerce_to_list_nonnull
# ---------------------------------------------------------------------------


@given(st.lists(st.integers(), min_size=0, max_size=20))
@HYPOTHESIS_PROFILE
def test_coerce_to_list_nonnull_lists_pass_through(items: list[int]) -> None:
    """A list that already contains the expected type is returned verbatim."""
    assert coerce_to_list_nonnull(items, class_=int) == items


@given(st.integers())
@HYPOTHESIS_PROFILE
def test_coerce_to_list_nonnull_singleton_is_wrapped(n: int) -> None:
    assert coerce_to_list_nonnull(n, class_=int) == [n]


@given(
    st.lists(st.one_of(st.integers(), st.text()), min_size=1, max_size=20),
)
@HYPOTHESIS_PROFILE
def test_coerce_to_list_nonnull_mixed_list_raises_when_one_is_wrong(
    items: list[int | str],
) -> None:
    """If at least one item is not the expected class, raise TypeMismatchError."""
    has_non_int = any(not isinstance(x, int) for x in items)
    if has_non_int:
        with pytest.raises(TypeMismatchError):
            coerce_to_list_nonnull(items, class_=int)
    else:
        assert coerce_to_list_nonnull(items, class_=int) == items


@given(
    st.lists(st.integers(), min_size=0, max_size=20),
    st.integers(min_value=0, max_value=20),
)
@HYPOTHESIS_PROFILE
def test_coerce_to_list_nonnull_min_size_boundary(items: list[int], min_size: int) -> None:
    if len(items) >= min_size:
        assert coerce_to_list_nonnull(items, class_=int, min_size=min_size) == items
    else:
        with pytest.raises(LengthError):
            coerce_to_list_nonnull(items, class_=int, min_size=min_size)


@given(
    st.lists(st.integers(), min_size=0, max_size=20),
    st.integers(min_value=0, max_value=20),
)
@HYPOTHESIS_PROFILE
def test_coerce_to_list_nonnull_max_size_boundary(items: list[int], max_size: int) -> None:
    if len(items) <= max_size:
        assert coerce_to_list_nonnull(items, class_=int, max_size=max_size) == items
    else:
        with pytest.raises(LengthError):
            coerce_to_list_nonnull(items, class_=int, max_size=max_size)


@given(st.booleans())
@HYPOTHESIS_PROFILE
def test_coerce_to_list_none_input_obeys_allow_none(allow_none: bool) -> None:
    if allow_none:
        assert coerce_to_list(None, class_=int, allow_none=True) is None
    else:
        with pytest.raises(InvalidUsageError):
            coerce_to_list(None, class_=int, allow_none=False)


# ---------------------------------------------------------------------------
# _core.resolve / _core.omit_none
# ---------------------------------------------------------------------------


# JSON-serializable primitive strategy. Excludes NaN / inf because Python's
# json.dumps rejects them.
_finite_floats = st.floats(allow_nan=False, allow_infinity=False, min_value=-1e9, max_value=1e9)
_json_primitives = st.one_of(
    st.none(),
    st.booleans(),
    st.integers(min_value=-(2**31), max_value=2**31 - 1),
    _finite_floats,
    st.text(max_size=64),
)
_json_dicts = st.dictionaries(st.text(min_size=1, max_size=8), _json_primitives)


@given(_json_primitives)
@HYPOTHESIS_PROFILE
def test_resolve_primitives_pass_through(value: object) -> None:
    """Primitives are returned unchanged. NaN / inf are excluded by the
    strategy because json.dumps rejects them anyway."""
    result = resolve(value)
    if isinstance(value, float):
        assert math.isclose(result, value)
    else:
        assert result == value


@given(_json_dicts)
@HYPOTHESIS_PROFILE
def test_resolve_dict_strips_only_none_values(d: dict[str, object]) -> None:
    """resolve() drops None-valued keys and keeps everything else."""
    result = resolve(d)
    assert isinstance(result, dict)
    # Every non-None value in d must be present in the result.
    for k, v in d.items():
        if v is None:
            assert k not in result
        else:
            assert k in result


@given(_json_dicts)
@HYPOTHESIS_PROFILE
def test_resolve_output_is_json_serializable(d: dict[str, object]) -> None:
    """The whole point of resolve(): json.dumps must succeed on the output."""
    json.dumps(resolve(d))


@given(_json_dicts)
@HYPOTHESIS_PROFILE
def test_omit_none_drops_none_only(d: dict[str, object]) -> None:
    """omit_none drops None values and only None values."""
    result = omit_none(d)
    for k, v in d.items():
        if v is None:
            assert k not in result
        else:
            assert k in result and result[k] is v


# ---------------------------------------------------------------------------
# Round-trip properties for simple composition objects
# ---------------------------------------------------------------------------


# Slack ``Text`` accepts any 1-3000 char string. We exercise a wide range,
# including non-ASCII and whitespace-heavy inputs.
_text_string = st.text(min_size=1, max_size=200)


@given(_text_string)
@HYPOTHESIS_PROFILE
def test_text_round_trip_markdown(s: str) -> None:
    """Text in MARKDOWN mode round-trips through _resolve / from_dict."""
    t = Text(s, type_=TextType.MARKDOWN)
    assert Text.from_dict(t._resolve())._resolve() == t._resolve()


@given(_text_string, st.booleans())
@HYPOTHESIS_PROFILE
def test_text_round_trip_plaintext_with_emoji(s: str, emoji: bool) -> None:
    t = Text(s, type_=TextType.PLAINTEXT, emoji=emoji)
    assert Text.from_dict(t._resolve())._resolve() == t._resolve()


@given(_text_string, st.text(min_size=1, max_size=64))
@HYPOTHESIS_PROFILE
def test_option_round_trip(text: str, value: str) -> None:
    """Option round-trips through _resolve / from_dict for arbitrary
    in-bound strings."""
    o = Option(text=text[:75], value=value[:75])
    assert Option.from_dict(o._resolve())._resolve() == o._resolve()


@given(st.text(min_size=1, max_size=32), st.text(min_size=1, max_size=32))
@HYPOTHESIS_PROFILE
def test_input_parameter_round_trip(name: str, value: str) -> None:
    ip = InputParameter(name=name, value=value)
    assert InputParameter.from_dict(ip._resolve())._resolve() == ip._resolve()


@given(st.text(min_size=1, max_size=64))
@HYPOTHESIS_PROFILE
def test_slack_file_round_trip_url(url: str) -> None:
    sf = SlackFile(url=url, id=None)
    assert SlackFile.from_dict(sf._resolve())._resolve() == sf._resolve()


@given(st.text(min_size=1, max_size=64))
@HYPOTHESIS_PROFILE
def test_slack_file_round_trip_id(file_id: str) -> None:
    sf = SlackFile(url=None, id=file_id)
    assert SlackFile.from_dict(sf._resolve())._resolve() == sf._resolve()
