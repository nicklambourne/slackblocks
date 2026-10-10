<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class IngressTest extends TestCase
{
    /** @return iterable<string,array{class-string<S\Value>,string}> */
    public static function badShapes(): iterable
    {
        yield 'text number' => [S\PlainText::class,'{"type":"plain_text","text":4}'];
        yield 'boolean string' => [S\PlainText::class,'{"type":"plain_text","text":"x","emoji":"false"}'];
        yield 'integer float' => [S\DateTimePickerElement::class,'{"type":"datetimepicker","initial_date_time":1.5}'];
        yield 'number bool' => [S\RawNumber::class,'{"type":"raw_number","value":true,"text":"1"}'];
        yield 'enum unknown' => [S\AlertBlock::class,'{"type":"alert","text":"x","level":"unknown"}'];
        yield 'metadata list' => [S\MessagePayload::class,'{"channel":"C1","metadata":[]}'];
        yield 'style list' => [S\RichTextText::class,'{"type":"text","text":"x","style":[]}'];
        yield 'style scalar' => [S\RichTextText::class,'{"type":"text","text":"x","style":false}'];
        yield 'style bool only' => [S\RichTextText::class,'{"type":"text","text":"x","style":{"bold":1}}'];
        yield 'list object' => [S\ActionsBlock::class,'{"type":"actions","elements":{}}'];
        yield 'nested list list' => [S\InputBlock::class,'{"type":"input","label":"x","element":[]}'];
        yield 'unknown role type' => [S\InputBlock::class,'{"type":"input","label":"x","element":{"type":"unknown"}}'];
        yield 'root list' => [S\PlainText::class,'[]'];
        yield 'unknown text role' => [S\SectionBlock::class,'{"type":"section","text":{"type":"text","text":"x"}}'];
        yield 'plan task shape' => [S\PlanBlock::class,'{"type":"plan","title":"x","tasks":[false]}'];
    }

    #[DataProvider('badShapes')]
    public function testShapeMismatch(string $class, string $json): void
    {
        try {
            $class::fromJson($json);
            self::fail('accepted bad shape');
        } catch (S\ValidationError $error) {
            self::assertSame(S\ErrorCategory::TypeMismatch, $error->category, $error->getMessage());
        }
    }

    public function testNestedPathsAndDefaultNullOmission(): void
    {
        $message = S\MessagePayload::fromArray(['channel' => 'C1','text' => null,'blocks' => [['type' => 'section','text' => 'coerced']]]);
        self::assertNull($message->text);
        self::assertSame('coerced', $message->blocks[0]->text->text);
        self::assertTrue($message->mrkdwn);
        self::assertNull(S\PlainText::fromArray(['type' => 'plain_text','text' => 'x','emoji' => null])->emoji);
        self::assertSame('x', S\SectionBlock::fromJson('{"type":"section","fields":["x"]}')->fields[0]->text);
        self::assertSame('x', S\HeaderBlock::fromJson('{"type":"header","text":"x"}')->text->text);
        try {
            S\MessagePayload::fromJson('{"channel":"C1","blocks":[{"type":"section","text":""}]}');
            self::fail('empty text accepted');
        } catch (S\ValidationError $error) {
            self::assertSame(S\ErrorCategory::LengthExceeded, $error->category);
            self::assertStringContainsString('blocks[0]', $error->path);
        }
    }
    public function testCyclicWireInputIsRejected(): void
    {
        $block = (object) ['type' => 'container', 'title' => 'x'];
        $block->child_blocks = [$block];
        $this->expectException(S\ValidationError::class);
        S\ContainerBlock::fromArray($block);
    }

}
