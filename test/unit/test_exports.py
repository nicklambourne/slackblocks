"""Tests verifying that public classes are reachable from the top-level
``slackblocks`` package.

Regression tests for #154 (FileInput and SlackFile were defined in submodules
but never re-exported)."""


def test_file_input_is_exported() -> None:
    from slackblocks import FileInput  # noqa: F401
    from slackblocks.elements import FileInput as FileInputFromSubmodule

    assert FileInput is FileInputFromSubmodule


def test_slack_file_is_exported() -> None:
    from slackblocks import SlackFile  # noqa: F401
    from slackblocks.objects import SlackFile as SlackFileFromSubmodule

    assert SlackFile is SlackFileFromSubmodule
