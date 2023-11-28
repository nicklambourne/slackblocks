# slackblocks <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/img/sb.png" align="right" width="250px"/>

![PyPI - License](https://img.shields.io/pypi/l/slackblocks)
![Python Versions](https://img.shields.io/pypi/pyversions/slackblocks)
[![PyPI](https://img.shields.io/pypi/v/slackblocks?color=yellow&label=PyPI&logo=python&logoColor=white)](https://pypi.org/project/slackblocks/#history)
[![Downloads](https://static.pepy.tech/badge/slackblocks)](https://pepy.tech/project/slackblocks)
[![Build Status](https://github.com/nicklambourne/slackblocks/actions/workflows/unit-tests.yml/badge.svg?branch=master)](https://github.com/nicklambourne/slackblocks/actions)

## What is it?

`slackblocks` is a Python API for building messages in the fancy Slack Block Kit API.

It was created by [Nicholas Lambourne](https://github.com/nicklambourne) for the [UQCS Slack Bot](https://github.com/UQComputingSociety/uqcsbot) because he hates writing JSON, naturally this project has subsequently involved writing more JSON than if he'd done the original task by hand.

## Requirements
`slackblocks` requires Python >= 3.7.

As of version 0.1.0 it has no dependencies outside the Python standard library.

## Installation

```bash
pip install slackblocks
```

## Usage

```python
from slackblocks import Message, SectionBlock


block = SectionBlock("Hello, world!")
message = Message(channel="#general", blocks=block)
message.json()

```

Will produce the following JSON string:
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
Which can be sent as payload to the Slack message API HTTP endpoints.

Of more practical uses is the ability to unpack the objects directly into 
the Python Slack Client in order to send messages:
```python
from os import environ
from slack import WebClient
from slackblocks import Message, SectionBlock


client = WebClient(token=environ["SLACK_API_TOKEN"])
block = SectionBlock("Hello, world!")
message = Message(channel="#general", blocks=block)

response = client.chat_postMessage(**message)
```

Note the `**` operator in front of the `message` object.

## Can I use this in my project?

Yes, please do! The code is all open source and BSD-3.0 licensed.
