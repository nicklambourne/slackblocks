# Slackblocks for PHP

Native, immutable Slack Block Kit values for **PHP 8.2+ (64-bit)**. PHP support is being prepared for Slackblocks **2.7.0** and is not published yet. The core requires JSON and PCRE, with no HTTP or framework dependencies.

## Development quickstart

From this repository's `php` directory, run `composer install`. Then:

```php
<?php

declare(strict_types=1);

require 'vendor/autoload.php';

use Slackblocks\{ButtonElement, MessagePayload, SectionBlock};

$message = new MessagePayload(
    channel: 'C0123456789',
    text: 'A deployment is ready',
    blocks: [new SectionBlock(
        text: '*Ready to deploy*',
        accessory: new ButtonElement(text: 'Deploy', actionId: 'deploy'),
    )],
);

echo $message->toJson();
```

Use named arguments and `declare(strict_types=1)` in calling files. PHP's own type errors reject invalid constructor shapes; `ValidationError` carries a shared `ErrorCategory` and wire-field path for invalid content. `fromArray()` and `fromJson()` provide checked ingress for untrusted payloads.

Properties are readonly. `$section->with(text: 'Updated')` creates a validated copy. Optional `null` fields are omitted; `false`, `0` and empty lists remain. `JsonObject` preserves empty objects and explicit nulls in application metadata, copying nested mutable input and exported objects. Block IDs are never generated automatically.

## Documentation and sending

- [PHP API reference](https://nicklambourne.github.io/slackblocks/reference/php)
- [Installation and support](https://nicklambourne.github.io/slackblocks/usage/installation?language=php)
- [Recipes](https://nicklambourne.github.io/slackblocks/usage/cookbook?language=php)
- [Tested JoliCode, native cURL and Laravel adapters](https://github.com/nicklambourne/slackblocks/tree/master/php/integrations)

The published package will be `nicklambourne/slackblocks` on Packagist. Until its
first release, use the checkout instructions above. HTTP examples are a separate
project; `Slackblocks\Examples` is not a core namespace. Full payloads work through
JoliCode/cURL; Laravel's template bridge accepts blocks and checks for lossy JSON.

## Checks

```sh
composer install
python3 generator/generate.py --check
composer test
composer analyse
composer format:check
python3 generator/test_generate.py
python3 bin/check-types.py
python3 bin/check-package.py
```

Generated classes come directly from the shared model. The package's tests and development tools are not runtime requirements. See [the implementation notes](IMPLEMENTATION.md) for design and support boundaries.

## Conformance and quality

The suite constructs all 105 valid shared fixtures and rejects all 167 invalid cases with the required categories. It checks every scalar limit, vocabulary, capability and exported class. The release skiplist is empty. Public constructors, enums, readonly fields and helper methods are independently inventoried.

PHPStan level 8 checks the library and positive/negative consumers. CI enforces at least 95% line coverage for handwritten code and reports generated coverage separately. Package checks install the actual Composer ZIP in a fresh external project on each supported platform. There are no third-party runtime dependencies to resolve at lower bounds; the development lock is resolved on PHP 8.2.

## Helpers

`AccordionSection::create()` produces a collapsible `ContainerBlock`; `Accordion::create()` returns sections to spread into a message's blocks. `Paginator::create()` takes one-based page numbers and returns content plus navigation controls. Controls count toward the enclosing surface's block limit. `Builder::url()` makes a preview URL. `Workflow::fromUrl()` accepts optional `InputParameter` values. `Color` provides semantic/hex strings for attachments.

## License

MIT. Third-party attribution is included in `LICENSE.BSD-3-Clause`.
