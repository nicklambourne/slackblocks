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

## Checks

```sh
composer install
python3 generator/generate.py --check
composer test
composer analyse
composer format:check
```

Generated classes come directly from the shared model. The package's tests and development tools are not runtime requirements. See [the implementation notes](IMPLEMENTATION.md) for design and support boundaries.

## License

MIT. Third-party attribution is included in `LICENSE.BSD-3-Clause`.
