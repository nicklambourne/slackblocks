from pathlib import Path
from typing import Union


def fetch_sample(path: Union[Path, str]) -> str:
    with open(path, "r") as file_:
        return file_.read()
