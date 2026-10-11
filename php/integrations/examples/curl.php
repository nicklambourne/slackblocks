<?php

declare(strict_types=1);

require 'vendor/autoload.php';

use Slackblocks\{MessagePayload, SectionBlock};
use Slackblocks\Examples\Sending;

$message = new MessagePayload(channel: 'C0123456789', text: 'Hello', blocks: [new SectionBlock('*Hello* from PHP')]);
return static fn(string $token, string $endpoint = 'https://slack.com/api/chat.postMessage'): array => Sending::curl($message, $token, $endpoint);
