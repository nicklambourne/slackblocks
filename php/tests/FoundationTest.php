<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class FoundationTest extends TestCase
{
    public function testNativeMessageAndEdit(): void
    {
        $section = new S\SectionBlock(text: '*Hello*', accessory: new S\ButtonElement(text: 'Open', actionId: 'open'));
        $message = new S\MessagePayload(channel: 'C123', blocks: [$section], unfurlLinks: false);
        self::assertInstanceOf(S\MarkdownText::class, $section->text);
        self::assertSame('*Hello*', $section->text->text);
        self::assertSame('Updated', $section->with(text: 'Updated')->text->text);
        self::assertFalse($message->toArray()['unfurl_links']);
        self::assertEquals($message, S\MessagePayload::fromJson($message->toJson()));
    }

    public function testOpaqueJsonIsDetached(): void
    {
        $source = (object) ['object' => (object) [], 'list' => [], 'zero' => 0, 'null' => null];
        $object = new S\JsonObject($source);
        $source->zero = 2;
        $object->jsonSerialize()->zero = 3;
        self::assertSame('{"object":{},"list":[],"zero":0,"null":null}', $object->toJson());
    }

    public function testPlansKeepPendingTaskContext(): void
    {
        $task = new S\TaskCardBlock(taskId: '1', title: 'Build', status: S\TaskStatus::Pending);
        $plan = new S\PlanBlock(title: 'Work', tasks: [$task]);
        self::assertSame('pending', $plan->toArray()['tasks'][0]->status);
        self::assertObjectNotHasProperty('type', $plan->toArray()['tasks'][0]);
        self::assertEquals($plan, S\PlanBlock::fromJson($plan->toJson()));
        $this->expectException(S\ValidationError::class);
        $task->toJson();
    }

    public function testUnicodeAndErrorCategory(): void
    {
        self::assertSame(str_repeat('🙂', 150), (new S\HeaderBlock(str_repeat('🙂', 150)))->text->text);
        try {
            new S\HeaderBlock(str_repeat('🙂', 151));
            self::fail('accepted');
        } catch (S\ValidationError $e) {
            self::assertSame(S\ErrorCategory::LengthExceeded, $e->category);
            self::assertSame('HeaderBlock.text', $e->path);
        }
    }
}
