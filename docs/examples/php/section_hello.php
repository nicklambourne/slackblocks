<?php

declare(strict_types=1);

require 'vendor/autoload.php';

use Slackblocks as S;

$message = new S\MessagePayload(channel: 'C0123456', blocks: [new S\SectionBlock(text: 'Hello from slackblocks!', blockId: 'hello')]);
echo $message->toJson();
