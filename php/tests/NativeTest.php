<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Slackblocks as S;

final class NativeTest extends TestCase
{
    /** @return iterable<string, array{\Closure, S\ErrorCategory, string}> */
    public static function rejected(): iterable
    {
        $mismatch = S\ErrorCategory::TypeMismatch;
        $range = S\ErrorCategory::OutOfRange;
        $usage = S\ErrorCategory::InvalidUsage;
        $missing = S\ErrorCategory::MissingRequired;
        yield 'invalid UTF-8' => [fn() => new S\PlainText("\xff"), $mismatch, 'PlainText.text'];
        yield 'nonfinite' => [fn() => new S\RawNumber(INF, 'infinite'), $range, 'RawNumber.value'];
        yield 'nan' => [fn() => new S\RawNumber(NAN, 'nan'), $range, 'RawNumber.value'];
        yield 'overflow JSON' => [fn() => S\RawNumber::fromJson('{"type":"raw_number","value":9223372036854775808,"text":"big"}'), $range, '$'];
        yield 'negative overflow JSON' => [fn() => S\JsonObject::fromJson('{"n":-9223372036854775809}'), $range, '$'];
        yield 'double overflow JSON' => [fn() => S\JsonObject::fromJson('{"n":1e999}'), $range, '$'];
        yield 'double underflow JSON' => [fn() => S\JsonObject::fromJson('{"n":1e-999}'), $range, '$'];
        yield 'syntax JSON' => [fn() => S\PlainText::fromJson('{'), $mismatch, '$'];
        yield 'scalar root' => [fn() => S\PlainText::fromJson('false'), $mismatch, 'PlainText'];
        yield 'wrong discriminator' => [fn() => S\PlainText::fromJson('{"type":"mrkdwn","text":"x"}'), $mismatch, 'PlainText.type'];
        yield 'required null' => [fn() => S\PlainText::fromArray(['type' => 'plain_text','text' => null]), $missing, 'PlainText.text'];
        yield 'nested list keys' => [fn() => new S\ActionsBlock([2 => new S\ButtonElement('A')]), $mismatch, 'ActionsBlock.elements'];
        yield 'nested wrong role' => [fn() => new S\ActionsBlock([new S\DividerBlock()]), $mismatch, 'ActionsBlock.elements[0]'];
        yield 'wrong table row' => [fn() => new S\TableBlock([new S\RawText('x')]), $mismatch, 'TableBlock.rows[0]'];
        yield 'style owner' => [fn() => new S\RichTextText('x', style: new S\RichTextStyle(unlink: false)), $mismatch, 'RichTextText.style.unlink'];
        yield 'style flag' => [fn() => S\RichTextText::fromArray(['type' => 'text','text' => 'x','style' => ['unknown' => true]]), $mismatch, 'RichTextText.style'];
        yield 'reserved extension' => [fn() => new S\PlainText('x', extensions: new S\JsonObject(['text' => 'replacement'])), $usage, 'PlainText.text'];
        yield 'reserved type' => [fn() => new S\PlainText('x', extensions: new S\JsonObject(['type' => 'changed'])), $usage, 'PlainText.type'];
        yield 'empty extension' => [fn() => new S\PlainText('x', extensions: new S\JsonObject(['' => 1])), $usage, 'PlainText.'];
        yield 'payload discriminator' => [fn() => S\MessagePayload::fromArray(['channel' => 'C1','type' => 'message']), $usage, 'MessagePayload.type'];
        yield 'file full anchor' => [fn() => new S\SlackFile(id: "F012345678\n"), $mismatch, 'SlackFile.id'];
        yield 'color full anchor' => [fn() => new S\Attachment([], color: "#abcdef\n"), $mismatch, 'Attachment.color'];
        yield 'color malformed' => [fn() => S\Color::hex('##ffffff'), $mismatch, 'Color.hex'];
        yield 'empty card' => [fn() => new S\CardBlock(), $missing, 'CardBlock'];
        yield 'card icons' => [fn() => new S\CardBlock(title: 'x', icon: new S\ImageElement('alt', imageUrl: 'https://example.org'), slackIcon: new S\SlackIcon('bot')), S\ErrorCategory::MutuallyExclusive, 'CardBlock'];
        yield 'container title' => [fn() => new S\ContainerBlock([new S\DividerBlock()]), $missing, 'ContainerBlock'];
        yield 'collapsed dependency' => [fn() => new S\ContainerBlock([new S\DividerBlock()], title: 'x', defaultCollapsed: true), $usage, 'ContainerBlock.default_collapsed'];
        yield 'header divider dependency' => [fn() => new S\ContainerBlock([new S\DividerBlock()], title: 'x', isCollapsible: true, hasHeaderDivider: true), $usage, 'ContainerBlock.has_header_divider'];
        yield 'unknown icon' => [fn() => new S\SlackIcon('not-an-icon'), $mismatch, 'SlackIcon.name'];
        yield 'empty filter' => [fn() => new S\ConversationFilter(), $missing, 'ConversationFilter'];
        yield 'file missing source' => [fn() => new S\SlackFile(), S\ErrorCategory::MutuallyExclusive, 'SlackFile'];
        yield 'file duplicate source' => [fn() => new S\SlackFile(id: 'F012345678', url: 'https://example.org'), S\ErrorCategory::MutuallyExclusive, 'SlackFile'];
        yield 'table ragged' => [fn() => new S\DataTableBlock([[new S\RawText('A')],[new S\RawText('B'),new S\RawText('C')]], caption: 'Table'), $usage, 'DataTableBlock.rows[1]'];
        yield 'table rich header' => [fn() => new S\DataTableBlock([[new S\RichTextBlock([])],[new S\RawText('A')]], caption: 'Table'), $mismatch, 'DataTableBlock.rows[0][0]'];
        yield 'table empty number text' => [fn() => new S\DataTableBlock([[new S\RawText('A')],[new S\RawNumber(1, '')]], caption: 'Table'), S\ErrorCategory::LengthExceeded, 'DataTableBlock.rows[1][0].text'];
        yield 'table unsupported settings' => [fn() => new S\DataTableBlock([[new S\RawText('A')],[new S\RawText('B')]], caption: 'Table', extensions: new S\JsonObject(['column_settings' => []])), $usage, 'DataTableBlock.column_settings'];
        yield 'duplicate categories' => [fn() => new S\AxisConfig(['A','A']), $usage, 'AxisConfig.categories'];
        yield 'duplicate series' => [fn() => new S\LineChart(axisConfig: new S\AxisConfig(['A']), series: [new S\DataSeries('same', [new S\DataPoint('A', 1)]),new S\DataSeries('same', [new S\DataPoint('A', 2)])]), $usage, 'LineChart.series'];
        yield 'missing category' => [fn() => new S\BarChart(axisConfig: new S\AxisConfig(['A','B']), series: [new S\DataSeries('x', [new S\DataPoint('A', 1)])]), $usage, 'BarChart.series[0].data'];
        yield 'attachment surface' => [fn() => new S\Attachment([new S\AlertBlock('Modal only')]), $mismatch, 'Attachment.blocks[0].type'];
        yield 'pending nested in container' => [fn() => new S\ContainerBlock([new S\TaskCardBlock('1', 'Task', S\TaskStatus::Pending)], title: 'x'), $mismatch, 'TaskCardBlock.status'];
        yield 'paginator empty' => [fn() => S\Paginator::create('page', []), $missing, 'Paginator.blocks'];
        yield 'paginator prefix' => [fn() => S\Paginator::create('', [new S\DividerBlock()]), $missing, 'Paginator.action_id_prefix'];
        yield 'paginator zero' => [fn() => S\Paginator::create('page', [new S\DividerBlock()], page: 0), $range, 'Paginator'];
        yield 'paginator size' => [fn() => S\Paginator::create('page', [new S\DividerBlock()], pageSize: 0), $range, 'Paginator'];
        yield 'paginator page' => [fn() => S\Paginator::create('page', [new S\DividerBlock()], page: 2), $range, 'Paginator.page'];
        yield 'paginator wrong block' => [fn() => S\Paginator::create('page', [new S\PlainText('x')]), $mismatch, 'Paginator.blocks[0]'];
        yield 'paginator associative list' => [fn() => S\Paginator::create('page', [2 => new S\DividerBlock()]), $mismatch, 'Paginator.blocks'];
        yield 'accordion empty' => [fn() => S\Accordion::create([]), $missing, 'Accordion.sections'];
        yield 'accordion noncollapsible' => [fn() => S\Accordion::create([new S\ContainerBlock([new S\DividerBlock()], title: 'x')]), $mismatch, 'Accordion.sections[0]'];
        yield 'accordion list' => [fn() => S\Accordion::create([3 => S\AccordionSection::create('x', [new S\DividerBlock()])]), $mismatch, 'Accordion.sections'];
        yield 'builder role' => [fn() => S\Builder::url([new S\PlainText('x')]), $mismatch, 'Builder.blocks'];
        yield 'builder list' => [fn() => S\Builder::url([2 => new S\DividerBlock()]), $mismatch, 'Builder.blocks'];
        yield 'opaque list root' => [fn() => S\JsonObject::fromJson('[]'), $mismatch, 'JsonObject'];
        yield 'opaque arbitrary object' => [fn() => new S\JsonObject(['x' => new \DateTimeImmutable()]), $mismatch, 'JsonObject.x'];
        yield 'opaque resource' => [fn() => new S\JsonObject(['x' => STDIN]), $mismatch, 'JsonObject.x'];
        yield 'opaque nonfinite' => [fn() => new S\JsonObject(['x' => -INF]), $range, 'JsonObject.x'];
        yield 'opaque UTF8 key' => [fn() => new S\JsonObject(["\xff" => 1]), $mismatch, 'JsonObject'];
    }

    #[DataProvider('rejected')]
    public function testRejected(\Closure $build, S\ErrorCategory $category, string $path): void
    {
        try {
            $build();
            self::fail('accepted invalid value');
        } catch (S\ValidationError $error) {
            self::assertSame($category, $error->category, $error->getMessage());
            self::assertSame($path, $error->path);
        }
    }

    public function testNullFalseZeroAndEmptyValues(): void
    {
        $value = new S\MessagePayload('C1', blocks: [], text: null, mrkdwn: false, metadata: new S\JsonObject());
        $wire = json_decode($value->toJson(), true);
        self::assertArrayNotHasKey('text', $wire);
        self::assertSame([], $wire['blocks']);
        self::assertFalse($wire['mrkdwn']);
        self::assertStringContainsString('"metadata":{}', $value->toJson());
        self::assertSame(0, (new S\RawNumber(0, 'zero'))->value);
        self::assertFalse((new S\ConversationFilter(excludeBotUsers: false))->excludeBotUsers);
        self::assertSame([], (new S\TableBlock([[]]))->rows[0]);
        self::assertSame(S\ResponseType::InChannel, (new S\MessageResponse())->responseType);
        self::assertFalse((new S\MessageResponse())->replaceOriginal);
    }

    public function testOpaqueFieldsNeverBecomeModeledContent(): void
    {
        $opaque = new S\JsonObject(['type' => 'task_card','status' => 'pending','blocks' => [['type' => 'markdown','text' => str_repeat('x', 20000)]], 'numeric' => (object) ['0' => 'zero']]);
        $message = new S\MessagePayload('C1', metadata: $opaque, extensions: new S\JsonObject(['app' => $opaque->jsonSerialize()]));
        self::assertEquals($message, S\MessagePayload::fromJson($message->toJson()));
        $wire = $message->toArray();
        $wire['metadata']->status = 'changed';
        self::assertSame('pending', $message->metadata->jsonSerialize()->status);
        self::assertStringContainsString('"numeric":{"0":"zero"}', $message->toJson());
    }

    public function testEditsAndCollectionsAreIsolated(): void
    {
        $blocks = [new S\SectionBlock('original')];
        $message = new S\MessagePayload('C1', blocks: $blocks);
        $blocks[] = new S\DividerBlock();
        self::assertCount(1, $message->blocks);
        $copy = $message->with(blocks: [new S\SectionBlock('changed')]);
        self::assertSame('original', $message->blocks[0]->text->text);
        self::assertSame('changed', $copy->blocks[0]->text->text);
        try {
            $message->with(channel: '');
            self::fail('bad edit accepted');
        } catch (S\ValidationError) {
        }
        self::assertSame('C1', $message->channel);
        try {
            $message->with(unknown: 1);
            self::fail('unknown field accepted');
        } catch (\InvalidArgumentException) {
        }
        try {
            $message->with(1);
            self::fail('positional change accepted');
        } catch (\InvalidArgumentException) {
        }
        $this->expectException(\Error::class);
        $message->blocks[] = new S\DividerBlock();
    }

    public function testNumericAndUnicodeBoundaries(): void
    {
        foreach ([PHP_INT_MIN, PHP_INT_MAX, 9007199254740993] as $n) {
            $value = new S\RawNumber($n, 'n');
            self::assertSame($n, S\RawNumber::fromJson($value->toJson())->value);
        }
        self::assertSame('1e-999', S\JsonObject::fromJson('{"string":"1e-999"}')->jsonSerialize()->string);
        self::assertSame(0.0, S\JsonObject::fromJson('{"n":0e-999}')->jsonSerialize()->n);
        self::assertSame('é', (new S\HeaderBlock('é'))->text->text);
        self::assertSame(150, preg_match_all('/./us', (new S\HeaderBlock(str_repeat("e\u{0301}", 75)))->text->text));
        self::assertSame('## []', S\PlainText::fromJson('{"type":"plain_text","text":"## []"}')->text);
    }

    public function testCycleIsRejectedWithoutMutation(): void
    {
        $object = new \stdClass();
        $object->self = $object;
        $this->expectException(S\ValidationError::class);
        new S\JsonObject($object);
    }

    public function testNativeHelperComposition(): void
    {
        $section = S\AccordionSection::create('Details', [new S\SectionBlock('Body')], expanded: true);
        self::assertFalse($section->defaultCollapsed);
        self::assertSame(S\ContainerWidth::Standard, $section->width);
        self::assertCount(1, S\Accordion::create([$section]));
        self::assertCount(1, (new S\HomeTabView(S\Accordion::create([$section])))->blocks);
        $blocks = [new S\SectionBlock('A'),new S\SectionBlock('B'),new S\SectionBlock('C')];
        self::assertSame($blocks, S\Paginator::create('p', $blocks));
        $first = S\Paginator::create('p', $blocks, pageSize: 1);
        self::assertSame('p.next', $first[2]->elements[0]->actionId);
        $middle = S\Paginator::create('p', $blocks, page: 2, pageSize: 1, blockId: 'controls');
        self::assertCount(2, $middle[2]->elements);
        self::assertSame('1', $middle[2]->elements[0]->value);
        self::assertSame('controls', $middle[2]->blockId);
        $last = S\Paginator::create('p', $blocks, page: 3, pageSize: 1, showPageIndicator: false);
        self::assertCount(2, $last);
        self::assertSame('p.previous', $last[1]->elements[0]->actionId);
        self::assertCount(3, S\Paginator::create('p', $blocks, pageSize: PHP_INT_MAX));
    }

    public function testPreviewWorkflowAndColors(): void
    {
        $block = new S\SectionBlock('🙂 ? #');
        foreach ([$block,[$block],new S\MessagePayload('C1', [$block]),new S\JsonObject(['blocks' => []])] as $payload) {
            $url = S\Builder::url($payload, 'T/test');
            self::assertStringStartsWith('https://app.slack.com/block-kit-builder/T%2Ftest#', $url);
            self::assertIsObject(json_decode(rawurldecode(explode('#', $url, 2)[1])));
        }
        self::assertStringStartsWith('https://app.slack.com/block-kit-builder/#', S\Builder::url([]));
        $workflow = S\Workflow::fromUrl('https://slack.com/shortcuts/Ft0/abc', [new S\InputParameter('name', 'value')]);
        self::assertCount(1, $workflow->trigger->customizableInputParameters);
        self::assertNull(S\Workflow::fromUrl('https://slack.com/shortcuts/Ft0/abc')->trigger->customizableInputParameters);
        self::assertSame('#aBcDeF', S\Color::hex('aBcDeF'));
        self::assertSame('#ffffff', S\Color::hex('#ffffff'));
        self::assertSame('#8800ff', (new S\Attachment([], color: S\Color::PURPLE))->color);
        self::assertSame('#abcdef', (new S\Attachment([], color: 'abcdef'))->color);
    }
}
