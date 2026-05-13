"""
Utilities for working with Slack's [Block Kit Builder](https://app.slack.com/block-kit-builder).

The Builder accepts a URL of the form ``https://app.slack.com/block-kit-builder/#<JSON>``
where ``<JSON>`` is a URL-encoded JSON payload (typically ``{"blocks": [...]}``).
``block_kit_builder_url`` constructs such a URL from any of the shapes the
rest of this library renders.
"""

from __future__ import annotations

from json import dumps
from typing import Any
from urllib.parse import quote

from slackblocks._core import Resolvable, resolve
from slackblocks.blocks import Block

_BUILDER_URL_PREFIX = "https://app.slack.com/block-kit-builder/"


def block_kit_builder_url(
    payload: Block | list[Block] | Resolvable | dict[str, Any],
    team_id: str | None = None,
) -> str:
    """Build a URL that opens ``payload`` in Slack's Block Kit Builder.

    Use this to preview a message, view, or list of blocks in the browser
    without manually copying JSON into the Builder.

    Args:
        payload: any of:

            - A single ``Block`` -- wrapped in ``{"blocks": [block]}``.
            - A list of ``Block`` -- wrapped in ``{"blocks": [...]}``.
            - Anything with a ``_resolve`` method (``Message``,
              ``WebhookMessage``, ``MessageResponse``, ``View``,
              ``ModalView``, ``HomeTabView``, etc.) -- used as-is.
            - A raw ``dict`` -- used as-is. This is the escape hatch for
              callers building payloads outside the type hierarchy.

        team_id: optional Slack team ID. When supplied, produces a
            workspace-specific Builder URL of the form
            ``https://app.slack.com/block-kit-builder/T0123#<JSON>``.
            When ``None`` (the default), produces the generic URL.

    Returns:
        A fully-formed Block Kit Builder URL.

    Examples:
        Single block::

            from slackblocks import SectionBlock, block_kit_builder_url

            url = block_kit_builder_url(SectionBlock("Hi"))

        Multiple blocks::

            url = block_kit_builder_url([
                SectionBlock("Heading"),
                DividerBlock(),
            ])

        Existing message::

            url = block_kit_builder_url(my_message)

        Targeting a specific workspace::

            url = block_kit_builder_url(my_message, team_id="T0123ABCD")
    """
    body: dict[str, Any]
    if isinstance(payload, Block):
        body = {"blocks": [resolve(payload)]}
    elif isinstance(payload, list):
        body = {"blocks": [resolve(item) for item in payload]}
    elif isinstance(payload, dict):
        body = payload
    else:
        # Anything else with a _resolve() method (Message, View, etc.).
        # resolve() raises TypeError downstream if the object is unsupported.
        resolved = resolve(payload)
        if not isinstance(resolved, dict):
            raise TypeError(
                f"block_kit_builder_url payload must resolve to a dict; got {type(resolved)}."
            )
        body = resolved

    encoded = quote(dumps(body, separators=(",", ":")), safe="")
    prefix = f"{_BUILDER_URL_PREFIX}{team_id}" if team_id else _BUILDER_URL_PREFIX
    return f"{prefix}#{encoded}"
