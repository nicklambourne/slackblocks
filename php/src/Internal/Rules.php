<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks as S;
use Slackblocks\ErrorCategory as C;

/** @internal Cross-field rules; contextual traversal follows modeled children only. */
final class Rules
{
    public static function validate(S\Value $value, \stdClass $wire): void
    {
        $path = Codec::name($value);
        if ($value instanceof S\SlackIcon && !in_array($value->name, Schema::VOCABULARY['slack_icon_names'], true)) {
            Validator::fail(C::TypeMismatch, $path . '.name', 'unknown Slack icon');
        }
        if ($value instanceof S\SectionBlock) {
            if ($value->text === null && ($value->fields === null || $value->fields === [])) {
                Validator::fail(C::MissingRequired, $path, 'expected text or fields');
            }
            if ($value->fields === []) {
                Validator::fail(C::MissingRequired, $path . '.fields', 'fields cannot be empty');
            }
            foreach ($value->fields ?? [] as $i => $field) {
                if (Json::length($field->getText(), $path) > Schema::LIMITS['section.fields.item_max_length']) {
                    Validator::fail(C::LengthExceeded, $path . '.fields[' . $i . '].text', 'section field exceeds limit');
                }
            }
        }
        if ($value instanceof S\StaticSelectElement || $value instanceof S\StaticMultiSelectElement) {
            if ($value->options !== null && $value->optionGroups !== null) {
                Validator::fail(C::MutuallyExclusive, $path, 'options and option_groups cannot coexist');
            }
        }
        if ($value instanceof S\CardBlock) {
            if ($value->heroImage === null && $value->title === null && $value->actions === null && $value->body === null) {
                Validator::fail(C::MissingRequired, $path, 'card requires content');
            }
            if ($value->icon !== null && $value->slackIcon !== null) {
                Validator::fail(C::MutuallyExclusive, $path, 'image and Slack icons cannot coexist');
            }
        }
        if ($value instanceof S\ContainerBlock) {
            if ($value->title === null && $value->richTextTitle === null) {
                Validator::fail(C::MissingRequired, $path, 'container requires a title');
            }
            if ($value->defaultCollapsed === true && $value->isCollapsible !== true) {
                Validator::fail(C::InvalidUsage, $path . '.default_collapsed', 'requires is_collapsible');
            }
            if ($value->hasHeaderDivider === true && $value->isCollapsible === true) {
                Validator::fail(C::InvalidUsage, $path . '.has_header_divider', 'requires a non-collapsible container');
            }
        }
        if ($value instanceof S\ImageBlock || $value instanceof S\ImageElement) {
            if ($value->imageUrl === null && $value->slackFile === null) {
                Validator::fail(C::MissingRequired, $path, 'expected an image source');
            }
            if ($value->imageUrl !== null && $value->slackFile !== null) {
                Validator::fail(C::MutuallyExclusive, $path, 'expected one image source');
            }
        }
        if ($value instanceof S\SlackFile) {
            if (($value->id === null) === ($value->url === null)) {
                Validator::fail(C::MutuallyExclusive, $path, 'expected exactly one of id or url');
            }
            if ($value->id !== null && preg_match('/\AF[A-Z0-9]{8,}\z/', $value->id) !== 1) {
                Validator::fail(C::TypeMismatch, $path . '.id', 'invalid Slack file ID');
            }
        }
        if ($value instanceof S\ConversationFilter) {
            if ($value->include === null && $value->excludeExternalSharedChannels === null && $value->excludeBotUsers === null) {
                Validator::fail(C::MissingRequired, $path, 'expected a filter field');
            }
            foreach ($value->include ?? [] as $i => $item) {
                if (!in_array($item, ['im', 'mpim', 'private', 'public'], true)) {
                    Validator::fail(C::TypeMismatch, $path . '.include[' . $i . ']', 'unknown conversation kind');
                }
            }
        }
        if ($value instanceof S\DispatchActionConfiguration && $value->triggerActionsOn === null) {
            Validator::fail(C::MissingRequired, $path . '.trigger_actions_on', 'required field is missing');
        }
        if ($value instanceof S\NumberInputElement && $value->minValue !== null && $value->maxValue !== null && $value->minValue > $value->maxValue) {
            Validator::fail(C::OutOfRange, $path, 'minimum exceeds maximum');
        }
        if ($value instanceof S\TableBlock || $value instanceof S\DataTableBlock) {
            self::table($value, $path);
        }
        if ($value instanceof S\AxisConfig) {
            foreach ($value->categories ?? [] as $i => $category) {
                Validator::limits($category, 'data_visualization.category_label', $path . '.categories[' . $i . ']');
            }
            if (count(array_unique($value->categories)) !== count($value->categories)) {
                Validator::fail(C::InvalidUsage, $path . '.categories', 'duplicate category');
            }
        }
        if ($value instanceof S\ChartSegment && $value->value <= Schema::LIMITS['data_visualization.segment.value.exclusive_min']) {
            Validator::fail(C::OutOfRange, $path . '.value', 'expected a positive segment value');
        }
        if ($value instanceof S\LineChart || $value instanceof S\BarChart || $value instanceof S\AreaChart) {
            self::chart($value, $path);
        }
        if ($value instanceof S\PlanBlock) {
            $ids = array_map(static fn(S\TaskCardBlock $task): string => $task->taskId, $value->tasks);
            if (count(array_unique($ids)) !== count($ids)) {
                Validator::fail(C::InvalidUsage, $path . '.tasks', 'duplicate task ID');
            }
        }
        if ($value instanceof S\Attachment) {
            if ($value->color !== null && !in_array($value->color, ['good', 'warning', 'danger'], true) && preg_match('/\A#[0-9a-fA-F]{6}\z/', $value->color) !== 1) {
                Validator::fail(C::TypeMismatch, $path . '.color', 'expected semantic color or six-digit hex color');
            }
            self::surface($value->blocks, 'message', $path . '.blocks');
        }
        if ($value instanceof S\MessagePayload || $value instanceof S\MessageResponse || $value instanceof S\WebhookMessage) {
            self::surface($value->blocks ?? [], 'message', $path . '.blocks');
            $blocks = $value->blocks ?? [];
            foreach ($value->attachments ?? [] as $attachment) {
                array_push($blocks, ...$attachment->blocks);
            }
            [$markdown, $table] = self::totals($blocks);
            if ($markdown > Schema::LIMITS['markdown.total_text.max_length']) {
                Validator::fail(C::LengthExceeded, $path, 'message markdown total exceeded');
            }
            if ($table > Schema::LIMITS['data_table.total_content.max_length']) {
                Validator::fail(C::LengthExceeded, $path, 'message data table total exceeded');
            }
        }
        if ($value instanceof S\ModalView || $value instanceof S\HomeTabView) {
            self::surface($value->blocks, $value instanceof S\ModalView ? 'modal' : 'home', $path . '.blocks');
            if ($value instanceof S\ModalView && $value->submit === null) {
                foreach ($value->blocks as $block) {
                    if ($block instanceof S\InputBlock) {
                        Validator::fail(C::MissingRequired, $path . '.submit', 'required when modal contains an input');
                    }
                }
            }
        }
    }

    /** @param list<S\Block> $blocks */
    private static function surface(array $blocks, string $surface, string $path): void
    {
        foreach ($blocks as $i => $block) {
            assert($block instanceof S\Value);
            if (!in_array(Schema::TYPES[Codec::name($block)]['wireType'], Schema::VOCABULARY['surface_block_types'][$surface], true)) {
                Validator::fail(C::TypeMismatch, $path . '[' . $i . '].type', 'unsupported block on ' . $surface . ' surface');
            }
        }
    }

    /** @param list<S\Block> $blocks
     * @return array{int, int}
     */
    private static function totals(array $blocks): array
    {
        $markdown = $table = 0;
        foreach ($blocks as $block) {
            if ($block instanceof S\MarkdownBlock) {
                $markdown += Json::length($block->text, 'text');
            }
            if ($block instanceof S\DataTableBlock) {
                $table += self::tableCharacters($block);
            }
            if ($block instanceof S\ContainerBlock) {
                [$a, $b] = self::totals($block->childBlocks);
                $markdown += $a;
                $table += $b;
            }
        }
        return [$markdown, $table];
    }

    private static function table(S\TableBlock|S\DataTableBlock $value, string $path): void
    {
        $data = $value instanceof S\DataTableBlock;
        foreach ($value->rows as $i => $row) {
            $p = $path . '.rows[' . $i . ']';
            if (count($row) > Schema::LIMITS[$data ? 'data_table.columns.max_items' : 'table.columns.max_items'] || ($data && $row === [])) {
                Validator::fail(C::LengthExceeded, $p, 'invalid column count');
            }
            if ($data && count($row) !== count($value->rows[0])) {
                Validator::fail(C::InvalidUsage, $p, 'data table rows must be rectangular');
            }
            if ($data) {
                foreach ($row as $j => $cell) {
                    if ($i === 0 && $cell instanceof S\RichTextBlock) {
                        Validator::fail(C::TypeMismatch, $p . '[' . $j . ']', 'header cells must be raw');
                    }
                    if (($cell instanceof S\RawText || $cell instanceof S\RawNumber) && $cell->text === '') {
                        Validator::fail(C::LengthExceeded, $p . '[' . $j . '].text', 'empty data table cell');
                    }
                }
            }
        }
        if ($value instanceof S\DataTableBlock) {
            if (array_key_exists('column_settings', $value->extensions?->toArray() ?? [])) {
                Validator::fail(C::InvalidUsage, $path . '.column_settings', 'unsupported data table field');
            }
            if (self::tableCharacters($value) > Schema::LIMITS['data_table.content.max_length']) {
                Validator::fail(C::LengthExceeded, $path . '.rows', 'table text total exceeded');
            }
        }
    }

    private static function tableCharacters(S\DataTableBlock $value): int
    {
        $total = 0;
        foreach ($value->rows as $row) {
            foreach ($row as $cell) {
                if ($cell instanceof S\RawText || $cell instanceof S\RawNumber) {
                    $total += Json::length($cell->text, 'text');
                }
                if ($cell instanceof S\RichTextBlock) {
                    foreach ($cell->elements as $element) {
                        $sections = $element instanceof S\RichTextList ? $element->elements : [$element];
                        foreach ($sections as $section) {
                            foreach (($section instanceof S\RichTextSection || $section instanceof S\RichTextQuote || $section instanceof S\RichTextCodeBlock) ? $section->elements : [] as $inline) {
                                if ($inline instanceof S\RichTextText || $inline instanceof S\RichTextLink) {
                                    $total += Json::length($inline->text ?? '', 'text');
                                }
                            }
                        }
                    }
                }
            }
        }
        return $total;
    }

    private static function chart(S\LineChart|S\BarChart|S\AreaChart $value, string $path): void
    {
        $categories = $value->axisConfig->categories;
        $names = [];
        foreach ($value->series as $i => $series) {
            if (in_array($series->name, $names, true)) {
                Validator::fail(C::InvalidUsage, $path . '.series', 'duplicate series name');
            }
            $names[] = $series->name;
            $labels = array_map(static fn(S\DataPoint $point): string => $point->label, $series->data);
            if (count($labels) !== count($categories) || count(array_unique($labels)) !== count($labels) || array_diff($labels, $categories) !== []) {
                Validator::fail(C::InvalidUsage, $path . '.series[' . $i . '].data', 'points must cover every category exactly once');
            }
        }
    }
}
