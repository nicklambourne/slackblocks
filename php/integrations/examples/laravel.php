<?php

declare(strict_types=1);

require 'vendor/autoload.php';

use Slackblocks\SectionBlock;
use Slackblocks\Examples\LaravelTemplate;

$message = LaravelTemplate::message('C0123456789', 'Hello', [new SectionBlock('*Hello* from Laravel')]);
$message->unfurlLinks(false);
return $message;
