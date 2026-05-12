"""
Shared infrastructure for rendering slackblocks objects into Slack-API-compatible
JSON structures.

This is a private module. The public API is unchanged; the helpers here are
used internally to deduplicate the ``_resolve`` boilerplate that every block,
element, and composition object class previously implemented by hand.

The two patterns this module captures:

1. ``Resolvable`` -- a structural protocol describing the contract that every
   slackblocks renderable class satisfies: a ``_resolve()`` method returning a
   ``dict[str, Any]`` ready to be passed to ``json.dumps``.

2. ``resolve(value)`` -- a recursive helper that walks a value (any
   combination of ``Resolvable`` instances, dicts, lists, tuples, and JSON
   primitives) and produces a plain JSON-serializable structure. This replaces
   the manual ``x._resolve()`` calls inside individual ``_resolve``
   implementations and prevents the entire class of bugs that produced eight
   of the Phase 1 P0 fixes (forgetting to recurse into a nested object).

3. ``omit_none(d)`` -- strip ``None``-valued keys from a dict. Replaces the
   pervasive ``if self.x is not None: out['x'] = self.x`` pattern.
"""

from __future__ import annotations

from typing import Any, Protocol, runtime_checkable


@runtime_checkable
class Resolvable(Protocol):
    """Structural type for any slackblocks class that renders to a JSON-compatible dict.

    Implemented (implicitly, via duck typing) by every block, element, and
    composition object in the library.
    """

    def _resolve(self) -> dict[str, Any]: ...


def resolve(value: Any) -> Any:
    """Recursively convert ``value`` into a JSON-serializable structure.

    - ``None`` and JSON primitives (``bool``, ``int``, ``float``, ``str``) are
      returned unchanged.
    - ``Resolvable`` instances (objects with a ``_resolve`` method) are
      converted by calling that method and recursively resolving the result.
    - ``dict`` is walked, resolving each value and stripping keys whose
      resolved value is ``None``.
    - ``list`` and ``tuple`` are walked element-wise; ``None`` elements are
      preserved (Slack does not generally expect ``None`` in arrays, but the
      caller controls list contents).

    The behaviour matches what the hand-written ``_resolve`` implementations
    do today, with the recursion guaranteed at every nesting depth.
    """
    if value is None or isinstance(value, bool | int | float | str):
        return value
    if isinstance(value, dict):
        out: dict[str, Any] = {}
        for key, raw in value.items():
            resolved = resolve(raw)
            if resolved is not None:
                out[key] = resolved
        return out
    if isinstance(value, list | tuple):
        return [resolve(item) for item in value]
    # Duck-typed Resolvable check: explicit attribute lookup is faster than
    # isinstance(value, Resolvable) and avoids the runtime_checkable overhead
    # on hot paths.
    resolve_method = getattr(value, "_resolve", None)
    if callable(resolve_method):
        return resolve(resolve_method())
    # Fall through: assume the value is already JSON-serializable (e.g. a
    # custom numeric type). json.dumps will raise TypeError if it isn't.
    return value


def omit_none(d: dict[str, Any]) -> dict[str, Any]:
    """Return a new dict with ``None``-valued keys removed.

    Convenience for the ``if self.x is not None: out['x'] = self.x`` pattern
    that appears throughout the library's ``_resolve`` methods. Prefer this
    over manual key-by-key omission so future code consistently treats
    ``None`` as "field not set".
    """
    return {key: value for key, value in d.items() if value is not None}
