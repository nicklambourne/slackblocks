from __future__ import annotations

import pytest

from slackblocks import (
    Attachment,
    Button,
    ConversationFilter,
    Image,
    InvalidUsageError,
    LengthError,
    MissingRequiredError,
    MutualExclusivityError,
    Option,
    OptionGroup,
    RangeError,
    SectionBlock,
    SlackFile,
    Text,
    TextType,
    TypeMismatchError,
)
from slackblocks.utils import (
    validate_action_id,
    validate_int,
    validate_string,
    validate_string_nonnull,
)


def test_invalid_usage_exception() -> None:
    """Regression: original test from before the Phase 5 split."""
    with pytest.raises(InvalidUsageError):
        attachment = Attachment(blocks=[], color="0000000000000")
        print(attachment)


# -- Subclass hierarchy contract -------------------------------------------


def test_subclasses_inherit_from_invalid_usage_error() -> None:
    """Every Phase 5 subclass must subclass InvalidUsageError so that legacy
    'except InvalidUsageError' code continues to catch all library errors."""
    for cls in (
        LengthError,
        MissingRequiredError,
        MutualExclusivityError,
        RangeError,
        TypeMismatchError,
    ):
        assert issubclass(cls, InvalidUsageError)


def test_subclasses_are_distinct_types() -> None:
    """Sanity: subclasses are independent leaves; they don't subclass each other."""
    leaves = {
        LengthError,
        MissingRequiredError,
        MutualExclusivityError,
        RangeError,
        TypeMismatchError,
    }
    for a in leaves:
        for b in leaves:
            if a is not b:
                assert not issubclass(a, b)


# -- Validator subclass propagation ----------------------------------------


def test_validate_string_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        validate_string("a" * 10, field_name="x", max_length=5)


def test_validate_string_none_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        validate_string(None, field_name="x")


def test_validate_string_nonnull_too_short_raises_length_error() -> None:
    with pytest.raises(LengthError):
        validate_string_nonnull("", field_name="x", min_length=1)


def test_validate_int_below_min_raises_range_error() -> None:
    with pytest.raises(RangeError):
        validate_int(1, min_value=5)


def test_validate_int_above_max_raises_range_error() -> None:
    with pytest.raises(RangeError):
        validate_int(10, max_value=5)


def test_validate_int_none_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        validate_int(None)


def test_validate_action_id_none_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        validate_action_id(None)


def test_validate_action_id_empty_raises_length_error() -> None:
    with pytest.raises(LengthError):
        validate_action_id("")


def test_validate_action_id_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        validate_action_id("a" * 256)


# -- Library raise-site subclass propagation ------------------------------


def test_image_without_source_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        Image(alt_text="x")


def test_image_with_both_sources_raises_mutual_exclusivity() -> None:
    with pytest.raises(MutualExclusivityError):
        Image(
            alt_text="x",
            image_url="https://x.png",
            slack_file=SlackFile(url="https://y.png", id=None),
        )


def test_slackfile_with_both_url_and_id_raises_mutual_exclusivity() -> None:
    with pytest.raises(MutualExclusivityError):
        SlackFile(url="https://x.png", id="F123")


def test_conversation_filter_without_args_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        ConversationFilter()


def test_section_block_without_text_or_fields_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        SectionBlock()


def test_text_max_length_violation_raises_length_error() -> None:
    with pytest.raises(LengthError):
        Text.to_text("abcdef", max_length=5)


def test_text_none_disallowed_raises_missing_required() -> None:
    with pytest.raises(MissingRequiredError):
        Text.to_text(None)


def test_text_invalid_type_raises_type_mismatch() -> None:
    with pytest.raises(TypeMismatchError):
        Text.to_text(123)  # type: ignore[arg-type]


def test_option_group_inferred_text_type_mismatch_raises_type_mismatch() -> None:
    """OptionGroup options with MARKDOWN-typed text inside a StaticSelectMenu
    triggers TypeMismatchError. Build via the helper class that wraps it."""
    from slackblocks.elements import StaticSelectMenu

    bad = Option(text=Text("hi", type_=TextType.MARKDOWN), value="x")
    good = Option(text=Text("hi", type_=TextType.PLAINTEXT), value="y")
    with pytest.raises(TypeMismatchError):
        StaticSelectMenu(action_id="a", options=[good, bad], placeholder="p")


def test_option_url_too_long_raises_length_error() -> None:
    with pytest.raises(LengthError):
        Option(text="x", value="v", url="https://" + "a" * 3000)


def test_attachment_invalid_color_raises_type_mismatch() -> None:
    with pytest.raises(TypeMismatchError):
        Attachment(blocks=[], color="zz1234")


def test_button_too_long_text_raises_length_error() -> None:
    """Button.text exceeding max length comes via Text.to_text -> LengthError."""
    with pytest.raises(LengthError):
        Button(text="a" * 100, action_id="a")


def test_legacy_except_invalid_usage_error_still_catches_all() -> None:
    """Backwards-compat contract: existing 'except InvalidUsageError' code
    continues to catch every subclass."""
    for raiser in (
        lambda: SlackFile(url="https://x.png", id="F123"),
        lambda: ConversationFilter(),
        lambda: SectionBlock(),
        lambda: validate_int(10, max_value=5),
        lambda: validate_string("a" * 10, field_name="x", max_length=5),
    ):
        with pytest.raises(InvalidUsageError):
            raiser()


def test_option_group_min_size_violation_raises_length_error() -> None:
    """OptionGroup requires at least one option (min_size=1)."""
    with pytest.raises(LengthError):
        OptionGroup(label="x", options=[])
