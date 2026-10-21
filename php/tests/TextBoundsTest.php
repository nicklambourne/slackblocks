<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks\{PlainTextInputElement, ValidationError, ErrorCategory};

final class TextBoundsTest extends TestCase
{
    public function testSatisfiableAndOmittedBounds(): void
    {
        foreach ([[null, null], [20, null], [null, 10], [0, 1], [10, 10], [3000, 3000]] as [$min, $max]) {
            $value = new PlainTextInputElement(minLength: $min, maxLength: $max);
            self::assertSame($min, $value->minLength);
            self::assertSame($max, $value->maxLength);
        }
        try {
            new PlainTextInputElement(minLength: 20, maxLength: 10);
            self::fail('accepted inverted bounds');
        } catch (ValidationError $error) {
            self::assertSame(ErrorCategory::OutOfRange, $error->category);
            self::assertSame('PlainTextInputElement.min_length', $error->path);
        }
    }
    public function testStructuralControlsRemainAccepted(): void
    {
        self::assertSame(0, (new \Slackblocks\ExternalSelectElement(minQueryLength: 0))->minQueryLength);
        self::assertSame(0, (new \Slackblocks\ExternalMultiSelectElement(minQueryLength: 0))->minQueryLength);
        foreach (['2028-02-29', '2028-02-30'] as $date) {
            self::assertSame($date, (new \Slackblocks\DatePickerElement(initialDate: $date))->initialDate);
        }
        foreach (['00:00', '23:59'] as $time) {
            self::assertSame($time, (new \Slackblocks\TimePickerElement(initialTime: $time))->initialTime);
        }
        self::assertSame('', (new \Slackblocks\ImageElement(imageUrl: 'https://example.com/image.png', altText: ''))->altText);
    }
}
