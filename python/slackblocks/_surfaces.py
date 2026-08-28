"""Internal Block Kit surface compatibility checks."""

from __future__ import annotations

from typing import Any, Literal

from ._core import resolve
from .errors import TypeMismatchError

BlockSurface = Literal["home", "message", "modal"]

_BLOCK_TYPES_BY_SURFACE: dict[BlockSurface, frozenset[str]] = {
    "message": frozenset(
        {
            "actions",
            "card",
            "carousel",
            "container",
            "context",
            "context_actions",
            "data_table",
            "data_visualization",
            "divider",
            "file",
            "header",
            "image",
            "markdown",
            "plan",
            "rich_text",
            "section",
            "table",
            "task_card",
            "video",
        }
    ),
    "modal": frozenset(
        {
            "actions",
            "alert",
            "card",
            "context",
            "divider",
            "header",
            "image",
            "input",
            "rich_text",
            "section",
            "video",
        }
    ),
    "home": frozenset(
        {
            "actions",
            "card",
            "carousel",
            "container",
            "context",
            "data_table",
            "divider",
            "header",
            "image",
            "input",
            "rich_text",
            "section",
            "table",
            "video",
        }
    ),
}


def block_type(block: Any) -> str | None:
    """Return the rendered Slack type for a block-like object."""
    rendered = resolve(block)
    if not isinstance(rendered, dict):
        return None
    value = rendered.get("type")
    return value if isinstance(value, str) else None


def validate_surface_blocks(blocks: list[Any], surface: BlockSurface) -> None:
    """Reject block types that Slack does not support on ``surface``."""
    allowed = _BLOCK_TYPES_BY_SURFACE[surface]
    for index, block in enumerate(blocks):
        type_ = block_type(block)
        if type_ not in allowed:
            raise TypeMismatchError(
                f"blocks[{index}] type {type_!r} is not supported on {surface} surfaces"
            )
