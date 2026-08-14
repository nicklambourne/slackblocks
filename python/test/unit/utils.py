from pathlib import Path

from slackblocks.objects import Option, Text, TextType

SPEC_ROOT = Path(__file__).resolve().parents[3] / "spec"
VALID_FIXTURES = SPEC_ROOT / "fixtures" / "valid"


class CanonicalJSON(str):
    """Fixture text whose equality follows the spec's semantic JSON contract."""

    __hash__ = None  # type: ignore[assignment]

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, str):
            return NotImplemented
        from json import loads

        return loads(self) == loads(other)


OPTION_A = Option(text=Text("A", type_=TextType.PLAINTEXT), value="A")
OPTION_B = Option(text=Text("B", type_=TextType.PLAINTEXT), value="B")
OPTION_C = Option(text=Text("C", type_=TextType.PLAINTEXT), value="C")
TWO_OPTIONS = [OPTION_A, OPTION_B]
THREE_OPTIONS = TWO_OPTIONS + [
    OPTION_C,
]


def fetch_sample(path: Path | str) -> CanonicalJSON:
    return CanonicalJSON((VALID_FIXTURES / path).read_text(encoding="utf-8"))
