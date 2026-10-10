<?php

declare(strict_types=1);

use Slackblocks\{ButtonElement, MessagePayload, SectionBlock, InputBlock, PlainTextInputElement, TaskStatus, TaskCardBlock};

$section = new SectionBlock(text: 'Hello', accessory: new ButtonElement(text: 'Open'));
$message = new MessagePayload(channel: 'C1', blocks: [$section]);
$input = new InputBlock(label: 'Name', element: new PlainTextInputElement());
$task = new TaskCardBlock(taskId: '1', title: 'Task', status: TaskStatus::Complete);
echo $message->with(text: 'Updated')->toJson();

if ($section->text !== null) {
    echo $section->text->getText();
}
foreach ($message->blocks ?? [] as $block) {
    echo $block->toJson();
}
