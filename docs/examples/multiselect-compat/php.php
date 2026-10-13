<?php
// Supply the Composer autoloader path. No network operations.
declare(strict_types=1);
require $argv[1] ?? __DIR__ . '/vendor/autoload.php';
use Slackblocks as S;
$elements = [
    'static' => new S\StaticMultiSelectElement(actionId: 'choose', options: [new S\Option(text: 'One', value: 'one')]),
    'external' => new S\ExternalMultiSelectElement(actionId: 'choose', minQueryLength: 0),
    'users' => new S\UserMultiSelectElement(actionId: 'choose'),
    'conversations' => new S\ConversationMultiSelectElement(actionId: 'choose'),
    'channels' => new S\ChannelMultiSelectElement(actionId: 'choose'),
];
foreach ($elements as $family => $element) {
    foreach ([
        'actions' => new S\ActionsBlock(elements: [$element], blockId: 'choices'),
        'section' => new S\SectionBlock(text: 'Choose values', accessory: $element, blockId: 'choices'),
        'input' => new S\InputBlock(label: 'Choose values', element: $element, blockId: 'choices'),
    ] as $parent => $block) {
        $payload = $parent === 'input'
            ? new S\ModalView(title: 'Multi-select probe', blocks: [$block], submit: 'Save')
            : new S\MessagePayload(channel: 'C0123456', text: 'Choose values', blocks: [$block]);
        echo json_encode(['family' => $family, 'parent' => $parent,
            'surface' => $parent === 'input' ? 'modal' : 'message', 'payload' => $payload], JSON_THROW_ON_ERROR), "\n";
    }
}
