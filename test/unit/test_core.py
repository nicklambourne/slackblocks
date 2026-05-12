"""Tests for the private ``slackblocks._core`` module that provides shared
JSON-resolution infrastructure used by the rest of the library."""

from __future__ import annotations

import json

from slackblocks._core import Resolvable, omit_none, resolve


class _ToyResolvable:
    """Minimal Resolvable used in tests; produces a dict with a nested
    Resolvable so we can verify recursion."""

    def __init__(self, value: int, child: _ToyResolvable | None = None) -> None:
        self.value = value
        self.child = child

    def _resolve(self) -> dict[str, object]:
        return {"value": self.value, "child": self.child}


def test_resolve_primitive_passthrough() -> None:
    assert resolve(None) is None
    assert resolve(True) is True
    assert resolve(42) == 42
    assert resolve(3.14) == 3.14
    assert resolve("hi") == "hi"


def test_resolve_dict_strips_none_values() -> None:
    assert resolve({"a": 1, "b": None, "c": "x"}) == {"a": 1, "c": "x"}


def test_resolve_list_passes_through_elements() -> None:
    assert resolve([1, "two", None, 3.0]) == [1, "two", None, 3.0]


def test_resolve_tuple_becomes_list() -> None:
    assert resolve((1, 2, 3)) == [1, 2, 3]


def test_resolve_calls_resolve_method_on_resolvable() -> None:
    toy = _ToyResolvable(value=7)
    assert resolve(toy) == {"value": 7}


def test_resolve_recurses_into_nested_resolvables() -> None:
    inner = _ToyResolvable(value=1)
    outer = _ToyResolvable(value=2, child=inner)
    assert resolve(outer) == {"value": 2, "child": {"value": 1}}


def test_resolve_output_is_json_serializable() -> None:
    """The whole point of resolve(): the output must be json.dumps-clean.
    This guards against the entire class of Phase 1 P0 bugs."""
    inner = _ToyResolvable(value=1)
    outer = _ToyResolvable(value=2, child=inner)
    payload = resolve({"top": outer, "extras": [inner, inner], "skip": None})
    json.dumps(payload)  # Must not raise.
    assert payload == {
        "top": {"value": 2, "child": {"value": 1}},
        "extras": [{"value": 1}, {"value": 1}],
    }


def test_omit_none_drops_only_none_values() -> None:
    """Falsy non-None values (0, '', False, []) must be preserved."""
    assert omit_none({"a": 1, "b": None, "c": 0, "d": "", "e": False, "f": []}) == {
        "a": 1,
        "c": 0,
        "d": "",
        "e": False,
        "f": [],
    }


def test_omit_none_returns_new_dict() -> None:
    original = {"a": 1, "b": None}
    result = omit_none(original)
    assert result is not original
    assert original == {"a": 1, "b": None}


def test_resolvable_protocol_runtime_checkable() -> None:
    """isinstance(x, Resolvable) should succeed for duck-typed objects."""
    toy = _ToyResolvable(value=1)
    assert isinstance(toy, Resolvable)
    # A class without _resolve must not pass.

    class NotResolvable:
        pass

    assert not isinstance(NotResolvable(), Resolvable)
