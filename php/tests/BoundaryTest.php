<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class BoundaryTest extends TestCase
{
    public function testReferencedBlocksCannotChangeValidatedMessagesOrEdits(): void
    {
        $block = new S\SectionBlock('Original');
        $message = new S\MessagePayload('C1', blocks: [&$block]);
        $copy = $message->with(channel: 'C2');
        $edited = $message->with(blocks: [&$block]);
        $expected = $message->toJson();
        $expectedCopy = $copy->toJson();
        $block = new S\InputBlock('Invalid on messages', new S\PlainTextInputElement());
        self::assertSame($expected, $message->toJson());
        self::assertSame($expectedCopy, $copy->toJson());
        self::assertSame($expected, $edited->toJson());
        $export = $message->blocks;
        $export[0] = new S\DividerBlock();
        self::assertSame($expected, $message->toJson());
    }

    public function testNestedRowsAndStringListsDetachTheirReferences(): void
    {
        $cell = new S\RawText('Original');
        $row = [&$cell];
        $table = new S\TableBlock([&$row]);
        $expected = $table->toJson();
        $cell = new S\RawText('Changed cell');
        self::assertSame($expected, $table->toJson());
        $row = [new S\RawText('Changed row')];
        self::assertSame($expected, $table->toJson());
        $export = $table->rows;
        $export[0][0] = new S\RawText('Changed export');
        self::assertSame($expected, $table->toJson());
        $category = 'Original';
        $axis = new S\AxisConfig([&$category]);
        $category = 'Changed';
        self::assertSame(['Original'], $axis->categories);
        $text = new S\RichTextText('Original');
        $section = new S\RichTextSection([&$text]);
        $text = new S\RichTextText('Changed');
        self::assertSame('Original', $section->elements[0]->text);
    }

    public function testTextListsNormalizeOnlyStringsAndDetachReferences(): void
    {
        $text = 'Original';
        $plain = new S\PlainText('Plain');
        $section = new S\SectionBlock(fields: [&$text, $plain]);
        $text = 'Changed';
        self::assertSame('Original', $section->fields[0]->text);
        self::assertSame($plain, $section->fields[1]);
        self::assertSame([], (new S\MessagePayload('C1', blocks: []))->blocks);
        self::assertNull((new S\SectionBlock('Text'))->fields);
    }

    /** @return iterable<string, array{mixed}> */
    public static function invalidTextItems(): iterable
    {
        yield 'integer' => [42];
        yield 'float' => [1.5];
        yield 'boolean' => [true];
        yield 'null' => [null];
        yield 'array' => [[]];
        yield 'wrong value' => [new S\DividerBlock()];
    }

    #[DataProvider('invalidTextItems')]
    public function testTextListsRejectWrongTypesInConstructionAndEdits(mixed $item): void
    {
        foreach ([fn() => new S\SectionBlock(fields: [$item]), fn() => (new S\SectionBlock('Original'))->with(fields: [$item])] as $construct) {
            try {
                $construct();
                self::fail('Accepted a non-text collection item');
            } catch (S\ValidationError $error) {
                self::assertSame(S\ErrorCategory::TypeMismatch, $error->category);
                self::assertSame('SectionBlock.fields[0]', $error->path);
            }
        }
    }

    public function testNormalizationPreservesInvalidKeysForValidation(): void
    {
        foreach ([fn() => new S\MessagePayload('C1', blocks: [2 => new S\DividerBlock()]), fn() => new S\TableBlock([2 => [new S\RawText('A')]]), fn() => new S\TableBlock([[2 => new S\RawText('A')]]), fn() => new S\SectionBlock(fields: [2 => 'A'])] as $construct) {
            try {
                $construct();
                self::fail('Reindexed an invalid list');
            } catch (S\ValidationError $error) {
                self::assertSame(S\ErrorCategory::TypeMismatch, $error->category);
            }
        }
        $cycle = [];
        $cycle[] = &$cycle;
        $this->expectException(S\ValidationError::class);
        new S\TableBlock($cycle);
    }

    /** @return iterable<string, array{string, string, bool}> */
    public static function invalidJsonNumbers(): iterable
    {
        foreach (['9223372036854775808', '-9223372036854775809', '1e-999', '-1e-999', '1e999'] as $number) {
            foreach (['plain' => 'a', 'escaped' => '\\"'] as $name => $padding) {
                foreach ([true, false] as $before) {
                    yield $number . '/' . $name . '/' . (int) $before => [$number, $padding, $before];
                }
            }
        }
    }

    #[DataProvider('invalidJsonNumbers')]
    public function testLongJsonCannotBypassNumberBounds(string $number, string $padding, bool $before): void
    {
        $string = '"padding":"' . str_repeat($padding, 10000) . '"';
        $numeric = '"number":' . $number;
        $json = '{' . ($before ? $string . ',' . $numeric : $numeric . ',' . $string) . '}';
        foreach ([fn() => S\JsonObject::fromJson($json), fn() => S\MessagePayload::fromJson('{"channel":"C1","metadata":' . $json . '}')] as $decode) {
            try {
                $decode();
                self::fail('Silently changed an out-of-range JSON number');
            } catch (S\ValidationError $error) {
                self::assertSame(S\ErrorCategory::OutOfRange, $error->category);
                self::assertSame('$', $error->path);
            }
        }
    }

    public function testLongValidJsonPreservesStringsAndBoundaryNumbers(): void
    {
        $data = ['text' => str_repeat('"\\ 9223372036854775808 1e-999 🙂', 1000), 'min' => PHP_INT_MIN, 'max' => PHP_INT_MAX, 'integer' => 9007199254740993, 'float' => 1.5e-300, 'list' => [false, null, 0, -2, 1.0]];
        $json = json_encode($data, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION);
        self::assertSame($data, S\JsonObject::fromJson($json)->toArray());
        self::assertSame($data, S\MessagePayload::fromJson('{"channel":"C1","metadata":' . $json . '}')->metadata->toArray());
        self::assertSame(0.0, S\JsonObject::fromJson('{"n":0e-999}')->toArray()['n']);
        self::assertSame([], S\JsonObject::fromJson('{}')->toArray());
    }
}
