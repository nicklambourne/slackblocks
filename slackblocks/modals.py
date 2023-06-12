from enum import Enum
from json import dumps
from typing import Any, Dict

from slackblocks.views import ModalView


class Modal(ModalView):
    """
    Kept for backwards compatibility - see slackblock.views.ModalView
    """

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)

    def to_dict(self) -> Dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)
