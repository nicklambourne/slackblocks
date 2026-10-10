<?php

declare(strict_types=1);

namespace Slackblocks;

/** Attachment color conveniences. Pass these strings to Attachment's color argument. */
final class Color
{
    /** Slack's positive semantic color. */
    public const GOOD = 'good';
    /** Slack's warning semantic color. */
    public const WARNING = 'warning';
    /** Slack's negative semantic color. */
    public const DANGER = 'danger';
    /** Red (#ff0000). */
    public const RED = '#ff0000';
    /** Blue (#0000ff). */
    public const BLUE = '#0000ff';
    /** Yellow (#ffff00). */
    public const YELLOW = '#ffff00';
    /** Green (#00ff00). */
    public const GREEN = '#00ff00';
    /** Orange (#ff8800). */
    public const ORANGE = '#ff8800';
    /** Purple (#8800ff). */
    public const PURPLE = '#8800ff';
    /** Black (#000000). */
    public const BLACK = '#000000';

    /** Validates six hexadecimal digits and adds one leading # if absent. */
    public static function hex(string $value): string
    {
        $hex = str_starts_with($value, '#') ? substr($value, 1) : $value;
        if (preg_match('/\A[0-9a-fA-F]{6}\z/', $hex) !== 1) {
            throw new ValidationError(ErrorCategory::TypeMismatch, 'Color.hex', 'expected six hexadecimal digits');
        }
        return '#' . $hex;
    }
}
