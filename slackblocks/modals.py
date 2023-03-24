from abc import abstractmethod
from json import dumps
from typing import Any, Dict, List, Optional, Union
from .blocks import Block
from .elements import Text
from enum import Enum


class ModalType(Enum):
    """
    Convenience class for holding modal types
    though currently there is only one.
    """

    MODAL = "modal"


class Modal:
    """
    A Slack modal object that can be converted into JSON
    for use with the Slack API
    """

    def __init__(
        self,
        title: Union[str, Text],
        close: Union[str, Text],
        submit: Optional[Union[str, Text]] = None,
        blocks: Optional[Union[List[Block], Block]] = None,
    ):
        self.type = ModalType.MODAL

        if isinstance(blocks, List):
            self.blocks = blocks
        elif isinstance(blocks, Block):
            self.blocks = [
                blocks,
            ]
        else:
            # Even if there's no blocks - an empty blocks entry is required.
            self.blocks = []

        self.title = Text.to_text(title, force_plaintext=True)
        self.submit = submit
        self.close = Text.to_text(close, force_plaintext=True)

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)

    def _attributes(self) -> Dict[str, Any]:
        return {"type": self.type.value}

    @abstractmethod
    def _resolve(self) -> Dict[str, Any]:
        modal = dict()

        modal["type"] = self.type.value

        modal["title"] = self.title._resolve()
        modal["close"] = self.close._resolve()

        if self.submit:
            self.submit = Text.to_text(self.submit, force_plaintext=True)
            modal["submit"] = self.submit._resolve()

        modal["blocks"] = [block._resolve() for block in self.blocks]

        return modal

    def to_dict(self) -> Dict[str, Any]:
        return self._resolve()

    def json(self) -> str:
        return dumps(self._resolve(), indent=4)
