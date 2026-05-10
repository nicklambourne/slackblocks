# Rich Text

The **Rich Text** API lets you build inline-formatted content (bold, italic, strikethrough, code, links, mentions, lists, quotes, code blocks) without writing markdown by hand.

Structure:

- A [`RichTextBlock`](blocks.md#blocks.RichTextBlock) wraps one or more *containers*.
- **Containers** group elements: [`RichTextSection`](#rich_text.objects.RichTextSection), [`RichTextList`](#rich_text.objects.RichTextList), [`RichTextQuote`](#rich_text.objects.RichTextQuote), [`RichTextCodeBlock`](#rich_text.objects.RichTextCodeBlock).
- **Elements** are the leaf primitives: [`RichText`](#rich_text.elements.RichText) (styled text), [`RichTextLink`](#rich_text.elements.RichTextLink), [`RichTextUser`](#rich_text.elements.RichTextUser), [`RichTextChannel`](#rich_text.elements.RichTextChannel), [`RichTextEmoji`](#rich_text.elements.RichTextEmoji), [`RichTextUserGroup`](#rich_text.elements.RichTextUserGroup).

See the [Rich-formatted alert cookbook recipe](../usage/cookbook.md#rich-formatted-alert) for a complete example.

Slack reference: <https://api.slack.com/reference/block-kit/blocks#rich_text>

::: rich_text

## Rich Text Elements (Primitives)
::: rich_text.elements
    options:
        filters: ["!^RichTextElement"]
        show_bases: false

## Rich Text Objects (Containers)
::: rich_text.objects
    options:
        filters: ["!^RichTextObject"]
        show_bases: false
