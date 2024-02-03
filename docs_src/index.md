# Welcome to `slackblocks`!

<p align="center">
  <img width="30%" src="./img/sb.png" />
</p>

`slackblocks` is Python package for creating complex Slack messages 
    using the Slack [BlockKit API](https://api.slack.com/block-kit).

It exists so you don't have to define block-based Slack messages by
    hand-writing JSON.

## Components

The [Slack BlockKit API](https://api.slack.com/block-kit) defines a number of 
    different resource types (all defined in JSON) which work together to 
    define Block-based messages.

`slackblocks` makes using this API easier by providing a hierarchy of Python
    classes that represent these resources.

### Objects
[`Objects`](/reference/objects) (e.g. [`Text`](/reference/objects/#objects.Text)) 
    are the lowest level pimitives that are used to populate 
    [`Elements`](/reference/elements) and [`Blocks`](/reference/blocks).

### Elements
[`Elements`](/reference/elements) are typically interactive UI elements that take
    in [`Object`](/reference/objects) to define their content. For example, the 
    [`CheckboxGroup`](/elements/#elements.CheckboxGroup) element takes in one or
    more [`Option`](/reference/objects/#objects.Option) items and presents a
    checkbox menu to the user with those options.

### Blocks
[`Blocks`](/reference/blocks) are the core element of the API, with different 
    [`Blocks`](/reference/blocks) used to create different types of visual
    elements. For example, the [`DividerBlock`](/reference/blocks/#blocks.DividerBlock), 
    when rendered, will show a visual element similar to a `<hr>` HTML element. The
    [`RichTextBlock`](/reference/blocks/#blocks.RichTextBlock) on the other hand
    allows for the display of text elements with visual styling like italics,
    block quotes, lists and code blocks. 

### Messages
[`Messages`](/reference/messages/) are a convenience wrapper around `Blocks` that
    can be unpacked as arguments straight into the official Slack Python SDK (or
    its legacy `slackclient` counterpart).

### Views
[`Views`](reference/views/) are an alternative usage for [`Blocks`](/reference/blocks)
    that allow for the creation of custom UI "surfaces" within Slack, e.g. for 
    third-party apps.

## Guides
In addition to a complete reference of all classes and functions provided by the 
    `slackblocks` library, this documentation contains guides on:

- [Installing `slackblocks`]()
- [Basic Usage](./usage/basic_usage.md)
- [Using Blocks](./usage/using_blocks.md)
- [Sending Block-based Messages](./usage/sending_messages.md)