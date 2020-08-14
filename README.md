# slackblocks <img src="https://github.com/nicklambourne/slackblocks/raw/master/docs/img/sb.png" align="right" width="250px"/>

![PyPI - License](https://img.shields.io/pypi/l/slackblocks)
![Python Versions](https://img.shields.io/pypi/pyversions/slackblocks)
[![PyPI version](https://badge.fury.io/py/slackblocks.svg)](https://pypi.org/project/slackblocks/)
[![Downloads](https://pepy.tech/badge/slackblocks)](https://pepy.tech/project/slackblocks)
[![Build Status](https://api.travis-ci.org/nicklambourne/slackblocks.svg?branch=master)](https://travis-ci.org/github/nicklambourne/slackblocks)
[![Coverage Status](https://coveralls.io/repos/github/nicklambourne/slackblocks/badge.svg?branch=master)](https://coveralls.io/github/nicklambourne/slackblocks?branch=master)

## What is it?

`slackblocks` is a Python API for building messages in the fancy new Slack Block Kit API.

It was created by [Nicholas Lambourne](https://github.com/nicklambourne) for the [UQCS Slack Bot](https://github.com/UQComputingSociety/uqcsbot) because he hates writing JSON.

As it turns out, the functionality that he was actually after exists in the outdated Slack Secondary Attachments API, but he was already in too deep to turn back.

N.B: This is still WIP software and I some of the more tricky interactive Block elements have yet to be implemented.

## Requirements
`slackblocks` requires Python >= 3.6.

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
                "text": "Hello, world!",
                "verbatim": false
            }
        }
    ]
}
```
Which can be sent as payload to the Slack message API HTTP endpoints.

Of more practical use is the ability to unpack the objects directly into 
the Python Slack Client to send messages:
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