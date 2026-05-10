# Contributing

Contributions are welcome — bug fixes, new block/element support, documentation improvements, and test coverage are all appreciated.

This page covers everything you need to develop, test, and submit changes to `slackblocks` itself. If you're looking for help *using* the library, see [Troubleshooting & FAQ](usage/troubleshooting.md) instead.

## Where to start

- **Bugs and feature requests:** open an issue at <https://github.com/nicklambourne/slackblocks/issues>.
- **Small fixes** (typos, docstrings, missed validation): a PR is fine; no issue needed first.
- **Larger changes** (new block types, behaviour changes, API additions): please open an issue first to discuss the approach. Block Kit evolves and not every Slack-side feature is a clean fit.

## Local development setup

`slackblocks` uses [uv](https://docs.astral.sh/uv/) to manage dependencies and the dev environment. Install uv first if you don't have it:

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
# or: brew install uv
```

Then clone and sync:

```bash
git clone https://github.com/nicklambourne/slackblocks.git
cd slackblocks
uv sync --all-groups
```

`uv sync --all-groups` installs the library in editable mode along with the `dev` group (test, lint, type-check tooling) and the `docs` group (mkdocs and friends). If you only need one of those groups, use `uv sync --group dev` or `uv sync --group docs`.

All commands below should be prefixed with `uv run ` so they execute inside the project's virtual environment.

### Python version

The library targets **Python 3.8.1+** and is tested against 3.8 through 3.14 in CI. You can develop against any supported version, but check that anything you add doesn't accidentally rely on a newer Python feature (e.g. `match` statements, the `|` union syntax in annotations evaluated at runtime, `from __future__ import annotations` is OK).

## Running checks

The project enforces five CI gates on every PR. Run them locally before pushing:

### Unit tests

```bash
uv run pytest test/unit
```

Tests live in `test/unit/` and assert that `slackblocks` constructs valid Block Kit JSON by comparing against fixture files in `test/samples/`. When adding or modifying a block/element/object, add a corresponding test that:

1. Constructs the object.
2. Compares its `repr()` (or `_resolve()`) against a JSON fixture file.

The shared helpers in `test/unit/utils.py` (`fetch_sample`, `OPTION_A`/`B`/`C`) keep tests concise.

!!! note
    `pyproject.toml` configures pytest to **turn warnings into errors** (`filterwarnings = ["error"]`). If a dependency emits a deprecation warning, the tests will fail. This applies only to this repository's CI — it doesn't affect your application.

### Black formatting

```bash
uv run black .            # format
uv run black . --check    # CI mode: fail if formatting changes are needed
```

CI runs `black . --check` and will fail the PR if anything is unformatted.

### flake8 linting

```bash
uv run flake8 slackblocks
```

Configuration lives in `pyproject.toml` under `[tool.flake8]`. Per-file ignores already cover the unavoidable cases (re-exports in `__init__.py`, escape sequences in regex literals).

### mypy type checking

```bash
uv run mypy slackblocks
```

`slackblocks` ships `py.typed`, so type hints are part of the public API. New public classes and functions should be fully annotated.

### twine package check

```bash
uv build
uv run twine check --strict dist/*
```

This validates that the package metadata and rendered README are accepted by PyPI. CI runs this on every PR — see [`.github/workflows/package-check.yml`](https://github.com/nicklambourne/slackblocks/blob/master/.github/workflows/package-check.yml).

### Run everything in one go

```bash
uv run pytest test/unit \
  && uv run black . --check \
  && uv run flake8 slackblocks \
  && uv run mypy slackblocks \
  && uv build \
  && uv run twine check --strict dist/*
```

## Working on the docs

The documentation site uses [MkDocs Material](https://squidfunk.github.io/mkdocs-material/) with [mkdocstrings](https://mkdocstrings.github.io/) for auto-generated API reference pages from the source docstrings.

```bash
uv run mkdocs serve
# visit http://127.0.0.1:8000/
```

Source files live in `docs_src/`:

- `docs_src/index.md` — landing page.
- `docs_src/usage/` — narrative guides (Installation, Using Blocks, Sending Messages, Cookbook, Troubleshooting).
- `docs_src/reference/` — auto-generated API reference. Each page is mostly a `:::` mkdocstrings include with a short orientation intro.
- `docs_src/img/` — images, including per-block screenshots in `docs_src/img/usage/`.

Navigation order is configured in `mkdocs.yml`. If you add a new page, remember to register it under `nav:`.

### Updating docstrings

The reference pages render directly from the docstrings on classes and functions. `slackblocks` uses **Google-style docstrings**:

```python
def something(arg: str, optional: int = 0) -> bool:
    """
    Short, single-sentence summary.

    Longer description if needed. Mention behaviour, edge cases, and the
    relevant Slack API doc page (with a URL).

    Args:
        arg: what `arg` is and what valid values look like.
        optional: defaults to `0`. Note the limit if Slack imposes one.

    Throws:
        InvalidUsageError: if `arg` exceeds the 255-character limit.
    """
```

When linking between docs, **use relative `.md` paths** (e.g. `[Blocks](reference/blocks.md)`) rather than absolute `/slackblocks/latest/...` URLs — relative links work in local previews, on GitHub, and on the deployed site; absolute URLs only work on the deployed site.

### Versioned docs

The deployed docs are versioned with [mike](https://github.com/jimporter/mike). The `docs.yml` workflow runs on each release and publishes a new versioned site under `latest/`. You don't need to run `mike` manually as part of a normal contribution.

## Validation in the library

A core selling point of `slackblocks` is that constructing an invalid object raises `InvalidUsageError` eagerly rather than letting it fail server-side at Slack. When adding a new field or block, check Slack's reference page (linked in every docstring) for:

- **Character limits** on text fields — use the helpers in `slackblocks/utils.py` (e.g. `validate_string`, `validate_action_id`) rather than duplicating limit checks.
- **Min/max counts** on collections — validate in `__init__` and raise `InvalidUsageError` with a message that includes the actual length and the limit.
- **Mutually exclusive arguments** — raise `InvalidUsageError` with both possibilities named.
- **Allowed element types** in containers (e.g. `ContextBlock` only accepts `Text` and `Image`).

Validation tests live in `test/unit/test_errors.py` and the per-module test files. Add a test for each new error path.

## Pull request checklist

Before opening a PR:

- [ ] The change is covered by a unit test (or has a clear reason not to be).
- [ ] `uv run pytest test/unit` passes.
- [ ] `uv run black . --check` passes.
- [ ] `uv run flake8 slackblocks` passes.
- [ ] `uv run mypy slackblocks` passes.
- [ ] `uv run twine check --strict dist/*` passes (after `uv build`).
- [ ] If you added or modified a public class/function, its docstring is updated in Google style.
- [ ] If you added a new block, element, or object, it's exported from `slackblocks/__init__.py`.
- [ ] If you added a new feature surface, the relevant `docs_src/usage/` page (and possibly the [Cookbook](usage/cookbook.md)) is updated.

PRs are reviewed against `master`. Keep changes focused — separate refactors from feature additions where practical.

## Project layout

```
slackblocks/                    # the library
├── __init__.py                 # public API surface (re-exports)
├── attachments.py              # Attachment, Color, Field
├── blocks.py                   # all *Block classes
├── elements.py                 # all interactive elements
├── errors.py                   # InvalidUsageError
├── messages.py                 # Message, WebhookMessage, MessageResponse
├── modals.py                   # Modal (legacy alias for ModalView)
├── objects.py                  # composition objects: Text, Option, etc.
├── rich_text/                  # rich text elements + containers
├── utils.py                    # shared validation helpers
└── views.py                    # ModalView, HomeTabView

test/
├── unit/                       # unit tests (run in CI)
├── integration/                # live Slack API tests (not run in CI)
└── samples/                    # JSON fixtures for unit tests

docs_src/                       # mkdocs source
.github/workflows/              # CI pipelines
pyproject.toml                  # project metadata, dep groups, tool config
uv.lock                         # locked dependency versions
```

## Releases

Releases are cut by the maintainer:

1. Bump `version` in `pyproject.toml` (under `[project]`).
2. Tag the commit (`v1.x.y`).
3. Push the tag — `publish.yml` runs `uv build` and publishes the resulting sdist and wheel to PyPI.
4. The GitHub release triggers `docs.yml`, which deploys the versioned docs.

Contributors don't need to do anything for a release; just open PRs and the maintainer will batch them.

## Questions?

Open a GitHub issue or start a discussion at <https://github.com/nicklambourne/slackblocks/issues>.
