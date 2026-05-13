"""
Views are app-customized visual areas within modals and Home tabs.

See: <https://api.slack.com/reference/surfaces/views>.
"""

from __future__ import annotations

from enum import Enum
from json import dumps
from typing import Any

from slackblocks._core import resolve
from slackblocks.blocks import Block
from slackblocks.objects import Text, TextLike
from slackblocks.utils import coerce_to_list, validate_string


class ViewType(Enum):
    MODAL = "modal"
    HOME = "home"


class View:
    """
    Base class for Slack app surfaces -- the visual areas an app can populate
    with blocks. Concrete subclasses are
    [`ModalView`](/slackblocks/latest/reference/views/#views.ModalView) (for
    pop-up modals) and
    [`HomeTabView`](/slackblocks/latest/reference/views/#views.HomeTabView)
    (for the per-user App Home tab).

    See: <https://api.slack.com/reference/surfaces/views>.

    Args:
        type: one of the `ViewType` enum members. Concrete subclasses set
            this for you; in practice you should construct `ModalView` or
            `HomeTabView` rather than `View` directly.
        blocks: 1-100 blocks that make up the contents of the view.
        private_metadata: a string (max 3000 characters) that will be sent
            back to your app in any view-related interaction payloads.
            Useful for stashing per-view server-side context.
        callback_id: an identifier (max 255 characters) for distinguishing
            this view's submissions from other views your app exposes.
        external_id: a custom identifier that is unique within the views of
            a given Slack team. Slack uses it to find views your app has
            previously published.

    Throws:
        InvalidUsageError: if any of the validation checks fail.
    """

    def __init__(
        self,
        type: ViewType,
        blocks: Block | list[Block],
        private_metadata: str | None = None,
        callback_id: str | None = None,
        external_id: str | None = None,
    ) -> None:
        self.type_ = type.value
        self.blocks = coerce_to_list(blocks, class_=Block, min_size=1, max_size=100)
        self.private_metadata = validate_string(
            private_metadata,
            field_name="private_metadata",
            max_length=3000,
            allow_none=True,
        )
        self.callback_id = validate_string(
            callback_id, field_name="callback_id", max_length=255, allow_none=True
        )
        self.external_id = external_id

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                "type": self.type_,
                "blocks": self.blocks,
                "private_metadata": self.private_metadata if self.private_metadata else None,
                "callback_id": self.callback_id if self.callback_id else None,
                "external_id": self.external_id if self.external_id else None,
            }
        )

    def to_dict(self) -> dict[str, Any]:
        return self._resolve()

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class ModalView(View):
    """
    Modal views are used with the `views.open`, `views.update` and `views.push`
        Slack Web API methods.

    See: <https://api.slack.com/reference/surfaces/views#modal>

    Args:
        title: heading that appears at the top left of the view.
        blocks: a list of blocks (max 100) that define the content of the view.
        close: the text of the close button (max 24 chars) in the view.
            Must be `Text.PLAINTEXT`.
        submit: the text of the submit button (max 24 chars) in the view.
            Must be `Text.PLAINTEXT`.
        private_metadata: a string (max 3000 chars) that will be sent to your app
            in `view_submission`.
        callback_id: A string that will identify submissions of this view.
        clear_on_close: when `True` all views in the model will be cleared when
            it is closed.
        notify_on_close: when `True` a `view_closed` event will be sent when the
            modal is closed.
        external_id: A custom identifier that is unique within the views of a
            given Slack team.
        submit_disabled: when `True` disabled submitting the form until one or
            more inputs have been provided. Used only for
            [`configuaration models`](https://api.slack.com/reference/workflows/configuration-view).
    """

    def __init__(
        self,
        title: TextLike,
        blocks: Block | list[Block],
        close: TextLike | None = None,
        submit: TextLike | None = None,
        private_metadata: str | None = None,
        callback_id: str | None = None,
        clear_on_close: bool | None = False,
        notify_on_close: bool | None = False,
        external_id: str | None = None,
        submit_disabled: bool | None = False,
    ) -> None:
        super().__init__(
            type=ViewType.MODAL,
            blocks=blocks,
            private_metadata=private_metadata,
            callback_id=callback_id,
            external_id=external_id,
        )
        self.title = Text.to_text_nonnull(title, force_plaintext=True, max_length=24)
        self.close = Text.to_text(close, force_plaintext=True, max_length=24, allow_none=True)
        self.submit = Text.to_text(submit, force_plaintext=True, max_length=24, allow_none=True)
        self.clear_on_close = clear_on_close
        self.notify_on_close = notify_on_close
        self.submit_disabled = submit_disabled

    def _resolve(self) -> dict[str, Any]:
        return resolve(
            {
                **super()._resolve(),
                "title": self.title,
                "close": self.close if self.close else None,
                "submit": self.submit if self.submit else None,
                "clear_on_close": self.clear_on_close if self.clear_on_close else None,
                "notify_on_close": self.notify_on_close if self.notify_on_close else None,
                "submit_disabled": self.submit_disabled if self.submit_disabled else None,
            }
        )


class HomeTabView(View):
    """
    `HomeTabViews` are used with the `views.publish` Web API method.

    See: <https://api.slack.com/reference/surfaces/views#home>.

    Args:
        blocks: A list of blocks that defines the content of the view (max 100).
        private_metadata: a string (max 3000 chars) that will be sent to your app
            in `view_submission`.
        callback_id: A string that will identify submissions of this view.
        external_id: A custom identifier that is unique within the views of a
            given Slack team.
    """

    def __init__(
        self,
        blocks: Block | list[Block],
        private_metadata: str | None = None,
        callback_id: str | None = None,
        external_id: str | None = None,
    ) -> None:
        super().__init__(
            type=ViewType.HOME,
            blocks=blocks,
            private_metadata=private_metadata,
            callback_id=callback_id,
            external_id=external_id,
        )
