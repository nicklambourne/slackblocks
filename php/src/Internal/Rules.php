<?php

declare(strict_types=1);

namespace Slackblocks\Internal;

use Slackblocks as S;
use Slackblocks\ErrorCategory as C;

/** @internal Cross-field rules; contextual traversal follows modeled children only. */
final class Rules
{
    public static function numberBound(string|int|float|null $value, string $path): ?string
    {
        if (!is_float($value)) {
            return $value === null ? null : (string) $value;
        }
        if (!is_finite($value)) {
            Validator::fail(C::TypeMismatch, $path, 'expected a finite numeric bound');
        }
        // H uses a locale-independent decimal point. Find the fewest significant
        // digits that retain the native double, without reading or changing INI precision.
        for ($digits = 1; $digits <= 17; ++$digits) {
            $text = sprintf('%.*H', $digits, $value);
            if ((float) $text === $value) {
                break;
            }
        }
        if (!str_contains($text, 'E')) {
            return $text;
        }
        // Keep numeric convenience inputs in plain decimal form. Expansion is
        // bounded by the native double's exponent; caller-supplied strings never enter here.
        [$mantissa, $exponent] = explode('E', $text);
        $sign = str_starts_with($mantissa, '-') ? '-' : '';
        [$whole, $fraction] = array_pad(explode('.', ltrim($mantissa, '-')), 2, '');
        $coefficient = $whole . $fraction;
        $point = strlen($whole) + (int) $exponent;
        if ($point <= 0) {
            $text = '0.' . str_repeat('0', -$point) . $coefficient;
        } elseif ($point >= strlen($coefficient)) {
            $text = $coefficient . str_repeat('0', $point - strlen($coefficient));
        } else {
            $text = substr($coefficient, 0, $point) . '.' . substr($coefficient, $point);
        }
        return $sign . (str_contains($text, '.') ? rtrim(rtrim($text, '0'), '.') : $text);
    }

    public static function validate(S\Value $value, \stdClass $wire): void
    {
        $path = Codec::name($value);
        if ($value instanceof S\SlackIcon && !in_array($value->name, Schema::VOCABULARY['slack_icon_names'], true)) {
            Validator::fail(C::TypeMismatch, $path . '.name', 'unknown Slack icon');
        }
        if ($value instanceof S\ExternalMultiSelectElement && $value->focusOnLoad !== null) {
            Validator::fail(C::InvalidUsage, $path . '.focus_on_load', 'unsupported for external multi-select menus; omit this field');
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
        if ($value instanceof S\DatePickerElement && $value->initialDate !== null && preg_match('/\A[0-9]{4}-[0-9]{2}-[0-9]{2}\z/', $value->initialDate) !== 1) {
            Validator::fail(C::TypeMismatch, $path . '.initial_date', 'expected YYYY-MM-DD');
        }
        if ($value instanceof S\TimePickerElement && $value->initialTime !== null && preg_match('/\A([01][0-9]|2[0-3]):[0-5][0-9]\z/', $value->initialTime) !== 1) {
            Validator::fail(C::TypeMismatch, $path . '.initial_time', 'expected HH:mm in 24-hour time');
        }
        if ($value instanceof S\PlainTextInputElement && $value->minLength !== null && $value->maxLength !== null && $value->minLength > $value->maxLength) {
            Validator::fail(C::OutOfRange, $path . '.min_length', 'cannot exceed max_length');
        }
        if ($value instanceof S\NumberInputElement) {
            foreach (['minValue' => 'min_value', 'maxValue' => 'max_value'] as $property => $field) {
                $text = $value->$property;
                if ($text === null) {
                    continue;
                }
                if (preg_match('/\A[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?\z/', $text) !== 1) {
                    Validator::fail(C::TypeMismatch, $path . '.' . $field, 'expected a numeric string');
                }
            }
            if ($value->minValue !== null && $value->maxValue !== null && self::compareDecimals($value->minValue, $value->maxValue) > 0) {
                Validator::fail(C::OutOfRange, $path, 'minimum exceeds maximum');
            }
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
            if (self::chartCount($blocks) > Schema::LIMITS['message.data_visualizations.max_items']) {
                Validator::fail(C::LengthExceeded, $path, 'a message may contain at most two data visualization blocks');
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

    /** Compare finite decimal strings without float rounding or exponent expansion. */
    private static function compareDecimals(string $left, string $right): int
    {
        [$leftSign, $leftRank, $leftDigits] = self::decimalParts($left);
        [$rightSign, $rightRank, $rightDigits] = self::decimalParts($right);
        if ($leftSign !== $rightSign) {
            return $leftSign <=> $rightSign;
        }
        if ($leftSign === 0) {
            return 0;
        }
        $rank = self::compareIntegers($leftRank, $rightRank);
        $width = max(strlen($leftDigits), strlen($rightDigits));
        return $leftSign * ($rank !== 0 ? $rank : strcmp(str_pad($leftDigits, $width, '0'), str_pad($rightDigits, $width, '0')));
    }

    /** @return array{int, string, string} */
    private static function decimalParts(string $text): array
    {
        $parts = preg_split('/[eE]/', ltrim($text, '+-'));
        // The caller has already checked the complete decimal grammar.
        assert($parts !== false);
        [$integer, $fraction] = array_pad(explode('.', $parts[0]), 2, '');
        $digits = ltrim($integer . $fraction, '0');
        if ($digits === '') {
            return [0, '0', ''];
        }
        $rank = self::addIntegers($parts[1] ?? '0', (string) (strlen($digits) - strlen($fraction)));
        return [str_starts_with($text, '-') ? -1 : 1, $rank, rtrim($digits, '0')];
    }

    /** Signed decimal integer comparison; arguments are normalized. */
    private static function compareIntegers(string $left, string $right): int
    {
        $leftNegative = str_starts_with($left, '-');
        $rightNegative = str_starts_with($right, '-');
        if ($leftNegative !== $rightNegative) {
            return $leftNegative ? -1 : 1;
        }
        $left = ltrim($left, '-');
        $right = ltrim($right, '-');
        $order = (strlen($left) <=> strlen($right)) ?: strcmp($left, $right);
        return $leftNegative ? -$order : $order;
    }

    /** Add a decimal exponent and a character count without native integer overflow. */
    private static function addIntegers(string $left, string $right): string
    {
        $leftNegative = str_starts_with($left, '-');
        $rightNegative = str_starts_with($right, '-');
        $left = ltrim(ltrim($left, '+-'), '0');
        $right = ltrim(ltrim($right, '+-'), '0');
        if ($leftNegative !== $rightNegative && ((strlen($left) <=> strlen($right)) ?: strcmp($left, $right)) < 0) {
            [$left, $right] = [$right, $left];
            [$leftNegative, $rightNegative] = [$rightNegative, $leftNegative];
        }
        $add = $leftNegative === $rightNegative;
        $result = '';
        $carry = 0;
        for ($i = strlen($left) - 1, $j = strlen($right) - 1; $i >= 0 || $j >= 0; $i--, $j--) {
            $a = $i >= 0 ? (int) $left[$i] : 0;
            $b = $j >= 0 ? (int) $right[$j] : 0;
            $digit = $a + ($add ? $b : -$b) + $carry;
            $carry = $add ? intdiv($digit, 10) : ($digit < 0 ? -1 : 0);
            $result = (string) (($digit + 10) % 10) . $result;
        }
        if ($carry > 0) {
            $result = (string) $carry . $result;
        }
        $result = ltrim($result, '0');
        return $result === '' ? '0' : ($leftNegative ? '-' : '') . $result;
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
     * @return int
     */
    private static function chartCount(array $blocks): int
    {
        $count = 0;
        foreach ($blocks as $block) {
            if ($block instanceof S\DataVisualizationBlock) {
                $count++;
            }
            if ($block instanceof S\ContainerBlock) {
                $count += self::chartCount($block->childBlocks);
            }
        }
        return $count;
    }

    /**
     * @param list<S\Block> $blocks
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
