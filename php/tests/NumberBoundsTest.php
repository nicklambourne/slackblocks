<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks\{ErrorCategory,NumberInputElement,ValidationError};

final class NumberBoundsTest extends TestCase
{
    public function testWireStringsAndNumericConvenience(): void
    {
        foreach ([0,-2,1.25,'-1.5','10.25'] as $value) {
            $input = new NumberInputElement(isDecimalAllowed: true, minValue: $value, maxValue: $value);
            $wire = json_decode($input->toJson(), true, 512, JSON_THROW_ON_ERROR);
            self::assertSame((string) $value, $wire['min_value']);
            self::assertSame((string) $value, $wire['max_value']);
            self::assertSame((string) $value, NumberInputElement::fromJson($input->toJson())->minValue);
        }
        self::assertNull((new NumberInputElement(isDecimalAllowed: true))->minValue);
    }
    public function testInvalidAndInvertedBounds(): void
    {
        foreach ([INF,NAN,'invalid',''] as $value) {
            try {
                new NumberInputElement(isDecimalAllowed: true, minValue: $value);
                self::fail('accepted invalid bound');
            } catch (ValidationError $error) {
                self::assertNotEmpty($error->getMessage());
            }
        }
        $this->expectException(ValidationError::class);
        new NumberInputElement(isDecimalAllowed: true, minValue: '10', maxValue: '2');
    }
    public function testExactDecimalComparison(): void
    {
        foreach ([
            ['0.10000000000000001', '0.1'],
            ['9007199254740993', '9007199254740992'],
            ['-0.1', '-0.10000000000000001'],
            ['1e-1000', '9e-1001'],
            ['1.00000000000000001e309', '1e309'],
            ['.0000000000000000001', '0'],
            ['0', '-1e-1000'],
            ['10e-999999999999999999999', '9e-999999999999999999999'],
            ['1e999999999999999999999', '9e999999999999999999998'],
        ] as [$higher, $lower]) {
            try {
                new NumberInputElement(isDecimalAllowed: true, minValue: $higher, maxValue: $lower);
                self::fail('accepted inverted precise bounds');
            } catch (ValidationError $error) {
                self::assertSame(ErrorCategory::OutOfRange, $error->category);
                self::assertSame('NumberInputElement', $error->path);
            }
            $input = new NumberInputElement(isDecimalAllowed: true, minValue: $lower, maxValue: $higher);
            self::assertSame($lower, $input->minValue);
            self::assertSame($higher, $input->maxValue);
            self::assertEquals($input, NumberInputElement::fromJson($input->toJson()));
        }
    }

    public function testEquivalentSpellingsAndOmission(): void
    {
        foreach ([
            ['0.1000', '1e-1'], ['-0.00', '+0e99'], ['0010.0', '+1.e1'],
            ['-.50', '-5e-1'], ['10e-999999999999999999999', '1e-999999999999999999998'],
            ['10e999999999999999999999', '1e1000000000000000000000'],
        ] as [$left, $right]) {
            foreach ([[$left, $right], [$right, $left]] as [$minimum, $maximum]) {
                $input = new NumberInputElement(isDecimalAllowed: true, minValue: $minimum, maxValue: $maximum);
                $wire = json_decode($input->toJson(), true, 512, JSON_THROW_ON_ERROR);
                self::assertSame($minimum, $wire['min_value']);
                self::assertSame($maximum, $wire['max_value']);
            }
        }
        self::assertArrayNotHasKey('min_value', json_decode((new NumberInputElement(isDecimalAllowed: true, maxValue: 0))->toJson(), true, 512, JSON_THROW_ON_ERROR));
        self::assertArrayNotHasKey('max_value', json_decode((new NumberInputElement(isDecimalAllowed: true, minValue: 0))->toJson(), true, 512, JSON_THROW_ON_ERROR));
    }

    public function testDecimalGrammar(): void
    {
        foreach (['NaN', 'Infinity', '0x10', '1_0', ' 1 ', '1e', '.', "1\n"] as $value) {
            try {
                new NumberInputElement(isDecimalAllowed: true, minValue: $value);
                self::fail('accepted invalid decimal grammar');
            } catch (ValidationError $error) {
                self::assertSame(ErrorCategory::TypeMismatch, $error->category);
                self::assertSame('NumberInputElement.min_value', $error->path);
            }
        }
    }
    public function testNativeNonfiniteBoundsFailBeforeStringCoercion(): void
    {
        foreach ([INF, -INF, NAN] as $value) {
            foreach (['minValue', 'maxValue'] as $field) {
                try {
                    new NumberInputElement(...['isDecimalAllowed' => true, $field => $value]);
                    self::fail('accepted nonfinite native bound');
                } catch (ValidationError $error) {
                    self::assertSame(ErrorCategory::TypeMismatch, $error->category);
                    self::assertSame('NumberInputElement.' . ($field === 'minValue' ? 'min_value' : 'max_value'), $error->path);
                }
            }
        }
    }

    public function testNativeFloatsRetainTheirValuesRegardlessOfRuntimePrecision(): void
    {
        $precision = ini_get('precision');
        $serializePrecision = ini_get('serialize_precision');
        $locale = setlocale(LC_NUMERIC, '0');
        try {
            foreach (['C', 'fr_FR.UTF-8', 'de_DE.UTF-8'] as $candidateLocale) {
                if (setlocale(LC_NUMERIC, $candidateLocale) === false) {
                    continue;
                }
                foreach ([[14, -1], [3, 3], [17, 17]] as [$digits, $serializeDigits]) {
                    ini_set('precision', (string) $digits);
                    ini_set('serialize_precision', (string) $serializeDigits);
                    foreach ([0.0, -0.0, 0.1, 0.1234567890123456, 1.0000000000000002, -1.0000000000000002, PHP_FLOAT_MIN, 5e-324, PHP_FLOAT_MAX] as $value) {
                        $input = new NumberInputElement(true, minValue: $value, maxValue: $value);
                        self::assertSame($value, (float) $input->minValue);
                        self::assertSame($value, (float) $input->maxValue);
                        self::assertSame(pack('d', $value), pack('d', (float) $input->minValue));
                        self::assertEquals($input, NumberInputElement::fromJson($input->toJson()));
                    }
                    $tenth = new NumberInputElement(true, minValue: 0.1, maxValue: '0.1');
                    self::assertSame('0.1', $tenth->minValue);
                    foreach ([[10.0, '10'], [100.0, '100'], [0.0001, '0.0001'], [0.00001, '0.00001']] as [$value, $expected]) {
                        $input = new NumberInputElement(true, minValue: $value, maxValue: $expected);
                        self::assertSame($expected, $input->minValue);
                    }
                    self::assertSame((string) $digits, ini_get('precision'));
                    self::assertSame((string) $serializeDigits, ini_get('serialize_precision'));
                    try {
                        new NumberInputElement(true, minValue: 1.0000000000000002, maxValue: 1.0);
                        self::fail('accepted inverted adjacent native floats');
                    } catch (ValidationError $error) {
                        self::assertSame(ErrorCategory::OutOfRange, $error->category);
                        self::assertSame('NumberInputElement', $error->path);
                    }
                }
            }
        } finally {
            ini_set('precision', $precision);
            ini_set('serialize_precision', $serializePrecision);
            if ($locale !== false) {
                setlocale(LC_NUMERIC, $locale);
            }
        }
    }
}
