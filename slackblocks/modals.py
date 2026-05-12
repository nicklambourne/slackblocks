"""
Modals are pop-up windows, primarily used for collecting data from
users.

This module is kept only for backwards compatibility, modals have
been largely subsumed as a subtype of view.

See: <https://api.slack.com/surfaces/modals>
"""

from __future__ import annotations

from json import dumps
from typing import Any

from slackblocks.views import ModalView


class Modal(ModalView):
    """
    Kept for backwards compatibility - see
        [`ModalView`](/slackblocks/latest/reference/views/#views.ModalView)
    """

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)

    def to_dict(self) -> dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)
