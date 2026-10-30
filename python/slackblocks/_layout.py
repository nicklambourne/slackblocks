"""Modeled composite-block traversal. Opaque payload metadata is never visited."""

from typing import Any

from .errors import InvalidUsageError


def child_blocks(block: dict[str, Any]) -> list[dict[str, Any]]:
    if block.get("type") == "container":
        return block.get("child_blocks", [])
    if block.get("type") == "layout":
        return [item["block"] for row in block.get("rows", []) for item in row["items"]]
    return []


def validate_composite_depth(
    block: dict[str, Any], depth: int = 0, has_layout: bool = False
) -> None:
    kind = block.get("type")
    has_layout = has_layout or kind == "layout"
    if kind in ("layout", "container"):
        depth += 1
    if has_layout and depth > 2:
        raise InvalidUsageError("Layout and container nesting cannot exceed two composite levels.")
    for nested in child_blocks(block):
        validate_composite_depth(nested, depth, has_layout)


def descendants(blocks: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Walk actual children and fallback content, without interpreting application data."""
    result = []
    for block in blocks:
        result.append(block)
        result.extend(descendants(child_blocks(block)))
        if block.get("type") == "layout":
            result.extend(descendants(block.get("fallback", [])))
    return result


def block_cost(block: dict[str, Any], nested: bool = False) -> int:
    """Conservative documented layout budget; fallback replaces, not accompanies, the grid."""
    children = child_blocks(block)
    if block.get("type") == "layout":
        return max(
            1 + sum(block_cost(b, True) for b in children),
            sum(block_cost(b) for b in block.get("fallback", [])),
        )
    if block.get("type") == "container" and (
        nested or any(b.get("type") == "layout" for b in descendants(children))
    ):
        return 1 + sum(block_cost(b, nested) for b in children)
    return 1
