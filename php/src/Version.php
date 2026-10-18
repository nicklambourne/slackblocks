<?php

declare(strict_types=1);

namespace Slackblocks;

/** Package and shared contract versions. PHP's first release is not yet published. */
final class Version
{
    /** Coordinated Composer package version, pending first publication. */
    public const PACKAGE = '2.8.0';
    /** Shared Block Kit contract version implemented by this package. */
    public const SPEC = '1.2.0';
}
