<?php

declare(strict_types=1);

namespace Slackblocks;

/** Invalid Slack content, with a stable category and a wire-field path. */
final class ValidationError extends \InvalidArgumentException
{
    public function __construct(public readonly ErrorCategory $category, public readonly string $path, string $message)
    {
        parent::__construct($path . ': ' . $message . ' (' . $category->value . ')');
    }
}
