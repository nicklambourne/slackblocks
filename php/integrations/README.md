# Sending Slackblocks PHP payloads

These runnable examples live in a separate Composer project. The core Slackblocks package has no HTTP or framework dependencies. JoliCode is a community Slack SDK; Laravel provides its own notification channel. Slack does not list an official PHP SDK.

## Run the checks

From this directory:

```sh
composer install
composer test
```

The development project installs the local PHP package as a copied Composer path dependency. Its version is pinned and guarded during release preparation. The independent `php/bin/check-package.py` gate separately proves installation of the actual ZIP outside the checkout.

The suite uses a loopback HTTP server with fake tokens for JoliCode/cURL and Laravel's real notification channel with its supported HTTP fake. It never contacts Slack. It verifies new blocks, attachments, metadata, large integers, false flags, authentication, response errors, redirects, deadlines and bounded rate limiting. The minimum PHP lock uses Laravel 12; an independent current-PHP resolution also exercises the newest compatible Laravel, and a minimum-PHP resolution exercises dependency lower bounds.

## JoliCode SDK

```php
<?php

require 'vendor/autoload.php';

use Slackblocks\{MessagePayload, SectionBlock};
use Slackblocks\Examples\Sending;

$message = new MessagePayload(channel: 'C0123456789', text: 'Hello', blocks: [new SectionBlock('Hello from PHP')]);
// Call deliberately with your application's token; this performs a network request.
$response = Sending::jolicode($message, getenv('SLACK_BOT_TOKEN'));
```

The example passes `blocks`, `attachments` and `metadata` as JSON strings to the SDK's `chatPostMessage` form parameters. It does not hydrate them into generated SDK Block Kit models, so new nested components survive unchanged. An SDK release may reject unknown top-level Web API parameter names; use cURL for a payload with a newer top-level extension that the SDK does not yet recognize.

The implementation is [Sending.php](src/Sending.php), with [RateLimitedClient.php](src/RateLimitedClient.php) providing PSR-18 middleware. Its default client has five-second request/connect limits and disables redirects. Only explicit HTTP 429 responses are retried: at most two retries, and only with a zero- or one-second `Retry-After`. Longer/missing delays fail so the application can schedule a later retry. Other HTTP/API failures and ambiguous network failures are never retried automatically. The maximum default request budget is three five-second requests plus two one-second waits. A caller-supplied PSR-18 client must configure its own deadlines and redirect policy.

## Native cURL

`Sending::curl($message, $token)` posts the complete `toJson()` payload to `https://slack.com/api/chat.postMessage`. It has the same bounded 429 policy, five-second connect/total request limits, and disabled redirects. It requires `ext-curl`, but the core package does not. Override the endpoint only for controlled test servers. Both examples throw on HTTP errors, Slack `ok: false`, malformed responses and exhausted retry budgets.

## Laravel notifications

```php
<?php

use Slackblocks\SectionBlock;
use Slackblocks\Examples\LaravelTemplate;

// Return this from your Notification::toSlack() method.
$message = LaravelTemplate::message('C0123456789', 'Fallback notification text', [new SectionBlock('*Hello* from Laravel')]);
$message->unfurlLinks(false);
```

[LaravelTemplate.php](src/LaravelTemplate.php) uses Laravel's `usingBlockKitTemplate()` API. This is explicitly a **blocks-only** bridge: set channel/fallback and other Laravel notification settings separately. It does not import a complete Slackblocks message or attachments. Laravel's template decoder turns empty JSON objects into arrays; the bridge checks for that change and throws rather than silently corrupting content. Use the full JoliCode/cURL path for empty object-shaped extensions. Laravel handles delivery/error behavior; this bridge adds no retries to its notification channel.

Adapt these example files into your application rather than treating `Slackblocks\Examples` as part of the published core API.
