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

=== "Equivalent `curl` Command"
    ```bash
    curl -H "Content-type: application/json" \
    --data '{"channel":"#general","blocks":[{"type":"section", "block_id": "992ceb6b-9ad4-496b-b8e6-1bd8a632e8b3", "text":{"type":"mrkdwn","text":"Hello, world"}}]}' \
    -H "Authorization: Bearer ${SLACK_API_TOKEN}" \
    -X POST https://slack.com/api/chat.postMessage
    ```

=== "Slack UI Output"
    ![Hello World Slack Image](../img/hello_world.png)