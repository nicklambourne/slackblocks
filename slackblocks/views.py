"""
Views are app-customized visual areas within modals and Home tabs.

See: <https://api.slack.com/reference/surfaces/views>.
"""

from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union

from slackblocks.blocks import Block
from slackblocks.objects import Text, TextLike
from slackblocks.utils import coerce_to_list, validate_string


class ViewType(Enum):
    MODAL = "modal"
    HOME = "home"


class View:
    """ """

    def __init__(
        self,
        type: ViewType,
        blocks: Union[Block, List[Block]],
        private_metadata: Optional[str] = None,
        callback_id: Optional[str] = None,
        external_id: Optional[str] = None,
    ):
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

    def _resolve(self) -> Dict[str, Any]:
        view = {}
        view["type"] = self.type_
        view["blocks"] = [block._resolve() for block in self.blocks]
        if self.private_metadata:
            view["private_metadata"] = self.private_metadata
        if self.callback_id:
            view["callback_id"] = self.callback_id
        if self.external_id:
            view["external_id"] = self.external_id
        return view

    def to_dict(self) -> Dict[str, Any]:
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
        blocks: Union[Block, List[Block]],
        close: Optional[TextLike] = None,
        submit: Optional[TextLike] = None,
        private_metadata: Optional[str] = None,
        callback_id: Optional[str] = None,
        clear_on_close: Optional[bool] = False,
        notify_on_close: Optional[bool] = False,
        external_id: Optional[str] = None,
        submit_disabled: Optional[bool] = False,
    ):
        super().__init__(
            type=ViewType.MODAL,
            blocks=blocks,
            private_metadata=private_metadata,
            callback_id=callback_id,
            external_id=external_id,
        )
        self.title = Text.to_text(title, force_plaintext=True, max_length=24)
        self.close = Text.to_text(
            close, force_plaintext=True, max_length=24, allow_none=True
        )
        self.submit = Text.to_text(
            submit, force_plaintext=True, max_length=24, allow_none=True
        )
        self.clear_on_close = clear_on_close
        self.notify_on_close = notify_on_close
        self.submit_disabled = submit_disabled

    def _resolve(self) -> Dict[str, Any]:
        modal_view = super()._resolve()
        modal_view["title"] = self.title._resolve()
        if self.close:
            modal_view["close"] = self.close._resolve()
        if self.submit:
            modal_view["submit"] = self.submit._resolve()
        if self.clear_on_close:
            modal_view["clear_on_close"] = self.clear_on_close
        if self.notify_on_close:
            modal_view["notify_on_close"] = self.notify_on_close
        if self.submit_disabled:
            modal_view["submit_disabled"] = self.submit_disabled
        return modal_view


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
        blocks: Union[Block, List[Block]],
        private_metadata: Optional[str] = None,
        callback_id: Optional[str] = None,
        external_id: Optional[str] = None,
    ) -> "HomeTabView":
        super().__init__(
            type=ViewType.HOME,
            blocks=blocks,
            private_metadata=private_metadata,
            callback_id=callback_id,
            external_id=external_id,
        )
