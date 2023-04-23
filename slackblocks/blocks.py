from abc import abstractmethod, ABC
from enum import Enum
from json import dumps
from typing import Any, Dict, List, Optional, Union
from uuid import uuid4
from slackblocks.elements import (
    CheckboxGroup, 
    Element, 
    ElementType, 
    PlainTextInput, 
    RadioButtonGroup, 
    SelectMenu, 
    ExternalMultiSelectMenu, 
    StaticMultiSelectMenu, 
    Text, 
    TextLike, 
    TextType,
)
from slackblocks.errors import InvalidUsageError


class BlockType(Enum):
    """
    Convenience class for identifying the different types of blocks available
    in the Slack Blocks API and their programmatic names.
    """

    ACTIONS = "actions"
    CONTEXT = "context"
    DIVIDER = "divider"
    FILE = "file"
    HEADER = "header"
    IMAGE = "image"
    INPUT = "input"
    SECTION = "section"
    VIDEO = "video"


class Block(ABC):
    """
    Basis block containing attributes and behaviour common to all blocks.
    N.B: Block is an abstract class and cannot be sent directly.
    """

    def __init__(self, type_: BlockType, block_id: Optional[str] = None):
        self.type = type_
        if block_id and len(block_id) > 255:
            raise InvalidUsageError(
                f"Block ID character limit (255 chars) exceeded with ID: {block_id}"
            )
        self.block_id = block_id if block_id else str(uuid4())

    def __add__(self, other: "Block"):
        return [self, other]

    def _attributes(self):
        return {"type": self.type.value, "block_id": self.block_id}

    @abstractmethod
    def _resolve(self) -> Dict[str, any]:
        pass

    def __repr__(self) -> str:
        return dumps(self._resolve(), indent=4)


class ActionsBlock(Block):
    """
    A block that is used to hold interactive elements.
    """

    def __init__(
        self, elements: Optional[List[Element]] = None, block_id: Optional[str] = None
    ):
        super().__init__(type_=BlockType.ACTIONS, block_id=block_id)
        if isinstance(elements, Element):
            self.elements = [
                elements,
            ]
        elif isinstance(elements, list) and all(
            [isinstance(el, Element) for el in elements]
        ):
            if len(elements) > 25:
                raise InvalidUsageError(
                    "There is maximum of 25 elements per action block"
                )
            self.elements = elements
        else:
            raise InvalidUsageError(
               
                "Elements must be a single element or list of elements"
            )

    def _resolve(self):
        actions = self._attributes()
        actions["elements"] = [element._resolve() for element in self.elements]
        return actions


class ContextBlock(Block):
    """
    Displays message context, which can include both images and text.
    """

    def __init__(
        self, elements: Optional[List[Element]] = None, block_id: Optional[str] = None
    ):
        super().__init__(type_=BlockType.CONTEXT, block_id=block_id)
        self.elements = []
        for element in elements:
            if element.type == ElementType.TEXT or element.type == ElementType.IMAGE:
                self.elements.append(element)
            else:
                raise InvalidUsageError(
                    "Context blocks can only hold image and text elements"
                )
        if len(self.elements) > 10:
            raise InvalidUsageError("Context blocks can hold a maximum of ten elements")

    def _resolve(self) -> Dict[str, any]:
        context = self._attributes()
        context["elements"] = [element._resolve() for element in self.elements]
        return context


class DividerBlock(Block):
    """
    A content divider, like an <hr>, to split up different blocks inside of
    a message. The divider block is nice and neat, requiring only a type.
    """

    def __init__(self, block_id: Optional[str] = None):
        super().__init__(type_=BlockType.DIVIDER, block_id=block_id)

    def _resolve(self):
        return self._attributes()


class FileBlock(Block):
    """
    Displays a remote file.
    """

    def __init__(self, external_id: str, source: str, block_id: Optional[str]):
        super().__init__(type_=BlockType.FILE, block_id=block_id)
        self.external_id = external_id
        self.source = source

    def _resolve(self) -> Dict[str, any]:
        file = self._attributes()
        file["external_id"] = self.external_id
        file["source"] = self.source
        return file


class HeaderBlock(Block):
    """
    A header is a plain-text block that displays in a larger, bold font.
    """

    def __init__(self, text: Union[str, Text], block_id: Optional[str] = None):
        super().__init__(type_=BlockType.HEADER, block_id=block_id)
        if type(text) is Text:
            self.text = text
        else:
            self.text = Text(text, type_=TextType.PLAINTEXT, verbatim=False)

    def _resolve(self) -> Dict[str, any]:
        header = self._attributes()
        header["text"] = self.text._resolve()
        return header


class ImageBlock(Block):
    """
    A simple image block, designed to make those cat photos really pop.
    """

    def __init__(
        self,
        image_url: str,
        alt_text: Optional[str] = "",
        title: Optional[Union[Text, str]] = None,
        block_id: Optional[str] = None,
    ):
        super().__init__(type_=BlockType.IMAGE, block_id=block_id)
        self.image_url = image_url
        self.alt_text = alt_text
        if title and type(title) is Text:
            if title.text_type == TextType.MARKDOWN:
                self.title = Text(
                    text=title.text,
                    type_=TextType.PLAINTEXT,
                    emoji=title.emoji,
                    verbatim=title.verbatim,
                )
            else:
                self.title = title
        elif title:
            self.title = Text(text=title, type_=TextType.PLAINTEXT)
        else:
            self.title = Text(text=" ", type_=TextType.PLAINTEXT)

    def _resolve(self) -> Dict[str, Any]:
        image = self._attributes()
        image["image_url"] = self.image_url
        image["alt_text"] = self.alt_text
        if self.title:
            image["title"] = self.title._resolve()
        return image


class InputBlock(Block):
    def __init__(
        self,
        label: TextLike,
        element: Union[
            CheckboxGroup, 
            DatePicker,
            ExternalMultiSelectMenu,
            PlainTextInput, 
            RadioButtonGroup,
            SelectMenu,
            StaticMultiSelectMenu,
        ],
        dispatch_action: bool = False,
        block_id: Optional[str] = None,
        hint: Optional[TextLike] = None,
        optional: bool = False,
    ):
        super().__init__(type_=BlockType.SECTION, block_id=block_id)
        self.label = Text.to_text(
            label, 
            force_plaintext=True, 
            max_length=2000, 
            allow_none=False
        )
        self.element = element
        self.dispatch_action = dispatch_action
        self.hint = Text.to_text(
            hint, 
            force_plaintext=True, 
            max_length=2000, 
            allow_none=True
        )
        self.optional = optional

    def _resolve() -> Dict[str, Any]:
        input_block = self._attributes()
        input_block["label"] = self.label._resolve()
        input_block["element"] = self.element._resolve()
        if self.dispatch_action is not None:
            input_block["dispatch_action"] = self.dispatch_action
        if self.hint is not None:
            input_block["hint"] = self.hint
        if self.optional is not None:
            input_block["optional"] = self.optional
        return input_block

    
class SectionBlock(Block):
    """
    A section is one of the most flexible blocks available -
    it can be used as a simple text block, in combination with text fields,
    or side-by-side with any of the available block elements.
    """

    def __init__(
        self,
        text: Optional[Union[str, Text]] = None,
        block_id: Optional[str] = None,
        fields: Optional[List[Text]] = None,
        accessory: Optional[Element] = None,
    ):
        super().__init__(type_=BlockType.SECTION, block_id=block_id)
        if text:
            if type(text) is Text:
                self.text = text
            else:
                self.text = Text(text)
        else:
            self.text = text
        self.fields = fields
        self.accessory = accessory

    def _resolve(self) -> Dict[str, Any]:
        section = self._attributes()
        if self.text:
            section["text"] = self.text._resolve()
        if self.fields:
            section["fields"] = [field._resolve() for field in self.fields]
        if self.accessory:
            section["accessory"] = self.accessory._resolve()
        return section


class VideoBlock(Block):
    def __init__(
        self,
        alt_text: str,
        title: str,
        thumbnail_url: str,
        video_url: str,
        author_name: Optional[str] = None,
        block_id: Optional[str] = None,
        description: Optional[str] = None,
        provider_icon_url: Optional[str] = None,
        provider_name: Optional[str] = None,
        title_url: Optional[str] = None,

    ):
        self.alt_text = alt_text
        if len(title) > 200:
            raise InvalidUsageError(
                f"Field `title` must be less than 200 characters: {title}"
            )
        self.title = title
        self.thumbnail_url = thumbnail_url
        self.video_url = video_url
        if len(author_name) > 50:
            raise InvalidUsageError(
                f"Field `author_name` must be less than 200 characters: {author_name}"
            )
        self.author_name = author_name
        self.block_id = block_id
        self.description = description
        self.provider_icon_url = provider_icon_url
        self.provider_name = provider_name
        self.title_url = title_url

    def _resolve() -> Dict[str, Any]:
        video_block = self._attributes()
        video_block["alt_text"] = self.alt_text
        video_block["title"] = self.title
        video_block["thumbnail_url"] = self.thumbnail_url
        video_block["video_url"] = self.video_url
        if self.author_name is not None:
            video_block["author_name"] = self.author_name
        if self.description is not None:
            video_block["description"] = self.description
        if self.provider_icon_url is not None:
            video_block["provider_icon_url"] = self.provider_icon_url
        if self.provider_name is not None:
            video_block["provider_name"] = self.provider_name
        if self.title_url is not None:
            video_block["title_url"] = self.title_url
        return video_block
        
