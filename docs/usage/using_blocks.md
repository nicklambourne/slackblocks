
## Section Block
:::blocks.SectionBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    
    ```python
    from slackblocks import Checkboxes, Option, SectionBlock

    SectionBlock(
        text="This is a section block with a checkbox accessory.", 
        block_id="fake_block_id"
        accessory=CheckboxGroup(
            action_id="checkboxes-action",
            options=[
                Option(
                    text="*Your Only Option*",
                    value="option_one"
                )
            ]
        )
    )
    ```

=== "JSON"
    ```json
    {
        "type": "section",
        "block_id": "fake_block_id",
        "text": {
            "type": "mrkdwn",
            "text": "This is a section block with a checkbox accessory."
        },
        "accessory": {
            "type": "checkboxes",
            "options": [
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*Your Only Option*"
                    },
                    "value": "option_one"
                }
            ],
            "action_id": "checkboxes-action"
        }
    }
    ```

=== "Slack UI"
    ![An example of the UI output of a Section Block](../img/usage/section.png)



## Rich Text Block
:::blocks.RichTextBlock
    options:
        show_bases: false
        show_source: false

=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## Header Block
:::blocks.HeaderBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## Image Block
:::blocks.ImageBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## Input Block
:::blocks.InputBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## Divider Block
:::blocks.DividerBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## File Block
:::blocks.FileBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()


## Context Block
:::blocks.ContextBlock
    options:
        show_bases: false
        show_source: false


=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()

## Actions Block
:::blocks.ActionsBlock
    options:
        show_bases: false
        show_source: false

=== "`slackblocks`"
    ```python
    
    ```

=== "JSON"
    ```json
    
    ```

=== "Slack UI"
    ![]()
