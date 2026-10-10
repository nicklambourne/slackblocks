<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class ConformanceTest extends TestCase
{
    private const ROOT = __DIR__ . '/../..';

    /** @return array<string, array{\Closure(): S\Value}> */
    public static function validCases(): array
    {
        return array_map(static fn(\Closure $build): array => [$build], ValidConstructions::all());
    }

    #[DataProvider('validCases')]
    public function testValidConstruction(\Closure $build): void
    {
        $expected = json_decode(file_get_contents(self::ROOT . '/spec/fixtures/valid/' . $this->dataName() . '.json'), flags: JSON_THROW_ON_ERROR);
        $value = $build();
        self::assertEquals($expected, json_decode($value->toJson(), flags: JSON_THROW_ON_ERROR));
        self::assertEquals($value, $value::fromJson($value->toJson()));
        self::assertEquals($value, $value->with());
    }

    /** @return array<string, array{\Closure(): S\Value, string}> */
    public static function invalidCases(): array
    {
        $manifest = json_decode(file_get_contents(self::ROOT . '/spec/fixtures/invalid/manifest.json'), true, flags: JSON_THROW_ON_ERROR);
        $categories = array_column($manifest['cases'], 'category', 'id');
        $cases = [];
        foreach (InvalidConstructions::all() as $id => $build) {
            $cases[$id] = [$build, $categories[$id]];
        }
        return $cases;
    }

    #[DataProvider('invalidCases')]
    public function testInvalidConstruction(\Closure $build, string $category): void
    {
        try {
            $build()->toJson();
            self::fail('Invalid construction accepted: ' . $this->dataName());
        } catch (S\ValidationError $error) {
            self::assertSame($category, $error->category->value, $error->getMessage());
            self::assertNotSame('', $error->path);
        }
    }

    public function testCompleteRegistriesAndLimits(): void
    {
        self::assertSame('', trim(file_get_contents(__DIR__ . '/../conformance/skiplist.txt')));
        $manifest = json_decode(file_get_contents(self::ROOT . '/spec/manifest.json'), true, flags: JSON_THROW_ON_ERROR);
        self::assertSame(S\Version::SPEC, $manifest['spec_version']);
        self::assertEqualsCanonicalizing(array_column($manifest['fixtures'], 'id'), array_keys(ValidConstructions::all()));
        $invalid = json_decode(file_get_contents(self::ROOT . '/spec/fixtures/invalid/manifest.json'), true, flags: JSON_THROW_ON_ERROR);
        self::assertEqualsCanonicalizing(array_column($invalid['cases'], 'id'), array_keys(InvalidConstructions::all()));
        $limits = json_decode(file_get_contents(self::ROOT . '/spec/limits.json'), true, flags: JSON_THROW_ON_ERROR);
        $flat = [];
        $flatten = function (array $node, string $prefix = '') use (&$flatten, &$flat): void {
            foreach ($node as $key => $value) {
                $path = $prefix === '' ? $key : $prefix . '.' . $key;
                if (is_array($value)) {
                    $flatten($value, $path);
                } else {
                    $flat[$path] = $value;
                }
            }
        };
        $flatten($limits);
        self::assertSame($flat, S\Internal\Schema::LIMITS);
        self::assertSame([], array_values(array_diff(array_keys($flat), array_column($invalid['cases'], 'constraint'))), 'Every scalar limit needs an executed invalid construction');
        self::assertSame(json_decode(file_get_contents(self::ROOT . '/spec/vocabulary.json'), true, flags: JSON_THROW_ON_ERROR), S\Internal\Schema::VOCABULARY);
    }
}
