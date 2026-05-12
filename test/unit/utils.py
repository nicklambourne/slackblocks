from __future__ import annotations

from typing import TYPE_CHECKING

from slackblocks.objects import Option, Text, TextType

if TYPE_CHECKING:
    from pathlib import Path

OPTION_A = Option(text=Text("A", type_=TextType.PLAINTEXT), value="A")
OPTION_B = Option(text=Text("B", type_=TextType.PLAINTEXT), value="B")
OPTION_C = Option(text=Text("C", type_=TextType.PLAINTEXT), value="C")
TWO_OPTIONS = [OPTION_A, OPTION_B]
THREE_OPTIONS = TWO_OPTIONS + [
    OPTION_C,
]


def fetch_sample(path: Path | str) -> str:
    with open(path) as file_:
        return file_.read()
