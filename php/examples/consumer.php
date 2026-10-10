<?php

declare(strict_types=1);

// Run with an installed Composer autoloader path as the first argument.
require $argv[1] ?? __DIR__ . '/../vendor/autoload.php';

use Slackblocks\{Builder, ButtonElement, DataTableBlock, JsonObject, MessagePayload, PlanBlock, RawNumber, RawText, SectionBlock, TaskCardBlock, TaskStatus, Version};

$message = new MessagePayload(
    channel: 'C0123456789',
    text: 'Build results',
    blocks: [new SectionBlock(text: '*Build complete*', accessory: new ButtonElement(text: 'View', actionId: 'view'))],
    metadata: new JsonObject(['event_type' => 'build', 'event_payload' => (object) []]),
);
$edited = $message->with(text: 'Updated results');
$plan = new PlanBlock(title: 'Build', tasks: [new TaskCardBlock(taskId: 'compile', title: 'Compile', status: TaskStatus::Pending)]);
$table = new DataTableBlock(rows: [[new RawText('Count')], [new RawNumber(9007199254740993, 'Exact')]], caption: 'Counts');
if ($message->text !== 'Build results' || $edited->text !== 'Updated results' || str_contains($plan->toJson(), 'task_card') || !str_contains($table->toJson(), '9007199254740993')) {
    throw new RuntimeException('Installed consumer behavior differs');
}
if (MessagePayload::fromJson($message->toJson())->toJson() !== $message->toJson() || !str_starts_with(Builder::url($message), 'https://app.slack.com/block-kit-builder/')) {
    throw new RuntimeException('Installed serialization/preview failed');
}
echo 'Installed Slackblocks ' . Version::PACKAGE . ' (spec ' . Version::SPEC . ') on PHP ' . PHP_VERSION . "\n";
