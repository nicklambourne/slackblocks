# Migrating from 1.x to 2.x

`slackblocks 2.0` is a major version bump that focuses on cleaning up internal
correctness issues, modernising the type-annotation surface, and adding several
ergonomic helpers. The good news: **for almost all users, upgrading is a
no-op** beyond changing the Python version pin.

This page covers what actually changes for you and what's worth adopting once
you've upgraded.

## Quick checklist

| Step | Action |
| --- | --- |
| 1 | Confirm you are on **Python 3.10 or newer**. If not, stay on `slackblocks ~= 1.2`. |
| 2 | Bump your dependency pin to `slackblocks ~= 2.0` (or `>=2,<3`). |
| 3 | Run your test suite. Existing code should continue to work unchanged. |
| 4 | Optionally adopt the new conveniences below. |

## Truly breaking changes

There is exactly one breaking change you might trip over:

### Python 3.8 and 3.9 are no longer supported

Both reached upstream end of life
([3.8 EOL Oct 2024](https://devguide.python.org/versions/),
[3.9 EOL Oct 2025](https://devguide.python.org/versions/)) and are not
receiving security patches.

If you are still on either:

- **Stay on the `1.x` line** until you can upgrade Python. The 1.x line is
  maintained on a best-effort basis for bugfixes; see the
  [Compatibility](compatibility.md) page.
- Pin via your package manager:

=== "pip (`requirements.txt`)"
    ```
    slackblocks>=1,<2
    ```

=== "poetry (`pyproject.toml`)"
    ```toml
    slackblocks = "^1"
    ```

=== "uv (`pyproject.toml`)"
    ```toml
    "slackblocks>=1,<2"
    ```

That's it for breaking changes. Nothing else in the public API was removed
or renamed.

## What's *not* breaking, but worth adopting

### `PlainText` and `Markdown` instead of `Text(type_=...)`

Before:

```python
from slackblocks import Text, TextType

heading = Text("Welcome", type_=TextType.PLAINTEXT, emoji=True)
body = Text("**bold**", type_=TextType.MARKDOWN, verbatim=True)
```

After:

```python
from slackblocks import PlainText, Markdown

heading = PlainText("Welcome", emoji=True)
body = Markdown("**bold**", verbatim=True)
```

`PlainText` and `Markdown` are subclasses of `Text`, so anywhere a `Text` or
`TextLike` was accepted before still accepts the new types. The rendered
JSON is byte-identical to the equivalent `Text` calls.

### `Workflow.from_url(...)` instead of nested `Workflow(trigger=Trigger(...))`

Before:

```python
from slackblocks import Workflow, Trigger, InputParameter

workflow = Workflow(
    trigger=Trigger(
        url="https://slack.com/shortcuts/...",
        customizable_input_parameters=[
            InputParameter(name="env", value="prod"),
            InputParameter(name="retries", value="3"),
        ],
    ),
)
```

After:

```python
from slackblocks import Workflow

workflow = Workflow.from_url(
    "https://slack.com/shortcuts/...",
    env="prod",
    retries="3",
)
```

The verbose form continues to work, so you do not have to migrate.

### `block_kit_builder_url(...)` for browser preview

New helper that turns any block, list of blocks, message, view, or raw dict
into a [Block Kit Builder](https://app.slack.com/block-kit-builder) URL. Open
the URL in your browser to see exactly what Slack will render before you
ship.

```python
from slackblocks import SectionBlock, block_kit_builder_url

print(block_kit_builder_url(SectionBlock("Hi")))
# https://app.slack.com/block-kit-builder/#%7B%22blocks%22%3A%5B...%5D%7D
```

Pass `team_id="T0123ABCD"` for a workspace-specific URL.

### Typed exceptions for finer-grained error handling

`InvalidUsageError` now has five subclasses, raised in well-defined
situations:

| Subclass | Raised when |
| --- | --- |
| `LengthError` | A string or list violates a min/max-length constraint. |
| `RangeError` | A numeric value violates a min/max-value constraint. |
| `TypeMismatchError` | Wrong type or unexpected discrete value. |
| `MutualExclusivityError` | Two args that must not both be set were both set. |
| `MissingRequiredError` | At least one of a set of args was required. |

```python
from slackblocks import Button, LengthError

try:
    Button(text="x" * 100, action_id="b")
except LengthError as e:
    # specifically a length violation
    handle_length(e)
```

**Existing `except InvalidUsageError` blocks continue to catch every
subclass** — no change required to upgrade. The new subclasses are purely
additive.

### Round-tripping incoming Slack JSON with `from_dict`

If your app receives Slack interactivity payloads or events and wants to
inspect the blocks, you can now parse them back into `slackblocks` objects:

```python
import json
from slackblocks import Block

incoming = json.loads(slack_request_body)
block = Block.from_dict(incoming["blocks"][0])
print(block.text.text)  # for a SectionBlock, etc.
```

`Block.from_dict` reads `data["type"]` and dispatches to the right subclass.
Composition objects (`Text`, `Option`, `Confirm`, etc.) and the simple
blocks (`DividerBlock`, `FileBlock`, `HeaderBlock`, `MarkdownBlock`,
`ImageBlock`, `SectionBlock`, `ContextBlock`, `VideoBlock`) round-trip fully.

Blocks that contain interactive elements (`ActionsBlock`, `InputBlock`,
`SectionBlock` with an `accessory`, `ContextBlock` with image elements)
raise `NotImplementedError` for now; these depend on `Element.from_dict`
which is planned for a follow-up release.

### Two new block types

Slack added these in 2024; they're now available:

```python
from slackblocks import MarkdownBlock, VideoBlock

# Slack's GitHub-flavored markdown block (richer than SectionBlock mrkdwn).
MarkdownBlock(text="# Heading\n\n- item 1\n- item 2")

# Embed a video from a Slack-supported provider.
VideoBlock(
    alt_text="Demo",
    thumbnail_url="https://example.com/thumb.png",
    title="Getting Started",
    video_url="https://example.com/video.mp4",
)
```

## Type-checking improvements

The 2.x series ships with substantially tighter type signatures. If you
type-check your code with mypy, pyright, or Pyre, expect that:

- `Text.to_text("hi")` is now correctly typed as `Text` (was previously
  `Text | None`). Code that did `assert text is not None` after `to_text`
  calls can drop the assertion.
- `Button(style="warning")` is now flagged at type-check time —
  `style` is `Literal["primary", "danger"] | ButtonStyle | None`.
- `ColumnSettings(align="LEFT")` is similarly flagged at type-check time.
- `ConversationFilter(include=["bogus"])` is flagged.

These constraints already existed at runtime; the difference is that mypy
now agrees with the runtime behaviour.

## Internal changes you might notice

The `_resolve()` method on every block, element, and composition object was
refactored to use a shared `slackblocks._core.resolve()` walker. This is
strictly internal — JSON output is byte-identical to 1.x — but if you have
been subclassing internal classes you may notice the `_resolve` bodies are
much shorter.

If you are extending `slackblocks` with custom classes:

- If your custom class has a `_resolve` method that follows the existing
  pattern (manual `if self.x is not None: out["x"] = self.x._resolve()`), it
  continues to work in 2.x without modification.
- If you want, you can switch to using the new `slackblocks._core.resolve`
  / `omit_none` helpers; doing so guarantees recursion into nested
  `_resolve` outputs and is much more concise.

## Reporting issues

If you hit a real upgrade problem, please [open an issue on
GitHub](https://github.com/nicklambourne/slackblocks/issues/new) — include
your Python version, your `slackblocks` version (old and new), and a
minimal repro.
