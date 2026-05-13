"""
Custom error classes for clearer error reporting.

``InvalidUsageError`` is the base type. The five subclasses below are raised
in well-defined situations so consumer code can ``except`` for a specific
failure category. All subclasses are instances of ``InvalidUsageError``, so
existing code that does ``except InvalidUsageError`` continues to work
unchanged.

| Subclass                  | Raised when                                              |
| ------------------------- | -------------------------------------------------------- |
| ``LengthError``           | A string or list violates a min/max-length constraint.   |
| ``RangeError``            | A numeric value violates a min/max-value constraint.     |
| ``TypeMismatchError``     | An argument is of the wrong type or unexpected value.    |
| ``MutualExclusivityError``| Two arguments that must not both be set were both set.   |
| ``MissingRequiredError``  | At least one of a set of arguments was required, but     |
|                           | none was provided.                                       |
"""

from __future__ import annotations


class InvalidUsageError(Exception):
    """
    You have violated the `slackblocks` API or Slack Web API in a way
        that has been caught by validation checks.

    All more-specific validation exceptions raised by ``slackblocks``
    subclass this exception, so ``except InvalidUsageError`` catches every
    library-raised validation failure.

    Args:
        message: a custom error message providing details of the API
            violation.
    """

    def __init__(self, message: str) -> None:
        return super().__init__(message)


class LengthError(InvalidUsageError):
    """A string or list length is outside the allowed bounds for a field.

    Typical sources: a ``Text`` body longer than the Slack-enforced limit,
    an option list with too many entries, an ``action_id`` over 255
    characters, etc.
    """


class RangeError(InvalidUsageError):
    """A numeric value is outside the allowed bounds for a field.

    Typical sources: ``NumberInput.min_value > max_value``, integer
    arguments outside an enforced ``min_value`` / ``max_value`` range.
    """


class TypeMismatchError(InvalidUsageError):
    """An argument is of the wrong type or an unexpected discrete value.

    Typical sources: passing a non-``Option`` element into a list expected
    to hold ``Option`` instances; supplying a colour string that is neither
    a hex code nor a ``Color`` enum; supplying a string outside an
    enumerated set (e.g. ``ColumnSettings.align='banana'``).
    """


class MutualExclusivityError(InvalidUsageError):
    """Two arguments that must not both be set were both set.

    Typical sources: passing both ``image_url`` and ``slack_file`` to
    ``Image``; passing both ``options`` and ``option_groups`` to a
    ``StaticSelectMenu``; passing both ``url`` and ``id`` to ``SlackFile``.
    """


class MissingRequiredError(InvalidUsageError):
    """At least one of a set of arguments was required, but none was provided.

    Typical sources: ``ConversationFilter`` invoked with none of its three
    filter fields set; ``SectionBlock`` invoked with neither ``text`` nor
    ``fields``.
    """
