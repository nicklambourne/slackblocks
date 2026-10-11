<?php

declare(strict_types=1);

require 'vendor/autoload.php';

use Slackblocks\{MessagePayload, SectionBlock};
use Slackblocks\Examples\Sending;

$message = new MessagePayload(channel: 'C0123456789', text: 'Hello', blocks: [new SectionBlock('*Hello* from PHP')]);
return static fn(string $token, ?\Psr\Http\Client\ClientInterface $http = null): array => Sending::jolicode($message, $token, $http);
