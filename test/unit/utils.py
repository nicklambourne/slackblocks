from pathlib import Path
from typing import Union

from slackblocks.objects import Option, Text, TextType

OPTION_A = Option(text=Text("A", type_=TextType.PLAINTEXT), value="A")
OPTION_B = Option(text=Text("B", type_=TextType.PLAINTEXT), value="B")
OPTION_C = Option(text=Text("C", type_=TextType.PLAINTEXT), value="C")
TWO_OPTIONS = [OPTION_A, OPTION_B]
THREE_OPTIONS = TWO_OPTIONS + [
    OPTION_C,
]


def fetch_sample(path: Union[Path, str]) -> str:
    with open(path, "r") as file_:
        return file_.read()
