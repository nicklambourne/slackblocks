"""Tests verifying that public classes are reachable from the top-level
``slackblocks`` package.

Regression tests for #154 (FileInput and SlackFile were defined in submodules
but never re-exported)."""

from __future__ import annotations


def test_file_input_is_exported() -> None:
    from slackblocks import FileInput  # noqa: F401
    from slackblocks.elements import FileInput as FileInputFromSubmodule

    assert FileInput is FileInputFromSubmodule


def test_slack_file_is_exported() -> None:
    from slackblocks import SlackFile  # noqa: F401
    from slackblocks.objects import SlackFile as SlackFileFromSubmodule

    assert SlackFile is SlackFileFromSubmodule


def test_public_type_aliases_are_exported() -> None:
    """Type aliases appearing in public signatures must be importable from
    the top-level package."""
    from slackblocks import AlertLevel, Chart, ContainerWidth, SlackIconName, TaskStatus
    from slackblocks.blocks import (
        AlertLevel as AlertLevelFromSubmodule,
    )
    from slackblocks.blocks import (
        ContainerWidth as ContainerWidthFromSubmodule,
    )
    from slackblocks.blocks import (
        TaskStatus as TaskStatusFromSubmodule,
    )
    from slackblocks.objects import (
        Chart as ChartFromSubmodule,
    )
    from slackblocks.objects import (
        SlackIconName as SlackIconNameFromSubmodule,
    )

    assert AlertLevel is AlertLevelFromSubmodule
    assert Chart is ChartFromSubmodule
    assert ContainerWidth is ContainerWidthFromSubmodule
    assert SlackIconName is SlackIconNameFromSubmodule
    assert TaskStatus is TaskStatusFromSubmodule


def test_spec_version_is_declared() -> None:
    """The shared conformance principles require each implementation to
    declare the spec version it conforms to."""
    import slackblocks

    assert slackblocks.SPEC_VERSION == "1.0.1"
