<?php

declare(strict_types=1);

namespace Slackblocks;

/** Package and shared contract versions. PHP's first release is not yet published. */
final class Version
{
    /** Coordinated Composer package version, pending first publication. */
    public const PACKAGE = '3.1.0';
    /** Shared Block Kit contract version implemented by this package. */
    public const SPEC = '1.5.0';
}
