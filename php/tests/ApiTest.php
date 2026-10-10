<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class ApiTest extends TestCase
{
    public function testIndependentPublicInventoryAndCapabilityCoverage(): void
    {
        $actual = [];
        foreach (glob(__DIR__ . '/../src/*.php') as $file) {
            $name = pathinfo($file, PATHINFO_FILENAME);
            $actual[$name] = new \ReflectionClass('Slackblocks\\' . $name);
        }
        $map = json_decode(file_get_contents(__DIR__ . '/../conformance/capabilities.json'), true, flags: JSON_THROW_ON_ERROR);
        $support = json_decode(file_get_contents(__DIR__ . '/../conformance/supporting-api.json'), true, flags: JSON_THROW_ON_ERROR);
        self::assertEqualsCanonicalizing(array_keys($actual), array_merge(array_keys($map), array_keys($support)), 'Every public class, helper, enum and role needs a deliberate classification');
        $coverage = json_decode(file_get_contents(__DIR__ . '/../../spec/coverage.json'), true, flags: JSON_THROW_ON_ERROR)['capabilities'];
        self::assertEqualsCanonicalizing(array_keys($coverage), array_values(array_unique($map)));
        $seen = [];
        $byFixture = [];
        $visit = function (mixed $value) use (&$visit, &$seen): void {
            if ($value instanceof S\Value) {
                $seen[(new \ReflectionClass($value))->getShortName()] = true;
                foreach (get_object_vars($value) as $field) {
                    $visit($field);
                }
            } elseif (is_array($value)) {
                foreach ($value as $field) {
                    $visit($field);
                }
            }
        };
        foreach (ValidConstructions::all() as $id => $build) {
            $seen = [];
            $visit($build());
            $byFixture[$id] = $seen;
        }
        $seen = array_merge(...array_values($byFixture));
        foreach ($map as $name => $capability) {
            self::assertTrue($actual[$name]->isFinal(), $name);
            self::assertTrue($actual[$name]->isReadOnly(), $name);
            self::assertArrayHasKey($name, $seen, 'A native construction must reach ' . $name);
            self::assertArrayHasKey($capability, $coverage);
            $reached = false;
            foreach ($coverage[$capability] as $id) {
                $reached = $reached || isset($byFixture[$id][$name]);
            }
            self::assertTrue($reached, $name . ' must appear in a fixture assigned to its capability');
            foreach ($coverage[$capability] as $id) {
                self::assertArrayHasKey($id, ValidConstructions::all());
            }
        }
        foreach ($support as $reason) {
            self::assertNotSame('', $reason);
        }
        $command = escapeshellarg(PHP_BINARY) . ' ' . escapeshellarg(__DIR__ . '/../bin/api-inventory.php');
        exec($command, $lines, $code);
        self::assertSame(0, $code);
        self::assertSame(json_decode(file_get_contents(__DIR__ . '/../conformance/api-inventory.json'), true, flags: JSON_THROW_ON_ERROR), json_decode(implode("\n", $lines), true, flags: JSON_THROW_ON_ERROR), 'Public signatures changed: review and update the independent inventory');
    }
}
