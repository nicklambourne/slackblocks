<?php

declare(strict_types=1);

namespace Slackblocks\Examples;

use JoliCode\Slack\ClientFactory;
use Psr\Http\Client\ClientInterface;
use Psr\Http\Message\ResponseInterface;
use Slackblocks\MessagePayload;
use Symfony\Component\HttpClient\{HttpClient, Psr18Client};

/** Runnable sending examples; these are not dependencies or transport APIs of the core library. */
final class Sending
{
    /**
     * Send via the community JoliCode SDK. JSON fields bypass generated Block Kit models.
     * An injected PSR-18 client must enforce its own timeouts and disable redirects.
     * @return array<string, mixed> The checked Slack response.
     */
    public static function jolicode(MessagePayload $message, string $token, ?ClientInterface $http = null): array
    {
        $http ??= new Psr18Client(HttpClient::create(['timeout' => 5, 'max_duration' => 5, 'max_redirects' => 0]));
        $client = ClientFactory::create($token, new RateLimitedClient($http));
        $parameters = $message->toArray();
        foreach (['blocks','attachments','metadata'] as $field) {
            if (array_key_exists($field, $parameters)) {
                $parameters[$field] = json_encode($parameters[$field], JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION);
            }
        }
        $response = $client->chatPostMessage($parameters, [], 'response');
        if (!$response instanceof ResponseInterface) {
            throw new \RuntimeException('Expected a raw SDK response');
        }
        return self::response($response->getStatusCode(), (string) $response->getBody());
    }

    /**
     * Send the complete JSON with native cURL; no SDK required. Redirects stay disabled.
     * @param string $endpoint Slack's endpoint; override only for a controlled test server.
     * @return array<string, mixed> The checked Slack response.
     */
    public static function curl(MessagePayload $message, string $token, string $endpoint = 'https://slack.com/api/chat.postMessage', int $timeoutMs = 5000): array
    {
        if ($timeoutMs < 1) {
            throw new \InvalidArgumentException('timeoutMs must be positive');
        }
        $body = $message->toJson();
        for ($attempt = 0; $attempt < 3; ++$attempt) {
            $retryAfter = null;
            $curl = curl_init($endpoint);
            if ($curl === false) {
                throw new \RuntimeException('Unable to initialize cURL');
            }
            curl_setopt_array($curl, [
                CURLOPT_POST => true, CURLOPT_POSTFIELDS => $body,
                CURLOPT_HTTPHEADER => ['Authorization: Bearer ' . $token, 'Content-Type: application/json'],
                CURLOPT_RETURNTRANSFER => true, CURLOPT_FOLLOWLOCATION => false,
                CURLOPT_PROTOCOLS => CURLPROTO_HTTPS | CURLPROTO_HTTP,
                CURLOPT_CONNECTTIMEOUT_MS => $timeoutMs, CURLOPT_TIMEOUT_MS => $timeoutMs,
                CURLOPT_HEADERFUNCTION => static function ($handle, string $line) use (&$retryAfter): int {
                    if (str_starts_with(strtolower($line), 'retry-after:')) {
                        $retryAfter = trim(substr($line, 12));
                    }
                    return strlen($line);
                },
            ]);
            $response = curl_exec($curl);
            $status = curl_getinfo($curl, CURLINFO_RESPONSE_CODE);
            $error = curl_error($curl);
            // CurlHandle closes when it leaves scope, including PHP 8.5 where curl_close is deprecated.
            unset($curl);
            if (!is_string($response)) {
                throw new \RuntimeException('Slack HTTP request failed: ' . $error);
            }
            if ($status !== 429) {
                return self::response($status, $response);
            }
            if ($attempt === 2 || ($retryAfter !== '0' && $retryAfter !== '1')) {
                throw new \RuntimeException('Slack rate limit: retry budget exhausted or Retry-After exceeds one second');
            }
            if ($retryAfter === '1') {
                usleep(1_000_000);
            }
        }
        throw new \LogicException('unreachable');
    }

    /** @return array<string, mixed> */
    private static function response(int $status, string $body): array
    {
        if ($status < 200 || $status >= 300) {
            throw new \RuntimeException('Slack HTTP status ' . $status);
        }
        $json = json_decode($body, true, flags: JSON_THROW_ON_ERROR);
        if (!is_array($json) || ($json['ok'] ?? null) !== true) {
            throw new \RuntimeException('Slack API error: ' . ($json['error'] ?? 'invalid response'));
        }
        return $json;
    }
}
