`slackblocks` is designed primarily for use with either the [`slack-sdk`](https://pypi.org/project/slack-sdk/) or (legacy) [`slackclient`](https://pypi.org/project/slackclient/) Python packages. Usage of `slackblocks` remains identical regardless of which Slack client library you're using.

While there's nothing stopping you from sending the rendered messages directly with `curl` or `requests`, we recommend using the `**` (dictionary unpacking)operator to unpack `slackblocks` `Messages` directly into the Slack `client`'s `chat_postMessage` function.

An example of this is provided below along with the JSON result of rendering the message, an equivalent `curl` command, and finally the result of the message as it appears in the Slack user interface.

### Sending a Message with the (Modern) `slack-sdk` Library
=== "Python (`slackblocks`)"
    ```python
    from os import environ
    from slack_sdk import WebClient
    from slackblocks import Message, SectionBlock


    client = WebClient(token=environ["SLACK_API_TOKEN"])
    block = SectionBlock("Hello, world!")
    message = Message(channel="#general", blocks=block)

    response = client.chat_postMessage(**message)
    ```

=== "JSON Message"
    ```json
    {
        "channel": "#general",
        "mrkdwn": true,
        "blocks": [
            {
                "type": "section",
                "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hello, world!"
                }
            }
        ]
    }
    ```
    * Note that the `block_id` field is a pseudorandomly generated UUID. You can pass a value to `Block` constructors should you desire deterministic `Blocks`.

=== "Equivalent `curl` Command"
    ```bash
    curl -H "Content-type: application/json" \
    --data '{"channel":"#general","blocks":[{"type":"section", "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3", "text":{"type":"mrkdwn","text":"Hello, world"}}]}' \
    -H "Authorization: Bearer ${SLACK_API_TOKEN}" \
    -X POST https://slack.com/api/chat.postMessage
    ```

=== "Slack UI Output"
    ![Hello World Slack Image](../img/hello_world.png)


### Sending a Message with the (Legacy) `slackclient` Library
=== "Python (`slackblocks`)"
    ```python
    from os import environ
    from slack import WebClient
    from slackblocks import Message, SectionBlock


    client = WebClient(token=environ["SLACK_API_TOKEN"])
    block = SectionBlock("Hello, world!")
    message = Message(channel="#general", blocks=block)

    response = client.chat_postMessage(**message)
    ```

=== "JSON Message"
    ```json
    {
        "channel": "#general",
        "mrkdwn": true,
        "blocks": [
            {
                "type": "section",
                "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hello, world!"
                }
            }
        ]
    }
    ```
    * Note that the `block_id` field is a pseudorandomly generated UUID. You can pass a value to `Block` constructors should you desire deterministic `Blocks`.

=== "Equivalent `curl` Command"
    ```bash
    curl -H "Content-type: application/json" \
    --data '{"channel":"#general","blocks":[{"type":"section", "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3", "text":{"type":"mrkdwn","text":"Hello, world"}}]}' \
    -H "Authorization: Bearer ${SLACK_API_TOKEN}" \
    -X POST https://slack.com/api/chat.postMessage
    ```

=== "Slack UI Output"
    ![Hello World Slack Image](../img/hello_world.png)