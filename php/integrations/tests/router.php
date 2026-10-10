<?php

declare(strict_types=1);

$directory = getenv('SLACKBLOCKS_MOCK_DIR');
$case = file_get_contents($directory . '/case');
$body = file_get_contents('php://input');
file_put_contents($directory . '/requests', json_encode(['path' => $_SERVER['REQUEST_URI'], 'headers' => getallheaders(), 'body' => $body], JSON_THROW_ON_ERROR) . "\n", FILE_APPEND);
$count = count(file($directory . '/requests'));
header('Content-Type: application/json');
if ($case === 'delay') {
    usleep(300_000);
}
if ($case === 'redirect') {
    header('Location: /redirect-target', true, 302);
    echo '{}';
    return;
}
if ($case === 'http') {
    http_response_code(503);
    echo '{"ok":false,"error":"unavailable"}';
    return;
}
if ($case === 'api') {
    echo '{"ok":false,"error":"channel_not_found"}';
    return;
}
if ($case === 'malformed') {
    echo 'not-json';
    return;
}
if ($case === 'limit' || $case === 'long-limit' || ($case === 'retry' && $count < 3)) {
    http_response_code(429);
    header('Retry-After: ' . ($case === 'long-limit' ? '60' : '0'));
    echo '{"ok":false,"error":"ratelimited"}';
    return;
}
echo '{"ok":true,"channel":"C123","ts":"123.456","message":{"text":"sent"}}';
