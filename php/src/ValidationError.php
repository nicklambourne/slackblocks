<?php

declare(strict_types=1);

namespace Slackblocks;

/** Invalid Slack content, with a stable category and a wire-field path. */
final class ValidationError extends \InvalidArgumentException
{
    /** Describes a validation failure without losing its machine-readable category/path.
     * @param ErrorCategory $category Stable shared classification of the failure.
     * @param string $path Failing wire-field path, including collection indexes when applicable.
     * @param string $message Human-readable explanation of the violated rule.
     */
    public function __construct(
        /** Stable shared classification of the validation failure. */
        public readonly ErrorCategory $category,
        /** Failing wire-field path, including collection indexes when applicable. */
        public readonly string $path,
        string $message,
    ) {
        parent::__construct($path . ': ' . $message . ' (' . $category->value . ')');
    }
}
